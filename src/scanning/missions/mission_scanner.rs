use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::{Result, Context};
use indicatif::{ProgressBar, ParallelProgressIterator};
use log::{debug, info, warn};
use pbo_tools::core::api::{PboApi, PboApiOps};
use walkdir::WalkDir;
use rayon::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;

use crate::scanning::types::{ScanDatabase, PboScanResult, SkipReason};
use super::mission_extractor::MissionExtractor;

#[derive(Debug, Serialize, Deserialize)]
pub struct MissionExtractionResult {
    pub mission_name: String,
    pub pbo_path: PathBuf,
    pub extracted_path: PathBuf,
    pub sqm_file: Option<PathBuf>,
    pub sqf_files: Vec<PathBuf>,
    pub cpp_files: Vec<PathBuf>,
}

pub struct MissionScanner<'a> {
    input_dir: &'a Path,
    cache_dir: &'a Path,
    threads: usize,
    db: Arc<Mutex<ScanDatabase>>,
}

impl<'a> MissionScanner<'a> {
    pub fn new(input_dir: &'a Path, cache_dir: &'a Path, threads: usize) -> Self {
        // Create database path
        let db_path = cache_dir.join("mission_scan_db.json");
        let db = Arc::new(Mutex::new(ScanDatabase::load_or_create(&db_path).unwrap_or_else(|_| {
            debug!("Creating new mission scan database");
            ScanDatabase::new()
        })));
        
        Self {
            input_dir,
            cache_dir,
            threads,
            db,
        }
    }
    
    pub async fn scan_and_extract(&self) -> Result<Vec<MissionExtractionResult>> {
        // Create cache directory if it doesn't exist
        std::fs::create_dir_all(self.cache_dir)
            .context("Failed to create cache directory")?;
        
        // Collect all mission PBO files
        let mission_files = self.collect_mission_files()
            .context("Failed to collect mission files")?;
        
        if mission_files.is_empty() {
            warn!("No mission PBO files found in {}", self.input_dir.display());
            return Ok(Vec::new());
        }
        
        info!("Found {} mission PBO files to process", mission_files.len());
        
        // Create progress bar
        let progress = indicatif::ProgressBar::new(mission_files.len() as u64);
        progress.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
        );
        progress.set_message("Scanning mission PBOs...");
        
        // Scan PBOs to determine which need extraction
        let scan_results = self.scan_pbo_files(&mission_files, progress.clone())
            .context("Failed to scan PBO files")?;
        
        if scan_results.is_empty() {
            info!("No mission PBOs need extraction (all cached)");
            
            // Return cached results
            let cached_results = self.collect_cached_results(&mission_files)
                .context("Failed to collect cached results")?;
            
            return Ok(cached_results);
        }
        
        info!("{} mission PBOs need extraction", scan_results.len());
        
        // Extract PBOs - only extract files needed for analysis, not the entire mission
        let extractor = MissionExtractor::new(self.cache_dir, self.threads);
        let extraction_results = extractor.extract_missions(&scan_results, progress)
            .context("Failed to extract mission PBOs")?;
        
        // Save database
        let db_path = self.cache_dir.join("mission_scan_db.json");
        {
            let db = self.db.lock().unwrap();
            db.save(&db_path)?;
        }
        
        Ok(extraction_results)
    }
    
    fn collect_mission_files(&self) -> Result<Vec<PathBuf>> {
        let mut mission_files = Vec::new();
        
        for entry in WalkDir::new(self.input_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "pbo" {
                    // Check if the PBO is likely a mission (contains mission.sqm)
                    let api = PboApi::builder()
                        .with_timeout(10)
                        .build();
                    
                    match api.list_contents(path) {
                        Ok(result) => {
                            if result.is_success() {
                                let files = result.get_file_list();
                                if files.iter().any(|f| f.ends_with("mission.sqm")) {
                                    mission_files.push(path.to_owned());
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to list contents of {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
        
        Ok(mission_files)
    }
    
    fn scan_pbo_files(&self, mission_files: &[PathBuf], progress: ProgressBar) -> Result<Vec<PboScanResult>> {
        progress.set_length(mission_files.len() as u64);
        
        let results: Vec<PboScanResult> = mission_files.par_iter()
            .progress_with(progress)
            .filter_map(|path| {
                // Calculate hash for the PBO
                let hash: String = match crate::scanning::pbo::calculate_file_hash(path) {
                    Ok(h) => h,
                    Err(e) => {
                        warn!("Failed to calculate hash for {}: {}", path.display(), e);
                        return None;
                    }
                };
                
                // Check if we've seen this PBO before
                let needs_processing = {
                    let db = self.db.lock().unwrap();
                    match db.get_pbo_info(path) {
                        Some(info) if info.hash == hash && !info.failed => {
                            // Check if the extracted directory actually exists and has files
                            let missions_dir = self.cache_dir.join("missions");
                            let extracted_path = missions_dir.join(path.file_name().unwrap())
                                .with_extension("");
                            
                            if extracted_path.exists() {
                                // Check if the directory has any files
                                let has_files = walkdir::WalkDir::new(&extracted_path)
                                    .max_depth(3)
                                    .into_iter()
                                    .filter_map(|e| e.ok())
                                    .any(|e| e.file_type().is_file());
                                
                                if has_files {
                                    debug!("Skipping unchanged mission PBO: {}", path.display());
                                    false
                                } else {
                                    debug!("Extraction directory exists but is empty for {}, re-extracting", path.display());
                                    true
                                }
                            } else {
                                debug!("Extraction directory doesn't exist for {}, re-extracting", path.display());
                                true
                            }
                        }
                        _ => true
                    }
                };
                
                if !needs_processing {
                    return None;
                }
                
                // If we need to process this PBO, delete the target directory if it exists
                let missions_dir = self.cache_dir.join("missions");
                let extracted_path = missions_dir.join(path.file_name().unwrap())
                    .with_extension("");
                
                if extracted_path.exists() {
                    debug!("Removing existing extraction directory for {}", path.display());
                    if let Err(e) = std::fs::remove_dir_all(&extracted_path) {
                        warn!("Failed to remove existing extraction directory {}: {}", extracted_path.display(), e);
                        // Continue anyway, the extraction might still work
                    }
                }
                
                // List PBO contents to check for mission files
                let api = PboApi::builder()
                    .with_timeout(30)
                    .with_config(pbo_tools::core::config::PboConfig::default())
                    .build();
                
                match api.list_contents(path) {
                    Ok(result) => {
                        if result.is_success() {
                            let files = result.get_file_list();
                            
                            // Check for mission.sqm file (case insensitive)
                            let has_mission_sqm = files.iter().any(|f| {
                                let file_name = Path::new(f).file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("");
                                file_name.to_lowercase() == "mission.sqm"
                            });
                            
                            // Also collect SQF files and other mission-related files
                            let mission_files: Vec<String> = files.into_iter()
                                .filter(|f| {
                                    let ext = Path::new(f)
                                        .extension()
                                        .and_then(|e| e.to_str())
                                        .unwrap_or("");
                                    ext.eq_ignore_ascii_case("sqm") || 
                                    ext.eq_ignore_ascii_case("sqf") ||
                                    ext.eq_ignore_ascii_case("hpp") ||
                                    ext.eq_ignore_ascii_case("cpp") ||
                                    ext.eq_ignore_ascii_case("fsm")
                                })
                                .collect();
                            
                            if !has_mission_sqm {
                                debug!("PBO doesn't contain mission.sqm: {}", path.display());
                                let mut db = self.db.lock().unwrap();
                                db.update_pbo_with_reason(
                                    path,
                                    &hash,
                                    false,
                                    SkipReason::NoMatchingFiles
                                );
                                None
                            } else if mission_files.is_empty() {
                                debug!("PBO doesn't contain any mission files: {}", path.display());
                                let mut db = self.db.lock().unwrap();
                                db.update_pbo_with_reason(
                                    path,
                                    &hash,
                                    false,
                                    SkipReason::NoMatchingFiles
                                );
                                None
                            } else {
                                debug!("Found {} mission files in {}", mission_files.len(), path.display());
                                Some(PboScanResult {
                                    path: path.clone(),
                                    expected_files: mission_files,
                                    hash,
                                })
                            }
                        } else {
                            warn!("Failed to list contents of {}: {}", path.display(), result);
                            let mut db = self.db.lock().unwrap();
                            db.update_pbo_with_reason(
                                path,
                                &hash,
                                true,
                                SkipReason::InvalidFormat
                            );
                            None
                        }
                    }
                    Err(e) => {
                        warn!("Error listing contents of {}: {}", path.display(), e);
                        let mut db = self.db.lock().unwrap();
                        db.update_pbo_with_reason(path, &hash, true, SkipReason::Failed);
                        None
                    }
                }
            })
            .collect();
        
        Ok(results)
    }
    
    fn collect_cached_results(&self, mission_files: &[PathBuf]) -> Result<Vec<MissionExtractionResult>> {
        let mut results = Vec::new();
        let missions_dir = self.cache_dir.join("missions");
        
        for path in mission_files {
            // Get the relative path for output
            let rel_path = path.strip_prefix(self.input_dir)
                .context(format!("Failed to strip prefix from {}", path.display()))?;
            
            let extracted_path = missions_dir.join(path.file_name().unwrap()).with_extension("");
            
            if !extracted_path.exists() {
                continue;
            }
            
            // Find mission.sqm file
            let sqm_file = WalkDir::new(&extracted_path)
                .max_depth(2) // Mission.sqm should be at the root or one level down
                .into_iter()
                .filter_map(|e| e.ok())
                .find(|e| {
                    e.file_name().to_string_lossy().eq_ignore_ascii_case("mission.sqm")
                })
                .map(|e| e.path().to_owned());
            
            // Find SQF files
            let sqf_files: Vec<PathBuf> = WalkDir::new(&extracted_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| ext.eq_ignore_ascii_case("sqf"))
                        .unwrap_or(false)
                })
                .map(|e| e.path().to_owned())
                .collect();
            
            // Find CPP/HPP files
            let cpp_files: Vec<PathBuf> = WalkDir::new(&extracted_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let ext = e.path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or("");
                    ext.eq_ignore_ascii_case("cpp") || ext.eq_ignore_ascii_case("hpp")
                })
                .map(|e| e.path().to_owned())
                .collect();
            
            let mission_name = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            results.push(MissionExtractionResult {
                mission_name,
                pbo_path: path.clone(),
                extracted_path,
                sqm_file,
                sqf_files,
                cpp_files,
            });
        }
        
        Ok(results)
    }
}