use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ReporterError>;

#[derive(Error, Debug)]
pub enum ReporterError {
    #[error("Failed to write report to {path}: {source}")]
    WriteError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to create report directory {path}: {source}")]
    CreateDirError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Database error: {0}")]
    DatabaseError(#[from] arma3_database::error::DatabaseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
