use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

mod error;
mod config;
mod scanner;

use config::ScanConfig;
use scanner::{
    gamedata::GameDataScanner,
    mission::MissionScanner,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the configuration file
    #[arg(short, long, default_value = "scan_config.json")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract PBOs without scanning (preparation step)
    Extract {
        /// Extract game data PBOs
        #[arg(long)]
        game_data: bool,
        
        /// Extract mission PBOs
        #[arg(long)]
        missions: bool,
        
        /// Only extract from specific directories
        #[arg(short, long)]
        dirs: Option<Vec<String>>,
    },
    
    /// Scan extracted PBO files
    Scan {
        /// Scan game data PBOs
        #[arg(long)]
        game_data: bool,
        
        /// Scan mission PBOs
        #[arg(long)]
        missions: bool,
    }
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
    
    // Process command
    match &cli.command {
        Commands::Extract { game_data, missions, dirs } => {
            if *game_data {
                info!("Extracting game data PBOs...");
                let mut scanner = GameDataScanner::new(pbo_config.clone())?;
                let extracted_paths = scanner.extract_only(dirs.clone()).await?;
                info!("Extracted {} game data PBO files", extracted_paths.len());
            }
            
            if *missions {
                info!("Extracting mission PBOs...");
                let mut scanner = MissionScanner::new(pbo_config.clone())?;
                let extracted_paths = scanner.extract_only(dirs.clone()).await?;
                info!("Extracted {} mission files", extracted_paths.len());
            }
            
            if !*game_data && !*missions {
                info!("No extraction type specified. Use --game-data or --missions flags.");
            }
        },
        
        Commands::Scan { game_data, missions } => {
            if *game_data {
                info!("Scanning game data PBOs...");
                let mut scanner = GameDataScanner::new(pbo_config.clone())?;
                let game_data = scanner.scan_only().await?;
                info!("Scanned {} game data classes", game_data.classes.len());
            }
            
            if *missions {
                info!("Scanning mission PBOs...");
                let mut scanner = MissionScanner::new(pbo_config.clone())?;
                let mission_data = scanner.scan_only(None).await?;
                info!("Scanned {} missions", mission_data.missions.len());
            }
            
            if !*game_data && !*missions {
                info!("No scan type specified. Use --game-data or --missions flags.");
            }
        }
    }
    
    Ok(())
} 