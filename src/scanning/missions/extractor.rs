use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use indicatif::ProgressBar;
use log::{info, warn, debug};
use rayon::prelude::*;
use pbo_tools::{
    core::api::{PboApi, PboApiOps},
    extract::ExtractOptions,
};
use std::collections::HashSet;

use crate::scanning::types::PboScanResult;
use super::scanner::MissionExtractionResult;

pub struct MissionExtractor<'a> {
    cache_dir: &'a Path,
    threads: usize,
}

impl<'a> MissionExtractor<'a> {
    pub fn new(cache_dir: &'a Path, threads: usize) -> Self {
        Self {
            cache_dir,
            threads,
        }
    }
    
    pub fn extract_missions(
        &self,
        scan_results: &[PboScanResult],
        progress: ProgressBar,
    ) -> Result<Vec<MissionExtractionResult>> {
        // Reset progress bar for extraction phase
        progress.set_length(scan_results.len() as u64);
        progress.set_message("Extracting mission PBOs...");
        progress.reset();
        
        // Process in parallel with thread limit
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build()
            .context("Failed to build thread pool")?;
        
        let results = pool.install(|| {
            scan_results.par_iter()
                .map(|result| self.extract_single_mission(result))
                .inspect(|_| progress.inc(1))
                .filter_map(|r| r.ok())
                .collect::<Vec<_>>()
        });
        
        progress.finish_with_message("Mission extraction complete");
        
        Ok(results)
    }
    
    fn extract_single_mission(&self, scan_result: &PboScanResult) -> Result<MissionExtractionResult> {
        // Create output directory path (strip .pbo extension)
        let output_dir = self.cache_dir.join(scan_result.path.file_name().unwrap())
            .with_extension("");
        
        // Delete the output directory if it exists to avoid extraction errors
        if output_dir.exists() {
            debug!("Removing existing extraction directory for {}", scan_result.path.display());
            if let Err(e) = std::fs::remove_dir_all(&output_dir) {
                warn!("Failed to remove existing extraction directory {}: {}", output_dir.display(), e);
                // Continue anyway, the extraction might still work
            }
        }
        
        // Create output directory
        fs::create_dir_all(&output_dir)
            .context(format!("Failed to create output directory: {}", output_dir.display()))?;
        
        // Configure PBO API
        let api = PboApi::builder()
            .with_timeout(60) // Longer timeout for mission PBOs
            .with_config(pbo_tools::core::config::PboConfig::default()) // Add default config with binary mappings
            .build();
        
        // Extract only analysis-relevant files (.sqf, .hpp, .sqm, .cpp, .ext, .fsm)
        // We're only extracting files needed for analysis, not duplicating the entire mission
        let options = ExtractOptions {
            no_pause: true,
            warnings_as_errors: false,
            file_filter: Some("*.{sqm,sqf,hpp,cpp,ext,fsm}".to_string()), // Only extract files needed for analysis
            verbose: true, // Enable verbose output for better debugging
            ..Default::default()
        };
        
        // Get initial file count to verify extraction worked
        let initial_file_count = count_files_in_directory(&output_dir)?;
        
        // Extract the PBO
        let result = api.extract_with_options(&scan_result.path, &output_dir, options)
            .context(format!("Failed to extract PBO: {}", scan_result.path.display()))?;
        
        if !result.is_success() {
            return Err(anyhow::anyhow!(
                "Extraction failed for {}: {}", 
                scan_result.path.display(), 
                result
            ));
        }
        
        // Verify files were actually extracted by checking file count
        let final_file_count = count_files_in_directory(&output_dir)?;
        let extracted_files = final_file_count - initial_file_count;
        
        if extracted_files == 0 {
            // Try again with same file filter but more permissive options
            warn!("No files extracted from {}. Trying again with adjusted options.", scan_result.path.display());
            
            let permissive_options = ExtractOptions {
                no_pause: true,
                warnings_as_errors: false,
                file_filter: Some("*.{sqm,sqf,hpp,cpp,ext,fsm}".to_string()), // Same extension update here
                verbose: true,
                // Remove the unsupported field - check the actual ExtractOptions struct definition
                ..Default::default()
            };
            
            let retry_result = api.extract_with_options(&scan_result.path, &output_dir, permissive_options)
                .context(format!("Failed to extract PBO (retry): {}", scan_result.path.display()))?;
            
            if !retry_result.is_success() {
                return Err(anyhow::anyhow!(
                    "Extraction retry failed for {}: {}", 
                    scan_result.path.display(), 
                    retry_result
                ));
            }
            
            // Check if retry worked
            let retry_file_count = count_files_in_directory(&output_dir)?;
            let retry_extracted_files = retry_file_count - initial_file_count;
            
            if retry_extracted_files == 0 {
                return Err(anyhow::anyhow!(
                    "Failed to extract any files from {} after multiple attempts", 
                    scan_result.path.display()
                ));
            }
            
            info!("Successfully extracted {} files on retry from {}", 
                  retry_extracted_files, 
                  scan_result.path.display());
        } else {
            info!("Successfully extracted {} files from {}", 
                  extracted_files, 
                  scan_result.path.display());
        }
        
        // Find mission.sqm file
        let sqm_file = walkdir::WalkDir::new(&output_dir)
            .max_depth(3) // Increase depth to find mission.sqm in subdirectories
            .into_iter()
            .filter_map(|e| e.ok())
            .find(|e| {
                e.file_name().to_string_lossy().eq_ignore_ascii_case("mission.sqm")
            })
            .map(|e| e.path().to_owned());
        
        if sqm_file.is_none() {
            warn!("No mission.sqm file found in extracted PBO: {}", scan_result.path.display());
        }
        
        // Find SQF files
        let sqf_files: Vec<PathBuf> = walkdir::WalkDir::new(&output_dir)
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
        let cpp_files: Vec<PathBuf> = walkdir::WalkDir::new(&output_dir)
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
        
        // Get mission name from PBO filename
        let mission_name = scan_result.path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Preliminary scan for class references using quick regex patterns
        let mut direct_class_references = HashSet::new();
        
        // Scan SQM file for class references
        if let Some(sqm_path) = &sqm_file {
            if let Ok(content) = fs::read_to_string(sqm_path) {
                self.extract_preliminary_class_references(&content, &mut direct_class_references);
            }
        }
        
        // Scan each SQF file for class references
        for sqf_path in &sqf_files {
            if let Ok(content) = fs::read_to_string(sqf_path) {
                self.extract_preliminary_class_references(&content, &mut direct_class_references);
            }
        }
        
        // Scan each CPP/HPP file for class references
        for cpp_path in &cpp_files {
            if let Ok(content) = fs::read_to_string(cpp_path) {
                self.extract_preliminary_class_references(&content, &mut direct_class_references);
            }
        }
        
        info!("Extracted mission '{}' with {} SQF files, {} CPP/HPP files, and discovered {} potential class references", 
              mission_name, 
              sqf_files.len(),
              cpp_files.len(),
              direct_class_references.len());
        
        Ok(MissionExtractionResult {
            mission_name,
            pbo_path: scan_result.path.clone(),
            extracted_path: output_dir,
            sqm_file,
            sqf_files,
            cpp_files,
            direct_class_references,
        })
    }
    
    // Helper function to find potential class references using regex patterns
    fn extract_preliminary_class_references(&self, content: &str, class_references: &mut HashSet<String>) {
        // Common patterns for class references in quotes
        let patterns = [
            r#"createVehicle\s*\[\s*"([^"]+)""#, // createVehicle ["ClassName"
            r#"createVehicle\s*"([^"]+)""#,      // createVehicle "ClassName"
            r#"addWeapon\s*"([^"]+)""#,          // addWeapon "ClassName"
            r#"addMagazine\s*"([^"]+)""#,        // addMagazine "ClassName" 
            r#"addItem\s*"([^"]+)""#,            // addItem "ClassName"
            r#"addBackpack\s*"([^"]+)""#,        // addBackpack "ClassName"
            r#"vehicle\s*=\s*"([^"]+)""#,        // vehicle = "ClassName"
            r#"type\s*=\s*"([^"]+)""#,           // type = "ClassName"
            r#"weapon\s*=\s*"([^"]+)""#,         // weapon = "ClassName"
            r#"magazine\s*=\s*"([^"]+)""#,       // magazine = "ClassName"
            r#"backpack\s*=\s*"([^"]+)""#,       // backpack = "ClassName"
            r#"uniform\s*=\s*"([^"]+)""#,        // uniform = "ClassName" 
            r#"vest\s*=\s*"([^"]+)""#,           // vest = "ClassName"
            r#"headgear\s*=\s*"([^"]+)""#,       // headgear = "ClassName"
            r#"goggles\s*=\s*"([^"]+)""#,        // goggles = "ClassName"
            r#"class\s+(\w+)"#,                   // class ClassName (for inheritance)
            r#"class\s+(\w+)\s*:\s*(\w+)"#,       // class ClassName: ParentClass
        ];
        
        // Process each line to avoid regex DoS on large files
        for line in content.lines() {
            // Skip comments in the file
            let line = line.trim();
            if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
                continue;
            }
            
            // Apply each pattern - using a simplified regex approach without the regex crate
            for pattern in &patterns {
                // Simple string-based pattern matching
                self.extract_patterns_manually(line, pattern, class_references);
            }
            
            // Special case for array literals with class names
            if line.contains('[') && line.contains(']') && line.contains('"') {
                let mut in_quotes = false;
                let mut current_item = String::new();
                
                for c in line.chars() {
                    match c {
                        '"' => {
                            in_quotes = !in_quotes;
                            if !in_quotes && !current_item.is_empty() {
                                // We've found a complete string - check if it looks like a class name
                                if !current_item.contains(' ') && current_item.len() > 2 && current_item.contains('_') {
                                    class_references.insert(current_item.clone());
                                }
                                current_item.clear();
                            }
                        },
                        _ if in_quotes => {
                            current_item.push(c);
                        },
                        _ => {}
                    }
                }
            }
        }
    }
    
    // Simple pattern matcher that doesn't rely on regex crate
    fn extract_patterns_manually(&self, line: &str, pattern: &str, class_references: &mut HashSet<String>) {
        // For patterns ending with "([^"]+)" to capture class names in quotes
        if pattern.ends_with(r#""([^"]+)""#) || pattern.ends_with(r#"(\w+)"#) {
            if let Some(pos) = line.find('"') {
                if let Some(end_pos) = line[pos+1..].find('"') {
                    let potential_class = &line[pos+1..pos+1+end_pos];
                    if !potential_class.is_empty() && !potential_class.contains(' ') {
                        class_references.insert(potential_class.to_string());
                    }
                }
            }
        }
        
        // For class inheritance pattern: class ClassName: ParentClass
        if pattern.contains("class") && pattern.contains(':') {
            if line.contains("class") && line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    if let Some(class_part) = parts.first() {
                        if let Some(class_pos) = class_part.find("class") {
                            let class_name = &class_part[class_pos + 5..].trim();
                            if !class_name.is_empty() && !class_name.contains(' ') {
                                class_references.insert(class_name.to_string());
                            }
                        }
                    }
                    
                    if let Some(parent_part) = parts.get(1) {
                        let parent_name = parent_part.trim().split_whitespace().next().unwrap_or("");
                        if !parent_name.is_empty() && !parent_name.contains(' ') {
                            class_references.insert(parent_name.to_string());
                        }
                    }
                }
            }
        }
        
        // For simple class definitions: class ClassName
        if pattern.contains("class") && !pattern.contains(':') {
            if line.contains("class") && !line.contains(':') {
                if let Some(class_pos) = line.find("class") {
                    let after_class = &line[class_pos + 5..].trim();
                    if let Some(first_word) = after_class.split_whitespace().next() {
                        if !first_word.is_empty() && !first_word.contains(' ') {
                            class_references.insert(first_word.to_string());
                        }
                    }
                }
            }
        }
    }
}

// Helper function to count files in a directory recursively
fn count_files_in_directory(dir: &Path) -> Result<usize> {
    let count = walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .count();
    
    Ok(count)
}