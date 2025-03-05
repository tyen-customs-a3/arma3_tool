pub mod class;
pub mod utils;
pub mod database;

// Re-export main types and functions for easier access
pub use class::{
    types::{ProcessedClass, ClassScanStats},
    scanner::ClassScanner,
    processor::ClassProcessor,
};

pub use database::{
    ClassDatabase,
    ClassDatabaseStats,
};

// Re-export utility functions
pub use utils::file_utils; 