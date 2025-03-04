use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use std::collections::HashSet;
use anyhow::{Result, Context};
use indicatif::{ProgressBar, ParallelProgressIterator};
use log::{info, warn, debug};
use rayon::prelude::*;
use pbo_tools::{
    core::api::{PboApi, PboApiOps},
    extract::ExtractOptions,
};

use crate::scanning::types::PboScanResult;
use crate::scanning::pbo::calculate_file_hash;
use super::types::{MissionExtractionResult, MissionDatabase};

/// Extract mission files from PBO archives
pub fn extract_missions(
    cache_dir: &Path,
    threads: usize,
    scan_results: &[PboScanResult],
    db: Arc<Mutex<MissionDatabase>>,
    progress: ProgressBar,
) -> Result<Vec<MissionExtractionResult>> {
    // Reset progress bar for extraction phase
    progress.set_length(scan_results.len() as u64);
    progress.set_message("Processing mission PBOs...");
    progress.reset();
    
    // Counters for tracking progress
    let skipped_count = Arc::new(AtomicUsize::new(0));
    let processed_count = Arc::new(AtomicUsize::new(0));
    let failed_count = Arc::new(AtomicUsize::new(0));
    
    // Process in parallel with thread limit
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .context("Failed to build thread pool")?;
    
    let results = pool.install(|| {
        scan_results.par_iter()
            .progress_with(progress.clone())
            .filter_map(|result| {
                let hash = match calculate_file_hash(&result.path) {
                    Ok(h) => h,
                    Err(e) => {
                        warn!("Failed to calculate hash for {}: {}", result.path.display(), e);
                        let mut db = db.lock().unwrap();
                        db.update_mission(&result.path, "failed-hash", true);
                        failed_count.fetch_add(1, Ordering::Relaxed);
                        progress.inc(1);
                        return None;
                    },
                };

                // Check if we need to process this mission
                let needs_processing = {
                    let db = db.lock().unwrap();
                    match db.get_mission_info(&result.path) {
                        // Skip if mission has same hash and didn't fail before
                        Some(info) if info.hash == hash && !info.failed => {
                            debug!("Skipping unchanged mission: {}", result.path.display());
                            skipped_count.fetch_add(1, Ordering::Relaxed);
                            false
                        },
                        // Process if mission failed before (retry)
                        Some(info) if info.failed => {
                            debug!("Retrying previously failed mission: {}", result.path.display());
                            true
                        },
                        // Process if mission has changed or is new
                        _ => true,
                    }
                };

                if !needs_processing {
                    progress.inc(1);
                    return load_existing_mission(cache_dir, &result.path).ok();
                }

                // Extract the mission
                match extract_single_mission(cache_dir, result, &hash) {
                    Ok(result) => {
                        let mut db = db.lock().unwrap();
                        db.update_mission(&result.pbo_path, &hash, false);
                        processed_count.fetch_add(1, Ordering::Relaxed);
                        Some(result)
                    },
                    Err(e) => {
                        warn!("Failed to extract mission {}: {}", result.path.display(), e);
                        let mut db = db.lock().unwrap();
                        db.update_mission(&result.path, &hash, true);
                        failed_count.fetch_add(1, Ordering::Relaxed);
                        None
                    },
                }
            })
            .collect::<Vec<_>>()
    });
    
    // Report statistics
    let skipped = skipped_count.load(Ordering::Relaxed);
    let processed = processed_count.load(Ordering::Relaxed);
    let failed = failed_count.load(Ordering::Relaxed);
    
    info!("Mission extraction processing complete:");
    info!("  Total missions: {}", scan_results.len());
    info!("  Newly extracted: {}", processed);
    info!("  Skipped (unchanged): {}", skipped);
    info!("  Failed: {}", failed);
    
    progress.finish_with_message("Mission extraction complete");
    
    Ok(results)
}

/// Load an existing extracted mission without re-extracting
fn load_existing_mission(cache_dir: &Path, pbo_path: &Path) -> Result<MissionExtractionResult> {
    // Get mission name from PBO filename
    let mission_name = pbo_path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    // Create output directory path (strip .pbo extension) in the missions subfolder
    let missions_dir = cache_dir.join("missions");
    let output_dir = missions_dir.join(pbo_path.file_name().unwrap())
        .with_extension("");
    
    if !output_dir.exists() {
        return Err(anyhow::anyhow!("Mission directory doesn't exist: {}", output_dir.display()));
    }
    
    // Find mission.sqm file
    let sqm_file = find_file_by_extension(&output_dir, "sqm");
    
    // Find all SQF files
    let sqf_files = find_files_by_extension(&output_dir, "sqf");
    
    // Find all CPP/HPP files
    let mut cpp_files = find_files_by_extension(&output_dir, "cpp");
    cpp_files.extend(find_files_by_extension(&output_dir, "hpp"));
    
    // Create extraction result
    let extraction_result = MissionExtractionResult {
        mission_name,
        pbo_path: pbo_path.to_path_buf(),
        extracted_path: output_dir,
        sqm_file,
        sqf_files,
        cpp_files,
    };
    
    debug!("Loaded existing mission: {} (SQM: {}, SQF: {}, CPP/HPP: {})",
          extraction_result.mission_name,
          extraction_result.sqm_file.is_some(),
          extraction_result.sqf_files.len(),
          extraction_result.cpp_files.len());
    
    Ok(extraction_result)
}

/// Extract a single mission PBO
fn extract_single_mission(cache_dir: &Path, scan_result: &PboScanResult, hash: &str) -> Result<MissionExtractionResult> {
    // Create missions subfolder
    let missions_dir = cache_dir.join("missions");
    if !missions_dir.exists() {
        fs::create_dir_all(&missions_dir)
            .context(format!("Failed to create missions directory: {}", missions_dir.display()))?;
    }

    // Create output directory path (strip .pbo extension) in the missions subfolder
    let output_dir = missions_dir.join(scan_result.path.file_name().unwrap())
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
    }
    
    // Find mission.sqm file
    let sqm_file = find_file_by_extension(&output_dir, "sqm");
    
    // Find all SQF files
    let sqf_files = find_files_by_extension(&output_dir, "sqf");
    
    // Find all CPP/HPP files
    let mut cpp_files = find_files_by_extension(&output_dir, "cpp");
    cpp_files.extend(find_files_by_extension(&output_dir, "hpp"));
    
    // Get mission name from PBO filename
    let mission_name = scan_result.path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    
    // Create extraction result
    let extraction_result = MissionExtractionResult {
        mission_name,
        pbo_path: scan_result.path.clone(),
        extracted_path: output_dir,
        sqm_file,
        sqf_files,
        cpp_files,
    };
    
    debug!("Extracted mission: {} (SQM: {}, SQF: {}, CPP/HPP: {})",
          extraction_result.mission_name,
          extraction_result.sqm_file.is_some(),
          extraction_result.sqf_files.len(),
          extraction_result.cpp_files.len());
    
    Ok(extraction_result)
}

/// Count files in a directory recursively
fn count_files_in_directory(dir: &Path) -> Result<usize> {
    let mut count = 0;
    
    if !dir.exists() {
        return Ok(0);
    }
    
    for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            count += 1;
        }
    }
    
    Ok(count)
}

/// Find a file with a specific extension in a directory (recursively)
fn find_file_by_extension(dir: &Path, extension: &str) -> Option<PathBuf> {
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
fn find_files_by_extension(dir: &Path, extension: &str) -> Vec<PathBuf> {
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