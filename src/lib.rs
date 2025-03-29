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
        use crate::scanner::gamedata::GameDataScanner;
        use crate::scanner::mission::MissionScanner;
        use log::info;
        use arma3_db::DatabaseManager;
        
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
            
            // Create database file path
            let db_path = cache_dir.join("arma3.db");
            
            // Create scanners with database support
            let mut game_data_scanner = GameDataScanner::with_database(config.clone(), db_path.clone())?;
            let mut mission_scanner = MissionScanner::new(config.clone())?;
            
            // Extract and scan game data
            info!("Scanning game data...");
            let game_data = game_data_scanner.scan(None, false).await?;
            
            // Save game data to database
            info!("Saving game data to database...");
            game_data_scanner.save_to_database(&game_data)?;
            
            // Extract and scan missions
            info!("Scanning missions...");
            let mission_data = mission_scanner.scan(None).await?;
            
            // Create database manager for mission data
            info!("Saving mission data to database...");
            // TODO: Add mission data database support
            
            info!("Cache building complete");
            
            Ok(())
        }
    }
}