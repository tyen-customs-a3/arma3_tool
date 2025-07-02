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
use std::collections::HashSet;

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
        let initial_pbo_count = pbos.len(); // Store initial count

        info!("Extracting {} PBOs to cache", initial_pbo_count);

        // Convert original extensions to HashSet for efficient lookup later
        let original_extensions_set: HashSet<String> = extensions.iter().cloned().collect();

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
            let original_extensions_set = original_extensions_set.clone();
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

                // --- Filter Augmentation for pbo_tools ---
                let mut pbo_tool_extensions = extensions.clone();
                let user_wants_cpp = extensions.iter().any(|e| e == "cpp");
                let bin_already_requested = extensions.iter().any(|e| e == "bin");

                if user_wants_cpp && !bin_already_requested {
                    info!("User requested 'cpp', adding 'bin' to pbo_tools filter for PBO: {}", pbo_path.display());
                    pbo_tool_extensions.push("bin".to_string());
                }

                // Set file extension filter for pbo_tools
                if !pbo_tool_extensions.is_empty() {
                    options.file_filter = Some(pbo_tool_extensions.join(","));
                }
                // -----------------------------------------

                // Try to extract the PBO
                match processor.extract_with_options(&pbo_path, temp_dir.path(), options) {
                    Ok(_) => {
                        // Add a small delay to ensure file handles are released
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        
                        // Find all extracted files
                        match tokio::task::spawn_blocking(move || PboProcessor::catalog_files(&temp_dir_path)).await {
                            Ok(Ok(extracted_files)) => {
                                let mut cache_paths = Vec::new();
                                let temp_path_str = temp_dir.path().to_string_lossy();

                                // --- Post-Extraction Filtering Loop ---
                                for file_path in &extracted_files {
                                    let file_name = match file_path.file_name() {
                                        Some(name) => name.to_string_lossy().to_lowercase(),
                                        None => continue, // Skip if no filename
                                    };

                                    let mut rename_to_cpp = false;
                                    let effective_extension: String;

                                    // Handle config.bin potential rename
                                    if file_name == "config.bin" {
                                        if user_wants_cpp {
                                            rename_to_cpp = true;
                                            effective_extension = "cpp".to_string();
                                        } else {
                                            continue; // Skip config.bin if user didn't ask for cpp
                                        }
                                    } else {
                                        // Get the actual extension for other files
                                        effective_extension = match file_path.extension() {
                                            Some(ext) => ext.to_string_lossy().to_lowercase(),
                                            None => continue, // Skip files without extensions
                                        };
                                    }

                                    // Check against user's *original* requested extensions
                                    if !original_extensions_set.contains(&effective_extension) {
                                        // info!("Skipping file {} (effective ext: {}) as it's not in requested set: {:?}", file_path.display(), effective_extension, original_extensions_set);
                                        continue; // Skip file if its effective extension wasn't requested
                                    }

                                    // --- File matches original filter (or is config.bin treated as cpp) ---

                                    // Compute the relative path from temp dir
                                    let rel_path_str = file_path.to_string_lossy()
                                        .replace(&*temp_path_str, "")
                                        .trim_start_matches(['/', '\\'])
                                        .to_string();

                                    let final_rel_path: PathBuf;
                                    if rename_to_cpp {
                                        // Create path ending in .cpp
                                        let mut temp_path = PathBuf::from(rel_path_str);
                                        temp_path.set_extension("cpp");
                                        final_rel_path = temp_path;
                                    } else {
                                        final_rel_path = PathBuf::from(rel_path_str);
                                    }

                                    let target_path = cache_dir.join(&final_rel_path);

                                    // Ensure the target directory exists
                                    if let Some(parent) = target_path.parent() {
                                        if !parent.exists() {
                                            if let Err(e) = fs::create_dir_all(parent) {
                                                warn!("Failed to create directory {}: {}", parent.display(), e);
                                                continue;
                                            }
                                        }
                                    }

                                    // Copy the file (from original temp path to final target path)
                                    let mut retries = 3;
                                    let mut last_error = None;

                                    while retries > 0 {
                                        match fs::copy(file_path, &target_path) {
                                            Ok(_) => {
                                                // Store the final relative path (e.g., .../config.cpp)
                                                cache_paths.push(final_rel_path);
                                                last_error = None; // Clear error on success
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

                                    if let Some(e) = last_error {
                                         warn!("Failed to copy {} to {} after 3 retries: {}",
                                             file_path.display(), target_path.display(), e);
                                    }
                                } // --- End Post-Extraction Filtering Loop ---

                                // Store the result (filtered paths)
                                if !cache_paths.is_empty() { // Only store if we actually kept some files
                                    let mut result_guard = results.lock().unwrap();
                                    result_guard.push((pbo_path, cache_paths));
                                }
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

        info!("Successfully processed {} PBOs ({} might have failed or yielded no matching files)", initial_pbo_count, initial_pbo_count - final_results.len());
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
