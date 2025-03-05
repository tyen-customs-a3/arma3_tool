mod file_collector;
mod parser;
mod progress;

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use anyhow::{Result, Context};
use log::{debug, warn};
use std::collections::HashSet;
use once_cell::sync::Lazy;

use crate::code_scanner::class::types::ClassScanOptions;
use cpp_parser::Class;

// Re-export from submodules
pub use file_collector::FileCollector;
pub use parser::ClassParser;
pub use progress::ProgressTracker;

// Thread-safe storage for error and timeout files
static ERROR_FILES: Lazy<Mutex<HashSet<PathBuf>>> = Lazy::new(|| Mutex::new(HashSet::new()));
static TIMEOUT_FILES: Lazy<Mutex<HashSet<PathBuf>>> = Lazy::new(|| Mutex::new(HashSet::new()));

/// Class scanner responsible for finding and parsing class files
#[derive(Debug)]
pub struct ClassScanner {
    /// Configuration options for scanning
    options: ClassScanOptions,
    
    /// Output directory for logs and temporary files
    output_dir: PathBuf,
    
    /// File collector for finding class files
    file_collector: FileCollector,
    
    /// Class parser for parsing class files
    parser: ClassParser,
    
    /// Progress tracker for displaying progress
    progress_tracker: ProgressTracker,
}

impl ClassScanner {
    /// Create a new class scanner with the given options and output directory
    pub fn new(options: ClassScanOptions, output_dir: impl AsRef<Path>) -> Self {
        let output_dir = output_dir.as_ref().to_path_buf();
        
        Self {
            options: options.clone(),
            output_dir: output_dir.clone(),
            file_collector: FileCollector::new(),
            parser: ClassParser::new(options, output_dir.clone()),
            progress_tracker: ProgressTracker::new(),
        }
    }
    
    /// Create a new class scanner with default options
    pub fn with_defaults(output_dir: impl AsRef<Path>) -> Self {
        Self::new(ClassScanOptions::default(), output_dir)
    }
    
    /// Collect all .cpp and .hpp files from the input directory
    pub fn collect_files(&self, input_dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
        self.file_collector.collect_files(input_dir)
    }
    
    /// Parse a single file and return the classes found in it
    pub fn parse_file(&self, file: impl AsRef<Path>) -> Result<Vec<Class>> {
        self.parser.parse_file(file)
    }
    
    /// Parse a single file with a timeout and return the classes found in it
    pub fn parse_file_with_timeout(&self, file: impl AsRef<Path>) -> Result<(Vec<Class>, bool)> {
        self.parser.parse_file_with_timeout(file, self.options.parse_timeout_seconds)
    }
    
    /// Scan files in parallel and return the classes found in each file
    pub fn scan_files_parallel(&self, files: &[PathBuf]) -> Result<Vec<(PathBuf, Vec<Class>)>> {
        // Clear the error and timeout files
        if let Ok(mut error_files) = ERROR_FILES.lock() {
            error_files.clear();
        }
        if let Ok(mut timeout_files) = TIMEOUT_FILES.lock() {
            timeout_files.clear();
        }
        
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context("Failed to create output directory")?;
        
        // Limit the number of files if specified
        let files = if let Some(max_files) = self.options.max_files {
            debug!("Limiting to {} files", max_files);
            files.iter().take(max_files).cloned().collect::<Vec<_>>()
        } else {
            files.to_vec()
        };
        
        // Configure thread pool if specified
        if let Some(threads) = self.options.parallel_threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build_global()
                .unwrap_or_else(|e| warn!("Failed to configure thread pool: {}", e));
        }
        
        // Process files in parallel with progress tracking
        let results = self.progress_tracker.track_parallel_progress(
            &files,
            |file| {
                // Parse the file with timeout
                match self.parse_file_with_timeout(file) {
                    Ok((classes, _timed_out)) => {
                        if !classes.is_empty() {
                            Some((file.clone(), classes))
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            }
        );
        
        // Log timeout files if any
        if let Ok(timeout_files) = TIMEOUT_FILES.lock() {
            if !timeout_files.is_empty() {
                warn!("Found {} files that timed out during parsing", timeout_files.len());
                // Convert HashSet to Vec for the log_timeout_files method
                let timeout_vec: Vec<PathBuf> = timeout_files.iter().cloned().collect();
                self.parser.log_timeout_files(&timeout_vec, &self.output_dir);
            }
        }
        
        Ok(results)
    }
    
    /// Get the list of files that failed to parse
    pub fn get_error_files() -> Vec<PathBuf> {
        ERROR_FILES.lock().map(|files| files.iter().cloned().collect()).unwrap_or_default()
    }
    
    /// Get the list of files that timed out during parsing
    pub fn get_timeout_files() -> Vec<PathBuf> {
        TIMEOUT_FILES.lock().map(|files| files.iter().cloned().collect()).unwrap_or_default()
    }
    
    /// Add a file to the error files list
    pub fn add_error_file(file: impl AsRef<Path>) {
        if let Ok(mut error_files) = ERROR_FILES.lock() {
            error_files.insert(file.as_ref().to_path_buf());
        }
    }
    
    /// Add a file to the timeout files list
    pub fn add_timeout_file(file: impl AsRef<Path>) {
        if let Ok(mut timeout_files) = TIMEOUT_FILES.lock() {
            timeout_files.insert(file.as_ref().to_path_buf());
        }
    }
} 