use anyhow::Result;
use arma3_extractor::{
    extract_game_data as pbo_extract_game_data, ExtractionConfig, ExtractionManager,
};
use log::info;

pub async fn run_extract(config: ExtractionConfig) -> Result<()> {
    // Save paths for logging later
    let game_data_cache_dir = config.game_data_cache_dir.clone();
    let mission_cache_dir = config.mission_cache_dir.clone();

    // Extract game data
    info!("Extracting game data...");
    let paths = pbo_extract_game_data(config.clone()).await?;
    info!(
        "Game data extraction complete: {} paths processed",
        paths.len()
    );

    // Extract missions using multithreaded extraction
    info!("Extracting missions...");

    // Create extractor manager
    let mut manager = ExtractionManager::new(config)?;

    // Process all missions in parallel
    let mission_results = manager.process_all_missions(false).await?;
    let total_missions = mission_results.len();
    let total_files: usize = mission_results.values().map(|files| files.len()).sum();

    info!(
        "Mission extraction complete: {} missions processed with {} total files",
        total_missions, total_files
    );

    info!("Extraction complete");
    info!("Game data extracted to: {}", game_data_cache_dir.display());
    info!("Missions extracted to: {}", mission_cache_dir.display());
    Ok(())
}
