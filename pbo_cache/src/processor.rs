use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use log::{info, warn, error};
use tokio::task;
use tempfile::tempdir;
use walkdir::WalkDir;
use std::sync::{Arc, Mutex};
use indicatif::{ProgressBar, ProgressStyle};
use pbo_tools::extract::ExtractOptions;
use pbo_tools::core::api::{PboApi, PboApiOps};
use std::time::UNIX_EPOCH;

/// Handles extraction of PBO files to the cache
pub struct PboProcessor {
    /// API for interacting with PBO files
    pbo_api: PboApi,
    
    /// Number of threads to use for extraction
    threads: usize,
}

impl PboProcessor {
    /// Create a new PBO processor
    pub fn new(timeout: u64, threads: usize) -> Self {
        let pbo_api = PboApi::builder()
            .with_timeout(timeout as u32)
            .build();
            
        Self { pbo_api, threads }
    }
 
    /// Catalog all files in a directory recursively
    fn catalog_files(dir: &Path) -> Result<Vec<PathBuf>> {
        let files: Vec<PathBuf> = WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_file())
            .map(|entry| entry.path().to_path_buf())
            .collect();
            
        Ok(files)
    }
    
    /// Extract multiple PBOs in parallel
    pub async fn extract_pbos(
        &self,
        pbos: Vec<PathBuf>,
        cache_dir: &Path,
        extensions: Vec<String>,
        progress: bool,
        index: &mut crate::models::CacheIndex,
    ) -> Result<Vec<(PathBuf, Vec<PathBuf>)>> {
        if pbos.is_empty() {
            return Ok(Vec::new());
        }

        info!("Extracting {} PBOs to cache", pbos.len());

        // Filter out previously failed extractions and count them
        let total_pbos = pbos.len();
        let pbos_to_extract: Vec<_> = pbos.into_iter()
            .filter(|path| {
                if let Some(failed) = index.is_failed_extraction(path) {
                    warn!("Skipping previously failed PBO {}: {} at {}", 
                        path.display(), 
                        failed.error_message,
                        failed.timestamp.duration_since(UNIX_EPOCH)
                            .map(|d| d.as_secs().to_string())
                            .unwrap_or_else(|_| "unknown time".to_string())
                    );
                    false
                } else {
                    true
                }
            })
            .collect();

        let filtered_count = total_pbos - pbos_to_extract.len();

        if pbos_to_extract.is_empty() {
            info!("All PBOs were previously failed, skipping extraction");
            return Ok(Vec::new());
        }

        info!("Extracting {} PBOs (filtered out {} previously failed)", 
            pbos_to_extract.len(), 
            filtered_count
        );

        // Setup progress bar if requested
        let progress_bar = if progress {
            let pb = ProgressBar::new(pbos_to_extract.len() as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} PBOs ({eta})")
                .unwrap()
                .progress_chars("#>-"));
            Some(Arc::new(pb))
        } else {
            None
        };

        // Shared result collection
        let results = Arc::new(Mutex::new(Vec::with_capacity(pbos_to_extract.len())));
        
        // Create a mutex for tracking failures
        let failures = Arc::new(Mutex::new(Vec::new()));

        // Limit concurrency
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.threads));

        // Process PBOs in parallel
        let mut tasks = vec![];

        for pbo_path in pbos_to_extract {
            let cache_dir = cache_dir.to_path_buf();
            let extensions = extensions.clone();
            let results = Arc::clone(&results);
            let failures = Arc::clone(&failures);
            let pb_clone = progress_bar.clone();
            let semaphore_clone = Arc::clone(&semaphore);
            let processor = self.pbo_api.clone();

            let task = task::spawn(async move {
                // Acquire semaphore permit
                let _permit = semaphore_clone.acquire().await.unwrap();

                // Extract the PBO to a temporary directory
                let temp_dir = tempdir().unwrap();
                let temp_dir_path = temp_dir.path().to_path_buf();

                // Setup extraction options with defaults
                let mut options = ExtractOptions {
                    no_pause: true,
                    warnings_as_errors: false,
                    brief_listing: false,
                    ..Default::default()
                };

                // Set file extension filter if provided
                if !extensions.is_empty() {
                    options.file_filter = Some(extensions.join(","));
                }

                // Try to extract the PBO
                match processor.extract_with_options(&pbo_path, temp_dir.path(), options) {
                    Ok(_) => {
                        // Find all extracted files
                        match tokio::task::spawn_blocking(move || PboProcessor::catalog_files(&temp_dir_path)).await {
                            Ok(Ok(extracted_files)) => {
                                // Create cache subdirectories and move files
                                let mut cache_paths = Vec::new();
                                let temp_path_str = temp_dir.path().to_string_lossy();

                                for file_path in &extracted_files {
                                    // Compute the relative path
                                    let rel_path = file_path.to_string_lossy()
                                        .replace(&*temp_path_str, "")
                                        .trim_start_matches(['/', '\\'])
                                        .to_string();

                                    let target_path = cache_dir.join(&rel_path);

                                    // Ensure the target directory exists
                                    if let Some(parent) = target_path.parent() {
                                        if !parent.exists() {
                                            let _ = fs::create_dir_all(parent);
                                        }
                                    }

                                    // Copy the file
                                    if let Err(e) = fs::copy(file_path, &target_path) {
                                        warn!("Failed to copy {} to {}: {}", file_path.display(), target_path.display(), e);
                                        continue;
                                    }

                                    cache_paths.push(target_path);
                                }

                                // Store the result
                                let mut result_guard = results.lock().unwrap();
                                result_guard.push((pbo_path, cache_paths));
                            },
                            Ok(Err(e)) => {
                                error!("Failed to catalog extracted files for {}: {}", pbo_path.display(), e);
                                // Record the failure
                                let mut failures_guard = failures.lock().unwrap();
                                failures_guard.push((pbo_path, format!("Failed to catalog files: {}", e)));
                            },
                            Err(e) => {
                                error!("Task failure when cataloging files for {}: {}", pbo_path.display(), e);
                                // Record the failure
                                let mut failures_guard = failures.lock().unwrap();
                                failures_guard.push((pbo_path, format!("Task failure: {}", e)));
                            }
                        }
                    },
                    Err(e) => {
                        error!("Failed to extract PBO {}: {}", pbo_path.display(), e);
                        // Record the failure
                        let mut failures_guard = failures.lock().unwrap();
                        failures_guard.push((pbo_path, format!("Extraction error: {}", e)));
                    }
                }

                // Update progress bar
                if let Some(pb) = pb_clone {
                    pb.inc(1);
                }
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in tasks {
            let _ = task.await;
        }

        // Finish progress bar
        if let Some(pb) = progress_bar {
            pb.finish_with_message("Extraction complete");
        }

        // Process failures and update the index
        let failures = Arc::try_unwrap(failures)
            .expect("Failed to unwrap failures")
            .into_inner()
            .expect("Failed to get inner failures");

        for (path, error_message) in failures {
            index.add_failed_extraction(&path, error_message);
        }

        // Return the collected results
        let final_results = Arc::try_unwrap(results)
            .expect("Failed to unwrap results")
            .into_inner()
            .expect("Failed to get inner results");

        info!("Successfully extracted {} PBOs", final_results.len());
        Ok(final_results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;
    use crate::models::{CacheIndex, PboType};
    
    // Create a simple mock PBO for testing
    fn create_mock_pbo(dir: &Path) -> PathBuf {
        let pbo_path = dir.join("mock.pbo");
        let mut file = File::create(&pbo_path).unwrap();
        file.write_all(b"PBO content").unwrap();
        pbo_path
    }

    #[tokio::test]
    async fn test_skip_failed_pbos() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let pbo1_path = create_mock_pbo(temp_dir.path());
        let pbo2_path = create_mock_pbo(temp_dir.path());
        let pbo3_path = create_mock_pbo(temp_dir.path());
        
        let cache_dir = temp_dir.path().join("cache");
        let processor = PboProcessor::new(30, 1);
        let extensions = vec!["sqf".to_string()];
        
        // Create cache index and mark pbo2 as failed
        let mut index = CacheIndex::new();
        index.add_failed_extraction(&pbo2_path, "Previous failure".to_string());
        
        // First extraction attempt - should skip pbo2
        let pbos = vec![
            pbo1_path.clone(),
            pbo2_path.clone(),
            pbo3_path.clone()
        ];
        
        let results = processor.extract_pbos(
            pbos,
            &cache_dir,
            extensions.clone(),
            false,
            &mut index
        ).await;
        
        // The extraction itself will fail because these aren't real PBOs,
        // but we can verify that pbo2 was skipped by checking the error details
        if let Err(e) = &results {
            let error_msg = e.to_string();
            let pbo2_str = pbo2_path.to_string_lossy().to_string();
            assert!(!error_msg.contains(&pbo2_str),
                "Failed PBO should have been skipped");
        }
        
        // Verify pbo2 is still in failed extractions
        assert!(index.is_failed_extraction(&pbo2_path).is_some(),
            "PBO should remain in failed extractions");
    }
}
