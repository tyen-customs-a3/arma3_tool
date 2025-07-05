use thiserror::Error;
use std::path::PathBuf;

/// Result type alias for database operations
pub type Result<T> = std::result::Result<T, DatabaseError>;

/// Error types for database operations
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// SQLite errors
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// Connection pool errors
    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),

    /// Schema version mismatch
    #[error("Database schema version mismatch: existing={existing}, required={required}")]
    SchemaVersionMismatch {
        existing: i32,
        required: i32,
    },

    /// Unsupported migration
    #[error("Unsupported schema migration from v{from} to v{to}")]
    UnsupportedMigration {
        from: i32,
        to: i32,
    },

    /// Entity not found
    #[error("Entity not found: {entity_type} with id={id}")]
    NotFound {
        entity_type: String,
        id: String,
    },

    /// IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// File system errors
    #[error("File system error for {path}: {message}")]
    FileSystem {
        path: PathBuf,
        message: String,
    },

    /// PBO extraction errors
    #[error("PBO extraction error for {pbo_path}: {message}")]
    PboExtraction {
        pbo_path: PathBuf,
        message: String,
    },

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Configuration(String),

    /// Invalid data
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Transaction error 
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Database busy
    #[error("Database busy")]
    DatabaseBusy,

    /// Unknown error
    #[error("Unknown database error: {0}")]
    Unknown(String),
}

/// Convert rusqlite::Error to specific DatabaseError types
impl From<String> for DatabaseError {
    fn from(error: String) -> Self {
        DatabaseError::Unknown(error)
    }
}

/// Convert anyhow::Error to DatabaseError
impl From<anyhow::Error> for DatabaseError {
    fn from(error: anyhow::Error) -> Self {
        DatabaseError::Unknown(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversions() {
        // Test rusqlite error conversion
        let rusqlite_err = rusqlite::Error::SqliteFailure(
            rusqlite::ffi::Error {
                code: rusqlite::ffi::ErrorCode::ConstraintViolation,
                extended_code: 0,
            },
            Some("constraint violation".to_string()),
        );
        
        let db_err: DatabaseError = rusqlite_err.into();
        assert!(matches!(db_err, DatabaseError::Sqlite(_)));

        // Test string error conversion
        let string_err = "test error".to_string();
        let db_err: DatabaseError = string_err.into();
        assert!(matches!(db_err, DatabaseError::Unknown(_)));
    }
} 