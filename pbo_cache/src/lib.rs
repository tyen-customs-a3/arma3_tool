//! PBO Cache - A library for managing extraction and caching of Arma 3 PBO files
//!
//! This crate provides functionality for:
//! - Scanning directories for PBO files
//! - Extracting PBO contents to a cache location
//! - Managing cache validity based on file modifications and extension filters
//! - Handling both game data and mission PBOs

pub mod manager;
pub mod models;
mod scanner;
mod processor;
mod index;
mod utils;
pub mod error;

// Public API exports
pub use manager::ExtractionManager;
pub use models::{CacheIndex, PboMetadata, PboType, ExtractionConfig};
pub use error::{CacheError, Result};

// Convenience re-exports of core functionality
pub use manager::{extract_game_data, extract_mission};

/// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION"); 