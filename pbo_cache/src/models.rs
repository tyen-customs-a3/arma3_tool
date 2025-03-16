use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::error::{Result, CacheError};

/// Type of PBO being processed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PboType {
    /// Game data PBO (addons, etc.)
    GameData,
    
    /// Mission PBO
    Mission,
}

/// Metadata about an extracted PBO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PboMetadata {
    /// Original path to the PBO file
    pub path: PathBuf,
    
    /// Last modified timestamp of the PBO
    pub last_modified: SystemTime,
    
    /// File size in bytes
    pub file_size: u64,
    
    /// When the PBO was last extracted
    pub extraction_time: SystemTime,
    
    /// Files that were extracted from this PBO
    pub extracted_files: Vec<PathBuf>,
    
    /// Extensions that were used for filtering during extraction
    pub used_extensions: Vec<String>,
    
    /// Type of PBO (game data or mission)
    pub pbo_type: PboType,
}

impl PboMetadata {
    /// Create new metadata for a PBO file
    pub fn new(
        path: PathBuf, 
        pbo_type: PboType, 
        used_extensions: Vec<String>
    ) -> Result<Self> {
        let metadata = std::fs::metadata(&path)
            .map_err(|e| CacheError::FileOperation(format!("Failed to get metadata for {}: {}", path.display(), e)))?;
        
        Ok(Self {
            path,
            last_modified: metadata.modified()
                .map_err(|e| CacheError::FileOperation(format!("Failed to get modified time: {}", e)))?,
            file_size: metadata.len(),
            extraction_time: SystemTime::now(),
            extracted_files: Vec::new(),
            used_extensions,
            pbo_type,
        })
    }
    
    /// Check if this PBO needs to be extracted again
    pub fn needs_extraction(
        &self, 
        path: &Path, 
        current_extensions: &[String]
    ) -> Result<bool> {
        // Check if file exists
        if !path.exists() {
            return Ok(true);
        }
        
        // Check if metadata has changed
        let metadata = std::fs::metadata(path)
            .map_err(|e| CacheError::FileOperation(format!("Failed to get metadata for {}: {}", path.display(), e)))?;
        let current_modified = metadata.modified()
            .map_err(|e| CacheError::FileOperation(format!("Failed to get modified time: {}", e)))?;
        let current_size = metadata.len();
        
        // Check if file was modified
        if current_modified > self.last_modified || current_size != self.file_size {
            return Ok(true);
        }
        
        // Check if extensions have changed
        if self.used_extensions != current_extensions {
            return Ok(true);
        }
        
        // No need to extract
        Ok(false)
    }
}

/// Information about a failed PBO extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedExtraction {
    /// When the extraction failed
    pub timestamp: SystemTime,
    /// The error message from the failed extraction
    pub error_message: String,
}

/// Cache index tracking all extracted PBOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheIndex {
    /// Game data PBOs by normalized path
    pub game_data: HashMap<String, PboMetadata>,
    
    /// Mission PBOs by normalized path
    pub missions: HashMap<String, PboMetadata>,

    /// Failed extractions by normalized path
    pub failed_extractions: HashMap<String, FailedExtraction>,
    
    /// When the index was last updated
    pub last_updated: SystemTime,
}

impl Default for CacheIndex {
    fn default() -> Self {
        Self {
            game_data: HashMap::new(),
            missions: HashMap::new(),
            failed_extractions: HashMap::new(),
            last_updated: SystemTime::now(),
        }
    }
}

impl CacheIndex {
    /// Create a new empty cache index
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Get a reference to the metadata for a PBO
    pub fn get_metadata(&self, path: &Path, pbo_type: PboType) -> Option<&PboMetadata> {
        let key = path.to_string_lossy().to_string();
        match pbo_type {
            PboType::GameData => self.game_data.get(&key),
            PboType::Mission => self.missions.get(&key),
        }
    }
    
    /// Add or update metadata for a PBO
    pub fn update_metadata(&mut self, metadata: PboMetadata) {
        let key = metadata.path.to_string_lossy().to_string();
        match metadata.pbo_type {
            PboType::GameData => {
                self.game_data.insert(key, metadata);
            },
            PboType::Mission => {
                self.missions.insert(key, metadata);
            },
        }
        self.last_updated = SystemTime::now();
    }
    
    /// Remove metadata for a PBO
    pub fn remove_metadata(&mut self, path: &Path, pbo_type: PboType) -> Option<PboMetadata> {
        let key = path.to_string_lossy().to_string();
        let result = match pbo_type {
            PboType::GameData => self.game_data.remove(&key),
            PboType::Mission => self.missions.remove(&key),
        };
        
        if result.is_some() {
            self.last_updated = SystemTime::now();
        }
        
        result
    }

    /// Add a failed extraction to the index
    pub fn add_failed_extraction(&mut self, path: &Path, error_message: String) {
        let key = path.to_string_lossy().to_string();
        let failed = FailedExtraction {
            timestamp: SystemTime::now(),
            error_message,
        };
        self.failed_extractions.insert(key, failed);
        self.last_updated = SystemTime::now();
    }

    /// Check if a PBO has previously failed extraction
    pub fn is_failed_extraction(&self, path: &Path) -> Option<&FailedExtraction> {
        let key = path.to_string_lossy().to_string();
        self.failed_extractions.get(&key)
    }

    /// Remove a failed extraction entry
    pub fn remove_failed_extraction(&mut self, path: &Path) -> Option<FailedExtraction> {
        let key = path.to_string_lossy().to_string();
        let result = self.failed_extractions.remove(&key);
        if result.is_some() {
            self.last_updated = SystemTime::now();
        }
        result
    }
}

/// Configuration for PBO extraction and caching
#[derive(Debug, Clone)]
pub struct ExtractionConfig {
    /// Base cache directory
    pub cache_dir: PathBuf,
    
    /// Game data cache subdirectory
    pub game_data_cache_dir: PathBuf,
    
    /// Mission cache subdirectory
    pub mission_cache_dir: PathBuf,
    
    /// Directories to scan for game data PBOs
    pub game_data_dirs: Vec<PathBuf>,
    
    /// File extensions to extract from game data PBOs
    pub game_data_extensions: Vec<String>,
    
    /// Directories to scan for mission PBOs
    pub mission_dirs: Vec<PathBuf>,
    
    /// File extensions to extract from mission PBOs
    pub mission_extensions: Vec<String>,
    
    /// Number of threads to use for parallel extraction
    pub threads: usize,
    
    /// Timeout in seconds for extraction operations
    pub timeout: u64,
    
    /// Whether to enable verbose logging
    pub verbose: bool,
}

impl ExtractionConfig {
    /// Create a new configuration with default values
    pub fn new(cache_dir: PathBuf) -> Self {
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        
        Self {
            cache_dir,
            game_data_cache_dir,
            mission_cache_dir,
            game_data_dirs: Vec::new(),
            game_data_extensions: vec!["hpp".into(), "cpp".into(), "sqf".into()],
            mission_dirs: Vec::new(),
            mission_extensions: vec!["hpp".into(), "cpp".into(), "sqf".into(), "sqm".into()],
            threads: num_cpus::get(),
            timeout: 60,
            verbose: false,
        }
    }
  
    /// Create a configuration for processing arma3_tool data
    pub fn for_arma3_tool(
        cache_dir: PathBuf,
        game_data_dirs: Vec<PathBuf>,
        game_data_extensions: Vec<String>,
        mission_dirs: Vec<PathBuf>,
        mission_extensions: Vec<String>,
        threads: usize,
        timeout: u64,
    ) -> Self {
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        
        Self {
            cache_dir,
            game_data_cache_dir,
            mission_cache_dir,
            game_data_dirs,
            game_data_extensions,
            mission_dirs,
            mission_extensions,
            threads,
            timeout,
            verbose: false,
        }
    }
}

#[cfg(test)]
mod tests {
    // ... existing code ...
} 