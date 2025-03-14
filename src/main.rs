use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{debug, info, warn, error};

mod config;
mod cache;
mod error;
mod scanner;

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
    /// Scan game data for class definitions
    ScanGameData {
        /// Only scan specific directories
        #[arg(short, long)]
        dirs: Option<Vec<String>>,
    },
    /// Scan missions for dependencies
    ScanMissions {
        /// Only scan specific directories
        #[arg(short, long)]
        dirs: Option<Vec<String>>,
    },
    /// Generate reports from scanned data
    GenerateReports {
        /// Only generate reports for specific missions
        #[arg(short, long)]
        missions: Option<Vec<String>>,
    },
    /// Run the full pipeline: scan game data, scan missions, and generate reports
    RunAll,
}

fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Load configuration
    let config_path = std::path::PathBuf::from(&cli.config);
    let config = config::ToolConfig::from_file(&config_path)?;
    
    info!("Arma 3 Tool started with configuration from {}", cli.config);
    debug!("Configuration: {:?}", config);
    
    // Initialize cache manager
    let cache_manager = cache::CacheManager::new(config.cache_dir.clone());
    
    // Process command
    match &cli.command {
        Commands::ScanGameData { dirs } => {
            info!("Scanning game data...");
            let scanner = scanner::gamedata::GameDataScanner::new(config.clone(), cache_manager);
            let game_data = scanner.scan(dirs.clone())?;
            info!("Game data scanning completed successfully");
        },
        Commands::ScanMissions { dirs } => {
            info!("Scanning missions...");
            let scanner = scanner::mission::MissionScanner::new(config.clone(), cache_manager);
            let mission_results = scanner.scan(dirs.clone())?;
            info!("Mission scanning completed successfully");
        },
        Commands::GenerateReports { missions } => {
            info!("Generating reports...");
            let report_generator = scanner::report::ReportGenerator::new(config.report_dir.clone());
            report_generator.generate(missions.clone())?;
            info!("Report generation completed successfully");
        },
        Commands::RunAll => {
            info!("Running full pipeline...");
            
            // Scan game data
            info!("Scanning game data...");
            let game_data_scanner = scanner::gamedata::GameDataScanner::new(config.clone(), cache_manager.clone());
            let game_data = game_data_scanner.scan(None)?;
            info!("Game data scanning completed successfully");
            
            // Scan missions
            info!("Scanning missions...");
            let mission_scanner = scanner::mission::MissionScanner::new(config.clone(), cache_manager);
            let mission_results = mission_scanner.scan(None)?;
            info!("Mission scanning completed successfully");
            
            // Generate reports
            info!("Generating reports...");
            let report_generator = scanner::report::ReportGenerator::new(config.report_dir.clone());
            report_generator.generate_from_results(&game_data, &mission_results)?;
            info!("Report generation completed successfully");
            
            info!("Full pipeline completed successfully");
        },
    }
    
    Ok(())
} 