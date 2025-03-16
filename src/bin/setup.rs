use anyhow::Result;
use clap::Parser;
use log::info;
use arma3_tool::{
    scanner::{
        gamedata::GameDataScanner,
        mission::MissionScanner,
    },
    config::ScanConfig,
};

#[derive(Parser)]
#[command(author, version, about = "Setup tool for Arma 3 PBO cache")]
struct Cli {
    /// Path to the configuration file
    #[arg(short, long, default_value = "scan_config.json")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Load configuration
    let config = ScanConfig::load(&cli.config)?;
    info!("Loaded configuration from {}", cli.config);
    
    // Create PBO cache configuration
    let pbo_config = config.to_pbo_cache_config();
    
    // Extract game data PBOs
    info!("Setting up game data PBOs...");
    let mut game_data_scanner = GameDataScanner::new(pbo_config.clone())?;
    let game_data_paths = game_data_scanner.extract_only(None).await?;
    info!("Extracted {} game data files", game_data_paths.len());
    
    // Extract mission PBOs
    info!("Setting up mission PBOs...");
    let mut mission_scanner = MissionScanner::new(pbo_config)?;
    let mission_paths = mission_scanner.extract_only(None).await?;
    info!("Extracted {} mission files", mission_paths.len());
    
    info!("Setup complete!");
    info!("Game data files in: {}", config.cache_dir.join("gamedata").display());
    info!("Mission files in: {}", config.cache_dir.join("missions").display());
    
    Ok(())
} 