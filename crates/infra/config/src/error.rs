use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for config operations
pub type Result<T> = std::result::Result<T, ConfigError>;

/// Errors that can occur during configuration operations
#[derive(Error, Debug, Clone)]
pub enum ConfigError {
    #[error("Configuration validation failed: {message}")]
    ValidationError { message: String },

    #[error("IO error accessing config file {path}: {message}")]
    IoError {
        path: PathBuf,
        message: String,
    },

    #[error("Failed to parse configuration: {message}")]
    ParseError { message: String },

    #[error("Failed to serialize configuration: {message}")]
    SerializationError { message: String },

    #[error("Environment variable error: {message}")]
    EnvironmentError { message: String },

    #[error("Configuration field error for '{field}': {message}")]
    FieldError {
        field: String,
        message: String,
    },

    #[error("Configuration file not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("Invalid configuration format: expected {expected}, found {actual}")]
    InvalidFormat {
        expected: String,
        actual: String,
    },

    #[error("Configuration merge conflict: {message}")]
    MergeConflict { message: String },
}

impl ConfigError {
    /// Create a new validation error
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
        }
    }

    /// Create a new IO error
    pub fn io_error(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::IoError {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a new parse error
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self::ParseError {
            message: message.into(),
        }
    }

    /// Create a new serialization error
    pub fn serialization_error(message: impl Into<String>) -> Self {
        Self::SerializationError {
            message: message.into(),
        }
    }

    /// Create a new environment error
    pub fn environment_error(message: impl Into<String>) -> Self {
        Self::EnvironmentError {
            message: message.into(),
        }
    }

    /// Create a new field error
    pub fn field_error(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self::FieldError {
            field: field.into(),
            message: message.into(),
        }
    }

    /// Create a file not found error
    pub fn file_not_found(path: impl Into<PathBuf>) -> Self {
        Self::FileNotFound {
            path: path.into(),
        }
    }

    /// Create an invalid format error
    pub fn invalid_format(expected: impl Into<String>, actual: impl Into<String>) -> Self {
        Self::InvalidFormat {
            expected: expected.into(),
            actual: actual.into(),
        }
    }

    /// Create a merge conflict error
    pub fn merge_conflict(message: impl Into<String>) -> Self {
        Self::MergeConflict {
            message: message.into(),
        }
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ConfigError::ValidationError { .. }
                | ConfigError::FieldError { .. }
                | ConfigError::EnvironmentError { .. }
        )
    }

    /// Get error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            ConfigError::ValidationError { .. } => "validation",
            ConfigError::IoError { .. } => "io",
            ConfigError::ParseError { .. } => "parse",
            ConfigError::SerializationError { .. } => "serialization",
            ConfigError::EnvironmentError { .. } => "environment",
            ConfigError::FieldError { .. } => "field",
            ConfigError::FileNotFound { .. } => "file_not_found",
            ConfigError::InvalidFormat { .. } => "invalid_format",
            ConfigError::MergeConflict { .. } => "merge_conflict",
        }
    }
}

/// Convert from std::io::Error
impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::io_error(PathBuf::new(), err.to_string())
    }
}

/// Convert from serde_json::Error
impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::parse_error(err.to_string())
    }
}

/// Convert from std::env::VarError
impl From<std::env::VarError> for ConfigError {
    fn from(err: std::env::VarError) -> Self {
        ConfigError::environment_error(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ConfigError::validation_error("Invalid value");
        assert!(error.to_string().contains("Invalid value"));
        assert!(error.is_recoverable());
        assert_eq!(error.category(), "validation");
    }

    #[test]
    fn test_io_error() {
        let error = ConfigError::io_error("/tmp/config.json", "File not accessible");
        assert!(error.to_string().contains("/tmp/config.json"));
        assert!(!error.is_recoverable());
        assert_eq!(error.category(), "io");
    }

    #[test]
    fn test_field_error() {
        let error = ConfigError::field_error("database_url", "Invalid URL format");
        assert!(error.to_string().contains("database_url"));
        assert!(error.to_string().contains("Invalid URL format"));
        assert!(error.is_recoverable());
        assert_eq!(error.category(), "field");
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let config_err: ConfigError = io_err.into();
        assert_eq!(config_err.category(), "io");

        let json_err: serde_json::Error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let config_err: ConfigError = json_err.into();
        assert_eq!(config_err.category(), "parse");

        let env_err = std::env::VarError::NotPresent;
        let config_err: ConfigError = env_err.into();
        assert_eq!(config_err.category(), "environment");
    }

    #[test]
    fn test_invalid_format_error() {
        let error = ConfigError::invalid_format("JSON", "YAML");
        assert!(error.to_string().contains("JSON"));
        assert!(error.to_string().contains("YAML"));
        assert_eq!(error.category(), "invalid_format");
    }

    #[test]
    fn test_merge_conflict_error() {
        let error = ConfigError::merge_conflict("Conflicting database configurations");
        assert!(error.to_string().contains("Conflicting database configurations"));
        assert_eq!(error.category(), "merge_conflict");
    }
}