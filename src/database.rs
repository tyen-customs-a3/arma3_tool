use std::path::{Path, PathBuf};
use std::fs;
use log::{debug, info, warn};
use serde::{Serialize, Deserialize};
use crate::error::{Result, ToolError};
use crate::scanner::models::{GameDataClasses, MissionData};

/// Database for storing scan results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanDatabase {
    /// Version of the database
    version: String,
    
    /// Game data classes
    game_data: Option<GameDataClasses>,
    
    /// Mission data
    mission_data: Option<MissionData>,
}

impl Default for ScanDatabase {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            game_data: None,
            mission_data: None,
        }
    }
}

/// Manager for the scan results database
#[derive(Clone)]
pub struct DatabaseManager {
    /// Path to the database file
    database_path: PathBuf,
    
    /// In-memory database
    database: ScanDatabase,
    
    /// Flag to track if database has been modified
    modified: bool,
}

impl DatabaseManager {
    /// Create a new database manager
    pub fn new(cache_dir: PathBuf) -> Self {
        let database_path = cache_dir.join("scan_database.json");
        
        // Load or create database
        let database = if database_path.exists() {
            match fs::read_to_string(&database_path) {
                Ok(content) => match serde_json::from_str::<ScanDatabase>(&content) {
                    Ok(db) => {
                        debug!("Loaded scan database");
                        db
                    },
                    Err(e) => {
                        warn!("Failed to parse scan database, creating new one: {}", e);
                        ScanDatabase::default()
                    }
                },
                Err(e) => {
                    warn!("Failed to read scan database, creating new one: {}", e);
                    ScanDatabase::default()
                }
            }
        } else {
            debug!("Creating new scan database");
            ScanDatabase::default()
        };
        
        Self {
            database_path,
            database,
            modified: false,
        }
    }
    
    /// Save game data to database
    pub fn save_game_data(&mut self, game_data: GameDataClasses) -> Result<()> {
        self.database.game_data = Some(game_data);
        self.modified = true;
        self.save()
    }
    
    /// Save mission data to database
    pub fn save_mission_data(&mut self, mission_data: MissionData) -> Result<()> {
        self.database.mission_data = Some(mission_data);
        self.modified = true;
        self.save()
    }
    
    /// Load game data from database
    pub fn load_game_data(&self) -> Result<GameDataClasses> {
        self.database.game_data.clone()
            .ok_or_else(|| ToolError::CacheError("Game data not found in database".to_string()))
    }
    
    /// Load mission data from database
    pub fn load_mission_data(&self) -> Result<MissionData> {
        self.database.mission_data.clone()
            .ok_or_else(|| ToolError::CacheError("Mission data not found in database".to_string()))
    }
    
    /// Save database to disk if modified
    pub fn save(&mut self) -> Result<()> {
        if !self.modified {
            return Ok(());
        }

        let json = serde_json::to_string_pretty(&self.database)
            .map_err(|e| ToolError::CacheError(format!("Failed to serialize database: {}", e)))?;
            
        // Write to a temporary file first
        let temp_path = self.database_path.with_extension("tmp");
        fs::write(&temp_path, &json)
            .map_err(|e| ToolError::CacheError(format!("Failed to write database: {}", e)))?;
            
        // Atomically rename the temporary file to the actual database file
        fs::rename(&temp_path, &self.database_path)
            .map_err(|e| ToolError::CacheError(format!("Failed to rename temporary database file: {}", e)))?;
            
        debug!("Saved scan database");
        self.modified = false;
        
        Ok(())
    }
} 