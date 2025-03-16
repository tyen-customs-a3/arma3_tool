use std::path::PathBuf;
use thiserror::Error;

/// Custom error types for the Arma 3 Tool
#[derive(Error, Debug)]
pub enum ToolError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Invalid path: {0}")]
    InvalidPath(PathBuf),
    
    #[error("PBO extraction error: {0}")]
    ExtractionError(String),
    
    #[error("Mission scanning error: {0}")]
    MissionScanError(String),
    
    #[error("Game data scanning error: {0}")]
    GameDataScanError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("JSON error: {0}")]
    JsonError(String),
}

/// Result type alias for the Arma 3 Tool
pub type Result<T> = std::result::Result<T, ToolError>;

/// Convert an anyhow::Error to a ToolError
pub fn from_anyhow(err: anyhow::Error) -> ToolError {
    if let Some(tool_err) = err.downcast_ref::<ToolError>() {
        return tool_err.clone();
    }
    
    if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
        return ToolError::IoError(io_err.to_string());
    }
    
    if let Some(json_err) = err.downcast_ref::<serde_json::Error>() {
        return ToolError::JsonError(json_err.to_string());
    }
    
    // Default to a generic error message
    ToolError::ConfigError(format!("Unknown error: {}", err))
}

// Allow cloning of ToolError
impl Clone for ToolError {
    fn clone(&self) -> Self {
        match self {
            ToolError::ConfigError(s) => ToolError::ConfigError(s.clone()),
            ToolError::InvalidPath(p) => ToolError::InvalidPath(p.clone()),
            ToolError::ExtractionError(s) => ToolError::ExtractionError(s.clone()),
            ToolError::MissionScanError(s) => ToolError::MissionScanError(s.clone()),
            ToolError::GameDataScanError(s) => ToolError::GameDataScanError(s.clone()),
            ToolError::CacheError(s) => ToolError::CacheError(s.clone()),
            ToolError::DatabaseError(s) => ToolError::DatabaseError(s.clone()),
            ToolError::IoError(s) => ToolError::IoError(s.clone()),
            ToolError::JsonError(s) => ToolError::JsonError(s.clone()),
        }
    }
}

// Implement From traits for common error types
impl From<std::io::Error> for ToolError {
    fn from(err: std::io::Error) -> Self {
        ToolError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for ToolError {
    fn from(err: serde_json::Error) -> Self {
        ToolError::JsonError(err.to_string())
    }
}

impl From<pbo_cache::CacheError> for ToolError {
    fn from(err: pbo_cache::CacheError) -> Self {
        ToolError::CacheError(err.to_string())
    }
}

impl From<anyhow::Error> for ToolError {
    fn from(err: anyhow::Error) -> Self {
        from_anyhow(err)
    }
}

impl From<arma3_tool_database::error::DatabaseError> for ToolError {
    fn from(err: arma3_tool_database::error::DatabaseError) -> Self {
        ToolError::DatabaseError(err.to_string())
    }
} 