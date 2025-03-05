use std::path::{Path, PathBuf};
use std::fs;
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use std::io::Write;

use anyhow::{Result, Context, anyhow};
use cpp_parser::{Class, parse_cpp};
use log::{debug, warn, error, trace};

use crate::code_scanner::class::types::ClassScanOptions;
use super::ClassScanner;

/// Class parser for parsing class files
#[derive(Debug)]
pub struct ClassParser {
    /// Configuration options for parsing
    options: ClassScanOptions,
    
    /// Output directory for logs and temporary files
    output_dir: PathBuf,
}

impl ClassParser {
    /// Create a new class parser with the given options and output directory
    pub fn new(options: ClassScanOptions, output_dir: impl AsRef<Path>) -> Self {
        Self {
            options,
            output_dir: output_dir.as_ref().to_path_buf(),
        }
    }
    
    /// Parse a single file and return the classes found in it
    pub fn parse_file(&self, file: impl AsRef<Path>) -> Result<Vec<Class>> {
        let file = file.as_ref();
        debug!("Processing file: {}", file.display());
        
        // Read the file content
        let content = fs::read_to_string(file)
            .with_context(|| format!("Failed to read file {}", file.display()))?;
        
        if content.trim().is_empty() {
            warn!("Empty file found: {}", file.display());
            return Ok(Vec::new());
        }
        
        trace!("File size: {} bytes, starting parse", content.len());
        
        // Parse the content
        match parse_cpp(&content) {
            Ok(classes) => {
                if classes.is_empty() {
                    debug!("No classes found in file: {}", file.display());
                } else {
                    debug!("Found {} classes in {}", classes.len(), file.display());
                    
                    // Only generate class names list for trace level logging
                    if log::log_enabled!(log::Level::Trace) {
                        let class_names = classes.iter()
                            .map(|c| c.name.clone().unwrap_or_else(|| "UnnamedClass".to_string()))
                            .collect::<Vec<_>>()
                            .join(", ");
                        trace!("Class names: {}", class_names);
                    }
                }
                Ok(classes)
            }
            Err(e) => {
                // Log detailed error information
                let err = anyhow!("Failed to parse file {}: {}", file.display(), e);
                error!("{}", err);
                
                // Try to extract a snippet of the problematic content and log to file
                if self.options.verbose_errors {
                    self.log_parse_error(file, &e, &content);
                }
                
                // Add to error files
                ClassScanner::add_error_file(file);
                
                Err(err)
            }
        }
    }
    
    /// Parse a single file with a timeout and return the classes found in it
    pub fn parse_file_with_timeout(&self, file: impl AsRef<Path>, timeout_seconds: u64) -> Result<(Vec<Class>, bool)> {
        let file = file.as_ref();
        debug!("Processing file with timeout: {}", file.display());
        
        // Create a channel to communicate between threads
        let (sender, receiver) = mpsc::channel();
        let file_path = file.to_path_buf();
        let output_dir = self.output_dir.clone();
        let verbose_errors = self.options.verbose_errors;
        
        // Spawn a thread to parse the file
        let parse_thread = thread::spawn(move || {
            // Create a new parser for this thread
            let parser = ClassParser::new(
                ClassScanOptions {
                    verbose_errors,
                    ..ClassScanOptions::default()
                },
                output_dir
            );
            
            // Parse the file
            let result = parser.parse_file(&file_path);
            
            // Send the result back to the main thread
            let _ = sender.send(result);
        });
        
        // Wait for the thread to complete or timeout
        let timeout_duration = Duration::from_secs(timeout_seconds);
        let result = match receiver.recv_timeout(timeout_duration) {
            Ok(result) => {
                // Thread completed within timeout
                let classes = result?;
                Ok((classes, false))
            }
            Err(_) => {
                // Timeout occurred
                warn!("Timeout parsing file: {}", file.display());
                
                // Add to timeout files
                ClassScanner::add_timeout_file(file);
                
                // Return empty classes with timeout flag
                Ok((Vec::new(), true))
            }
        };
        
        // Ensure the thread is properly cleaned up
        let _ = parse_thread.join();
        
        result
    }
    
    /// Helper function to log parse errors to a file
    pub fn log_parse_error(&self, file: &Path, error: &impl std::fmt::Display, content: &str) {
        if let Some(error_location) = error.to_string().find("line") {
            let error_info = &error.to_string()[error_location..];
            
            // Create a thread-safe error log file
            let error_file_name = format!(
                "parse_error_{}.log",
                file.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .replace(|c: char| !c.is_alphanumeric(), "_")
            );
            
            let error_file_path = self.output_dir.join(error_file_name);
            
            // Use a more robust approach to write the error file
            match std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&error_file_path)
            {
                Ok(mut file_handle) => {
                    let _ = writeln!(file_handle, "Error parsing file: {}", file.display());
                    let _ = writeln!(file_handle, "Error details: {}", error);
                    let _ = writeln!(file_handle, "Location: {}", error_info);
                    let _ = writeln!(file_handle, "\nFile content:\n{}", content);
                    debug!("Wrote detailed error information to {}", error_file_path.display());
                }
                Err(write_err) => {
                    error!("Failed to write error log file: {}", write_err);
                }
            }
        }
    }
    
    /// Helper function to log timeout files to a file
    pub fn log_timeout_files(&self, timeout_files: &[PathBuf], output_dir: &Path) {
        if timeout_files.is_empty() {
            return;
        }
        
        let timeout_log_path = output_dir.join("timeout_files.log");
        
        match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&timeout_log_path)
        {
            Ok(mut file_handle) => {
                let _ = writeln!(file_handle, "Files that timed out during parsing:");
                for file in timeout_files {
                    let _ = writeln!(file_handle, "{}", file.display());
                }
                debug!("Wrote timeout files list to {}", timeout_log_path.display());
            }
            Err(write_err) => {
                error!("Failed to write timeout log file: {}", write_err);
            }
        }
    }
} 