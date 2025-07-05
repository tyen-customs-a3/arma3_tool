use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use crate::models::{ScanResult, FolderHash};
use super::Serializer;

/// Cache management for scan results
pub struct Cache {
    path: PathBuf,
}

impl Cache {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
        }
    }

    /// Save scan result using the provided serializer
    pub fn save_scan_result(&self, result: &ScanResult, serializer: &dyn Serializer) -> Result<()> {
        let data = serializer.serialize(result)?;
        std::fs::write(&self.path, data)
            .context("Failed to write cache file")?;
        Ok(())
    }

    /// Load scan result using the provided serializer
    pub fn load_scan_result(&self, serializer: &dyn Serializer) -> Result<ScanResult> {
        let data = std::fs::read(&self.path)
            .context("Failed to read cache file")?;
        serializer.deserialize(&data)
    }

    /// Get folder hash from cached scan result
    pub fn get_folder_hash(&self, serializer: &dyn Serializer) -> Result<Option<FolderHash>> {
        if !self.path.exists() {
            return Ok(None);
        }
        
        let result = self.load_scan_result(serializer)?;
        Ok(Some(FolderHash {
            hash: result.folder_hash,
            file_hashes: Default::default(), // We don't store individual file hashes in cache
        }))
    }

    /// Check if cache file exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Get cache file size
    pub fn size(&self) -> Result<u64> {
        let metadata = std::fs::metadata(&self.path)?;
        Ok(metadata.len())
    }

    /// Get cache file modification time
    pub fn last_modified(&self) -> Result<std::time::SystemTime> {
        let metadata = std::fs::metadata(&self.path)?;
        Ok(metadata.modified()?)
    }

    /// Clear the cache
    pub fn clear(&self) -> Result<()> {
        if self.exists() {
            std::fs::remove_file(&self.path)?;
        }
        Ok(())
    }

    /// Get cache file path
    pub fn path(&self) -> &Path {
        &self.path
    }
}
