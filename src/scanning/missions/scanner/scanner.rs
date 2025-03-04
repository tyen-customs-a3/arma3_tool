use std::path::Path;
use std::sync::{Arc, Mutex};
use anyhow::{Result, Context};
use indicatif::{ProgressBar, ParallelProgressIterator};
use log::{info, warn, debug};

use crate::scanning::types::{ScanDatabase, PboScanResult};
use crate::scanning::missions::extractor::{MissionExtractor, MissionExtractionResult};
use super::collector::collect_mission_files;

/// Scan and extract mission PBO files
pub async fn scan_and_extract(
    input_dir: &Path,
    cache_dir: &Path,
    threads: usize,
    db: &Arc<Mutex<ScanDatabase>>
) -> Result<Vec<MissionExtractionResult>> {
    // Create cache directory if it doesn't exist
    std::fs::create_dir_all(cache_dir)
        .context("Failed to create cache directory")?;
    
    // Collect all mission PBO files
    let mission_files = collect_mission_files(input_dir)
        .context("Failed to collect mission files")?;
    
    if mission_files.is_empty() {
        warn!("No mission PBO files found in {}", input_dir.display());
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
    let scan_results = scan_pbo_files(&mission_files, progress.clone(), db)
        .context("Failed to scan PBO files")?;
    
    if scan_results.is_empty() {
        info!("No mission PBOs need extraction (all cached)");
        
        // Return cached results
        let cached_results = collect_cached_results(&mission_files, cache_dir, db)
            .context("Failed to collect cached results")?;
        
        return Ok(cached_results);
    }
    
    info!("{} mission PBOs need extraction", scan_results.len());
    
    // Extract PBOs - only extract files needed for analysis, not the entire mission
    let extractor = MissionExtractor::new(cache_dir, threads)?; // Added ? operator here to extract from Result
    let extraction_results = extractor.extract_missions(&scan_results, progress)
        .context("Failed to extract mission PBOs")?;
    
    // Save database
    let db_path = cache_dir.join("mission_scan_db.json");
    {
        let db = db.lock().unwrap();
        db.save(&db_path)?;
    }
    
    Ok(extraction_results)
}

/// Scan PBO files to determine which need extraction
fn scan_pbo_files(
    mission_files: &[std::path::PathBuf],
    progress: ProgressBar,
    db: &Arc<Mutex<ScanDatabase>>
) -> Result<Vec<PboScanResult>> {
    use rayon::prelude::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // Get current timestamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    // Process files in parallel with progress bar
    let results: Vec<PboScanResult> = mission_files.par_iter()
        .progress_with(progress)
        .filter_map(|path| {
            // Check if file exists and is readable
            if !path.exists() {
                debug!("File does not exist: {}", path.display());
                return None;
            }
            
            // Get file metadata
            let metadata = match std::fs::metadata(path) {
                Ok(meta) => meta,
                Err(e) => {
                    warn!("Failed to get metadata for {}: {}", path.display(), e);
                    return None;
                }
            };
            
            // Skip directories
            if metadata.is_dir() {
                debug!("Skipping directory: {}", path.display());
                return None;
            }
            
            // Get file modification time
            let modified = metadata.modified().ok()
                .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs())
                .unwrap_or(0);
            
            // Check if file is in database and up to date
            let db_guard = db.lock().unwrap();
            let needs_extraction = match db_guard.get_pbo_info(path) {
                Some(info) => {
                    // Check if file has been modified since last scan
                    if info.failed {
                        debug!("Previous extraction failed, needs extraction: {}", path.display());
                        true
                    } else {
                        debug!("File unchanged since last scan: {}", path.display());
                        false
                    }
                },
                None => {
                    debug!("File not in database, needs extraction: {}", path.display());
                    true
                }
            };
            
            // If file needs extraction, add it to results
            if needs_extraction {
                // Update database with current file info
                drop(db_guard); // Release lock before updating
                let mut db_guard = db.lock().unwrap();
                db_guard.update_pbo(path, "", false);
                
                Some(PboScanResult {
                    path: path.clone(),
                    hash: "".to_string(), // Empty hash, not used for missions
                    expected_files: Vec::new(), // No expected files, not used for missions
                })
            } else {
                None
            }
        })
        .collect();
    
    Ok(results)
}

/// Collect cached extraction results for files that don't need extraction
fn collect_cached_results(
    mission_files: &[std::path::PathBuf],
    cache_dir: &Path,
    db: &Arc<Mutex<ScanDatabase>>
) -> Result<Vec<MissionExtractionResult>> {
    let missions_dir = cache_dir.join("missions");
    let mut results = Vec::new();
    
    for path in mission_files {
        // Check if file is in database
        let db_guard = db.lock().unwrap();
        if db_guard.get_pbo_info(path).is_none() {
            continue;
        }
        
        // Get mission name from PBO filename
        let mission_name = path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        
        // Get extracted path
        let extracted_path = missions_dir.join(path.file_name().unwrap())
            .with_extension("");
        
        if !extracted_path.exists() {
            debug!("Extracted path does not exist: {}", extracted_path.display());
            continue;
        }
        
        // Find mission.sqm file
        let sqm_file = find_file_by_extension(&extracted_path, "sqm");
        
        // Find all SQF files
        let sqf_files = find_files_by_extension(&extracted_path, "sqf");
        
        // Find all CPP/HPP files
        let mut cpp_files = find_files_by_extension(&extracted_path, "cpp");
        cpp_files.extend(find_files_by_extension(&extracted_path, "hpp"));
        
        // Create extraction result
        let extraction_result = MissionExtractionResult {
            mission_name,
            pbo_path: path.clone(),
            extracted_path,
            sqm_file,
            sqf_files,
            cpp_files,
        };
        
        results.push(extraction_result);
    }
    
    Ok(results)
}

/// Find a file with a specific extension in a directory (recursively)
fn find_file_by_extension(dir: &Path, extension: &str) -> Option<std::path::PathBuf> {
    for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext.to_string_lossy().to_lowercase() == extension {
                    return Some(entry.path().to_path_buf());
                }
            }
        }
    }
    
    None
}

/// Find all files with a specific extension in a directory (recursively)
fn find_files_by_extension(dir: &Path, extension: &str) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    
    for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext.to_string_lossy().to_lowercase() == extension {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }
    
    files
}