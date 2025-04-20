//! PBO Cache - A library for managing extraction and caching of Arma 3 PBO files
//!
//! This crate provides core functionality for:
//! - Scanning directories for PBO files
//! - Extracting PBO contents to a cache location
//! - Managing cache validity based on file modifications and extension filters
//! - Handling both game data and mission PBOs

pub mod manager;
pub mod models;
mod scanner;
mod processor;
pub mod file_db;
mod utils;
pub mod error;

// Public API exports
pub use manager::ExtractionManager;
pub use models::{PboMetadata, PboType, ExtractionConfig};
pub use error::{CacheError, Result};
pub use file_db::{FileDbManager, ExtractedFileInfo, PboRecord};

// Convenience re-exports of core functionality
pub use manager::{extract_game_data, extract_mission};

/// Find the PBO source for a given file path
pub async fn find_source_pbo(config: ExtractionConfig, file_path: &std::path::Path) -> Result<Option<PboRecord>> {
    let manager = ExtractionManager::new(config)?;
    manager.find_pbo_for_file(file_path)
}

/// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 