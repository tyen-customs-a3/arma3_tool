use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::fs;
use anyhow::Result;
use log::{info, warn};
use rayon::prelude::*;
use indicatif::ProgressBar;
use pbo_tools::{
    core::api::{PboApi, PboApiOps},
    extract::ExtractOptions,
    core::config::PboConfig,
};

use super::utils::count_matching_files;
use super::super::types::{ScanDatabase, PboScanResult, SkipReason};

pub struct PboProcessor<'a> {
    input_dir: &'a Path,
    cache_dir: &'a Path,
    extensions: &'a str,
    db: Arc<Mutex<ScanDatabase>>,
}

impl<'a> PboProcessor<'a> {
    pub fn new(
        input_dir: &'a Path,
        cache_dir: &'a Path,
        extensions: &'a str,
        db: Arc<Mutex<ScanDatabase>>,
    ) -> Self {
        Self {
            input_dir,
            cache_dir,
            extensions,
            db,
        }
    }

    pub fn process_all(&self, scan_results: &[PboScanResult], progress: ProgressBar) -> Result<()> {
        scan_results.par_iter().for_each(|result| {
            if let Err(e) = self.process_pbo(result) {
                warn!("Failed to process {}: {}", result.path.display(), e);
                let mut db = self.db.lock().unwrap();
                db.update_pbo_with_reason(&result.path, &result.hash, true, SkipReason::Failed);
            }
            progress.inc(1);
        });
        
        progress.finish_with_message("Extraction complete");
        Ok(())
    }

    fn process_pbo(&self, scan_result: &PboScanResult) -> Result<()> {
        // Get the relative path for output
        let rel_path = scan_result.path.strip_prefix(self.input_dir)?;
        let output_dir = self.cache_dir.join(rel_path).with_extension("");
        
        // Get initial file count
        let initial_files = if output_dir.exists() {
            count_matching_files(&output_dir, self.extensions)?
        } else {
            0
        };

        let config = PboConfig::default(); // Get default config with binary mappings
        let api = PboApi::builder()
            .with_config(config.clone())
            .with_timeout(60) // Increase timeout for larger PBOs
            .build();

        // Create a combined filter that includes both target extensions and binary files
        // Also include mission-specific file types
        let bin_exts = vec!["bin", "binpbo"]; // Add binary extensions
        let mission_exts = vec!["sqm", "sqf", "hpp", "cpp", "fsm", "h", "inc", "ext"]; // Add mission file extensions
        let mut all_exts = self.extensions.split(',').collect::<Vec<_>>();
        all_exts.extend(bin_exts.iter());
        all_exts.extend(mission_exts.iter());
        let filter = format!("*.{{{}}}", all_exts.join(","));

        let options = ExtractOptions {
            no_pause: true,
            warnings_as_errors: false,
            file_filter: Some(filter),
            verbose: true, // Enable verbose output for better debugging
            ..Default::default()
        };

        // Create output directory only if we're going to extract
        fs::create_dir_all(&output_dir)?;

        match api.extract_with_options(&scan_result.path, &output_dir, options) {
            Ok(result) => {
                let mut db = self.db.lock().unwrap();
                if result.is_success() {
                    // After extraction, we need to check both original extensions and converted files
                    // Include mission file types in the check
                    let check_extensions = format!("{},cpp,sqm,sqf,hpp,fsm", self.extensions);
                    match count_matching_files(&output_dir, &check_extensions) {
                        Ok(final_files) => {
                            let new_files = final_files - initial_files;
                            if new_files == 0 {
                                // Try again with no file filter
                                info!("No files extracted with filter, trying without filter for: {}", 
                                      scan_result.path.display());
                                
                                let permissive_options = ExtractOptions {
                                    no_pause: true,
                                    warnings_as_errors: false,
                                    file_filter: None, // Extract all files
                                    verbose: true,
                                    ..Default::default()
                                };
                                
                                match api.extract_with_options(&scan_result.path, &output_dir, permissive_options) {
                                    Ok(retry_result) => {
                                        if retry_result.is_success() {
                                            match count_matching_files(&output_dir, &check_extensions) {
                                                Ok(retry_files) => {
                                                    let retry_new_files = retry_files - initial_files;
                                                    if retry_new_files == 0 {
                                                        warn!("Failed to extract any files from {} after retry", 
                                                              scan_result.path.display());
                                                        db.update_pbo_with_reason(
                                                            &scan_result.path, 
                                                            &scan_result.hash, 
                                                            true, 
                                                            SkipReason::Failed
                                                        );
                                                    } else {
                                                        info!("Successfully extracted {} files on retry from: {}", 
                                                              retry_new_files, 
                                                              scan_result.path.display());
                                                        db.update_pbo_with_reason(
                                                            &scan_result.path, 
                                                            &scan_result.hash, 
                                                            false, 
                                                            SkipReason::None
                                                        );
                                                    }
                                                },
                                                Err(e) => {
                                                    warn!("Failed to verify extracted files on retry from {}: {}", 
                                                          scan_result.path.display(), 
                                                          e);
                                                    db.update_pbo_with_reason(
                                                        &scan_result.path, 
                                                        &scan_result.hash, 
                                                        true, 
                                                        SkipReason::Failed
                                                    );
                                                }
                                            }
                                        } else {
                                            warn!("Retry extraction failed for: {} - {}", 
                                                  scan_result.path.display(), 
                                                  retry_result);
                                            db.update_pbo_with_reason(
                                                &scan_result.path, 
                                                &scan_result.hash, 
                                                true, 
                                                SkipReason::Failed
                                            );
                                        }
                                    },
                                    Err(e) => {
                                        warn!("Failed to retry extraction for {}: {}", 
                                              scan_result.path.display(), 
                                              e);
                                        db.update_pbo_with_reason(
                                            &scan_result.path, 
                                            &scan_result.hash, 
                                            true, 
                                            SkipReason::Failed
                                        );
                                    }
                                }
                                return Ok(());
                            }
                            
                            if new_files < scan_result.expected_files.len() {
                                warn!(
                                    "Extracted fewer files than expected from {}: got {}, expected {}", 
                                    scan_result.path.display(),
                                    new_files,
                                    scan_result.expected_files.len()
                                );
                            }
                            
                            db.update_pbo_with_reason(
                                &scan_result.path, 
                                &scan_result.hash, 
                                false, 
                                SkipReason::None
                            );
                            info!(
                                "Successfully extracted {} new files from: {}", 
                                new_files, 
                                scan_result.path.display()
                            );
                        }
                        Err(e) => {
                            warn!(
                                "Failed to verify extracted files from {}: {}", 
                                scan_result.path.display(), 
                                e
                            );
                            db.update_pbo_with_reason(
                                &scan_result.path, 
                                &scan_result.hash, 
                                true, 
                                SkipReason::Failed
                            );
                        }
                    }
                } else {
                    db.update_pbo_with_reason(
                        &scan_result.path, 
                        &scan_result.hash, 
                        true, 
                        SkipReason::Failed
                    );
                    warn!("Extraction failed for: {} - {}", scan_result.path.display(), result);
                }
            }
            Err(e) => {
                warn!("Failed to extract {}: {}", scan_result.path.display(), e);
                let mut db = self.db.lock().unwrap();
                db.update_pbo_with_reason(
                    &scan_result.path, 
                    &scan_result.hash, 
                    true, 
                    SkipReason::Failed
                );
            }
        }
        
        Ok(())
    }
}