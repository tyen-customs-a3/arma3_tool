use std::path::{Path, PathBuf};
use std::time::SystemTime;
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
            db_path: cache_dir.join("cache.db"),
        }
    }
    
    /// Add a game data directory to scan
    pub fn add_game_data_dir(mut self, dir: PathBuf) -> Self {
        self.game_data_dirs.push(dir);
        self
    }
    
    /// Add a mission directory to scan
    pub fn add_mission_dir(mut self, dir: PathBuf) -> Self {
        self.mission_dirs.push(dir);
        self
    }
    
    /// Set the game data extensions to extract
    pub fn with_game_data_extensions(mut self, extensions: Vec<String>) -> Self {
        self.game_data_extensions = extensions;
        self
    }
    
    /// Set the mission extensions to extract
    pub fn with_mission_extensions(mut self, extensions: Vec<String>) -> Self {
        self.mission_extensions = extensions;
        self
    }
    
    /// Set the number of threads to use
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }
    
    /// Set the timeout in seconds
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// Enable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Set custom database path
    pub fn with_db_path(mut self, db_path: PathBuf) -> Self {
        self.db_path = db_path;
        self
    }
}