use std::path::PathBuf;
use anyhow::Result;
use arma3_extractor::{ExtractionConfig, ExtractionManager};
use log::{info, LevelFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();
    
    // Create extraction configuration
    info!("Setting up configuration...");
    let config = ExtractionConfig {
        cache_dir: PathBuf::from("./cache"),
        game_data_cache_dir: PathBuf::from("./cache/gamedata"),
        mission_cache_dir: PathBuf::from("./cache/missions"),
        game_data_dirs: vec![
            PathBuf::from("./test_pbos/gamedata"),
        ],
        game_data_extensions: vec!["hpp".to_string(), "cpp".to_string(), "sqf".to_string()],
        mission_dirs: vec![
            PathBuf::from("./test_pbos/missions"),
        ],
        mission_extensions: vec![
            "hpp".to_string(), 
            "cpp".to_string(), 
            "sqf".to_string(), 
            "sqm".to_string()
        ],
        threads: num_cpus::get(),
        timeout: 340,
        verbose: true,
        db_path: PathBuf::from("./cache/basic_usage_extractor.db"),
    };
    
    // --- First Run --- 
    info!("--- Starting First Extraction Run --- ");
    { // Scope for the first manager instance
        info!("Creating first ExtractionManager instance...");
        // Create extraction manager - this will create the DB if it doesn't exist
        let mut manager = ExtractionManager::new(config.clone())?;

        // Process game data
        info!("Processing game data (Run 1)... ");
        let game_files = manager.process_game_data(false).await?;
        info!("Processed {} game data files (Run 1)", game_files.len());

        // Process missions
        info!("Processing missions (Run 1)... ");
        let mission_files = manager.process_all_missions(false).await?;
        info!("Processed {} missions (Run 1)", mission_files.len());
        // Manager goes out of scope here
    }
    info!("--- First Extraction Run Finished --- ");

    // --- Second Run --- 
    // Simulate running the application again later
    info!("--- Starting Second Extraction Run --- ");
     { // Scope for the second manager instance
        info!("Creating second ExtractionManager instance (should load existing DB)... ");
        // Create another extraction manager with the same config
        // This time, `new` should load the database from config.db_path
        let mut manager = ExtractionManager::new(config.clone())?;

        // Process game data again
        // If PBOs haven't changed, this should be faster and skip extractions based on DB
        info!("Processing game data (Run 2)... ");
        let game_files = manager.process_game_data(false).await?;
        // The number of *returned* files might be the same, but DEBUG logs should show skipping
        info!("Processed {} game data files (Run 2)", game_files.len());

        // Process missions again
        info!("Processing missions (Run 2)... ");
        let mission_files = manager.process_all_missions(false).await?;
        info!("Processed {} missions (Run 2)", mission_files.len());
         // Manager goes out of scope here
    }
    info!("--- Second Extraction Run Finished --- ");

    info!("Example finished! Check logs for DEBUG messages about DB loading and skipped extractions.");
    Ok(())
} 