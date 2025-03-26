use std::path::{Path, PathBuf};
use std::fs;
use log::{debug, warn};
use crate::models::{CacheIndex, PboMetadata, PboType};
use crate::error::{Result, CacheError};

/// The filename of the cache index
const CACHE_INDEX_FILENAME: &str = "cache_index.json";

/// Manages the cache index file
pub struct IndexManager {
    /// Path to the cache index file
    index_path: PathBuf,
    
    /// The loaded cache index
    index: CacheIndex,
}

impl IndexManager {
    /// Create a new index manager
    pub fn new(cache_dir: &Path) -> Result<Self> {
        // Ensure the cache directory exists
        if !cache_dir.exists() {
            fs::create_dir_all(cache_dir)
                .map_err(|e| CacheError::CacheDirectory(format!("Failed to create cache directory: {}", e)))?;
        }
        
        let index_path = cache_dir.join(CACHE_INDEX_FILENAME);
        let index = Self::load_or_create(&index_path)?;
        
        Ok(Self { index_path, index })
    }
    
    /// Load cache index from disk or create a new one
    fn load_or_create(path: &Path) -> Result<CacheIndex> {
        if path.exists() {
            debug!("Loading cache index from {}", path.display());
            let file = fs::File::open(path)
                .map_err(|e| CacheError::IndexOperation(format!("Failed to open cache index file: {}", e)))?;
            
            match serde_json::from_reader(file) {
                Ok(index) => {
                    debug!("Cache index loaded successfully");
                    Ok(index)
                },
                Err(e) => {
                    warn!("Failed to parse cache index file: {}", e);
                    warn!("Creating new cache index");
                    Ok(CacheIndex::new())
                }
            }
        } else {
            debug!("Cache index not found, creating new one");
            Ok(CacheIndex::new())
        }
    }
    
    /// Save the cache index to disk
    pub fn save(&self) -> Result<()> {
        debug!("Saving cache index to {}", self.index_path.display());
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = self.index_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| CacheError::CacheDirectory(format!("Failed to create parent directory for cache index: {}", e)))?;
            }
        }
        
        // Write to a temporary file first, then rename
        let temp_path = self.index_path.with_extension("tmp");
        let file = fs::File::create(&temp_path)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to create temporary cache index file: {}", e)))?;
        
        serde_json::to_writer_pretty(file, &self.index)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to write cache index data: {}", e)))?;
        
        // Replace the old file with the new one
        fs::rename(&temp_path, &self.index_path)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to replace cache index file: {}", e)))?;
        
        debug!("Cache index saved successfully");
        Ok(())
    }
    
    /// Get the cache index
    pub fn get_index(&self) -> &CacheIndex {
        &self.index
    }
    
    /// Get a mutable reference to the cache index
    pub fn get_index_mut(&mut self) -> &mut CacheIndex {
        &mut self.index
    }
    
    /// Check if a PBO needs to be extracted
    pub fn needs_extraction(
        &self, 
        path: &Path, 
        pbo_type: PboType,
        extensions: &[String]
    ) -> Result<bool> {
        if !path.exists() {
            return Ok(true);
        }
        
        // Check if we have metadata for this PBO
        if let Some(metadata) = self.index.get_metadata(path, pbo_type) {
            // Use the metadata to check if extraction is needed
            metadata.needs_extraction(path, extensions)
        } else {
            // No metadata, needs extraction
            Ok(true)
        }
    }
    
    /// Update the metadata for a PBO
    pub fn update_metadata(&mut self, metadata: PboMetadata) {
        self.index.update_metadata(metadata);
    }
}

impl CacheIndex {
    /// Get a copy of all game data metadata
    pub fn get_game_data_metadata(&self) -> Vec<PboMetadata> {
        self.game_data.values().cloned().collect()
    }
    
    /// Get a copy of all mission metadata
    pub fn get_mission_metadata(&self) -> Vec<PboMetadata> {
        self.missions.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_new_index_manager() {
        let temp_dir = tempdir().unwrap();
        let manager = IndexManager::new(temp_dir.path()).unwrap();
        
        // Save the index to create the file
        manager.save().unwrap();
        
        assert!(manager.index_path.exists());
        assert_eq!(manager.index.game_data.len(), 0);
        assert_eq!(manager.index.missions.len(), 0);
    }
    
    #[test]
    fn test_save_and_load() {
        let temp_dir = tempdir().unwrap();
        let mut manager = IndexManager::new(temp_dir.path()).unwrap();
        
        // Add a test entry
        let path = PathBuf::from("/path/to/test.pbo");
        let metadata = PboMetadata {
            path: path.clone(),
            base_dir: PathBuf::new(),
            last_modified: std::time::SystemTime::now(),
            file_size: 1234,
            extraction_time: std::time::SystemTime::now(),
            extracted_files: vec![PathBuf::from("test.sqf")],
            used_extensions: vec!["sqf".to_string()],
            pbo_type: PboType::GameData,
        };
        
        manager.index.update_metadata(metadata);
        manager.save().unwrap();
        
        // Load a new manager with the same path
        let new_manager = IndexManager::new(temp_dir.path()).unwrap();
        
        // Check that the entry was loaded
        let key = path.to_string_lossy().to_string();
        assert!(new_manager.index.game_data.contains_key(&key));
        assert_eq!(new_manager.index.game_data[&key].file_size, 1234);
    }
} 