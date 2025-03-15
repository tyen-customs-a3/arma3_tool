use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Read;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use log::{debug, warn};
use crate::error::{Result, ToolError};
use walkdir::WalkDir;
use std::time::SystemTime;
use crate::scanner::models::{GameDataClasses, MissionData};

/// Database entry for a cached PBO
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    /// Path to the PBO file
    pbo_path: PathBuf,
    
    /// Path to the extracted cache directory
    cache_path: PathBuf,
    
    /// Quick hash of the PBO file (size + mtime)
    quick_hash: String,
    
    /// File size in bytes
    file_size: u64,
    
    /// Last modification time as seconds since epoch
    mtime: u64,
    
    /// Timestamp of when the cache was last updated
    timestamp: u64,
    
    /// Whether the PBO caused an error during extraction
    had_error: bool,
    
    /// Error message if there was an error
    error_message: Option<String>,
    
    /// File extensions that were extracted
    extracted_extensions: Vec<String>,
    
    /// File filter pattern used for extraction
    file_filter_pattern: Option<String>,
    
    /// Number of files extracted
    files_extracted: usize,
}

/// Cache database structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheDatabase {
    /// Version of the cache database
    version: String,
    
    /// Map of PBO paths to cache entries
    entries: HashMap<String, CacheEntry>,
}

impl Default for CacheDatabase {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            entries: HashMap::new(),
        }
    }
}

/// Manages caching of extracted files
#[derive(Clone)]
pub struct CacheManager {
    /// Directory for game data cache
    game_data_cache_dir: PathBuf,
    
    /// Directory for mission cache
    mission_cache_dir: PathBuf,
    
    /// Path to the game data cache database file
    game_data_database_path: PathBuf,
    
    /// Path to the mission cache database file
    mission_database_path: PathBuf,
    
    /// In-memory game data cache database
    game_data_database: CacheDatabase,
    
    /// In-memory mission cache database
    mission_database: CacheDatabase,

    /// Flag to track if game data database has been modified
    game_data_modified: bool,

    /// Flag to track if mission database has been modified
    mission_modified: bool,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new(cache_dir: PathBuf) -> Self {
        let game_data_cache_dir = cache_dir.join("game_data");
        let mission_cache_dir = cache_dir.join("missions");
        let game_data_database_path = cache_dir.join("game_data_cache_db.json");
        let mission_database_path = cache_dir.join("mission_cache_db.json");
        
        // Create cache directories if they don't exist
        if !game_data_cache_dir.exists() {
            fs::create_dir_all(&game_data_cache_dir).unwrap_or_else(|e| {
                log::warn!("Failed to create game data cache directory: {}", e);
            });
        }
        
        if !mission_cache_dir.exists() {
            fs::create_dir_all(&mission_cache_dir).unwrap_or_else(|e| {
                log::warn!("Failed to create mission cache directory: {}", e);
            });
        }
        
        // Load or create game data cache database
        let game_data_database = if game_data_database_path.exists() {
            match fs::read_to_string(&game_data_database_path) {
                Ok(content) => match serde_json::from_str::<CacheDatabase>(&content) {
                    Ok(db) => {
                        debug!("Loaded game data cache database with {} entries", db.entries.len());
                        db
                    },
                    Err(e) => {
                        warn!("Failed to parse game data cache database, creating new one: {}", e);
                        CacheDatabase::default()
                    }
                },
                Err(e) => {
                    warn!("Failed to read game data cache database, creating new one: {}", e);
                    CacheDatabase::default()
                }
            }
        } else {
            debug!("Creating new game data cache database");
            CacheDatabase::default()
        };
        
        // Load or create mission cache database
        let mission_database = if mission_database_path.exists() {
            match fs::read_to_string(&mission_database_path) {
                Ok(content) => match serde_json::from_str::<CacheDatabase>(&content) {
                    Ok(db) => {
                        debug!("Loaded mission cache database with {} entries", db.entries.len());
                        db
                    },
                    Err(e) => {
                        warn!("Failed to parse mission cache database, creating new one: {}", e);
                        CacheDatabase::default()
                    }
                },
                Err(e) => {
                    warn!("Failed to read mission cache database, creating new one: {}", e);
                    CacheDatabase::default()
                }
            }
        } else {
            debug!("Creating new mission cache database");
            CacheDatabase::default()
        };
        
        Self {
            game_data_cache_dir,
            mission_cache_dir,
            game_data_database_path,
            mission_database_path,
            game_data_database,
            mission_database,
            game_data_modified: false,
            mission_modified: false,
        }
    }
    
    /// Save the game data cache database to disk if modified
    fn save_game_data_database(&mut self) -> Result<()> {
        if !self.game_data_modified {
            return Ok(());
        }

        let json = serde_json::to_string_pretty(&self.game_data_database)
            .map_err(|e| ToolError::CacheError(format!("Failed to serialize game data cache database: {}", e)))?;
            
        // Write to a temporary file first
        let temp_path = self.game_data_database_path.with_extension("tmp");
        fs::write(&temp_path, &json)
            .map_err(|e| ToolError::CacheError(format!("Failed to write game data cache database: {}", e)))?;
            
        // Atomically rename the temporary file to the actual database file
        fs::rename(&temp_path, &self.game_data_database_path)
            .map_err(|e| ToolError::CacheError(format!("Failed to rename temporary database file: {}", e)))?;
            
        debug!("Saved game data cache database with {} entries", self.game_data_database.entries.len());
        self.game_data_modified = false;
        
        Ok(())
    }
    
    /// Save the mission cache database to disk if modified
    fn save_mission_database(&mut self) -> Result<()> {
        if !self.mission_modified {
            return Ok(());
        }

        let json = serde_json::to_string_pretty(&self.mission_database)
            .map_err(|e| ToolError::CacheError(format!("Failed to serialize mission cache database: {}", e)))?;
            
        // Write to a temporary file first
        let temp_path = self.mission_database_path.with_extension("tmp");
        fs::write(&temp_path, &json)
            .map_err(|e| ToolError::CacheError(format!("Failed to write mission cache database: {}", e)))?;
            
        // Atomically rename the temporary file to the actual database file
        fs::rename(&temp_path, &self.mission_database_path)
            .map_err(|e| ToolError::CacheError(format!("Failed to rename temporary database file: {}", e)))?;
            
        debug!("Saved mission cache database with {} entries", self.mission_database.entries.len());
        self.mission_modified = false;
        
        Ok(())
    }

    /// Save all modified databases to disk
    pub fn save_all(&mut self) -> Result<()> {
        self.save_game_data_database()?;
        self.save_mission_database()?;
        Ok(())
    }
    
    /// Get the cache path for a game data PBO
    pub fn get_game_data_cache_path(&self, pbo_path: &Path) -> PathBuf {
        // Create a unique path based on the PBO path
        let pbo_name = pbo_path.file_stem().unwrap_or_default();
        let hash = self.calculate_path_hash(pbo_path);
        self.game_data_cache_dir.join(format!("{}_{}", pbo_name.to_string_lossy(), hash))
    }
    
    /// Get the cache path for a mission PBO
    pub fn get_mission_cache_path(&self, pbo_path: &Path) -> PathBuf {
        // Create a unique path based on the PBO path
        let pbo_name = pbo_path.file_stem().unwrap_or_default();
        let hash = self.calculate_path_hash(pbo_path);
        self.mission_cache_dir.join(format!("{}_{}", pbo_name.to_string_lossy(), hash))
    }
    
    /// Check if a game data PBO is cached and up to date
    pub fn is_game_data_cached(&self, pbo_path: &Path, cache_path: &Path) -> bool {
        self.is_cached_in_database(pbo_path, cache_path, &self.game_data_database)
    }
    
    /// Check if a mission PBO is cached and up to date
    pub fn is_mission_cached(&self, pbo_path: &Path, cache_path: &Path) -> bool {
        self.is_cached_in_database(pbo_path, cache_path, &self.mission_database)
    }
    
    /// Check if a PBO is cached and up to date in the specified database
    fn is_cached_in_database(&self, pbo_path: &Path, cache_path: &Path, database: &CacheDatabase) -> bool {
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Check if entry exists in database
        if let Some(entry) = database.entries.get(&path_key) {
            // If this is a permanent error, always consider it cached to skip it
            if let Some(error_msg) = &entry.error_message {
                if error_msg.starts_with("PERMANENT_ERROR:") {
                    warn!("Skipping PBO with permanent error: {} - {}", pbo_path.display(), 
                          error_msg.strip_prefix("PERMANENT_ERROR: ").unwrap_or(error_msg));
                    return true;
                }
            }
            
            // Check if the cache path matches
            if entry.cache_path != cache_path {
                return false;
            }
            
            // If the PBO had an error before, consider it cached to skip it
            if entry.had_error {
                warn!("Skipping PBO that previously had an error: {}", pbo_path.display());
                return true;
            }
            
            // Check if cache directory exists
            if !cache_path.exists() {
                return false;
            }
            
            // Calculate current quick hash
            match self.calculate_quick_hash(pbo_path) {
                Ok((quick_hash, _, _)) => {
                    // Compare quick hashes
                    return entry.quick_hash == quick_hash;
                },
                Err(_) => return false,
            }
        }
        
        false
    }
    
    /// Check if extraction patterns have changed for game data
    pub fn has_game_data_pattern_changed(&self, pbo_path: &Path, extensions: &[String], file_filter: Option<&str>) -> bool {
        self.has_pattern_changed_in_database(pbo_path, extensions, file_filter, &self.game_data_database)
    }
    
    /// Check if extraction patterns have changed for missions
    pub fn has_mission_pattern_changed(&self, pbo_path: &Path, extensions: &[String], file_filter: Option<&str>) -> bool {
        self.has_pattern_changed_in_database(pbo_path, extensions, file_filter, &self.mission_database)
    }
    
    /// Check if extraction patterns have changed in the specified database
    fn has_pattern_changed_in_database(&self, pbo_path: &Path, extensions: &[String], file_filter: Option<&str>, database: &CacheDatabase) -> bool {
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Check if entry exists in database
        if let Some(entry) = database.entries.get(&path_key) {
            // If this is a permanent error, patterns should never be considered changed
            if let Some(error_msg) = &entry.error_message {
                if error_msg.starts_with("PERMANENT_ERROR:") {
                    debug!("PBO has permanent error, ignoring pattern changes: {}", pbo_path.display());
                    return false;
                }
            }
            
            // Convert extensions to HashSet for comparison
            let current_extensions: HashSet<_> = extensions.iter().collect();
            let cached_extensions: HashSet<_> = entry.extracted_extensions.iter().collect();
            
            // Check if extensions have changed
            if current_extensions != cached_extensions {
                debug!("Extraction patterns changed for PBO: {}", pbo_path.display());
                return true;
            }
            
            // Check if file filter has changed
            let current_filter = file_filter.map(|s| s.to_string());
            if entry.file_filter_pattern != current_filter {
                debug!("File filter changed for PBO: {}", pbo_path.display());
                return true;
            }
        }
        
        false
    }
    
    /// Update the game data cache for a PBO
    pub fn update_game_data_cache(&mut self, pbo_path: &Path, cache_path: &Path, extensions: &[String], file_filter: Option<&str>) -> Result<()> {
        // Calculate quick hash
        let (quick_hash, file_size, mtime) = self.calculate_quick_hash(pbo_path)?;
        
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Count files in the cache directory
        let files_extracted = self.count_files_in_directory(cache_path);
        
        // Create cache entry
        let entry = CacheEntry {
            pbo_path: pbo_path.to_path_buf(),
            cache_path: cache_path.to_path_buf(),
            quick_hash,
            file_size,
            mtime,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            had_error: false,
            error_message: None,
            extracted_extensions: extensions.to_vec(),
            file_filter_pattern: file_filter.map(|s| s.to_string()),
            files_extracted,
        };
        
        // Update database
        self.game_data_database.entries.insert(path_key, entry);
        self.game_data_modified = true;
        
        Ok(())
    }
    
    /// Update the mission cache for a PBO
    pub fn update_mission_cache(&mut self, pbo_path: &Path, cache_path: &Path, extensions: &[String], file_filter: Option<&str>) -> Result<()> {
        // Calculate quick hash
        let (quick_hash, file_size, mtime) = self.calculate_quick_hash(pbo_path)?;
        
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Count files in the cache directory
        let files_extracted = self.count_files_in_directory(cache_path);
        
        // Create cache entry
        let entry = CacheEntry {
            pbo_path: pbo_path.to_path_buf(),
            cache_path: cache_path.to_path_buf(),
            quick_hash,
            file_size,
            mtime,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            had_error: false,
            error_message: None,
            extracted_extensions: extensions.to_vec(),
            file_filter_pattern: file_filter.map(|s| s.to_string()),
            files_extracted,
        };
        
        // Update database
        self.mission_database.entries.insert(path_key, entry);
        self.mission_modified = true;
        
        Ok(())
    }
    
    /// Update the cache for a PBO in the specified database
    #[allow(dead_code)]
    fn update_cache_in_database(&mut self, pbo_path: &Path, cache_path: &Path, extensions: &[String], file_filter: Option<&str>, database: &mut CacheDatabase) -> Result<()> {
        // Calculate quick hash
        let (quick_hash, file_size, mtime) = self.calculate_quick_hash(pbo_path)?;
        
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Count files in the cache directory
        let files_extracted = self.count_files_in_directory(cache_path);
        
        // Create or update cache entry
        database.entries.insert(path_key, CacheEntry {
            pbo_path: pbo_path.to_path_buf(),
            cache_path: cache_path.to_path_buf(),
            quick_hash,
            file_size,
            mtime,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            had_error: false,
            error_message: None,
            extracted_extensions: extensions.to_vec(),
            file_filter_pattern: file_filter.map(|s| s.to_string()),
            files_extracted,
        });
        
        Ok(())
    }
    
    /// Mark a game data PBO as having an error
    pub fn mark_game_data_error(&mut self, pbo_path: &Path, error_message: &str, extensions: &[String], file_filter: Option<&str>) -> Result<()> {
        // Check if this is a "no files to extract" error, which is not a real error
        if self.is_no_files_to_extract_error(error_message) {
            debug!("PBO had no files to extract, treating as success: {}", pbo_path.display());
            // Create an empty cache directory if it doesn't exist
            let cache_path = self.get_game_data_cache_path(pbo_path);
            if !cache_path.exists() {
                fs::create_dir_all(&cache_path)
                    .map_err(|e| ToolError::CacheError(format!("Failed to create cache directory: {}", e)))?;
            }
            // Mark as successful extraction with 0 files
            return self.update_game_data_cache(pbo_path, &cache_path, extensions, file_filter);
        }
        
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Calculate quick hash (best effort)
        let (quick_hash, file_size, mtime) = self.calculate_quick_hash(pbo_path).unwrap_or_else(|_| {
            (String::new(), 0, 0)
        });
        
        // Create error entry
        let entry = CacheEntry {
            pbo_path: pbo_path.to_path_buf(),
            cache_path: PathBuf::new(), // Empty path since extraction failed
            quick_hash,
            file_size,
            mtime,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            had_error: true,
            error_message: Some(error_message.to_string()),
            extracted_extensions: extensions.to_vec(),
            file_filter_pattern: file_filter.map(|s| s.to_string()),
            files_extracted: 0,
        };
        
        // Update database
        self.game_data_database.entries.insert(path_key, entry);
        self.game_data_modified = true;
        
        Ok(())
    }
    
    /// Mark a mission PBO as having an error
    pub fn mark_mission_error(&mut self, pbo_path: &Path, error_message: &str, extensions: &[String], file_filter: Option<&str>) -> Result<()> {
        // Check if this is a "no files to extract" error, which is not a real error
        if self.is_no_files_to_extract_error(error_message) {
            debug!("Mission PBO had no files to extract, treating as success: {}", pbo_path.display());
            // Create an empty cache directory if it doesn't exist
            let cache_path = self.get_mission_cache_path(pbo_path);
            if !cache_path.exists() {
                fs::create_dir_all(&cache_path)
                    .map_err(|e| ToolError::CacheError(format!("Failed to create cache directory: {}", e)))?;
            }
            // Mark as successful extraction with 0 files
            return self.update_mission_cache(pbo_path, &cache_path, extensions, file_filter);
        }
        
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Calculate quick hash (best effort)
        let (quick_hash, file_size, mtime) = self.calculate_quick_hash(pbo_path).unwrap_or_else(|_| {
            (String::new(), 0, 0)
        });
        
        // Create error entry
        let entry = CacheEntry {
            pbo_path: pbo_path.to_path_buf(),
            cache_path: PathBuf::new(), // Empty path since extraction failed
            quick_hash,
            file_size,
            mtime,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            had_error: true,
            error_message: Some(error_message.to_string()),
            extracted_extensions: extensions.to_vec(),
            file_filter_pattern: file_filter.map(|s| s.to_string()),
            files_extracted: 0,
        };
        
        // Update database
        self.mission_database.entries.insert(path_key, entry);
        self.mission_modified = true;
        
        Ok(())
    }
    
    /// Mark a PBO as having an error in the specified database
    #[allow(dead_code)]
    fn mark_error_in_database(&mut self, pbo_path: &Path, error_message: &str, extensions: &[String], file_filter: Option<&str>, database: &mut CacheDatabase) -> Result<()> {
        // Check if this is a "no files to extract" error, which is not a real error
        if self.is_no_files_to_extract_error(error_message) {
            debug!("PBO had no files to extract, treating as success: {}", pbo_path.display());
            // This is a utility method, so we don't handle the success case here
            // The caller should use update_cache_in_database instead
            return Ok(());
        }
        
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Calculate quick hash (best effort)
        let (quick_hash, file_size, mtime) = self.calculate_quick_hash(pbo_path).unwrap_or_else(|_| {
            (String::new(), 0, 0)
        });
        
        // Create or update cache entry
        database.entries.insert(path_key, CacheEntry {
            pbo_path: pbo_path.to_path_buf(),
            cache_path: PathBuf::new(), // Empty path since extraction failed
            quick_hash,
            file_size,
            mtime,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            had_error: true,
            error_message: Some(error_message.to_string()),
            extracted_extensions: extensions.to_vec(),
            file_filter_pattern: file_filter.map(|s| s.to_string()),
            files_extracted: 0,
        });
        
        Ok(())
    }
    
    /// Check if a game data PBO has previously had an error
    pub fn has_game_data_error(&self, pbo_path: &Path) -> bool {
        self.has_error_in_database(pbo_path, &self.game_data_database)
    }
    
    /// Check if a mission PBO has previously had an error
    pub fn has_mission_error(&self, pbo_path: &Path) -> bool {
        self.has_error_in_database(pbo_path, &self.mission_database)
    }
    
    /// Check if a PBO has previously had an error in the specified database
    fn has_error_in_database(&self, pbo_path: &Path, database: &CacheDatabase) -> bool {
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Check if entry exists in database and had an error
        if let Some(entry) = database.entries.get(&path_key) {
            return entry.had_error;
        }
        
        false
    }
    
    /// Get error message for a game data PBO
    pub fn get_game_data_error_message(&self, pbo_path: &Path) -> Option<String> {
        self.get_error_message_from_database(pbo_path, &self.game_data_database)
    }
    
    /// Get error message for a mission PBO
    pub fn get_mission_error_message(&self, pbo_path: &Path) -> Option<String> {
        self.get_error_message_from_database(pbo_path, &self.mission_database)
    }
    
    /// Get error message for a PBO from the specified database
    fn get_error_message_from_database(&self, pbo_path: &Path, database: &CacheDatabase) -> Option<String> {
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Check if entry exists in database and get error message
        if let Some(entry) = database.entries.get(&path_key) {
            return entry.error_message.clone();
        }
        
        None
    }
    
    /// Count files in a directory recursively
    fn count_files_in_directory(&self, dir_path: &Path) -> usize {
        let mut count = 0;
        
        if !dir_path.exists() || !dir_path.is_dir() {
            return 0;
        }
        
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                count += 1;
            }
        }
        
        count
    }
    
    /// Calculate a hash for a file path (not content)
    fn calculate_path_hash(&self, path: &Path) -> String {
        let path_str = path.to_string_lossy();
        let mut hasher = Sha256::new();
        hasher.update(path_str.as_bytes());
        format!("{:x}", hasher.finalize())[..8].to_string()
    }
    
    /// Calculate a quick hash based on file size and modification time
    fn calculate_quick_hash(&self, path: &Path) -> Result<(String, u64, u64)> {
        let metadata = fs::metadata(path)
            .map_err(|e| ToolError::CacheError(format!("Failed to get file metadata: {}", e)))?;
        
        let file_size = metadata.len();
        
        // Get modification time as seconds since epoch
        let mtime = metadata.modified()
            .map_err(|e| ToolError::CacheError(format!("Failed to get file modification time: {}", e)))?
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| ToolError::CacheError(format!("Failed to convert modification time: {}", e)))?
            .as_secs();
        
        // Create a quick hash from size and mtime
        let mut hasher = Sha256::new();
        hasher.update(format!("{}:{}", file_size, mtime).as_bytes());
        let quick_hash = format!("{:x}", hasher.finalize())[..16].to_string();
        
        debug!("Quick hash for {}: {} (size={}, mtime={})", path.display(), quick_hash, file_size, mtime);
        
        Ok((quick_hash, file_size, mtime))
    }
    
    /// Calculate a hash for a file's content (legacy method, kept for reference)
    #[allow(dead_code)]
    fn calculate_file_hash(&self, path: &Path) -> Result<String> {
        let mut file = File::open(path)
            .map_err(|e| ToolError::CacheError(format!("Failed to open file for hashing: {}", e)))?;
            
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        
        loop {
            let bytes_read = file.read(&mut buffer)
                .map_err(|e| ToolError::CacheError(format!("Failed to read file for hashing: {}", e)))?;
                
            if bytes_read == 0 {
                break;
            }
            
            hasher.update(&buffer[..bytes_read]);
        }
        
        Ok(format!("{:x}", hasher.finalize()))
    }
    
    /// Check if an error message indicates "no files to extract" (error code 11)
    /// This is not a real error, but rather an indication that the PBO doesn't contain
    /// any files matching our filter criteria.
    pub fn is_no_files_to_extract_error(&self, error_message: &str) -> bool {
        error_message.contains("return code 11") || 
        error_message.contains("no file(s) to extract") ||
        error_message.contains("DePbo:No file(s) found to extract")
    }
    
    /// Check if an error message indicates a bad SHA validation error
    /// These PBOs are corrupted or modified and should not be attempted to extract again
    pub fn is_bad_sha_error(&self, error_message: &str) -> bool {
        error_message.contains("Bad Sha detected") || 
        error_message.contains("Bad Sha") ||
        error_message.contains("Bad SHA")
    }
    
    /// Mark a game data PBO as having a permanent error (like bad SHA)
    /// This will ensure we never try to extract this PBO again, even if patterns change
    pub fn mark_game_data_permanent_error(&mut self, pbo_path: &Path, error_message: &str) -> Result<()> {
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Calculate quick hash (best effort)
        let (quick_hash, file_size, mtime) = self.calculate_quick_hash(pbo_path).unwrap_or_else(|_| {
            (String::new(), 0, 0)
        });
        
        // Create error entry with a special marker for permanent errors
        let entry = CacheEntry {
            pbo_path: pbo_path.to_path_buf(),
            cache_path: PathBuf::new(), // Empty path since extraction failed
            quick_hash,
            file_size,
            mtime,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            had_error: true,
            error_message: Some(format!("PERMANENT_ERROR: {}", error_message)),
            extracted_extensions: Vec::new(), // Empty since we won't extract
            file_filter_pattern: None,
            files_extracted: 0,
        };
        
        // Update database
        self.game_data_database.entries.insert(path_key, entry);
        self.game_data_modified = true;
        
        Ok(())
    }
    
    /// Mark a mission PBO as having a permanent error (like bad SHA)
    /// This will ensure we never try to extract this PBO again, even if patterns change
    pub fn mark_mission_permanent_error(&mut self, pbo_path: &Path, error_message: &str) -> Result<()> {
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Calculate quick hash (best effort)
        let (quick_hash, file_size, mtime) = self.calculate_quick_hash(pbo_path).unwrap_or_else(|_| {
            (String::new(), 0, 0)
        });
        
        // Create error entry with a special marker for permanent errors
        let entry = CacheEntry {
            pbo_path: pbo_path.to_path_buf(),
            cache_path: PathBuf::new(), // Empty path since extraction failed
            quick_hash,
            file_size,
            mtime,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            had_error: true,
            error_message: Some(format!("PERMANENT_ERROR: {}", error_message)),
            extracted_extensions: Vec::new(), // Empty since we won't extract
            file_filter_pattern: None,
            files_extracted: 0,
        };
        
        // Update database
        self.mission_database.entries.insert(path_key, entry);
        self.mission_modified = true;
        
        Ok(())
    }
    
    /// Check if a game data PBO has a permanent error
    pub fn has_game_data_permanent_error(&self, pbo_path: &Path) -> bool {
        self.has_permanent_error_in_database(pbo_path, &self.game_data_database)
    }
    
    /// Check if a mission PBO has a permanent error
    pub fn has_mission_permanent_error(&self, pbo_path: &Path) -> bool {
        self.has_permanent_error_in_database(pbo_path, &self.mission_database)
    }
    
    /// Check if a PBO has a permanent error in the specified database
    fn has_permanent_error_in_database(&self, pbo_path: &Path, database: &CacheDatabase) -> bool {
        // Get normalized path string for database lookup
        let path_key = pbo_path.to_string_lossy().to_string();
        
        // Check if entry exists in database and has a permanent error
        if let Some(entry) = database.entries.get(&path_key) {
            if let Some(error_msg) = &entry.error_message {
                return error_msg.starts_with("PERMANENT_ERROR:");
            }
        }
        
        false
    }

    /// Load cached game data
    pub fn load_game_data(&self) -> Result<GameDataClasses> {
        let cache_file = self.game_data_cache_dir.join("game_data.json");
        if !cache_file.exists() {
            return Err(ToolError::CacheError("Game data cache not found".to_string()));
        }

        let content = fs::read_to_string(&cache_file)
            .map_err(|e| ToolError::CacheError(format!("Failed to read game data cache: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| ToolError::CacheError(format!("Failed to parse game data cache: {}", e)))
    }

    /// Load cached mission data
    pub fn load_mission_data(&self) -> Result<MissionData> {
        let cache_file = self.mission_cache_dir.join("mission_data.json");
        if !cache_file.exists() {
            return Err(ToolError::CacheError("Mission data cache not found".to_string()));
        }

        let content = fs::read_to_string(&cache_file)
            .map_err(|e| ToolError::CacheError(format!("Failed to read mission data cache: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| ToolError::CacheError(format!("Failed to parse mission data cache: {}", e)))
    }

    /// Get the cache directory path
    pub fn get_cache_dir(&self) -> &Path {
        self.game_data_cache_dir.parent().unwrap_or(&self.game_data_cache_dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};
    use std::thread::sleep;
    use std::time::Duration;
    
    #[test]
    fn test_quick_hash_calculation() {
        let temp_dir = TempDir::new().unwrap();
        let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
        
        // Create a test file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        // Calculate quick hash
        let (quick_hash, file_size, mtime) = cache_manager.calculate_quick_hash(file.path()).unwrap();
        
        // Quick hash should be a 16-character hex string
        assert_eq!(quick_hash.len(), 16);
        assert!(quick_hash.chars().all(|c| c.is_ascii_hexdigit()));
        
        // File size should be correct
        assert_eq!(file_size, fs::metadata(file.path()).unwrap().len());
        
        // Modify file and check that hash changes
        sleep(Duration::from_secs(1)); // Ensure mtime changes
        writeln!(file, "modified content").unwrap();
        
        let (new_quick_hash, new_file_size, new_mtime) = cache_manager.calculate_quick_hash(file.path()).unwrap();
        
        // Hash should be different
        assert_ne!(quick_hash, new_quick_hash);
        
        // File size should be larger
        assert!(new_file_size > file_size);
        
        // Modification time should be later
        assert!(new_mtime >= mtime);
    }
    
    #[test]
    fn test_cache_update_and_check() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
        
        // Create a test file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        // Create cache directory
        let cache_path = temp_dir.path().join("cache_test");
        fs::create_dir_all(&cache_path).unwrap();
        
        // Create a test file in the cache directory
        let test_file = cache_path.join("test.txt");
        File::create(&test_file).unwrap().write_all(b"test").unwrap();
        
        // Update cache with extensions
        let extensions = vec!["txt".to_string(), "cfg".to_string()];
        cache_manager.update_game_data_cache(file.path(), &cache_path, &extensions, Some("*.txt,*.cfg")).unwrap();
        
        // Check if cached
        assert!(cache_manager.is_game_data_cached(file.path(), &cache_path));
        
        // Check if pattern changed detection works
        let new_extensions = vec!["txt".to_string(), "sqf".to_string()];
        assert!(cache_manager.has_game_data_pattern_changed(file.path(), &new_extensions, Some("*.txt,*.sqf")));
        
        // Check if pattern unchanged detection works
        assert!(!cache_manager.has_game_data_pattern_changed(file.path(), &extensions, Some("*.txt,*.cfg")));
        
        // Modify file
        sleep(Duration::from_secs(1)); // Ensure mtime changes
        writeln!(file, "modified content").unwrap();
        
        // Should no longer be cached
        assert!(!cache_manager.is_game_data_cached(file.path(), &cache_path));
    }
    
    #[test]
    fn test_error_tracking() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
        
        // Create a test file
        let file = NamedTempFile::new().unwrap();
        
        // Mark as error with extensions
        let extensions = vec!["txt".to_string(), "cfg".to_string()];
        cache_manager.mark_game_data_error(file.path(), "Test error message", &extensions, Some("*.txt,*.cfg")).unwrap();
        
        // Check if has error
        assert!(cache_manager.has_game_data_error(file.path()));
        
        // Check error message
        assert_eq!(cache_manager.get_game_data_error_message(file.path()), Some("Test error message".to_string()));
        
        // Should be considered cached to skip it
        assert!(cache_manager.is_game_data_cached(file.path(), &temp_dir.path().join("nonexistent")));
        
        // Check if pattern changed detection works for errors
        let new_extensions = vec!["txt".to_string(), "sqf".to_string()];
        assert!(cache_manager.has_game_data_pattern_changed(file.path(), &new_extensions, Some("*.txt,*.sqf")));
    }
    
    #[test]
    fn test_count_files_in_directory() {
        let temp_dir = TempDir::new().unwrap();
        let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
        
        // Create some test files
        fs::create_dir_all(temp_dir.path().join("subdir")).unwrap();
        File::create(temp_dir.path().join("file1.txt")).unwrap().write_all(b"test").unwrap();
        File::create(temp_dir.path().join("file2.txt")).unwrap().write_all(b"test").unwrap();
        File::create(temp_dir.path().join("subdir/file3.txt")).unwrap().write_all(b"test").unwrap();
        
        // Count files
        let count = cache_manager.count_files_in_directory(temp_dir.path());
        assert_eq!(count, 3);
    }
    
    #[test]
    fn test_bad_sha_detection() {
        let temp_dir = TempDir::new().unwrap();
        let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
        
        // Test various bad SHA error messages
        assert!(cache_manager.is_bad_sha_error("Bad Sha detected"));
        assert!(cache_manager.is_bad_sha_error("warning only: Bad Sha detected"));
        assert!(cache_manager.is_bad_sha_error("***warning***file.pbo:Bad Sha"));
        assert!(cache_manager.is_bad_sha_error("Some error with Bad SHA in it"));
        
        // Test non-matching messages
        assert!(!cache_manager.is_bad_sha_error("Some other error"));
        assert!(!cache_manager.is_bad_sha_error("return code 11"));
    }
    
    #[test]
    fn test_permanent_error_handling() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
        
        // Create a test file
        let file = NamedTempFile::new().unwrap();
        
        // Mark as permanent error
        cache_manager.mark_game_data_permanent_error(file.path(), "Bad Sha detected").unwrap();
        
        // Check if has permanent error
        assert!(cache_manager.has_game_data_permanent_error(file.path()));
        
        // Should be considered cached to skip it
        assert!(cache_manager.is_game_data_cached(file.path(), &temp_dir.path().join("nonexistent")));
        
        // Pattern changes should be ignored for permanent errors
        let extensions = vec!["txt".to_string(), "cfg".to_string()];
        assert!(!cache_manager.has_game_data_pattern_changed(file.path(), &extensions, Some("*.txt,*.cfg")));
        
        // Different extensions should still be ignored
        let new_extensions = vec!["txt".to_string(), "sqf".to_string()];
        assert!(!cache_manager.has_game_data_pattern_changed(file.path(), &new_extensions, Some("*.txt,*.sqf")));
    }
} 