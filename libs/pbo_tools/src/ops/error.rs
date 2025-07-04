//! Error handling for PBO operations

use std::path::PathBuf;
use thiserror::Error;

/// Result type for PBO operations
pub type PboOperationResult<T> = Result<T, PboOperationError>;

/// Comprehensive error type for PBO operations
#[derive(Error, Debug)]
pub enum PboOperationError {
    /// File not found or inaccessible
    #[error("PBO file not found: {path}")]
    FileNotFound { path: PathBuf },

    /// Invalid PBO format or structure
    #[error("Invalid PBO format: {reason}")]
    InvalidFormat { reason: String },

    /// File extraction failed
    #[error("Failed to extract file '{file_path}' from PBO: {reason}")]
    ExtractionFailed { file_path: String, reason: String },

    /// File within PBO not found
    #[error("File '{file_path}' not found in PBO")]
    FileNotFoundInPbo { file_path: String },

    /// I/O operation failed
    #[error("I/O operation failed: {operation}")]
    IoError {
        operation: String,
        #[source]
        source: std::io::Error,
    },

    /// Validation failed
    #[error("PBO validation failed: {reason}")]
    ValidationFailed { reason: String },

    /// Checksum verification failed
    #[error("Checksum verification failed for PBO: {path}")]
    ChecksumMismatch { path: PathBuf },

    /// Permission denied
    #[error("Permission denied: {operation}")]
    PermissionDenied { operation: String },

    /// Unsupported operation or feature
    #[error("Unsupported operation: {operation}")]
    UnsupportedOperation { operation: String },

    /// Invalid path or filename
    #[error("Invalid path: {path}")]
    InvalidPath { path: String },

    /// Resource exhaustion (memory, disk space, etc.)
    #[error("Resource exhausted: {resource}")]
    ResourceExhausted { resource: String },

    /// Timeout during operation
    #[error("Operation timed out: {operation}")]
    Timeout { operation: String },

    /// HEMTT library error
    #[error("HEMTT library error: {0}")]
    HemttError(#[from] hemtt_pbo::Error),

    /// Multiple errors occurred
    #[error("Multiple errors occurred: {count} errors")]
    MultipleErrors {
        count: usize,
        errors: Vec<PboOperationError>,
    },

    /// Unknown or unexpected error
    #[error("Unknown error: {message}")]
    Unknown { message: String },
}

impl PboOperationError {
    /// Create a new file not found error
    pub fn file_not_found(path: impl Into<PathBuf>) -> Self {
        Self::FileNotFound { path: path.into() }
    }

    /// Create a new invalid format error
    pub fn invalid_format(reason: impl Into<String>) -> Self {
        Self::InvalidFormat {
            reason: reason.into(),
        }
    }

    /// Create a new extraction failed error
    pub fn extraction_failed(file_path: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::ExtractionFailed {
            file_path: file_path.into(),
            reason: reason.into(),
        }
    }

    /// Create a new file not found in PBO error
    pub fn file_not_found_in_pbo(file_path: impl Into<String>) -> Self {
        Self::FileNotFoundInPbo {
            file_path: file_path.into(),
        }
    }

    /// Create a new I/O error
    pub fn io_error(operation: impl Into<String>, source: std::io::Error) -> Self {
        Self::IoError {
            operation: operation.into(),
            source,
        }
    }

    /// Create a new validation failed error
    pub fn validation_failed(reason: impl Into<String>) -> Self {
        Self::ValidationFailed {
            reason: reason.into(),
        }
    }

    /// Create a new checksum mismatch error
    pub fn checksum_mismatch(path: impl Into<PathBuf>) -> Self {
        Self::ChecksumMismatch { path: path.into() }
    }

    /// Create a new permission denied error
    pub fn permission_denied(operation: impl Into<String>) -> Self {
        Self::PermissionDenied {
            operation: operation.into(),
        }
    }

    /// Create a new unsupported operation error
    pub fn unsupported_operation(operation: impl Into<String>) -> Self {
        Self::UnsupportedOperation {
            operation: operation.into(),
        }
    }

    /// Create a new invalid path error
    pub fn invalid_path(path: impl Into<String>) -> Self {
        Self::InvalidPath { path: path.into() }
    }

    /// Create a new resource exhausted error
    pub fn resource_exhausted(resource: impl Into<String>) -> Self {
        Self::ResourceExhausted {
            resource: resource.into(),
        }
    }

    /// Create a new timeout error
    pub fn timeout(operation: impl Into<String>) -> Self {
        Self::Timeout {
            operation: operation.into(),
        }
    }

    /// Create a new unknown error
    pub fn unknown(message: impl Into<String>) -> Self {
        Self::Unknown {
            message: message.into(),
        }
    }

    /// Create a multiple errors wrapper
    pub fn multiple_errors(errors: Vec<PboOperationError>) -> Self {
        let count = errors.len();
        Self::MultipleErrors { count, errors }
    }

    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::IoError { .. } | Self::Timeout { .. } | Self::ResourceExhausted { .. } => true,
            Self::FileNotFound { .. }
            | Self::InvalidFormat { .. }
            | Self::ValidationFailed { .. }
            | Self::ChecksumMismatch { .. }
            | Self::PermissionDenied { .. }
            | Self::UnsupportedOperation { .. }
            | Self::InvalidPath { .. } => false,
            Self::ExtractionFailed { .. } | Self::FileNotFoundInPbo { .. } => false,
            Self::HemttError(_) => false, // Conservative approach
            Self::MultipleErrors { errors, .. } => errors.iter().any(|e| e.is_retryable()),
            Self::Unknown { .. } => false,
        }
    }

    /// Check if the error is critical (requires immediate attention)
    pub fn is_critical(&self) -> bool {
        match self {
            Self::InvalidFormat { .. }
            | Self::ValidationFailed { .. }
            | Self::ChecksumMismatch { .. }
            | Self::ResourceExhausted { .. } => true,
            Self::MultipleErrors { errors, .. } => errors.iter().any(|e| e.is_critical()),
            _ => false,
        }
    }

    /// Get a user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            Self::FileNotFound { path } => {
                format!("Could not find PBO file: {}", path.display())
            }
            Self::InvalidFormat { reason } => {
                format!("The PBO file is corrupted or invalid: {}", reason)
            }
            Self::ExtractionFailed { file_path, reason } => {
                format!("Failed to extract '{}': {}", file_path, reason)
            }
            Self::FileNotFoundInPbo { file_path } => {
                format!("File '{}' does not exist in the PBO", file_path)
            }
            Self::IoError { operation, .. } => {
                format!("File system error during {}", operation)
            }
            Self::ValidationFailed { reason } => {
                format!("PBO validation failed: {}", reason)
            }
            Self::ChecksumMismatch { .. } => {
                "PBO file integrity check failed".to_string()
            }
            Self::PermissionDenied { operation } => {
                format!("Permission denied for {}", operation)
            }
            Self::UnsupportedOperation { operation } => {
                format!("Operation '{}' is not supported", operation)
            }
            Self::InvalidPath { path } => {
                format!("Invalid file path: {}", path)
            }
            Self::ResourceExhausted { resource } => {
                format!("Insufficient {}", resource)
            }
            Self::Timeout { operation } => {
                format!("Operation '{}' timed out", operation)
            }
            Self::HemttError(err) => {
                format!("PBO processing error: {}", err)
            }
            Self::MultipleErrors { count, .. } => {
                format!("{} errors occurred during PBO operation", count)
            }
            Self::Unknown { message } => {
                format!("Unexpected error: {}", message)
            }
        }
    }
}

impl From<std::io::Error> for PboOperationError {
    fn from(err: std::io::Error) -> Self {
        Self::io_error("I/O operation", err)
    }
}

impl From<crate::error::types::PboError> for PboOperationError {
    fn from(err: crate::error::types::PboError) -> Self {
        match err {
            crate::error::types::PboError::InvalidPath(path) => {
                Self::file_not_found(path)
            }
            crate::error::types::PboError::InvalidFormat(reason) => {
                Self::invalid_format(reason)
            }
            crate::error::types::PboError::ValidationFailed(reason) => {
                Self::validation_failed(reason)
            }
            crate::error::types::PboError::CommandNotFound(cmd) => {
                Self::unsupported_operation(format!("Command not found: {}", cmd))
            }
            crate::error::types::PboError::Extraction(extract_err) => {
                Self::extraction_failed("unknown", format!("{:?}", extract_err))
            }
            crate::error::types::PboError::FileSystem(fs_err) => {
                Self::unknown(format!("File system error: {:?}", fs_err))
            }
            crate::error::types::PboError::Timeout(timeout) => {
                Self::timeout(format!("Operation timed out after {} seconds", timeout))
            }
            crate::error::types::PboError::Encoding { context, path } => {
                Self::invalid_format(format!("Encoding error: {} for {}", context, path.display()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_error_creation() {
        let path = PathBuf::from("test.pbo");
        let error = PboOperationError::file_not_found(path.clone());
        
        match error {
            PboOperationError::FileNotFound { path: error_path } => {
                assert_eq!(error_path, path);
            }
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_error_is_retryable() {
        let io_error = PboOperationError::io_error("test", std::io::Error::new(std::io::ErrorKind::TimedOut, "timeout"));
        assert!(io_error.is_retryable());

        let format_error = PboOperationError::invalid_format("bad format");
        assert!(!format_error.is_retryable());
    }

    #[test]
    fn test_error_is_critical() {
        let validation_error = PboOperationError::validation_failed("validation failed");
        assert!(validation_error.is_critical());

        let file_not_found = PboOperationError::file_not_found("test.pbo");
        assert!(!file_not_found.is_critical());
    }

    #[test]
    fn test_user_message() {
        let error = PboOperationError::file_not_found("test.pbo");
        let message = error.user_message();
        assert!(message.contains("Could not find PBO file"));
        assert!(message.contains("test.pbo"));
    }

    #[test]
    fn test_multiple_errors() {
        let errors = vec![
            PboOperationError::file_not_found("test1.pbo"),
            PboOperationError::invalid_format("bad format"),
        ];
        let multi_error = PboOperationError::multiple_errors(errors);
        
        match multi_error {
            PboOperationError::MultipleErrors { count, errors } => {
                assert_eq!(count, 2);
                assert_eq!(errors.len(), 2);
            }
            _ => panic!("Expected MultipleErrors"),
        }
    }
}