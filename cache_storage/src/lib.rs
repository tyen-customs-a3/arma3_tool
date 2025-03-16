use std::path::PathBuf;
use std::fs::{self, File};
use std::io::{self, Read};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use thiserror::Error;
use arma3_tool_models::{GameDataClasses, MissionData};

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] rmp_serde::encode::Error),
    
    #[error("Deserialization error: {0}")]
    Deserialization(#[from] rmp_serde::decode::Error),
}

pub type Result<T> = std::result::Result<T, StorageError>;

#[derive(Serialize, Deserialize)]
pub struct CacheData {
    pub timestamp: DateTime<Utc>,
    pub game_data: GameDataClasses,
    pub mission_data: MissionData,
}

impl CacheData {
    pub fn new(game_data: GameDataClasses, mission_data: MissionData) -> Self {
        Self {
            timestamp: Utc::now(),
            game_data,
            mission_data,
        }
    }
}

pub struct StorageManager {
    cache_dir: PathBuf,
}

impl StorageManager {
    pub fn new(cache_dir: impl Into<PathBuf>) -> Self {
        Self {
            cache_dir: cache_dir.into(),
        }
    }
    
    fn get_cache_path(&self) -> PathBuf {
        self.cache_dir.join("arma3_cache.mpack")
    }
    
    pub fn save(&self, data: &CacheData) -> Result<()> {
        // Ensure cache directory exists
        fs::create_dir_all(&self.cache_dir)?;
        
        let cache_path = self.get_cache_path();
        let mut buf = Vec::new();
        rmp_serde::encode::write(&mut buf, data)?;
        fs::write(cache_path, buf)?;
        Ok(())
    }
    
    pub fn load(&self) -> Result<CacheData> {
        let cache_path = self.get_cache_path();
        let mut file = File::open(cache_path)?;
        
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        
        let data = rmp_serde::from_slice(&buf)?;
        Ok(data)
    }
    
    pub fn cache_exists(&self) -> bool {
        self.get_cache_path().exists()
    }
} 