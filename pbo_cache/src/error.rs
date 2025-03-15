use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during PBO cache operations
#[derive(Error, Debug)]
pub enum CacheError {
    /// Error accessing the cache directory
    #[error("Cache directory error: {0}")]
    CacheDirectory(String),
    
    /// Error with PBO extraction
    #[error("PBO extraction error for {pbo_path}: {message}")]
    PboExtraction {
        pbo_path: PathBuf,
        message: String,
    },
    
    /// Error with cache index operations
    #[error("Cache index error: {0}")]
    IndexOperation(String),
    
    /// Error with file operations
    #[error("File operation error: {0}")]
    FileOperation(String),
    
    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    Configuration(String),
    
    /// IO error wrapper
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for cache operations
pub type Result<T> = std::result::Result<T, CacheError>;

impl From<anyhow::Error> for CacheError {
    fn from(err: anyhow::Error) -> Self {
        CacheError::FileOperation(err.to_string())
    }
} 