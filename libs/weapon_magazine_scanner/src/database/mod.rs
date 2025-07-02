//! Database module for caching and persistence
//! 
//! This module provides high-performance data storage with:
//! - Binary serialization with compression
//! - JSON fallback for compatibility
//! - Cache management
//! - Performance optimization

pub mod cache;
pub mod serialization;
pub mod format_detection;

// Re-export main types
pub use cache::Cache;
pub use serialization::{BinarySerializer, JsonSerializer, Serializer};
pub use format_detection::FormatDetector;

/// Main database interface
pub struct Database {
    cache: Cache,
    serializer: Box<dyn Serializer>,
    #[allow(dead_code)]
    path: std::path::PathBuf,
}

impl Database {
    pub fn new(path: &std::path::Path) -> anyhow::Result<Self> {
        use std::fs;
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Detect format and choose serializer
        let serializer: Box<dyn Serializer> = if FormatDetector::should_use_binary(path)? {
            Box::new(BinarySerializer::new())
        } else {
            Box::new(JsonSerializer::new())
        };
        
        Ok(Self {
            cache: Cache::new(path),
            serializer,
            path: path.to_path_buf(),
        })
    }

    pub fn save_scan_result(&mut self, result: &crate::models::ScanResult) -> anyhow::Result<()> {
        self.cache.save_scan_result(result, &*self.serializer)
    }

    pub fn load_scan_result(&self) -> anyhow::Result<crate::models::ScanResult> {
        self.cache.load_scan_result(&*self.serializer)
    }

    pub fn get_folder_hash(&self) -> anyhow::Result<Option<crate::models::FolderHash>> {
        self.cache.get_folder_hash(&*self.serializer)
    }

    pub fn save_scan_result_binary(&mut self, result: &crate::models::ScanResult) -> anyhow::Result<()> {
        let binary_serializer = BinarySerializer::new();
        self.cache.save_scan_result(result, &binary_serializer)
    }

    pub fn load_scan_result_binary(&self) -> anyhow::Result<crate::models::ScanResult> {
        let binary_serializer = BinarySerializer::new();
        self.cache.load_scan_result(&binary_serializer)
    }
}
