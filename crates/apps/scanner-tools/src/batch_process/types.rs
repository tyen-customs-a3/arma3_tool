use std::collections::HashMap;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Serializable version of ParseWarning for reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableParseWarning {
    pub code: String,
    pub message: String,
    pub severity: String,
    pub file_path: String,
}

impl From<::parser_hpp::ParseWarning> for SerializableParseWarning {
    fn from(warning: ::parser_hpp::ParseWarning) -> Self {
        Self {
            code: warning.code,
            message: warning.message,
            severity: warning.severity,
            file_path: warning.file_path,
        }
    }
}

/// Represents the complete report of a batch parsing operation
#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    /// Timestamp when the report was created
    pub timestamp: String,
    
    /// List of files that were successfully processed
    pub successful_files: Vec<String>,
    
    /// List of files that failed to process
    pub failures: Vec<FileFailure>,
    
    /// Statistics about the processing
    pub stats: ReportStats,
    
    /// Whether processing was stopped early due to too many failures
    pub stopped_early: bool,
    
    /// Collected parser warnings: tuples of (file path, list of warnings for that file)
    pub collected_warnings: Vec<(String, Vec<SerializableParseWarning>)>,
}

impl Report {
    /// Create a new empty report
    pub fn new() -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        Self {
            timestamp: now.to_rfc3339(),
            successful_files: Vec::new(),
            failures: Vec::new(),
            stats: ReportStats::default(),
            stopped_early: false,
            collected_warnings: Vec::new(),
        }
    }
    
    /// Update statistics based on the current report state
    pub fn update_stats(&mut self) {
        let total_files = self.successful_files.len() + self.failures.len();
        let error_files = self.failures.iter()
            .filter(|f| f.error_severity == "Error")
            .count();
        let warning_files = self.failures.iter()
            .filter(|f| f.error_severity == "Warning")
            .count();
        
        self.stats = ReportStats {
            total_files,
            successful_files: self.successful_files.len(),
            failed_files: error_files,
            warning_files,
            timeout_files: self.failures.iter().filter(|f| f.is_timeout).count(),
            total_parser_warnings: self.collected_warnings.iter()
                .map(|(_, warnings)| warnings.len())
                .sum(),
        };
    }
}

/// Statistics about the batch parsing operation
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReportStats {
    /// Total number of files processed
    pub total_files: usize,
    
    /// Number of files that were successfully processed
    pub successful_files: usize,
    
    /// Number of files that failed to process with errors
    pub failed_files: usize,
    
    /// Number of files that had warnings but passed
    pub warning_files: usize,
    
    /// Number of files that timed out during processing
    pub timeout_files: usize,
    
    /// Total number of parser warnings across all files
    pub total_parser_warnings: usize,
}

/// Represents a failure to process a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFailure {
    /// Main error message
    pub error_message: String,
    
    /// List of diagnostic messages
    pub diagnostics: Vec<String>,
    
    /// Whether the failure was due to a timeout
    pub is_timeout: bool,
    
    /// Size of the file in bytes
    pub file_size: u64,
    
    /// Line number where the error occurred (if known)
    pub error_line_number: Option<usize>,
    
    /// Context around the error (if available)
    pub error_context: Option<String>,
    
    /// Category of the error
    pub error_category: String,
    
    /// Severity of the error (Warning or Error)
    pub error_severity: String,
    
    /// List of related files
    pub related_files: Vec<String>,
    
    /// Time taken to process the file in milliseconds
    pub parse_duration_ms: u64,
} 