use std::path::{Path, PathBuf};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::error::{Result, CacheError};
use arma3_db::models::pbo::PboMetadataConversion;

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
    /// Original path to the PBO file (now stored as relative to base_dir)
    pub path: PathBuf,
    
    /// Original base directory (can be empty if already relative)
    pub base_dir: PathBuf,
    
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
            base_dir: PathBuf::new(),
            last_modified: metadata.modified()
                .map_err(|e| CacheError::FileOperation(format!("Failed to get modified time: {}", e)))?,
            file_size: metadata.len(),
            extraction_time: SystemTime::now(),
            extracted_files: Vec::new(),
            used_extensions,
            pbo_type,
        })
    }
    
    /// Create new metadata with a relative path
    pub fn new_with_base_dir(
        path: PathBuf,
        base_dir: PathBuf,
        pbo_type: PboType,
        used_extensions: Vec<String>
    ) -> Result<Self> {
        // Get the full path for accessing metadata
        let full_path = if path.is_relative() {
            base_dir.join(&path)
        } else {
            path.clone()
        };
        
        let metadata = std::fs::metadata(&full_path)
            .map_err(|e| CacheError::FileOperation(format!("Failed to get metadata for {}: {}", full_path.display(), e)))?;
        
        Ok(Self {
            path,
            base_dir,
            last_modified: metadata.modified()
                .map_err(|e| CacheError::FileOperation(format!("Failed to get modified time: {}", e)))?,
            file_size: metadata.len(),
            extraction_time: SystemTime::now(),
            extracted_files: Vec::new(),
            used_extensions,
            pbo_type,
        })
    }
    
    /// Get the full path to the PBO file
    pub fn get_full_path(&self) -> PathBuf {
        if self.path.is_absolute() || self.base_dir.as_os_str().is_empty() {
            self.path.clone()
        } else {
            self.base_dir.join(&self.path)
        }
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

    /// Get full paths to extracted files by combining with a cache directory
    pub fn get_full_extracted_paths(&self, cache_dir: &Path) -> Vec<PathBuf> {
        self.extracted_files
            .iter()
            .map(|path| cache_dir.join(path))
            .collect()
    }
    
    // Convert SystemTime to DateTime<Utc>
    fn system_time_to_datetime(system_time: SystemTime) -> DateTime<Utc> {
        let duration = system_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = duration.as_secs() as i64;
        let nsecs = duration.subsec_nanos();
        DateTime::<Utc>::from_timestamp(secs, nsecs).unwrap_or_else(|| Utc::now())
    }
}

// Implement PboMetadataConversion trait for PboMetadata
impl PboMetadataConversion for PboMetadata {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    
    fn get_full_path(&self) -> PathBuf {
        self.get_full_path()
    }
    
    fn get_base_dir(&self) -> Option<&PathBuf> {
        if self.base_dir.as_os_str().is_empty() {
            None
        } else {
            Some(&self.base_dir)
        }
    }
    
    fn get_file_size(&self) -> u64 {
        self.file_size
    }
    
    fn get_last_modified(&self) -> DateTime<Utc> {
        Self::system_time_to_datetime(self.last_modified)
    }
    
    fn get_extraction_time(&self) -> DateTime<Utc> {
        Self::system_time_to_datetime(self.extraction_time)
    }
    
    fn get_pbo_type(&self) -> &str {
        match self.pbo_type {
            PboType::GameData => "GameData",
            PboType::Mission => "Mission",
        }
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
    
    /// Path to the database file
    pub db_path: PathBuf,
}

impl ExtractionConfig {
    /// Create a new configuration with default values
    pub fn new(cache_dir: PathBuf) -> Self {
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        
        Self {
            cache_dir: cache_dir.clone(),
            game_data_cache_dir,
            mission_cache_dir,
            game_data_dirs: Vec::new(),
            game_data_extensions: vec!["hpp".into(), "cpp".into(), "sqf".into()],
            mission_dirs: Vec::new(),
            mission_extensions: vec!["hpp".into(), "cpp".into(), "sqf".into(), "sqm".into()],
            threads: num_cpus::get(),
            timeout: 400,
            verbose: false,
            db_path: cache_dir.join("arma3.db"),
        }
    }
}