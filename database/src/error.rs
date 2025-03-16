use std::fmt;
use std::error::Error;
use std::io;

/// Database operation errors
#[derive(Debug)]
pub enum DatabaseError {
    /// IO error
    IO(String),
    /// Serialization error
    Serialization(String),
    /// Invalid data error
    InvalidData(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseError::IO(msg) => write!(f, "IO error: {}", msg),
            DatabaseError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            DatabaseError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl Error for DatabaseError {}

impl From<io::Error> for DatabaseError {
    fn from(err: io::Error) -> Self {
        DatabaseError::IO(err.to_string())
    }
}

impl From<serde_json::Error> for DatabaseError {
    fn from(err: serde_json::Error) -> Self {
        DatabaseError::Serialization(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, DatabaseError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error as IoError, ErrorKind};
    
    #[test]
    fn test_io_error_conversion() {
        let io_error = IoError::new(ErrorKind::NotFound, "file not found");
        let db_error: DatabaseError = io_error.into();
        
        match db_error {
            DatabaseError::IO(msg) => assert!(msg.contains("file not found")),
            _ => panic!("Expected IO error variant"),
        }
    }
    
    #[test]
    fn test_serialization_error_conversion() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let db_error: DatabaseError = json_error.into();
        
        match db_error {
            DatabaseError::Serialization(msg) => assert!(msg.contains("expected value")),
            _ => panic!("Expected Serialization error variant"),
        }
    }
    
    #[test]
    fn test_error_display() {
        let errors = vec![
            DatabaseError::IO("test io".to_string()),
            DatabaseError::Serialization("test serialization".to_string()),
            DatabaseError::InvalidData("test invalid".to_string()),
        ];
        
        let messages = vec![
            "IO error: test io",
            "Serialization error: test serialization",
            "Invalid data: test invalid",
        ];
        
        for (error, expected_msg) in errors.iter().zip(messages.iter()) {
            assert_eq!(error.to_string(), *expected_msg);
        }
    }
    
    #[test]
    fn test_error_debug() {
        let error = DatabaseError::IO("test debug".to_string());
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("IO"));
        assert!(debug_str.contains("test debug"));
    }
} 