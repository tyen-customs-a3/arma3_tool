use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Represents a processed class from a parsed file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedClass {
    /// Name of the class
    pub name: String,
    
    /// Parent class name, if any
    pub parent: Option<String>,
    
    /// Properties of the class as key-value pairs
    pub properties: Vec<(String, String)>,
    
    /// Path to the file where this class was found
    pub file_path: Option<PathBuf>,
}

/// Statistics about the class scanning process
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClassScanStats {
    /// Total number of files processed
    pub total_files: usize,
    
    /// Total number of classes found
    pub total_classes: usize,
    
    /// Number of empty files encountered
    pub empty_files: usize,
    
    /// Number of files containing classes
    pub files_with_classes: usize,
    
    /// Number of files that failed to parse
    pub error_files: usize,
    
    /// Paths to files that failed to parse
    pub error_file_paths: Vec<PathBuf>,
    
    /// Number of files that timed out during parsing
    pub timeout_files: usize,
    
    /// Paths to files that timed out during parsing
    pub timeout_file_paths: Vec<PathBuf>,
}

/// Configuration options for class scanning
#[derive(Debug, Clone)]
pub struct ClassScanOptions {
    /// Whether to log verbose error information
    pub verbose_errors: bool,
    
    /// Maximum number of files to process
    pub max_files: Option<usize>,
    
    /// Timeout duration for parsing a single file (in seconds)
    pub parse_timeout_seconds: u64,
    
    /// Number of parallel threads to use for scanning
    pub parallel_threads: Option<usize>,
}

impl Default for ClassScanOptions {
    fn default() -> Self {
        Self {
            verbose_errors: false,
            max_files: None,
            parse_timeout_seconds: 10,
            parallel_threads: None,
        }
    }
}

/// Result of a class scanning operation
#[derive(Debug, Clone)]
pub struct ClassScanResult {
    /// The processed classes found during scanning
    pub classes: Vec<ProcessedClass>,
    
    /// Statistics about the scanning process
    pub stats: ClassScanStats,
} 