use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use sha2::{Sha256, Digest};
use std::fs;
use walkdir::WalkDir;

use crate::models::FolderHash;

/// Handles hash calculation for change detection
pub struct HashCalculator;

impl HashCalculator {
    pub fn new() -> Self {
        Self
    }

    /// Calculate hash for an entire folder structure
    pub fn calculate_folder_hash(&self, target: &Path) -> Result<FolderHash> {
        let mut hasher = Sha256::new();
        let mut file_hashes = HashMap::new();
        
        let mut files: Vec<_> = WalkDir::new(target)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| {
                let path = entry.path();
                path.is_file() && matches!(
                    path.extension().and_then(|s| s.to_str()).unwrap_or(""),
                    "cpp" | "hpp" | "h"
                )
            })
            .map(|entry| entry.path().to_path_buf())
            .collect();
        
        // Sort for consistent hashing
        files.sort();
        
        for file in files {
            match self.calculate_file_hash_fast(&file) {
                Ok(file_hash) => {
                    hasher.update(file_hash.as_bytes());
                    file_hashes.insert(file.clone(), file_hash);
                }
                Err(e) => {
                    log::warn!("Failed to hash {}: {}", file.display(), e);
                    // Continue with other files
                }
            }
        }
        
        let hash = format!("{:x}", hasher.finalize());
        
        Ok(FolderHash {
            hash,
            file_hashes,
        })
    }

    /// Fast file hash calculation using file metadata
    pub fn calculate_file_hash_fast(&self, file: &Path) -> Result<String> {
        let metadata = fs::metadata(file)?;
        let mut hasher = Sha256::new();
        
        // Hash file path, size, and modification time for fast comparison
        hasher.update(file.to_string_lossy().as_bytes());
        hasher.update(&metadata.len().to_le_bytes());
        
        if let Ok(modified) = metadata.modified() {
            if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                hasher.update(&duration.as_secs().to_le_bytes());
                hasher.update(&duration.subsec_nanos().to_le_bytes());
            }
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Full file hash calculation using file contents
    pub fn calculate_file_hash_full(&self, file: &Path) -> Result<String> {
        let contents = fs::read(file)?;
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        Ok(format!("{:x}", hasher.finalize()))
    }
}
