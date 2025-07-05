use arma3_pbo::{PboApi, PboApiOps, PboOperationError};
use std::path::{Path, PathBuf};
use async_trait::async_trait;
use log::{debug, warn, info};
use walkdir::WalkDir;
use std::fs;
use std::sync::Arc;
use tokio::task;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Trait for abstracting PBO extraction operations
#[async_trait]
pub trait PboExtractor: Send + Sync {
    /// Extract files matching a filter pattern from a PBO
    async fn extract_filtered(
        &self,
        pbo_path: &Path,
        output_dir: &Path,
        filter: &str,
    ) -> Result<Vec<PathBuf>, PboOperationError>;
    
    /// Extract files from multiple PBOs in parallel
    async fn extract_filtered_parallel(
        &self,
        pbo_paths: &[&Path],
        output_dir: &Path,
        filter: &str,
        max_concurrent: usize,
    ) -> Result<HashMap<PathBuf, Vec<PathBuf>>, PboOperationError>;
    
    /// List all files that were extracted to a directory
    async fn list_extracted_files(
        &self,
        output_dir: &Path,
    ) -> Result<Vec<PathBuf>, PboOperationError>;
    
    /// Convert binary config files to text format
    /// Returns a list of (original_path, converted_path) tuples for successful conversions
    async fn convert_binary_configs(
        &self,
        extracted_files: &[PathBuf],
        output_dir: &Path,
    ) -> Result<Vec<(PathBuf, PathBuf)>, PboOperationError>;
    
    /// Benchmark extraction performance
    async fn benchmark_extraction(
        &self,
        pbo_path: &Path,
        output_dir: &Path,
        filter: &str,
        iterations: usize,
    ) -> Result<ExtractionBenchmark, PboOperationError>;
}

/// Benchmark results for PBO extraction operations
#[derive(Debug, Clone)]
pub struct ExtractionBenchmark {
    pub iterations: usize,
    pub total_duration: Duration,
    pub average_duration: Duration,
    pub min_duration: Duration,
    pub max_duration: Duration,
    pub files_extracted: usize,
    pub total_bytes: u64,
    pub throughput_mb_per_sec: f64,
    pub files_per_second: f64,
}

impl ExtractionBenchmark {
    pub fn new(iterations: usize) -> Self {
        Self {
            iterations,
            total_duration: Duration::ZERO,
            average_duration: Duration::ZERO,
            min_duration: Duration::MAX,
            max_duration: Duration::ZERO,
            files_extracted: 0,
            total_bytes: 0,
            throughput_mb_per_sec: 0.0,
            files_per_second: 0.0,
        }
    }
    
    pub fn add_iteration(&mut self, duration: Duration, files: usize, bytes: u64) {
        self.total_duration += duration;
        self.files_extracted = files; // Use the last iteration's file count
        self.total_bytes = bytes; // Use the last iteration's byte count
        
        if duration < self.min_duration {
            self.min_duration = duration;
        }
        if duration > self.max_duration {
            self.max_duration = duration;
        }
    }
    
    pub fn finalize(&mut self) {
        if self.iterations > 0 {
            self.average_duration = self.total_duration / self.iterations as u32;
            
            // Calculate throughput based on average duration
            let avg_secs = self.average_duration.as_secs_f64();
            if avg_secs > 0.0 {
                self.throughput_mb_per_sec = (self.total_bytes as f64 / 1_048_576.0) / avg_secs;
                self.files_per_second = self.files_extracted as f64 / avg_secs;
            }
        }
    }
    
    pub fn print_summary(&self) {
        println!("=== PBO Extraction Benchmark Results ===");
        println!("Iterations: {}", self.iterations);
        println!("Files extracted: {}", self.files_extracted);
        println!("Total bytes: {} ({:.2} MB)", self.total_bytes, self.total_bytes as f64 / 1_048_576.0);
        println!("Average duration: {:.2}ms", self.average_duration.as_millis());
        println!("Min duration: {:.2}ms", self.min_duration.as_millis());
        println!("Max duration: {:.2}ms", self.max_duration.as_millis());
        println!("Throughput: {:.2} MB/s", self.throughput_mb_per_sec);
        println!("Files per second: {:.2}", self.files_per_second);
    }
}

/// HEMTT-based PBO extractor implementation
pub struct HemttPboExtractor {
    pbo_api: PboApi,
    timeout_seconds: u64,
}

impl HemttPboExtractor {
    /// Create a new HEMTT PBO extractor with the specified timeout
    pub fn new(timeout_seconds: u64) -> Self {
        Self {
            pbo_api: PboApi::new(timeout_seconds as u32),
            timeout_seconds,
        }
    }
    
    /// Create a new HEMTT PBO extractor with custom configuration
    pub fn with_config(timeout_seconds: u64) -> Self {
        Self::new(timeout_seconds)
    }
    
    /// Extract files from a single PBO for use in parallel operations
    async fn extract_single_pbo(
        &self,
        pbo_path: &Path,
        output_dir: &Path,
        filter: &str,
    ) -> Result<Vec<PathBuf>, PboOperationError> {
        self.extract_filtered(pbo_path, output_dir, filter).await
    }
    
    /// Get the timeout value for creating new API instances
    #[allow(dead_code)]
    fn get_timeout(&self) -> u64 {
        self.timeout_seconds
    }
    
    /// Try to convert a binary config file to text format
    /// Returns Ok(true) if conversion succeeded, Ok(false) if no converter available
    async fn try_convert_binary_config(
        &self,
        binary_path: &Path,
        output_path: &Path,
    ) -> Result<bool, PboOperationError> {
        // Check if the binary file exists
        if !binary_path.exists() {
            return Err(PboOperationError::file_not_found(binary_path));
        }
        
        // Check if we can use hemtt-config to debinarize
        // For now, we'll check if the file is actually a binary config by reading the header
        match fs::read(binary_path) {
            Ok(data) => {
                // Check for raP signature (binary config marker)
                if data.len() >= 4 && &data[0..4] == b"\0raP" {
                    debug!("Detected binary config with raP signature");
                    
                    // TODO: When HEMTT provides debinarize functionality, use it here
                    // For now, we'll create a placeholder file indicating conversion is needed
                    let placeholder_content = format!(
                        "// This file was detected as a binary config (config.bin)\n\
                         // Binary conversion is not yet implemented\n\
                         // Original file: {}\n",
                        binary_path.file_name().unwrap_or_default().to_string_lossy()
                    );
                    
                    // Create parent directories if needed
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)
                            .map_err(|e| PboOperationError::io_error("creating output directory", e))?;
                    }
                    
                    // Write the placeholder file
                    fs::write(output_path, placeholder_content)
                        .map_err(|e| PboOperationError::io_error("writing placeholder file", e))?;
                    
                    // Return false to indicate we didn't actually convert (just created placeholder)
                    // When real conversion is available, return true
                    Ok(false)
                } else {
                    // Not a binary config, might be a text file already
                    debug!("File does not have binary config signature, might be text already");
                    Ok(false)
                }
            }
            Err(e) => {
                Err(PboOperationError::io_error("reading binary config", e))
            }
        }
    }
}

#[async_trait]
impl PboExtractor for HemttPboExtractor {
    async fn extract_filtered(
        &self,
        pbo_path: &Path,
        output_dir: &Path,
        filter: &str,
    ) -> Result<Vec<PathBuf>, PboOperationError> {
        debug!("Extracting PBO {} with filter '{}' to {}", 
               pbo_path.display(), filter, output_dir.display());
        
        // Check if this is a mission PBO (ends with .MapName.pbo)
        let is_mission_pbo = if let Some(file_name) = pbo_path.file_name() {
            let name = file_name.to_string_lossy();
            // Mission PBOs have format: missionName.MapName.pbo
            name.matches('.').count() >= 2 && name.ends_with(".pbo")
        } else {
            false
        };
        
        // For mission PBOs, use the filename (without .pbo) as the subdirectory
        // For mod PBOs, use the internal prefix
        let (extract_base_dir, dir_prefix) = if is_mission_pbo {
            // Extract mission name without .pbo extension
            let mission_name = pbo_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown_mission");
            let mission_dir = output_dir.join(mission_name);
            fs::create_dir_all(&mission_dir)
                .map_err(|e| PboOperationError::io_error("creating mission directory", e))?;
            (mission_dir, Some(mission_name.to_string()))
        } else {
            // For mods, get the PBO properties to check for a prefix
            let properties = self.pbo_api.get_properties(pbo_path).await?;
            let prefix = properties.prefix.clone()
                .map(|p| p.replace('\\', "/")); // Normalize to forward slashes
            (output_dir.to_path_buf(), prefix)
        };
        
        // OPTIMIZATION: Use HEMTT's native filtering instead of post-extraction filtering
        // This is much more efficient as it only extracts files that match the filter
        let target_extract_dir = if is_mission_pbo {
            extract_base_dir.clone()
        } else if let Some(ref prefix_str) = dir_prefix {
            if !prefix_str.is_empty() {
                output_dir.join(prefix_str)
            } else {
                output_dir.to_path_buf()
            }
        } else {
            output_dir.to_path_buf()
        };
        
        // Ensure the target directory exists
        fs::create_dir_all(&target_extract_dir)
            .map_err(|e| PboOperationError::io_error("creating target directory", e))?;
        
        // Use HEMTT's direct filtered extraction - this is the key optimization!
        self.pbo_api.extract_filtered(pbo_path, &target_extract_dir, filter).await?;
        
        // Collect the extracted files by walking the target directory
        let mut extracted_files = Vec::new();
        
        for entry in WalkDir::new(&target_extract_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                // Calculate the relative path from the output_dir
                if let Ok(rel_path) = entry.path().strip_prefix(output_dir) {
                    extracted_files.push(rel_path.to_path_buf());
                } else {
                    warn!("Failed to calculate relative path for {}", entry.path().display());
                }
            }
        }
        
        if let Some(ref prefix_str) = dir_prefix {
            if !prefix_str.is_empty() {
                if is_mission_pbo {
                    info!("Extracted {} filtered files from mission PBO to '{}'", extracted_files.len(), prefix_str);
                } else {
                    info!("Extracted {} filtered files from mod PBO with prefix '{}'", extracted_files.len(), prefix_str);
                }
            }
        }
        
        Ok(extracted_files)
    }
    
    async fn extract_filtered_parallel(
        &self,
        pbo_paths: &[&Path],
        output_dir: &Path,
        filter: &str,
        max_concurrent: usize,
    ) -> Result<HashMap<PathBuf, Vec<PathBuf>>, PboOperationError> {
        info!("Starting parallel extraction of {} PBOs with filter '{}'", pbo_paths.len(), filter);
        
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));
        let mut tasks = Vec::new();
        
        // Create a task for each PBO
        for &pbo_path in pbo_paths {
            let pbo_path = pbo_path.to_path_buf();
            let output_dir = output_dir.to_path_buf();
            let filter = filter.to_string();
            let semaphore = semaphore.clone();
            
            // Create a new PBO API instance for each task
            let timeout_seconds = self.timeout_seconds;
            
            let task = task::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                
                debug!("Starting extraction of PBO: {}", pbo_path.display());
                
                // Create a temporary extractor for this task
                let extractor = HemttPboExtractor::new(timeout_seconds);
                
                let result = extractor.extract_single_pbo(&pbo_path, &output_dir, &filter).await;
                
                match result {
                    Ok(files) => {
                        info!("Successfully extracted {} files from {}", files.len(), pbo_path.display());
                        Ok((pbo_path, files))
                    }
                    Err(e) => {
                        warn!("Failed to extract from {}: {}", pbo_path.display(), e);
                        Err((pbo_path, e))
                    }
                }
            });
            
            tasks.push(task);
        }
        
        // Wait for all tasks to complete
        let mut results = HashMap::new();
        let mut errors = Vec::new();
        
        for task in tasks {
            match task.await {
                Ok(Ok((pbo_path, files))) => {
                    results.insert(pbo_path, files);
                }
                Ok(Err((pbo_path, error))) => {
                    errors.push((pbo_path, error));
                }
                Err(join_error) => {
                    warn!("Task join error: {}", join_error);
                }
            }
        }
        
        // Report summary
        info!("Parallel extraction completed: {} successful, {} failed", 
              results.len(), errors.len());
        
        if !errors.is_empty() {
            warn!("Errors occurred during parallel extraction:");
            for (pbo_path, error) in &errors {
                warn!("  {}: {}", pbo_path.display(), error);
            }
        }
        
        // Return results (successful extractions)
        Ok(results)
    }
    
    async fn list_extracted_files(
        &self,
        output_dir: &Path,
    ) -> Result<Vec<PathBuf>, PboOperationError> {
        let mut extracted_files = Vec::new();
        
        // Walk the output directory to find all extracted files
        for entry in WalkDir::new(output_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                // Get the relative path from the output directory
                if let Ok(rel_path) = entry.path().strip_prefix(output_dir) {
                    extracted_files.push(rel_path.to_path_buf());
                } else {
                    warn!("Failed to get relative path for {}", entry.path().display());
                }
            }
        }
        
        debug!("Found {} extracted files in {}", 
               extracted_files.len(), output_dir.display());
        Ok(extracted_files)
    }
    
    async fn convert_binary_configs(
        &self,
        extracted_files: &[PathBuf],
        output_dir: &Path,
    ) -> Result<Vec<(PathBuf, PathBuf)>, PboOperationError> {
        let mut conversions = Vec::new();
        
        for rel_path in extracted_files {
            let full_path = output_dir.join(rel_path);
            
            // Check if this is a binary config file
            if let Some(filename) = rel_path.file_name() {
                if is_binary_config(&filename.to_string_lossy()) {
                    debug!("Found binary config file: {}", rel_path.display());
                    
                    // Get the converted filename
                    let converted_path = get_converted_config_name(rel_path);
                    let full_converted_path = output_dir.join(&converted_path);
                    
                    // Try to convert the binary config
                    match self.try_convert_binary_config(&full_path, &full_converted_path).await {
                        Ok(true) => {
                            info!("Successfully converted {} to {}", 
                                  rel_path.display(), converted_path.display());
                            conversions.push((rel_path.clone(), converted_path));
                            
                            // Remove the original binary file after successful conversion
                            if let Err(e) = fs::remove_file(&full_path) {
                                warn!("Failed to remove original binary file {}: {}", 
                                      full_path.display(), e);
                            }
                        }
                        Ok(false) => {
                            // Conversion not available, keep the original file
                            debug!("Binary conversion not available for {}", rel_path.display());
                        }
                        Err(e) => {
                            warn!("Failed to convert binary config {}: {}", 
                                  rel_path.display(), e);
                            // Keep the original file on error
                        }
                    }
                }
            }
        }
        
        if !conversions.is_empty() {
            info!("Converted {} binary config files", conversions.len());
        }
        
        Ok(conversions)
    }
    
    async fn benchmark_extraction(
        &self,
        pbo_path: &Path,
        output_dir: &Path,
        filter: &str,
        iterations: usize,
    ) -> Result<ExtractionBenchmark, PboOperationError> {
        info!("Starting benchmark of {} iterations for PBO: {}", iterations, pbo_path.display());
        
        let mut benchmark = ExtractionBenchmark::new(iterations);
        
        // Create a temporary directory for benchmark extractions
        let bench_dir = output_dir.join(".benchmark_temp");
        
        for i in 0..iterations {
            let iteration_dir = bench_dir.join(format!("iteration_{}", i));
            
            // Clean up previous iteration if it exists
            if iteration_dir.exists() {
                fs::remove_dir_all(&iteration_dir)
                    .map_err(|e| PboOperationError::io_error("cleaning benchmark directory", e))?;
            }
            
            fs::create_dir_all(&iteration_dir)
                .map_err(|e| PboOperationError::io_error("creating benchmark directory", e))?;
            
            // Time the extraction
            let start = Instant::now();
            let extracted_files = self.extract_filtered(pbo_path, &iteration_dir, filter).await?;
            let duration = start.elapsed();
            
            // Calculate total bytes extracted
            let mut total_bytes = 0u64;
            for rel_path in &extracted_files {
                let full_path = iteration_dir.join(rel_path);
                if let Ok(metadata) = fs::metadata(&full_path) {
                    total_bytes += metadata.len();
                }
            }
            
            benchmark.add_iteration(duration, extracted_files.len(), total_bytes);
            
            debug!("Benchmark iteration {}: {:.2}ms, {} files, {} bytes", 
                   i + 1, duration.as_millis(), extracted_files.len(), total_bytes);
        }
        
        // Clean up benchmark directory
        let _ = fs::remove_dir_all(&bench_dir);
        
        benchmark.finalize();
        
        info!("Benchmark completed - Average: {:.2}ms, Throughput: {:.2} MB/s", 
              benchmark.average_duration.as_millis(), benchmark.throughput_mb_per_sec);
        
        Ok(benchmark)
    }
}

/// Check if a filename should be treated as a binary config that needs conversion
pub fn is_binary_config(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower == "config.bin" || lower.ends_with("/config.bin") || lower.ends_with("\\config.bin")
}

/// Get the converted filename for a binary config (config.bin -> config.cpp)
pub fn get_converted_config_name(path: &Path) -> PathBuf {
    let mut new_path = path.to_path_buf();
    new_path.set_extension("cpp");
    new_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_binary_config_detection() {
        assert!(is_binary_config("config.bin"));
        assert!(is_binary_config("Config.BIN"));
        assert!(is_binary_config("some/path/config.bin"));
        assert!(is_binary_config("some\\path\\config.bin"));
        assert!(!is_binary_config("config.cpp"));
        assert!(!is_binary_config("not_config.bin"));
    }

    #[test]
    fn test_converted_config_name() {
        let path = Path::new("some/path/config.bin");
        let converted = get_converted_config_name(path);
        assert_eq!(converted, Path::new("some/path/config.cpp"));
    }
    
    #[tokio::test]
    async fn test_binary_config_conversion() {
        let temp_dir = TempDir::new().unwrap();
        let extractor = HemttPboExtractor::new(30);
        
        // Create a mock binary config file with raP signature
        let config_bin_path = temp_dir.path().join("config.bin");
        let binary_data = b"\0raP\x00\x00\x00\x00mock binary config data";
        fs::write(&config_bin_path, binary_data).unwrap();
        
        // Test conversion
        let extracted_files = vec![PathBuf::from("config.bin")];
        let result = extractor.convert_binary_configs(&extracted_files, temp_dir.path()).await;
        
        assert!(result.is_ok());
        let conversions = result.unwrap();
        
        // Since we don't have actual debinarize, it should create a placeholder
        // but return Ok(false) from try_convert_binary_config, so no conversions
        assert_eq!(conversions.len(), 0);
        
        // The original file should still exist
        assert!(config_bin_path.exists());
    }
    
    #[tokio::test]
    async fn test_non_binary_file_not_converted() {
        let temp_dir = TempDir::new().unwrap();
        let extractor = HemttPboExtractor::new(30);
        
        // Create a regular text file named config.bin
        let config_bin_path = temp_dir.path().join("config.bin");
        fs::write(&config_bin_path, "class CfgPatches {};").unwrap();
        
        // Test conversion
        let extracted_files = vec![PathBuf::from("config.bin")];
        let result = extractor.convert_binary_configs(&extracted_files, temp_dir.path()).await;
        
        assert!(result.is_ok());
        let conversions = result.unwrap();
        assert_eq!(conversions.len(), 0);
        
        // The original file should still exist
        assert!(config_bin_path.exists());
    }
}