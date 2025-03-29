use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Path to the database file
    pub db_path: PathBuf,
    
    /// Path to the cache directory for extracted files
    pub cache_dir: PathBuf,
    
    /// Game data cache subdirectory
    pub game_data_cache_dir: PathBuf,
    
    /// Mission cache subdirectory
    pub mission_cache_dir: PathBuf,
    
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    
    /// SQLite busy timeout in milliseconds
    pub busy_timeout_ms: u32,
    
    /// Whether to use WAL mode
    pub use_wal: bool,
    
    /// Cache size in pages (negative for kibibytes)
    pub cache_size: i32,
    
    /// Synchronous mode (0=OFF, 1=NORMAL, 2=FULL, 3=EXTRA)
    pub synchronous: i32,
}

impl DatabaseConfig {
    /// Create a new configuration with the given database path and cache name
    pub fn new(db_path: impl Into<PathBuf>, cache_name: &str) -> Self {
        let db_path = db_path.into();
        let cache_parent = db_path.parent().unwrap_or(Path::new("."));
        let cache_dir = cache_parent.join(cache_name);
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        
        Self {
            db_path,
            cache_dir,
            game_data_cache_dir,
            mission_cache_dir,
            max_connections: 10,
            busy_timeout_ms: 5000,
            use_wal: true,
            cache_size: -10000, // 10MB
            synchronous: 1,     // NORMAL
        }
    }
    
    /// Create a new configuration with specific cache directories
    pub fn with_cache_dirs(
        db_path: PathBuf,
        cache_dir: PathBuf,
        game_data_cache_dir: PathBuf,
        mission_cache_dir: PathBuf,
    ) -> Self {
        Self {
            db_path,
            cache_dir,
            game_data_cache_dir,
            mission_cache_dir,
            max_connections: 10,
            busy_timeout_ms: 5000,
            use_wal: true,
            cache_size: -10000, // 10MB
            synchronous: 1,     // NORMAL
        }
    }
    
    /// Apply pragma settings to a connection
    pub fn apply_pragmas(&self, conn: &rusqlite::Connection) -> crate::error::Result<()> {
        if self.use_wal {
            conn.execute_batch("PRAGMA journal_mode = WAL;")?;
        }
        
        conn.execute_batch(&format!(
            "PRAGMA busy_timeout = {};
             PRAGMA cache_size = {};
             PRAGMA synchronous = {};
             PRAGMA temp_store = MEMORY;
             PRAGMA foreign_keys = ON;",
            self.busy_timeout_ms,
            self.cache_size,
            self.synchronous,
        ))?;
        
        Ok(())
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::new(
            PathBuf::from("arma3_cache.db"),
            "cache",
        )
    }
}

/// Database statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    /// Number of classes
    pub class_count: usize,
    
    /// Number of PBO files
    pub pbo_count: usize,
    
    /// Number of extracted files
    pub extracted_file_count: usize,
    
    /// Number of failed extractions
    pub failed_extraction_count: usize,
    
    /// Number of missions
    pub mission_count: usize,
    
    /// Number of mission components
    pub mission_component_count: usize,
    
    /// Number of mission dependencies
    pub mission_dependency_count: usize,
    
    /// Database file size in bytes
    pub db_size: u64,
    
    /// Cache usage on disk in bytes
    pub cache_size: u64,
    
    /// Last modified timestamp
    pub last_modified: String,
    
    /// Schema version
    pub schema_version: i32,
}

impl Default for DatabaseStats {
    fn default() -> Self {
        Self {
            class_count: 0,
            pbo_count: 0,
            extracted_file_count: 0,
            failed_extraction_count: 0,
            mission_count: 0,
            mission_component_count: 0,
            mission_dependency_count: 0,
            db_size: 0,
            cache_size: 0,
            last_modified: String::new(),
            schema_version: 0,
        }
    }
}

impl DatabaseStats {
    /// Format the stats as a string
    pub fn format(&self) -> String {
        format!(
            "Database Statistics:
- Classes: {}
- PBOs: {}
- Extracted Files: {}
- Failed Extractions: {}
- Missions: {}
- Mission Components: {}
- Mission Dependencies: {}
- Schema Version: {}
- Last Modified: {}
- Database Size: {:.2} MB
- Cache Size: {:.2} MB",
            self.class_count,
            self.pbo_count,
            self.extracted_file_count,
            self.failed_extraction_count,
            self.mission_count,
            self.mission_component_count,
            self.mission_dependency_count,
            self.schema_version,
            self.last_modified,
            self.db_size as f64 / (1024.0 * 1024.0),
            self.cache_size as f64 / (1024.0 * 1024.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use tempfile::tempdir;
    
    #[test]
    fn test_config_defaults() {
        let config = DatabaseConfig::default();
        
        assert_eq!(config.db_path, PathBuf::from("arma3_cache.db"));
        assert_eq!(config.cache_dir, PathBuf::from("cache"));
        assert_eq!(config.game_data_cache_dir, PathBuf::from("cache/gamedata"));
        assert_eq!(config.mission_cache_dir, PathBuf::from("cache/missions"));
        assert!(config.use_wal);
        assert_eq!(config.cache_size, -10000);
    }
    
    #[test]
    fn test_apply_pragmas() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let config = DatabaseConfig::new(&db_path, "cache");
        
        let conn = Connection::open(&db_path).unwrap();
        config.apply_pragmas(&conn).unwrap();
        
        // Verify pragmas were applied
        let journal_mode: String = conn.query_row(
            "PRAGMA journal_mode",
            [],
            |row| row.get(0),
        ).unwrap();
        
        if config.use_wal {
            assert_eq!(journal_mode.to_uppercase(), "WAL");
        }
        
        let cache_size: i32 = conn.query_row(
            "PRAGMA cache_size",
            [],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(cache_size, config.cache_size);
    }
} 