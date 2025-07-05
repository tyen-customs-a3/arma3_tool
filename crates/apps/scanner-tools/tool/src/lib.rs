//! Batch Scanner Library
//!
//! This library provides batch processing capabilities for scanning and parsing
//! Arma 3 gamedata files using the gamedata_scanner crate.

pub mod config;
pub mod batch_process;

// Re-export the main functionality
pub use batch_process::run;
pub use batch_process::Args;
