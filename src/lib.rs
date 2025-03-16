pub mod error;
pub mod scanner;
pub mod config;

use std::path::{Path, PathBuf};
use log::{debug, info, warn, error};

pub use error::{Result, ToolError};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Re-export necessary types from pbo_cache
pub use pbo_cache::{
    PboMetadata,
    PboType,
    CacheIndex,
    ExtractionConfig,
    ExtractionManager,
};

pub mod bin {
    pub mod cache_builder {
        use std::path::PathBuf;
        use crate::error::Result;
        use pbo_cache::ExtractionConfig;
        use arma3_tool_cache_storage::{StorageManager, CacheData};
        use crate::scanner::gamedata::GameDataScanner;
        use crate::scanner::mission::MissionScanner;
        use log::info;
        
        pub async fn build_cache(cache_dir: PathBuf) -> Result<()> {
            let config = ExtractionConfig {
                cache_dir: cache_dir.clone(),
                game_data_cache_dir: cache_dir.join("gamedata"),
                mission_cache_dir: cache_dir.join("missions"),
                game_data_dirs: vec![PathBuf::from("./addons")],
                game_data_extensions: vec!["hpp".into(), "cpp".into(), "sqf".into()],
                mission_dirs: vec![PathBuf::from("./missions")],
                mission_extensions: vec!["sqm".into(), "sqf".into()],
                threads: num_cpus::get(),
                timeout: 300,
                verbose: true,
            };
            
            // Create scanners
            let mut game_data_scanner = GameDataScanner::new(config.clone())?;
            let mut mission_scanner = MissionScanner::new(config.clone())?;
            
            // Extract and scan game data
            info!("Scanning game data...");
            let game_data = game_data_scanner.scan(None).await?;
            
            // Extract and scan missions
            info!("Scanning missions...");
            let mission_data = mission_scanner.scan(None).await?;
            
            // Create and save cache
            info!("Creating cache...");
            let storage = StorageManager::new(&cache_dir);
            let cache_data = CacheData::new(game_data, mission_data);
            storage.save(&cache_data).map_err(|e| {
                crate::error::ToolError::CacheError(format!("Failed to save cache: {}", e))
            })?;
            
            Ok(())
        }
    }
}