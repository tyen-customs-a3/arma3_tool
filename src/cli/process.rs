use crate::scanner::{gamedata::GameDataScanner, mission::MissionScanner};
use anyhow::Result;
use arma3_database::models::CacheConfig;
use arma3_database::DatabaseManager;
use arma3_extractor::ExtractionConfig;
use log::info;

pub async fn run_process(config: ExtractionConfig, db_path: std::path::PathBuf) -> Result<()> {
    // Process extracted files
    info!("Processing extracted files...");

    // Create database manager
    let db_config = CacheConfig::with_cache_dirs(
        db_path.clone(),
        config.cache_dir.clone(),
        config.game_data_cache_dir.clone(),
        config.mission_cache_dir.clone(),
    );
    let db_manager = DatabaseManager::with_config(db_config)
        .map_err(|e| anyhow::anyhow!("Failed to create database manager: {}", e))?;

    // Process extracted game data
    info!(
        "Processing game data files from {}...",
        config.game_data_cache_dir.display()
    );
    let scanner = GameDataScanner::with_database(config.clone(), db_manager.clone())?;
    let game_data = scanner.scan().await?;
    info!("Processed {} game data classes", game_data.classes.len());

    // Save game data to database
    scanner.save_to_database(&game_data)?;
    info!("Saved game data to database");

    // Process extracted missions
    info!(
        "Processing mission files from {}...",
        config.mission_cache_dir.display()
    );
    let scanner = MissionScanner::with_database(config.clone(), db_manager)?;
    let mission_data = scanner.scan().await?;
    info!("Processed {} missions", mission_data.missions.len());

    Ok(())
}
