use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, WorkflowError>;

/// Errors that can occur during workflow execution
#[derive(Error, Debug, Clone)]
pub enum WorkflowError {
    #[error("Workflow execution failed: {message}")]
    ExecutionFailed { message: String },

    #[error("Configuration error: {message}")]
    ConfigurationError { message: String },

    #[error("Extraction error: {message}")]
    ExtractionError { message: String },

    #[error("Processing error: {message}")]
    ProcessingError { message: String },

    #[error("Reporting error: {message}")]
    ReportingError { message: String },

    #[error("Export error: {message}")]
    ExportError { message: String },

    #[error("IO error at {path}: {message}")]
    IoError {
        path: PathBuf,
        message: String,
    },

    #[error("Database error: {message}")]
    DatabaseError { message: String },

    #[error("Workflow was cancelled")]
    Cancelled,

    #[error("Workflow timeout after {duration_secs} seconds")]
    Timeout { duration_secs: u64 },

    #[error("Invalid workflow state: expected {expected}, found {actual}")]
    InvalidState { expected: String, actual: String },

    #[error("Dependency error: {dependency} failed")]
    DependencyError { dependency: String },

    #[error("Validation error: {message}")]
    ValidationError { message: String },
}

impl WorkflowError {
    /// Create a new execution failed error
    pub fn execution_failed(message: impl Into<String>) -> Self {
        Self::ExecutionFailed { message: message.into() }
    }

    /// Create a new configuration error
    pub fn configuration_error(message: impl Into<String>) -> Self {
        Self::ConfigurationError { message: message.into() }
    }

    /// Create a new extraction error
    pub fn extraction_error(message: impl Into<String>) -> Self {
        Self::ExtractionError { message: message.into() }
    }

    /// Create a new processing error
    pub fn processing_error(message: impl Into<String>) -> Self {
        Self::ProcessingError { message: message.into() }
    }

    /// Create a new reporting error
    pub fn reporting_error(message: impl Into<String>) -> Self {
        Self::ReportingError { message: message.into() }
    }

    /// Create a new export error
    pub fn export_error(message: impl Into<String>) -> Self {
        Self::ExportError { message: message.into() }
    }

    /// Create a new IO error
    pub fn io_error(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::IoError { 
            path: path.into(), 
            message: message.into() 
        }
    }

    /// Create a new database error
    pub fn database_error(message: impl Into<String>) -> Self {
        Self::DatabaseError { message: message.into() }
    }

    /// Create a timeout error
    pub fn timeout(duration_secs: u64) -> Self {
        Self::Timeout { duration_secs }
    }

    /// Create an invalid state error
    pub fn invalid_state(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::InvalidState { 
            expected: expected.into(), 
            actual: actual.into() 
        }
    }

    /// Create a dependency error
    pub fn dependency_error(dependency: impl Into<String>) -> Self {
        Self::DependencyError { dependency: dependency.into() }
    }

    /// Create a validation error
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError { message: message.into() }
    }

    /// Check if this is a cancellation error
    pub fn is_cancelled(&self) -> bool {
        matches!(self, WorkflowError::Cancelled)
    }

    /// Check if this is a timeout error
    pub fn is_timeout(&self) -> bool {
        matches!(self, WorkflowError::Timeout { .. })
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(self, 
            WorkflowError::IoError { .. } |
            WorkflowError::DatabaseError { .. } |
            WorkflowError::Timeout { .. }
        )
    }
}

/// Convert from anyhow::Error
impl From<anyhow::Error> for WorkflowError {
    fn from(err: anyhow::Error) -> Self {
        WorkflowError::execution_failed(err.to_string())
    }
}

/// Convert from std::io::Error
impl From<std::io::Error> for WorkflowError {
    fn from(err: std::io::Error) -> Self {
        WorkflowError::io_error(PathBuf::new(), err.to_string())
    }
}

/// Convert from database errors
impl From<arma3_database::error::DatabaseError> for WorkflowError {
    fn from(err: arma3_database::error::DatabaseError) -> Self {
        WorkflowError::database_error(err.to_string())
    }
}

/// Convert from reporter errors
impl From<arma3_reporter::ReporterError> for WorkflowError {
    fn from(err: arma3_reporter::ReporterError) -> Self {
        WorkflowError::reporting_error(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = WorkflowError::execution_failed("test message");
        assert!(err.to_string().contains("test message"));
    }

    #[test]
    fn test_error_classification() {
        let cancelled = WorkflowError::Cancelled;
        assert!(cancelled.is_cancelled());
        assert!(!cancelled.is_timeout());
        assert!(!cancelled.is_recoverable());

        let timeout = WorkflowError::timeout(30);
        assert!(!timeout.is_cancelled());
        assert!(timeout.is_timeout());
        assert!(timeout.is_recoverable());

        let io_error = WorkflowError::io_error("/tmp/test", "file not found");
        assert!(!io_error.is_cancelled());
        assert!(!io_error.is_timeout());
        assert!(io_error.is_recoverable());
    }

    #[test]
    fn test_error_conversion() {
        let anyhow_err = anyhow::anyhow!("test error");
        let workflow_err: WorkflowError = anyhow_err.into();
        assert!(workflow_err.to_string().contains("test error"));

        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let workflow_err: WorkflowError = io_err.into();
        assert!(workflow_err.to_string().contains("file not found"));
    }
}