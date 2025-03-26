pub mod config;
pub mod error;
pub mod scanner;
pub mod types;
pub mod ui;
pub mod workflow;

use std::path::{Path, PathBuf};
use log::{debug, info, warn, error};
use std::collections::HashMap;
use arma3_tool_shared_models::{GameDataClasses, MissionData};
use arma3_tool_cache_storage::{StorageManager, CacheData, PboCache};
use arma3_tool_pbo_cache::{ExtractionConfig, ExtractionManager};

pub use error::{Result, ToolError};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Re-export necessary types from pbo_cache
pub use arma3_tool_pbo_cache::{
    PboMetadata,
    PboType,
    CacheIndex,
};

pub mod bin {
    pub mod cache_builder {
        use std::collections::HashMap;
        use std::path::PathBuf;
        use crate::error::Result;
        use arma3_tool_shared_models::{GameDataClasses, MissionData};
        use arma3_tool_pbo_cache::ExtractionConfig;
        use arma3_tool_cache_storage::{CacheData, PboCache, StorageManager};
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
            let game_data = game_data_scanner.scan(None, false).await?;
            
            // Extract and scan missions
            info!("Scanning missions...");
            let mission_data = mission_scanner.scan(None).await?;
            
            // Create and save cache
            info!("Creating cache...");
            let storage = StorageManager::new(&cache_dir);
            let game_data = GameDataClasses::new();
            let mission_data = MissionData::new();
            let pbo_cache = PboCache { game_data: HashMap::new() };
            let cache_data = CacheData::new(game_data, mission_data, pbo_cache);
            storage.save(&cache_data).map_err(|e| {
                crate::error::ToolError::CacheError(format!("Failed to save cache: {}", e))
            })?;
            
            Ok(())
        }
    }
}