use std::path::PathBuf;
use std::fs::{self, File};
use std::io::{self, Read};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use thiserror::Error;
use arma3_tool_shared_models::{GameDataClasses, MissionData};
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PboCacheEntry {
    pub path: PathBuf,
    pub last_modified: DateTime<Utc>,
    pub file_size: u64,
    pub extraction_time: DateTime<Utc>,
    pub extracted_files: Vec<PathBuf>,
    pub used_extensions: Vec<String>,
    pub pbo_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PboCache {
    pub game_data: HashMap<PathBuf, PboCacheEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct CacheData {
    pub timestamp: DateTime<Utc>,
    pub game_data: GameDataClasses,
    pub mission_data: MissionData,
    pub pbo_cache: PboCache,
}

impl CacheData {
    pub fn new(game_data: GameDataClasses, mission_data: MissionData, pbo_cache: PboCache) -> Self {
        Self {
            timestamp: Utc::now(),
            game_data,
            mission_data,
            pbo_cache,
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