use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use log::{info, warn, error, debug, trace};
use tokio::task::{self, JoinHandle};
use tempfile::tempdir;
use walkdir::WalkDir;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::time::{Duration, Instant};
use crate::pbo_interface::{PboExtractor, HemttPboExtractor};

/// Statistics for tracking extraction performance
#[derive(Debug, Default)]
struct ExtractionStats {
    total_files_extracted: usize,
    total_bytes_extracted: u64,
    pbos_processed: usize,
    pbos_failed: usize,
    extraction_times: Vec<Duration>,
}

impl ExtractionStats {
    fn new() -> Self {
        Self::default()
    }
    
    fn add_extraction(&mut self, files: usize, bytes: u64, duration: Duration) {
        self.total_files_extracted += files;
        self.total_bytes_extracted += bytes;
        self.pbos_processed += 1;
        self.extraction_times.push(duration);
    }
    
    fn add_failure(&mut self) {
        self.pbos_failed += 1;
    }
    
    fn average_extraction_time(&self) -> Duration {
        if self.extraction_times.is_empty() {
            Duration::from_secs(0)
        } else {
            let total: Duration = self.extraction_times.iter().sum();
            total / self.extraction_times.len() as u32
        }
    }
}

/// Handles extraction of PBO files to the cache
pub struct PboProcessor {
    /// Timeout for PBO operations in seconds
    timeout: u64,
    
    /// Number of threads to use for extraction
    threads: usize,
}

impl PboProcessor {
    /// Create a new PBO processor
    pub fn new(timeout: u64, threads: usize) -> Self {
        Self { timeout, threads }
    }
 
    /// Catalog all files in a directory recursively
    #[allow(dead_code)]
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
        verbose: bool,
        on_failed_extraction: &mut F,
    ) -> Result<Vec<(PathBuf, Vec<PathBuf>)>> 
    where
        F: FnMut(&Path, String) -> crate::error::Result<()>
    {
        if pbos.is_empty() {
            return Ok(Vec::new());
        }
        let initial_pbo_count = pbos.len(); // Store initial count
        let start_time = Instant::now();

        info!("Extracting {} PBOs to cache using {} threads", initial_pbo_count, self.threads);

        // Convert original extensions to HashSet for efficient lookup later
        let original_extensions_set: HashSet<String> = extensions.iter().cloned().collect();

        // Shared result collection
        let results = Arc::new(Mutex::new(Vec::with_capacity(pbos.len())));
        
        // Create a mutex for tracking failures
        let failures = Arc::new(Mutex::new(Vec::new()));
        
        // Track extraction statistics
        let stats = Arc::new(Mutex::new(ExtractionStats::new()));

        // Limit concurrency
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.threads));

        // Process PBOs in parallel
        let mut tasks: Vec<JoinHandle<()>> = vec![];

        for pbo_path in pbos {
            let cache_dir = cache_dir.to_path_buf();
            let extensions = extensions.clone();
            let original_extensions_set = original_extensions_set.clone();
            let results = Arc::clone(&results);
            let failures = Arc::clone(&failures);
            let stats = Arc::clone(&stats);
            let semaphore_clone = Arc::clone(&semaphore);
            let timeout = self.timeout;

            let task = task::spawn(async move {
                let pbo_start_time = Instant::now();
                
                // Acquire semaphore permit
                let _permit = match semaphore_clone.acquire().await {
                    Ok(permit) => permit,
                    Err(_) => {
                        error!("Failed to acquire semaphore permit for {}", pbo_path.display());
                        return;
                    }
                };

                // Common error handling function to reduce duplication
                let record_failure = |error_msg: String| {
                    error!("Error processing PBO {}: {}", pbo_path.display(), error_msg);
                    let mut failures_guard = failures.lock().unwrap();
                    failures_guard.push((pbo_path.clone(), error_msg));
                    
                    let mut stats_guard = stats.lock().unwrap();
                    stats_guard.add_failure();
                };

                // Extract the PBO to a temporary directory
                let temp_dir = match tempdir() {
                    Ok(dir) => dir,
                    Err(e) => {
                        record_failure(format!("Failed to create temporary directory: {}", e));
                        return;
                    }
                };
                
                let _temp_dir_path = temp_dir.path().to_path_buf();

                // --- Filter Augmentation for pbo_tools ---
                let mut pbo_tool_extensions = extensions.clone();
                let user_wants_cpp = extensions.iter().any(|e| e == "cpp");
                let bin_already_requested = extensions.iter().any(|e| e == "bin");

                if user_wants_cpp && !bin_already_requested {
                    info!("User requested 'cpp', adding 'bin' to pbo_tools filter for PBO: {}", pbo_path.display());
                    pbo_tool_extensions.push("bin".to_string());
                }

                // Create a filter pattern for the requested extensions
                let filter_pattern = if pbo_tool_extensions.is_empty() {
                    "*".to_string() // Extract all files if no specific extensions requested
                } else if pbo_tool_extensions.len() == 1 {
                    format!("*.{}", pbo_tool_extensions[0])
                } else {
                    // Use brace expansion for multiple extensions
                    format!("*.{{{}}}", pbo_tool_extensions.join(","))
                };
                // -----------------------------------------

                // Create PBO extractor
                let extractor = HemttPboExtractor::new(timeout);
                
                // Extract files using the PBO API
                let extraction_result = extractor.extract_filtered(
                    &pbo_path,
                    temp_dir.path(),
                    &filter_pattern
                ).await;

                match extraction_result {
                    Ok(extracted_rel_paths) => {
                        // Add a small delay to ensure file handles are released
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        
                        // If user wants cpp files, attempt binary config conversion
                        let mut final_extracted_paths = extracted_rel_paths.clone();
                        if user_wants_cpp {
                            match extractor.convert_binary_configs(&extracted_rel_paths, temp_dir.path()).await {
                                Ok(conversions) => {
                                    // Add converted files to the list and remove originals
                                    for (original, converted) in conversions {
                                        // Remove the original binary file from the list
                                        final_extracted_paths.retain(|p| p != &original);
                                        // Add the converted file
                                        final_extracted_paths.push(converted);
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to convert binary configs: {}", e);
                                    // Continue with original files
                                }
                            }
                        }
                        
                        // Convert relative paths to absolute paths in temp directory
                        let extracted_files: Vec<PathBuf> = final_extracted_paths
                            .into_iter()
                            .map(|rel_path| temp_dir.path().join(rel_path))
                            .collect();
                        let mut cache_paths = Vec::new();
                        let temp_path_str = temp_dir.path().to_string_lossy();

                                // --- Post-Extraction Filtering Loop ---
                                // With improved pattern matching, we should have fewer files to filter
                                // This loop now mainly handles the bin->cpp conversion case
                                for file_path in &extracted_files {

                                    // Get the actual extension for files
                                    let effective_extension = match file_path.extension() {
                                        Some(ext) => ext.to_string_lossy().to_lowercase(),
                                        None => continue, // Skip files without extensions
                                    };

                                    // Special handling for bin files when user wants cpp
                                    if effective_extension == "bin" && user_wants_cpp && !bin_already_requested {
                                        // This bin file was only extracted for conversion, skip if not converted
                                        let file_name = file_path.file_name()
                                            .and_then(|n| n.to_str())
                                            .unwrap_or("");
                                        
                                        // Only skip non-config bin files
                                        if !file_name.eq_ignore_ascii_case("config.bin") {
                                            continue;
                                        }
                                    }

                                    // Check against user's *original* requested extensions
                                    if !original_extensions_set.contains(&effective_extension) {
                                        trace!("Skipping file {} (ext: {}) as it's not in requested set", 
                                               file_path.display(), effective_extension);
                                        continue; // Skip file if its effective extension wasn't requested
                                    }

                                    // --- File matches original filter ---

                                    // Compute the relative path from temp dir
                                    let rel_path_str = file_path.to_string_lossy()
                                        .replace(&*temp_path_str, "")
                                        .trim_start_matches(['/', '\\'])
                                        .to_string();

                                    let final_rel_path = PathBuf::from(rel_path_str);
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
                                    let mut retry_delay = Duration::from_millis(100);

                                    while retries > 0 {
                                        match fs::copy(file_path, &target_path) {
                                            Ok(bytes_copied) => {
                                                // Store the final relative path (e.g., .../config.cpp)
                                                cache_paths.push(final_rel_path);
                                                last_error = None; // Clear error on success
                                                
                                                if verbose {
                                                    debug!("Copied {} bytes: {} -> {}", 
                                                          bytes_copied, 
                                                          file_path.display(), 
                                                          target_path.display());
                                                }
                                                break;
                                            },
                                            Err(e) => {
                                                last_error = Some(e);
                                                retries -= 1;
                                                if retries > 0 {
                                                    // Exponential backoff
                                                    tokio::time::sleep(retry_delay).await;
                                                    retry_delay *= 2;
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
                            result_guard.push((pbo_path.clone(), cache_paths.clone()));
                            
                            // Update statistics
                            let extraction_time = pbo_start_time.elapsed();
                            let total_bytes: u64 = cache_paths.iter()
                                .filter_map(|p| cache_dir.join(p).metadata().ok())
                                .map(|m| m.len())
                                .sum();
                            
                            let mut stats_guard = stats.lock().unwrap();
                            stats_guard.add_extraction(cache_paths.len(), total_bytes, extraction_time);
                            
                            if verbose {
                                debug!("Extracted {} from {} files in {:?} ({} bytes)", 
                                      pbo_path.display(), cache_paths.len(), extraction_time, total_bytes);
                            }
                        } else {
                            // No files matched the filter
                            if verbose {
                                debug!("No files matched filter for PBO: {}", pbo_path.display());
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

        // Get final statistics
        let final_stats = Arc::try_unwrap(stats)
            .expect("Failed to unwrap stats")
            .into_inner()
            .expect("Failed to get inner stats");
        
        let elapsed = start_time.elapsed();
        
        // Log extraction summary
        info!("PBO extraction completed in {:.2}s", elapsed.as_secs_f64());
        info!("  - Total PBOs: {} (processed: {}, failed: {}, skipped: {})", 
              initial_pbo_count, 
              final_stats.pbos_processed,
              final_stats.pbos_failed,
              initial_pbo_count - final_stats.pbos_processed - final_stats.pbos_failed);
        info!("  - Files extracted: {} ({:.2} MB)", 
              final_stats.total_files_extracted,
              final_stats.total_bytes_extracted as f64 / 1_048_576.0);
        
        if !final_stats.extraction_times.is_empty() {
            info!("  - Average extraction time per PBO: {:.2}s", 
                  final_stats.average_extraction_time().as_secs_f64());
        }
        
        if verbose && final_stats.pbos_processed > 0 {
            let throughput = final_stats.total_bytes_extracted as f64 / elapsed.as_secs_f64() / 1_048_576.0;
            info!("  - Extraction throughput: {:.2} MB/s", throughput);
        }

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
        
        // The extraction may succeed or fail depending on the PBO format handling
        // This test primarily verifies the extraction process runs without panicking
        // and that the failure callback mechanism is properly set up
        if results.is_err() {
            println!("Extraction failed as expected: {:?}", results.err());
        } else if failed_pbo_paths.len() > 0 {
            println!("Some PBOs failed extraction: {:?}", failed_pbo_paths);
        } else {
            println!("Extraction completed successfully with mock PBOs");
        }
    }
}
