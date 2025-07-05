//! Common error types and traits for the Arma3 Tool workspace
//! 
//! This crate provides unified error handling across all components,
//! ensuring consistent error reporting and handling patterns.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Result type alias using our common error type
pub type Result<T> = std::result::Result<T, Error>;

/// Common error type for all Arma3 Tool operations
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    /// I/O operation failed
    #[error("I/O error: {message}")]
    Io { message: String },

    /// File parsing failed
    #[error("Parse error in {file}: {message}")]
    Parse { file: String, message: String },

    /// Configuration error
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// Database operation failed
    #[error("Database error: {message}")]
    Database { message: String },

    /// PBO extraction/manipulation failed
    #[error("PBO error: {message}")]
    Pbo { message: String },

    /// Validation failed
    #[error("Validation error: {message}")]
    Validation { message: String },

    /// Network operation failed
    #[error("Network error: {message}")]
    Network { message: String },

    /// Generic error with custom message
    #[error("{message}")]
    Generic { message: String },
}

impl Error {
    /// Create a new I/O error
    pub fn io<S: Into<String>>(message: S) -> Self {
        Error::Io {
            message: message.into(),
        }
    }

    /// Create a new parse error
    pub fn parse<S: Into<String>, F: Into<String>>(file: F, message: S) -> Self {
        Error::Parse {
            file: file.into(),
            message: message.into(),
        }
    }

    /// Create a new configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Error::Config {
            message: message.into(),
        }
    }

    /// Create a new database error
    pub fn database<S: Into<String>>(message: S) -> Self {
        Error::Database {
            message: message.into(),
        }
    }

    /// Create a new PBO error
    pub fn pbo<S: Into<String>>(message: S) -> Self {
        Error::Pbo {
            message: message.into(),
        }
    }

    /// Create a new validation error
    pub fn validation<S: Into<String>>(message: S) -> Self {
        Error::Validation {
            message: message.into(),
        }
    }

    /// Create a new network error
    pub fn network<S: Into<String>>(message: S) -> Self {
        Error::Network {
            message: message.into(),
        }
    }

    /// Create a generic error
    pub fn generic<S: Into<String>>(message: S) -> Self {
        Error::Generic {
            message: message.into(),
        }
    }
}

/// Convert from std::io::Error
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::io(err.to_string())
    }
}

/// Convert from serde_json::Error
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::parse("JSON", err.to_string())
    }
}

/// Convert from toml::de::Error
impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::parse("TOML", err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let io_err = Error::io("file not found");
        assert!(matches!(io_err, Error::Io { .. }));

        let parse_err = Error::parse("test.txt", "invalid syntax");
        assert!(matches!(parse_err, Error::Parse { .. }));

        let config_err = Error::config("missing field");
        assert!(matches!(config_err, Error::Config { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = Error::parse("test.hpp", "unexpected token");
        let display = format!("{}", err);
        assert!(display.contains("Parse error"));
        assert!(display.contains("test.hpp"));
        assert!(display.contains("unexpected token"));
    }

    #[test]
    fn test_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let converted: Error = io_err.into();
        assert!(matches!(converted, Error::Io { .. }));
    }

    #[test]
    fn test_result_type() {
        fn sample_function() -> Result<String> {
            Ok("success".to_string())
        }

        fn failing_function() -> Result<String> {
            Err(Error::generic("something went wrong"))
        }

        assert!(sample_function().is_ok());
        assert!(failing_function().is_err());
    }
}