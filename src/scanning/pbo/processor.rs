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
            .with_timeout(30)
            .build();

        // Create a combined filter that includes both target extensions and binary files
        let bin_exts = vec!["bin", "binpbo"]; // Add binary extensions
        let mut all_exts = self.extensions.split(',').collect::<Vec<_>>();
        all_exts.extend(bin_exts.iter());
        let filter = format!("*.{{{}}}", all_exts.join(","));

        let options = ExtractOptions {
            no_pause: true,
            warnings_as_errors: false,
            file_filter: Some(filter),
            verbose: false,
            ..Default::default()
        };

        // Create output directory only if we're going to extract
        fs::create_dir_all(&output_dir)?;

        match api.extract_with_options(&scan_result.path, &output_dir, options) {
            Ok(result) => {
                let mut db = self.db.lock().unwrap();
                if result.is_success() {
                    // After extraction, we need to check both original extensions and converted files
                    match count_matching_files(&output_dir, &format!("{},cpp", self.extensions)) {
                        Ok(final_files) => {
                            let new_files = final_files - initial_files;
                            if new_files == 0 {
                                db.update_pbo_with_reason(
                                    &scan_result.path, 
                                    &scan_result.hash, 
                                    true, 
                                    SkipReason::Failed
                                );
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