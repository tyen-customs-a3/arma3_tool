use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use log::{debug, info, warn, error};
use tokio::task;
use tempfile::{tempdir, TempDir};
use walkdir::WalkDir;
use std::sync::{Arc, Mutex};
use indicatif::{ProgressBar, ProgressStyle};
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
    
    /// Extract a single PBO file to a temporary directory
    pub fn extract_to_temp(
        &self,
        pbo_path: &Path,
        extensions: &[String],
    ) -> Result<(TempDir, Vec<PathBuf>)> {
        debug!("Extracting PBO to temp: {}", pbo_path.display());
        
        // Create a temporary directory
        let temp_dir = tempdir()
            .context("Failed to create temporary directory")?;
            
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
        
        debug!("Extraction options: {:?}", options);
        
        // Extract the PBO
        self.pbo_api.extract_with_options(pbo_path, temp_dir.path(), options)
            .context(format!("Failed to extract PBO: {}", pbo_path.display()))?;
            
        // Catalog the extracted files
        let extracted_files = Self::catalog_files(temp_dir.path())?;
        debug!("Extracted {} files from {}", extracted_files.len(), pbo_path.display());
        
        Ok((temp_dir, extracted_files))
    }
    
    /// Move extracted files from temp directory to cache
    pub fn move_to_cache(
        &self,
        temp_dir: &Path,
        cache_dir: &Path,
        extracted_files: &[PathBuf],
    ) -> Result<Vec<PathBuf>> {
        debug!("Moving {} files from temp to cache", extracted_files.len());
        
        // Ensure cache directory exists
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir)
                .context("Failed to create cache directory")?;
        }
        
        let mut cache_paths = Vec::with_capacity(extracted_files.len());
        let temp_path_str = temp_dir.to_string_lossy();
        
        for file_path in extracted_files {
            // Compute the relative path
            let rel_path = file_path.to_string_lossy()
                .replace(&*temp_path_str, "")
                .trim_start_matches(['/', '\\'])
                .to_string();
                
            let target_path = cache_dir.join(&rel_path);
            
            // Ensure the target directory exists
            if let Some(parent) = target_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)
                        .context(format!("Failed to create directory: {}", parent.display()))?;
                }
            }
            
            // Move the file
            debug!("Moving {} to {}", file_path.display(), target_path.display());
            fs::copy(file_path, &target_path)
                .context(format!("Failed to copy file to cache: {}", file_path.display()))?;
                
            cache_paths.push(target_path);
        }
        
        Ok(cache_paths)
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
    ) -> Result<Vec<(PathBuf, Vec<PathBuf>)>> {
        if pbos.is_empty() {
            return Ok(Vec::new());
        }
        
        info!("Extracting {} PBOs to cache", pbos.len());
        
        // Setup progress bar if requested
        let progress_bar = if progress {
            let pb = ProgressBar::new(pbos.len() as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} PBOs ({eta})")
                .unwrap()
                .progress_chars("#>-"));
            Some(Arc::new(pb))
        } else {
            None
        };
        
        // Shared result collection
        let results = Arc::new(Mutex::new(Vec::with_capacity(pbos.len())));
        
        // Limit concurrency
        let semaphore = Arc::new(tokio::sync::Semaphore::new(self.threads));
        
        // Process PBOs in parallel
        let mut tasks = vec![];
        
        for pbo_path in pbos {
            let cache_dir = cache_dir.to_path_buf();
            let extensions = extensions.clone();
            let results = Arc::clone(&results);
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
                            },
                            Err(e) => {
                                error!("Task failure when cataloging files for {}: {}", pbo_path.display(), e);
                            }
                        }
                    },
                    Err(e) => {
                        error!("Failed to extract PBO {}: {}", pbo_path.display(), e);
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
    
    // Test that fails because we can't actually create a real PBO in tests
    // This is just to demonstrate the structure
    #[test]
    #[ignore]
    fn test_extract_to_temp() {
        let temp_dir = tempdir().unwrap();
        let pbo_path = create_mock_pbo(temp_dir.path());
        
        let processor = PboProcessor::new(30, 1);
        let extensions = vec!["sqf".to_string(), "hpp".to_string()];
        
        let result = processor.extract_to_temp(&pbo_path, &extensions);
        assert!(result.is_err()); // Will fail because we're not using a real PBO
    }
} 