//! Weapon Magazine Scanner Library
//! 
//! A high-performance tool for analyzing Arma 3 game configuration files
//! to extract weapon and magazine compatibility information.
//!
//! # Features
//! 
//! - **High Performance**: SIMD-accelerated string searching, memory-mapped I/O
//! - **Parallel Processing**: Multi-threaded file processing with timeout protection
//! - **Smart Caching**: Binary serialization with LZ4 compression
//! - **Multiple Formats**: JSON, YAML, CSV, and plain text output
//! - **Modular Design**: Clean separation of concerns with focused modules

pub mod models;
pub mod config;
pub mod database;
pub mod scanner;
pub mod report;
pub mod utils;

// Re-export main types for convenience
pub use models::{WeaponInfo, MagazineWellInfo, ScanResult, FolderHash, AppConfig};
pub use config::ConfigManager;
pub use scanner::WeaponMagazineScanner;
pub use database::Database;
pub use report::{ReportGenerator, WeaponsByModExporter, ModStatistics};
pub use utils::{ConfigValidator, FileUtils};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main result type used throughout the library
pub type Result<T> = anyhow::Result<T>;
