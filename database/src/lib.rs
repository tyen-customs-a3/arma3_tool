//! # Arma 3 Database
//! 
//! SQLite database integration for Arma 3 Tool.
//! 
//! This crate provides a database-backed system for storing and querying
//! Arma 3 class hierarchies, PBO files, and related data.

pub mod error;
pub mod models;
pub mod schema;
pub mod queries;
pub mod repos;

use std::path::{Path, PathBuf};
use std::fs;
use log::info;
use rusqlite::OptionalExtension;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

pub use error::{Result, DatabaseError};
pub use models::{CacheConfig, DatabaseStats};
pub use schema::initialize_schema;
// Re-export repository types for easier access
pub use queries::cache::Cache;
// Use our custom ClassRepository instead of the queries one
// pub use queries::class_repository::ClassRepository;
pub use queries::mission_repository::MissionRepository;
pub use queries::graph_query_engine::GraphQueryEngine;
// Re-export model types for easier access
pub use models::pbo::{PboModel, PboType, ExtractedFile, FailedExtraction, normalize_path};
pub use models::class::{ClassModel, ClassHierarchyNode};
pub use models::mission::{MissionModel, MissionComponentModel, MissionDependencyModel};
// Re-export repo types
pub use repos::ClassRepository;

/// Version of the crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Database manager for Arma 3 data
#[derive(Clone)]
pub struct DatabaseManager {
    /// Connection pool
    pool: Pool<SqliteConnectionManager>,
    
    /// Configuration
    config: CacheConfig,
}

impl DatabaseManager {
    /// Create a new database manager with default configuration
    pub fn new(db_path: impl Into<PathBuf>) -> Result<Self> {
        let config = CacheConfig::new(db_path, "cache");
        Self::with_config(config)
    }
    
    /// Create a new database manager with custom configuration
    pub fn with_config(config: CacheConfig) -> Result<Self> {
        // Ensure parent directories exist
        if let Some(parent) = config.db_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                DatabaseError::FileSystem {
                    path: parent.to_path_buf(),
                    message: format!("Failed to create directory: {}", e),
                }
            })?;
        }
        
        // Ensure cache directories exist
        for dir in [
            &config.cache_dir,
            &config.game_data_cache_dir,
            &config.mission_cache_dir,
        ] {
            fs::create_dir_all(dir).map_err(|e| {
                DatabaseError::FileSystem {
                    path: dir.to_path_buf(),
                    message: format!("Failed to create directory: {}", e),
                }
            })?;
        }
        
        // Create connection manager
        let manager = SqliteConnectionManager::file(&config.db_path);
        
        let result = r2d2::Pool::new(manager).map_err(|e| DatabaseError::Pool(e))?;
        let mut conn = result.get().map_err(|e| DatabaseError::Pool(e))?;
        initialize_schema(&mut conn)?;
        
        info!("Database initialized at {}", config.db_path.display());
        
        Ok(Self { pool: result, config })
    }
    
    /// Get a connection from the pool
    pub fn connection(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>> {
        self.pool.get().map_err(|e| DatabaseError::Pool(e))
    }
    
    /// Execute a function with a database connection
    pub fn with_connection<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&rusqlite::Connection) -> Result<T>,
    {
        let conn = self.connection()?;
        f(&conn)
    }
    
    /// Execute a function within a transaction
    pub fn with_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&rusqlite::Transaction) -> Result<T>,
    {
        let mut conn = self.connection()?;
        let tx = conn.transaction()?;
        let result = f(&tx)?;
        tx.commit()?;
        Ok(result)
    }
    
    /// Get the database configuration
    pub fn config(&self) -> &CacheConfig {
        &self.config
    }
    
    /// Get database statistics
    pub fn get_stats(&self) -> Result<DatabaseStats> {
        let conn = self.connection()?;
        
        let mut stats = DatabaseStats::default();
        
        // Get counts
        stats.class_count = conn.query_row(
            "SELECT COUNT(*) FROM classes",
            [],
            |row| row.get(0),
        )?;
        
        stats.pbo_count = conn.query_row(
            "SELECT COUNT(*) FROM pbo_files",
            [],
            |row| row.get(0),
        )?;
        
        stats.extracted_file_count = conn.query_row(
            "SELECT COUNT(*) FROM extracted_files",
            [],
            |row| row.get(0),
        )?;
        
        stats.failed_extraction_count = conn.query_row(
            "SELECT COUNT(*) FROM failed_extractions",
            [],
            |row| row.get(0),
        )?;
        
        // Get mission stats
        stats.mission_count = conn.query_row(
            "SELECT COUNT(*) FROM missions",
            [],
            |row| row.get(0),
        ).unwrap_or(0);
        
        stats.mission_component_count = conn.query_row(
            "SELECT COUNT(*) FROM mission_components",
            [],
            |row| row.get(0),
        ).unwrap_or(0);
        
        stats.mission_dependency_count = conn.query_row(
            "SELECT COUNT(*) FROM mission_dependencies",
            [],
            |row| row.get(0),
        ).unwrap_or(0);
        
        // Get schema version
        stats.schema_version = conn.query_row(
            "SELECT CAST(value AS INTEGER) FROM metadata WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )?;
        
        // Get last modified
        stats.last_modified = conn.query_row(
            "SELECT value FROM metadata WHERE key = 'last_modified'",
            [],
            |row| row.get(0),
        ).unwrap_or_else(|_| "never".to_string());
        
        // Get database file size
        if let Ok(metadata) = fs::metadata(&self.config.db_path) {
            stats.db_size = metadata.len();
        }
        
        // Calculate cache size
        stats.cache_size = self.calculate_cache_size()?;
        
        Ok(stats)
    }
    
    /// Calculate total cache size
    fn calculate_cache_size(&self) -> Result<u64> {
        fn dir_size(path: &Path) -> Result<u64> {
            let mut size = 0;
            
            if path.is_dir() {
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();
                    
                    if path.is_file() {
                        size += fs::metadata(&path)?.len();
                    } else if path.is_dir() {
                        size += dir_size(&path)?;
                    }
                }
            }
            
            Ok(size)
        }
        
        let mut total_size = 0;
        
        // Game data cache
        if self.config.game_data_cache_dir.exists() {
            total_size += dir_size(&self.config.game_data_cache_dir)?;
        }
        
        // Mission cache
        if self.config.mission_cache_dir.exists() {
            total_size += dir_size(&self.config.mission_cache_dir)?;
        }
        
        Ok(total_size)
    }
    
    /// Get a metadata value from the database
    pub fn get_metadata(&self, key: &str) -> Result<Option<String>> {
        self.with_connection(|conn| {
            conn.query_row(
                "SELECT value FROM metadata WHERE key = ?1",
                [key],
                |row| row.get(0),
            ).optional()
            .map_err(|e| e.into())
        })
    }
    
    /// Set a metadata value in the database
    pub fn set_metadata(&self, key: &str, value: &str) -> Result<()> {
        self.with_connection(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO metadata (key, value) VALUES (?1, ?2)",
                [key, value],
            )?;
            
            Ok(())
        })
    }
    
    /// Run database schema verification
    pub fn verify_schema(&self) -> Result<()> {
        let version = self.get_metadata("schema_version")?
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(0);
        
        if version != schema::SCHEMA_VERSION {
            return Err(DatabaseError::SchemaVersionMismatch {
                existing: version,
                required: schema::SCHEMA_VERSION,
            });
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_database_manager() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        // Create a new database manager
        let db = DatabaseManager::new(&db_path).unwrap();
        
        // Set and get metadata
        db.set_metadata("test_key", "test_value").unwrap();
        let value = db.get_metadata("test_key").unwrap();
        
        assert_eq!(value, Some("test_value".to_string()));
        
        // Get statistics
        let stats = db.get_stats().unwrap();
        
        assert_eq!(stats.class_count, 0);
        assert_eq!(stats.pbo_count, 0);
    }
} 