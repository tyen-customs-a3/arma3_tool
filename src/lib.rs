pub mod error;
pub mod scanner;
pub mod config;

use std::path::{Path, PathBuf};
use log::{debug, info, warn, error};

pub use error::{Result, ToolError};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Re-export necessary types from pbo_cache
pub use pbo_cache::{
    PboMetadata,
    PboType,
    CacheIndex,
    ExtractionConfig,
    ExtractionManager,
};

// Re-export database types
pub use arma3_tool_database as database;
