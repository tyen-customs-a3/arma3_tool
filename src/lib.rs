pub mod config;
pub mod error;
pub mod scanner;
pub mod types;


pub use error::{Result, ToolError};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// Re-export necessary types from pbo_cache
pub use arma3_tool_pbo_cache::{
    PboMetadata,
    PboType,
};

pub mod bin {
    pub mod cache_builder {
        
        use crate::error::Result;
        use arma3_tool_pbo_cache::{extract_game_data, ExtractionManager};
        use crate::scanner::gamedata::GameDataScanner;
        use crate::scanner::mission::MissionScanner;
        use crate::config::ScanConfig;
        use log::info;
        
        
        pub async fn build_cache(config: ScanConfig) -> Result<()> {
            let pbo_cache_config = config.to_pbo_cache_config();
            let cache_dir = config.cache_dir.clone();
            
            // Determine database path from config
            let db_path = match &config.database_path {
                Some(path) => path.clone(),
                None => cache_dir.join("arma3.db"),
            };
            
            info!("Using database at {}", db_path.display());
            info!("Game data will be extracted to: {}", pbo_cache_config.game_data_cache_dir.display());
            info!("Missions will be extracted to: {}", pbo_cache_config.mission_cache_dir.display());
            
            // First, extract all game data files
            info!("Extracting game data...");
            let game_data_paths = extract_game_data(pbo_cache_config.clone()).await
                .map_err(|e| crate::error::ToolError::ExtractionError(format!("Failed to extract game data: {}", e)))?;
            info!("Extracted {} game data files", game_data_paths.len());
            
            // Next, extract all mission files
            info!("Extracting missions...");
            let mut extraction_manager = ExtractionManager::new(pbo_cache_config.clone())
                .map_err(|e| crate::error::ToolError::ExtractionError(format!("Failed to create extraction manager: {}", e)))?;
            let mission_results = extraction_manager.process_all_missions(false).await
                .map_err(|e| crate::error::ToolError::ExtractionError(format!("Failed to extract missions: {}", e)))?;
            let total_missions = mission_results.len();
            let total_files: usize = mission_results.values().map(|files| files.len()).sum();
            info!("Extracted {} missions with {} total files", total_missions, total_files);
            
            // Create scanners with database support
            let game_data_scanner = GameDataScanner::with_database_path(pbo_cache_config.clone(), &db_path)?;
            let mission_scanner = MissionScanner::with_database_path(pbo_cache_config.clone(), &db_path)?;
            
            // Scan game data
            info!("Scanning game data from {}...", pbo_cache_config.game_data_cache_dir.display());
            let game_data = game_data_scanner.scan(false).await?;
            
            // Save game data to database
            info!("Saving game data to database at {}...", db_path.display());
            game_data_scanner.save_to_database(&game_data)?;
            
            // Scan missions
            info!("Scanning missions from {}...", pbo_cache_config.mission_cache_dir.display());
            let _mission_data = mission_scanner.scan().await?;
            
            // Mission data is saved individually during the scan process
            
            info!("Cache building complete");
            
            Ok(())
        }
    }
}