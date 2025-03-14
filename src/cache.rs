use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read};
use sha2::{Sha256, Digest};
use crate::error::{Result, ToolError};

/// Manages caching of extracted files
#[derive(Clone)]
pub struct CacheManager {
    /// Root directory for cache
    cache_dir: PathBuf,
    
    /// Directory for game data cache
    game_data_cache_dir: PathBuf,
    
    /// Directory for mission cache
    mission_cache_dir: PathBuf,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new(cache_dir: PathBuf) -> Self {
        let game_data_cache_dir = cache_dir.join("game_data");
        let mission_cache_dir = cache_dir.join("missions");
        
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
        
        Self {
            cache_dir,
            game_data_cache_dir,
            mission_cache_dir,
        }
    }
    
    /// Get the cache directory for game data
    pub fn game_data_cache_dir(&self) -> &Path {
        &self.game_data_cache_dir
    }
    
    /// Get the cache directory for missions
    pub fn mission_cache_dir(&self) -> &Path {
        &self.mission_cache_dir
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
    
    /// Check if a PBO is cached and up to date
    pub fn is_cached(&self, pbo_path: &Path, cache_path: &Path) -> bool {
        // Check if cache directory exists
        if !cache_path.exists() {
            return false;
        }
        
        // Check if hash file exists
        let hash_file = cache_path.join(".hash");
        if !hash_file.exists() {
            return false;
        }
        
        // Read stored hash
        let stored_hash = match fs::read_to_string(&hash_file) {
            Ok(hash) => hash,
            Err(_) => return false,
        };
        
        // Calculate current hash
        let current_hash = match self.calculate_file_hash(pbo_path) {
            Ok(hash) => hash,
            Err(_) => return false,
        };
        
        // Compare hashes
        stored_hash == current_hash
    }
    
    /// Update the cache for a PBO
    pub fn update_cache(&self, pbo_path: &Path, cache_path: &Path) -> Result<()> {
        // Calculate hash
        let hash = self.calculate_file_hash(pbo_path)?;
        
        // Write hash to file
        let hash_file = cache_path.join(".hash");
        fs::write(&hash_file, hash)
            .map_err(|e| ToolError::CacheError(format!("Failed to write hash file: {}", e)))?;
        
        Ok(())
    }
    
    /// Calculate a hash for a file path (not content)
    fn calculate_path_hash(&self, path: &Path) -> String {
        let path_str = path.to_string_lossy();
        let mut hasher = Sha256::new();
        hasher.update(path_str.as_bytes());
        format!("{:x}", hasher.finalize())[..8].to_string()
    }
    
    /// Calculate a hash for a file's content
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
    
    /// Clear the cache
    pub fn clear_cache(&self) -> Result<()> {
        // Clear game data cache
        if self.game_data_cache_dir.exists() {
            fs::remove_dir_all(&self.game_data_cache_dir)
                .map_err(|e| ToolError::CacheError(format!("Failed to clear game data cache: {}", e)))?;
            fs::create_dir_all(&self.game_data_cache_dir)
                .map_err(|e| ToolError::CacheError(format!("Failed to recreate game data cache directory: {}", e)))?;
        }
        
        // Clear mission cache
        if self.mission_cache_dir.exists() {
            fs::remove_dir_all(&self.mission_cache_dir)
                .map_err(|e| ToolError::CacheError(format!("Failed to clear mission cache: {}", e)))?;
            fs::create_dir_all(&self.mission_cache_dir)
                .map_err(|e| ToolError::CacheError(format!("Failed to recreate mission cache directory: {}", e)))?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};
    
    #[test]
    fn test_cache_path_generation() {
        let temp_dir = TempDir::new().unwrap();
        let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
        
        let pbo_path = Path::new("/path/to/test.pbo");
        let cache_path = cache_manager.get_game_data_cache_path(pbo_path);
        
        assert!(cache_path.starts_with(cache_manager.game_data_cache_dir()));
        assert!(cache_path.to_string_lossy().contains("test_"));
    }
    
    #[test]
    fn test_file_hash_calculation() {
        let temp_dir = TempDir::new().unwrap();
        let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
        
        // Create a test file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        // Calculate hash
        let hash = cache_manager.calculate_file_hash(file.path()).unwrap();
        
        // Hash should be a 64-character hex string
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_cache_update_and_check() {
        let temp_dir = TempDir::new().unwrap();
        let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
        
        // Create a test file
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "test content").unwrap();
        
        // Create cache directory
        let cache_path = temp_dir.path().join("cache_test");
        fs::create_dir_all(&cache_path).unwrap();
        
        // Update cache
        cache_manager.update_cache(file.path(), &cache_path).unwrap();
        
        // Check if cached
        assert!(cache_manager.is_cached(file.path(), &cache_path));
        
        // Modify file
        writeln!(file, "modified content").unwrap();
        
        // Should no longer be cached
        assert!(!cache_manager.is_cached(file.path(), &cache_path));
    }
} 