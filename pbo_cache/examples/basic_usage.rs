use std::path::PathBuf;
use anyhow::Result;
use arma3_tool_pbo_cache::{ExtractionConfig, ExtractionManager};
use log::{info, LevelFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .init();
    
    // Create extraction configuration
    let config = ExtractionConfig {
        cache_dir: PathBuf::from("./cache"),
        game_data_cache_dir: PathBuf::from("./cache/gamedata"),
        mission_cache_dir: PathBuf::from("./cache/missions"),
        game_data_dirs: vec![
            PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/Arma 3"),
        ],
        game_data_extensions: vec!["hpp".to_string(), "cpp".to_string(), "sqf".to_string()],
        mission_dirs: vec![
            PathBuf::from("./missions"),
        ],
        mission_extensions: vec![
            "hpp".to_string(), 
            "cpp".to_string(), 
            "sqf".to_string(), 
            "sqm".to_string()
        ],
        threads: num_cpus::get(),
        timeout: 60,
        verbose: true,
        db_path: Some(PathBuf::from("./cache/pbo_cache.db")),
    };
    
    // Create extraction manager
    let mut manager = ExtractionManager::new(config)?;
    
    // Process game data
    info!("Processing game data...");
    let game_files = manager.process_game_data(false).await?;
    info!("Processed {} game data files", game_files.len());
    
    // Process missions
    info!("Processing missions...");
    let mission_files = manager.process_all_missions(false).await?;
    info!("Processed {} missions", mission_files.len());
    
    // Clean up cache
    info!("Cleaning up cache...");
    manager.cleanup_cache()?;
    
    info!("All done!");
    Ok(())
} 