use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use log::{info, warn, error};
use tokio::task;
use tempfile::tempdir;
use walkdir::WalkDir;
use std::sync::{Arc, Mutex};
use pbo_tools::extract::ExtractOptions;
use pbo_tools::core::api::{PboApi, PboApiOps};

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
    pub async fn extract_pbos<F>(
        &self,
        pbos: Vec<PathBuf>,
        cache_dir: &Path,
        extensions: Vec<String>,
        _verbose: bool,
        on_failed_extraction: &mut F,
    ) -> Result<Vec<(PathBuf, Vec<PathBuf>)>> 
    where
        F: FnMut(&Path, String) -> crate::error::Result<()>
    {
        if pbos.is_empty() {
            return Ok(Vec::new());
        }

        info!("Extracting {} PBOs to cache", pbos.len());

        // Shared result collection
        let results = Arc::new(Mutex::new(Vec::with_capacity(pbos.len())));
        
        // Create a mutex for tracking failures
        let failures = Arc::new(Mutex::new(Vec::new()));

        // Limit concurrency
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.threads));

        // Process PBOs in parallel
        let mut tasks = vec![];

        for pbo_path in pbos {
            let cache_dir = cache_dir.to_path_buf();
            let extensions = extensions.clone();
            let results = Arc::clone(&results);
            let failures = Arc::clone(&failures);
            let semaphore_clone = Arc::clone(&semaphore);
            let processor = self.pbo_api.clone();

            let task = task::spawn(async move {
                // Acquire semaphore permit
                let _permit = semaphore_clone.acquire().await.unwrap();

                // Common error handling function to reduce duplication
                let record_failure = |error_msg: String| {
                    error!("Error processing PBO {}: {}", pbo_path.display(), error_msg);
                    let mut failures_guard = failures.lock().unwrap();
                    failures_guard.push((pbo_path.clone(), error_msg));
                };

                // Extract the PBO to a temporary directory
                let temp_dir = match tempdir() {
                    Ok(dir) => dir,
                    Err(e) => {
                        record_failure(format!("Failed to create temporary directory: {}", e));
                        return;
                    }
                };
                
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
                        // Add a small delay to ensure file handles are released
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        
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
                                            if let Err(e) = fs::create_dir_all(parent) {
                                                warn!("Failed to create directory {}: {}", parent.display(), e);
                                                continue;
                                            }
                                        }
                                    }

                                    // Copy the file
                                    let mut retries = 3;
                                    let mut last_error = None;
                                    
                                    while retries > 0 {
                                        match fs::copy(file_path, &target_path) {
                                            Ok(_) => {
                                                // Store only the relative path, not the full cache path
                                                cache_paths.push(PathBuf::from(rel_path));
                                                break;
                                            },
                                            Err(e) => {
                                                last_error = Some(e);
                                                retries -= 1;
                                                if retries > 0 {
                                                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                                                }
                                            }
                                        }
                                    }
                                    
                                    if retries == 0 {
                                        if let Some(e) = last_error {
                                            warn!("Failed to copy {} to {} after 3 retries: {}", 
                                                file_path.display(), target_path.display(), e);
                                        }
                                    }
                                }

                                // Store the result
                                let mut result_guard = results.lock().unwrap();
                                result_guard.push((pbo_path, cache_paths));
                            },
                            Ok(Err(e)) => {
                                record_failure(format!("Failed to catalog extracted files: {}", e));
                            },
                            Err(e) => {
                                record_failure(format!("Task failure when cataloging files: {}", e));
                            }
                        }
                    },
                    Err(e) => {
                        record_failure(format!("Extraction error: {}", e));
                    }
                }
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in tasks {
            let _ = task.await;
        }

        // Process failures and update the index using the callback
        let failures = Arc::try_unwrap(failures)
            .expect("Failed to unwrap failures")
            .into_inner()
            .expect("Failed to get inner failures");

        for (path, error_message) in failures {
            if let Err(e) = on_failed_extraction(&path, error_message) {
                warn!("Failed to record extraction failure for {}: {}", path.display(), e);
            }
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
        
        // Track failed extractions
        let mut failed_pbo_paths = Vec::new();
        let mut on_failed_extraction = |path: &Path, _error: String| {
            failed_pbo_paths.push(path.to_path_buf());
            Ok(())
        };
        
        // First extraction attempt
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
            &mut on_failed_extraction
        ).await;
        
        // The extraction itself will fail because these aren't real PBOs,
        // but we can verify that failures were recorded correctly
        assert!(results.is_err() || failed_pbo_paths.len() > 0);
    }
}
