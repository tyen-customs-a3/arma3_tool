//! Scanner module for processing weapon and magazine configuration files
//! 
//! This module provides high-performance scanning capabilities with:
//! - Parallel processing
//! - Memory-mapped file I/O
//! - Pre-filtering for performance
//! - Timeout protection

pub mod core;
pub mod file_processor;
pub mod hash_calculator;
pub mod weapon_extractor;
pub mod magazine_extractor;
pub mod compatibility_resolver;

// Re-export main types
pub use core::WeaponMagazineScanner;
pub use file_processor::{FileProcessingResult, FileProcessor};
pub use hash_calculator::HashCalculator;
pub use weapon_extractor::WeaponExtractor;
pub use magazine_extractor::MagazineExtractor;
pub use compatibility_resolver::{CompatibilityResolver, MagazineWellCache, CacheStats};
