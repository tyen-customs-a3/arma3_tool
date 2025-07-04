//! Core data types for PBO operations

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Information about a file within a PBO
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PboFileInfo {
    /// Path of the file within the PBO (using forward slashes)
    pub file_path: String,
    /// Uncompressed size of the file in bytes
    pub size: u64,
    /// Compressed size of the file in bytes (may be same as size if not compressed)
    pub compressed_size: u64,
    /// Timestamp of the file (Unix timestamp)
    pub timestamp: u64,
    /// Original size field from PBO header
    pub original_size: u64,
    /// Reserved field from PBO header
    pub reserved: u64,
    /// MIME type of the file
    pub mime_type: String,
}

impl PboFileInfo {
    /// Create a new PboFileInfo instance
    pub fn new(
        file_path: String,
        size: u64,
        compressed_size: u64,
        timestamp: u64,
        original_size: u64,
        reserved: u64,
        mime_type: String,
    ) -> Self {
        Self {
            file_path,
            size,
            compressed_size,
            timestamp,
            original_size,
            reserved,
            mime_type,
        }
    }

    /// Get the compression ratio as a percentage (0.0 to 1.0)
    pub fn compression_ratio(&self) -> f64 {
        if self.size == 0 {
            0.0
        } else {
            1.0 - (self.compressed_size as f64 / self.size as f64)
        }
    }

    /// Check if the file is compressed
    pub fn is_compressed(&self) -> bool {
        self.compressed_size < self.size
    }

    /// Get the file extension if present
    pub fn extension(&self) -> Option<&str> {
        std::path::Path::new(&self.file_path)
            .extension()
            .and_then(|ext| ext.to_str())
    }

    /// Get the filename without directory path
    pub fn filename(&self) -> Option<&str> {
        std::path::Path::new(&self.file_path)
            .file_name()
            .and_then(|name| name.to_str())
    }
}

/// Properties and metadata of a PBO file
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PboProperties {
    /// Version information
    pub version: Option<String>,
    /// Author information
    pub author: Option<String>,
    /// Prefix path for the PBO
    pub prefix: Option<String>,
    /// Custom properties from the PBO header
    pub custom_properties: HashMap<String, String>,
    /// Total number of files in the PBO
    pub file_count: usize,
    /// Total uncompressed size of all files
    pub total_size: u64,
    /// Total compressed size of all files
    pub total_compressed_size: u64,
    /// PBO checksum (SHA-1 hash)
    pub checksum: Option<String>,
}

impl PboProperties {
    /// Create a new PboProperties instance
    pub fn new() -> Self {
        Self {
            version: None,
            author: None,
            prefix: None,
            custom_properties: HashMap::new(),
            file_count: 0,
            total_size: 0,
            total_compressed_size: 0,
            checksum: None,
        }
    }

    /// Get the overall compression ratio for the PBO
    pub fn compression_ratio(&self) -> f64 {
        if self.total_size == 0 {
            0.0
        } else {
            1.0 - (self.total_compressed_size as f64 / self.total_size as f64)
        }
    }

    /// Get a custom property value
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.custom_properties.get(key)
    }

    /// Set a custom property
    pub fn set_property(&mut self, key: String, value: String) {
        self.custom_properties.insert(key, value);
    }

    /// Check if the PBO has a valid prefix
    pub fn has_prefix(&self) -> bool {
        self.prefix.as_ref().map_or(false, |p| !p.is_empty())
    }
}

impl Default for PboProperties {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result for a PBO file
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PboValidation {
    /// Whether the PBO is valid overall
    pub is_valid: bool,
    /// List of errors found during validation
    pub errors: Vec<ValidationIssue>,
    /// List of warnings found during validation
    pub warnings: Vec<ValidationIssue>,
    /// Checksum validation result
    pub checksum_valid: Option<bool>,
    /// Whether files are sorted correctly
    pub files_sorted: bool,
    /// Path to the validated PBO file
    pub pbo_path: PathBuf,
}

/// A validation issue (error or warning)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Type of the issue
    pub issue_type: ValidationIssueType,
    /// Human-readable description of the issue
    pub message: String,
    /// File path related to the issue (if applicable)
    pub file_path: Option<String>,
    /// Severity level
    pub severity: ValidationSeverity,
}

/// Types of validation issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationIssueType {
    /// Checksum mismatch
    ChecksumMismatch,
    /// Corrupted file header
    CorruptedHeader,
    /// Missing or invalid prefix
    InvalidPrefix,
    /// Files not sorted properly
    UnsortedFiles,
    /// Invalid file path
    InvalidFilePath,
    /// Missing required files
    MissingRequiredFiles,
    /// File size mismatch
    FileSizeMismatch,
    /// Invalid timestamp
    InvalidTimestamp,
    /// Unknown or unsupported MIME type
    UnsupportedMimeType,
    /// General corruption
    Corruption,
}

/// Severity levels for validation issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationSeverity {
    /// Critical error that prevents normal operation
    Critical,
    /// Error that may cause issues
    Error,
    /// Warning that should be addressed
    Warning,
    /// Informational message
    Info,
}

impl PboValidation {
    /// Create a new validation result
    pub fn new(pbo_path: PathBuf) -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            checksum_valid: None,
            files_sorted: true,
            pbo_path,
        }
    }

    /// Add an error to the validation result
    pub fn add_error(&mut self, issue_type: ValidationIssueType, message: String, file_path: Option<String>) {
        self.errors.push(ValidationIssue {
            issue_type,
            message,
            file_path,
            severity: ValidationSeverity::Error,
        });
        self.is_valid = false;
    }

    /// Add a warning to the validation result
    pub fn add_warning(&mut self, issue_type: ValidationIssueType, message: String, file_path: Option<String>) {
        self.warnings.push(ValidationIssue {
            issue_type,
            message,
            file_path,
            severity: ValidationSeverity::Warning,
        });
    }

    /// Add a critical error to the validation result
    pub fn add_critical_error(&mut self, issue_type: ValidationIssueType, message: String, file_path: Option<String>) {
        self.errors.push(ValidationIssue {
            issue_type,
            message,
            file_path,
            severity: ValidationSeverity::Critical,
        });
        self.is_valid = false;
    }

    /// Check if there are any critical errors
    pub fn has_critical_errors(&self) -> bool {
        self.errors.iter().any(|e| e.severity == ValidationSeverity::Critical)
    }

    /// Get the total number of issues (errors + warnings)
    pub fn total_issues(&self) -> usize {
        self.errors.len() + self.warnings.len()
    }

    /// Get all issues sorted by severity
    pub fn all_issues(&self) -> Vec<&ValidationIssue> {
        let mut issues: Vec<&ValidationIssue> = self.errors.iter().chain(self.warnings.iter()).collect();
        issues.sort_by(|a, b| {
            use ValidationSeverity::*;
            match (&a.severity, &b.severity) {
                (Critical, Critical) | (Error, Error) | (Warning, Warning) | (Info, Info) => std::cmp::Ordering::Equal,
                (Critical, _) => std::cmp::Ordering::Less,
                (_, Critical) => std::cmp::Ordering::Greater,
                (Error, _) => std::cmp::Ordering::Less,
                (_, Error) => std::cmp::Ordering::Greater,
                (Warning, Info) => std::cmp::Ordering::Less,
                (Info, Warning) => std::cmp::Ordering::Greater,
            }
        });
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbo_file_info_creation() {
        let file_info = PboFileInfo::new(
            "config.cpp".to_string(),
            1024,
            512,
            1640995200, // 2022-01-01 00:00:00 UTC
            1024,
            0,
            "text/plain".to_string(),
        );

        assert_eq!(file_info.file_path, "config.cpp");
        assert_eq!(file_info.size, 1024);
        assert_eq!(file_info.compressed_size, 512);
        assert!(file_info.is_compressed());
        assert_eq!(file_info.compression_ratio(), 0.5);
        assert_eq!(file_info.extension(), Some("cpp"));
        assert_eq!(file_info.filename(), Some("config.cpp"));
    }

    #[test]
    fn test_pbo_properties_creation() {
        let mut props = PboProperties::new();
        props.version = Some("1.0.0".to_string());
        props.set_property("custom".to_string(), "value".to_string());

        assert_eq!(props.version, Some("1.0.0".to_string()));
        assert_eq!(props.get_property("custom"), Some(&"value".to_string()));
        assert!(!props.has_prefix());
    }

    #[test]
    fn test_pbo_validation_creation() {
        let mut validation = PboValidation::new(PathBuf::from("test.pbo"));
        
        assert!(validation.is_valid);
        assert_eq!(validation.total_issues(), 0);
        
        validation.add_warning(
            ValidationIssueType::InvalidPrefix,
            "Missing prefix".to_string(),
            None,
        );
        
        assert!(validation.is_valid); // Still valid with just warnings
        assert_eq!(validation.total_issues(), 1);
        
        validation.add_error(
            ValidationIssueType::ChecksumMismatch,
            "Checksum does not match".to_string(),
            None,
        );
        
        assert!(!validation.is_valid); // Invalid with errors
        assert_eq!(validation.total_issues(), 2);
    }
}