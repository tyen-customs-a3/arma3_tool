use anyhow::Result;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use walkdir::WalkDir;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::panic::{self, AssertUnwindSafe};
use std::time::Duration;
use std::thread;
use crossbeam_channel;
use std::sync::Arc;

use parser_advanced::{AdvancedProjectParser};
use crate::models::{WeaponInfo, MagazineWellInfo};
use super::{WeaponExtractor, MagazineExtractor};

/// Convert Windows UNC path to clean display format
fn clean_path_display(path: &Path) -> String {
    let path_str = path.to_string_lossy();
    if path_str.starts_with(r"\\?\") {
        path_str.strip_prefix(r"\\?\").unwrap_or(&path_str).to_string()
    } else {
        path_str.to_string()
    }
}

/// Result of processing a single file
#[derive(Debug)]
pub struct FileProcessingResult {
    pub weapons: Vec<WeaponInfo>,
    pub magazine_wells: HashMap<String, MagazineWellInfo>,
}

/// Extract mod source from CfgPatches
fn extract_mod_source_from_patches(game_classes: &[parser_advanced::GameClass]) -> Option<String> {
    for class in game_classes {
        if class.name == "CfgPatches" {
            // CfgPatches contains patch classes, each representing a mod
            for prop in &class.properties {
                if let parser_advanced::PropertyValue::Class(_) = prop.value {
                    // The property name is the mod name
                    return Some(prop.name.clone());
                }
            }
        }
    }
    None
}

/// Handles parallel file processing with timeout protection
pub struct FileProcessor {
    timeout_seconds: u64,
    parser: Arc<AdvancedProjectParser>,
    project_root_path: PathBuf, // Added
}

impl FileProcessor {
    pub fn new(project_root: &Path, _timeout_seconds: u64) -> Result<Self> {
        const DEFAULT_TIMEOUT: u64 = 10; // Enforce consistent 10 second timeout
        log::info!("Initializing FileProcessor for project root: {}", clean_path_display(project_root));
        let parser_instance = AdvancedProjectParser::new(project_root, None)
            .map_err(|e| anyhow::anyhow!("Failed to create AdvancedProjectParser for root {}: {}", clean_path_display(project_root), e))?;
        Ok(Self {
            timeout_seconds: DEFAULT_TIMEOUT,
            parser: Arc::new(parser_instance),
            project_root_path: project_root.to_path_buf(),
        })
    }

    /// Process all files in a directory
    pub fn process_directory(&self, target_dir_to_scan: &Path) -> Result<Vec<FileProcessingResult>> {
        let files_processed = AtomicUsize::new(0);
        let parser_project_root = &self.project_root_path;

        log::info!("Starting directory scan: {}", clean_path_display(target_dir_to_scan));

        if target_dir_to_scan != parser_project_root {
            log::warn!(
                "target_dir_to_scan ({}) differs from parser's project_root ({}). Paths might not be correctly relativized for parsing.",
                clean_path_display(target_dir_to_scan),
                clean_path_display(parser_project_root)
            );
        }

        log::info!("Collecting files to process...");
        let results: Vec<FileProcessingResult> = WalkDir::new(target_dir_to_scan)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| {
                let path = entry.path();
                path.is_file() && matches!(
                    path.extension().and_then(|s| s.to_str()).unwrap_or(""),
                    "cpp" | "hpp" | "h"
                )
            })
            .collect::<Vec<_>>()
            .par_iter()
            .filter_map(|entry| {
                let count = files_processed.fetch_add(1, Ordering::Relaxed) + 1;
                let absolute_path = entry.path();
                
                log::info!("Processing file {} ({}/total)", clean_path_display(absolute_path), count);
                
                let result = match self.process_file_with_timeout(absolute_path) {
                    Ok(result) => {
                        if result.weapons.is_empty() && result.magazine_wells.is_empty() {
                            log::debug!("No weapons or magazine wells found in {}", clean_path_display(absolute_path));
                        } else {
                            log::info!("Found {} weapons and {} magazine wells in {}",
                                result.weapons.len(),
                                result.magazine_wells.len(),
                                clean_path_display(absolute_path));
                        }
                        Some(result)
                    },
                    Err(e) => {
                        log::warn!("Skipping file {}: {}", clean_path_display(absolute_path), e);
                        None
                    }
                };
                
                log::debug!("Completed processing of file {}", clean_path_display(absolute_path));
                result
            })
            .filter(|result| !result.weapons.is_empty() || !result.magazine_wells.is_empty())
            .collect();

        Ok(results)
    }

    /// Process a single file with timeout protection
    fn process_file_with_timeout(&self, absolute_file_path: &Path) -> Result<FileProcessingResult> {
        let timeout = Duration::from_secs(self.timeout_seconds);
        let (sender, receiver) = crossbeam_channel::bounded(1);
        
        let file_to_process = absolute_file_path.to_path_buf();
        let project_root = &self.project_root_path;

        let relative_file_path = match file_to_process.strip_prefix(project_root) {
            Ok(p) => p.to_path_buf(),
            Err(_) => {
                log::error!("File {} is not within the project root {}. Cannot process.", 
                           clean_path_display(&file_to_process), clean_path_display(project_root));
                return Err(anyhow::anyhow!("File {} is not within the project root {}", 
                                         clean_path_display(&file_to_process), clean_path_display(project_root)));
            }
        };

        let parser_clone = Arc::clone(&self.parser);
        let original_path_for_extraction = file_to_process.clone();
        let original_path_for_logging = file_to_process.clone();

        let worker_thread = thread::spawn(move || {
            let result = panic::catch_unwind(AssertUnwindSafe(move || {
                Self::parse_and_extract(&parser_clone, &relative_file_path, &original_path_for_extraction)
            }));
            
            let processing_result = match result {
                Ok(parse_result) => parse_result,
                Err(_) => {
                    log::warn!("Parser panicked while processing {}, skipping file", clean_path_display(&original_path_for_logging));
                    Ok(FileProcessingResult {
                        weapons: Vec::new(),
                        magazine_wells: HashMap::new(),
                    })
                }
            };
            let _ = sender.send(processing_result);
        });
        
        match receiver.recv_timeout(timeout) {
            Ok(result) => {
                let _ = worker_thread.join();
                result
            }
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => {
                log::warn!("File processing timed out after {}s: {}", self.timeout_seconds, clean_path_display(&file_to_process));
                Ok(FileProcessingResult { weapons: Vec::new(), magazine_wells: HashMap::new() })
            }
            Err(crossbeam_channel::RecvTimeoutError::Disconnected) => {
                let _ = worker_thread.join();
                log::error!("Worker thread disconnected for {}", clean_path_display(&file_to_process));
                Err(anyhow::anyhow!("Worker thread disconnected for {}", clean_path_display(&file_to_process)))
            }
        }
    }

    // New method containing the core parsing and extraction logic
    fn parse_and_extract(
        parser: &Arc<AdvancedProjectParser>, // The shared parser instance
        relative_file_path: &Path,      // Path relative to project_root for the parser
        original_absolute_path: &Path   // Original absolute path for context (e.g., for extractors)
    ) -> Result<FileProcessingResult> {
        log::debug!("Parsing relative path: {} (from original: {})",
                   relative_file_path.display(), clean_path_display(original_absolute_path));
        
        let (game_classes, _parse_warnings) = parser.parse_file(relative_file_path) // Use the shared parser and relative path
            .map_err(|e| anyhow::anyhow!("Parser error for {}: {}", relative_file_path.display(), e))?;
            
        // Extract mod source from CfgPatches
        let mod_source = extract_mod_source_from_patches(&game_classes);
        
        let mut weapons = Vec::new();
        let mut magazine_wells = HashMap::new();

        for class in &game_classes {
            if class.name == "CfgWeapons" {
                for prop in &class.properties {
                    if let parser_advanced::PropertyValue::Class(ref weapon_class_value) = prop.value {
                         if let Some(mut weapon_info) = WeaponExtractor::extract_weapon_info_static(weapon_class_value, original_absolute_path) {
                            weapon_info.mod_source = mod_source.clone();
                            weapons.push(weapon_info);
                        }
                    }
                }
            } else {
                for prop in &class.properties {
                    if prop.name == "CfgWeapons" {
                        if let parser_advanced::PropertyValue::Class(ref cfg_weapons_class) = prop.value {
                            for inner_prop in &cfg_weapons_class.properties {
                                if let parser_advanced::PropertyValue::Class(ref weapon_class_value) = inner_prop.value {
                                    if let Some(mut weapon_info) = WeaponExtractor::extract_weapon_info_static(weapon_class_value, original_absolute_path) {
                                        weapon_info.mod_source = mod_source.clone();
                                        weapons.push(weapon_info);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if class.name == "CfgMagazineWells" {
                 MagazineExtractor::extract_magazine_wells_static(class, original_absolute_path, &mod_source, &mut magazine_wells);
            } else {
                for prop in &class.properties {
                    if prop.name == "CfgMagazineWells" {
                         if let parser_advanced::PropertyValue::Class(ref cfg_magazine_wells_class) = prop.value {
                            MagazineExtractor::extract_magazine_wells_static(cfg_magazine_wells_class, original_absolute_path, &mod_source, &mut magazine_wells);
                        }
                    }
                }
            }
        }

        Ok(FileProcessingResult {
            weapons,
            magazine_wells,
        })
    }
}
