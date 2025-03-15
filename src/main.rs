use anyhow::Result;
use clap::{Parser, Subcommand};
use log::{debug, info, warn, error};

mod config;
mod cache;
mod error;
mod scanner;
mod database;

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
    /// Scan game data for class definitions
    ScanGameData {
        /// Only scan specific directories
        #[arg(short, long)]
        dirs: Option<Vec<String>>,
        
        /// Skip extraction phase (use already extracted PBOs)
        #[arg(long)]
        skip_extraction: bool,
    },
    /// Scan missions for dependencies
    ScanMissions {
        /// Only scan specific directories
        #[arg(short, long)]
        dirs: Option<Vec<String>>,
        
        /// Skip extraction phase (use already extracted PBOs)
        #[arg(long)]
        skip_extraction: bool,
    },
    /// Generate reports from scanned data
    GenerateReports {
        /// Only generate reports for specific missions
        #[arg(short, long)]
        missions: Option<Vec<String>>,
        
        /// Scan game data before generating reports
        #[arg(long)]
        scan_game_data: bool,
        
        /// Scan missions before generating reports
        #[arg(long)]
        scan_missions: bool,
        
        /// Skip extraction phase when scanning (use already extracted PBOs)
        #[arg(long)]
        skip_extraction: bool,
    },
    /// Run the full pipeline: scan game data, scan missions, and generate reports
    RunAll {
        /// Skip extraction phase (use already extracted PBOs)
        #[arg(long)]
        skip_extraction: bool,
    },
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
    let mut cache_manager = cache::CacheManager::new(config.cache_dir.clone());
    
    // Process command
    match &cli.command {
        Commands::Extract { game_data, missions, dirs } => {
            if *game_data {
                info!("Extracting game data PBOs...");
                let mut extractor = scanner::gamedata::GameDataScanner::new(config.clone(), cache_manager.clone());
                extractor.extract_only(dirs.clone())?;
                info!("Game data PBO extraction completed successfully");
            }
            
            if *missions {
                info!("Extracting mission PBOs...");
                let mut extractor = scanner::mission::MissionScanner::new(config.clone(), cache_manager.clone());
                extractor.extract_only(dirs.clone())?;
                info!("Mission PBO extraction completed successfully");
            }
            
            if !*game_data && !*missions {
                info!("No extraction type specified. Use --game-data or --missions flags.");
            }
        },
        Commands::ScanGameData { dirs, skip_extraction } => {
            info!("Scanning game data...");
            let mut scanner = scanner::gamedata::GameDataScanner::new(config.clone(), cache_manager);
            let game_data = if *skip_extraction {
                scanner.scan_only(dirs.clone())?
            } else {
                scanner.scan(dirs.clone())?
            };
            info!("Game data scanning completed successfully");
        },
        Commands::ScanMissions { dirs, skip_extraction } => {
            info!("Scanning missions...");
            let mut scanner = scanner::mission::MissionScanner::new(config.clone(), cache_manager);
            let mission_results = if *skip_extraction {
                scanner.scan_only(dirs.clone())?
            } else {
                scanner.scan(dirs.clone())?
            };
            info!("Mission scanning completed successfully");
        },
        Commands::GenerateReports { missions, scan_game_data, scan_missions, skip_extraction } => {
            // Scan game data if requested
            if *scan_game_data {
                info!("Scanning game data...");
                let mut game_data_scanner = scanner::gamedata::GameDataScanner::new(config.clone(), cache_manager.clone());
                if *skip_extraction {
                    game_data_scanner.scan_only(None)?;
                } else {
                    game_data_scanner.scan(None)?;
                }
                info!("Game data scanning completed successfully");
            }
            
            // Scan missions if requested
            if *scan_missions {
                info!("Scanning missions...");
                let mut mission_scanner = scanner::mission::MissionScanner::new(config.clone(), cache_manager.clone());
                if *skip_extraction {
                    mission_scanner.scan_only(None)?;
                } else {
                    mission_scanner.scan(None)?;
                }
                info!("Mission scanning completed successfully");
            }
            
            // Generate reports
            info!("Generating reports...");
            let mut report_generator = scanner::report::ReportGenerator::new(
                config.report_dir.clone(),
                cache_manager.clone()
            );
            report_generator.generate(missions.clone())?;
            info!("Report generation completed successfully");
        },
        Commands::RunAll { skip_extraction } => {
            info!("Running full pipeline...");
            
            // Scan game data
            info!("Scanning game data...");
            let mut game_data_scanner = scanner::gamedata::GameDataScanner::new(config.clone(), cache_manager.clone());
            let game_data = if *skip_extraction {
                Box::leak(Box::new(game_data_scanner.scan_only(None)?))
            } else {
                Box::leak(Box::new(game_data_scanner.scan(None)?))
            };
            info!("Game data scanning completed successfully");
            
            // Scan missions
            info!("Scanning missions...");
            let mut mission_scanner = scanner::mission::MissionScanner::new(config.clone(), cache_manager.clone());
            let mission_results = if *skip_extraction {
                mission_scanner.scan_only(None)?
            } else {
                mission_scanner.scan(None)?
            };
            info!("Mission scanning completed successfully");
            
            // Generate reports
            info!("Generating reports...");
            let mut report_generator = scanner::report::ReportGenerator::new(
                config.report_dir.clone(),
                cache_manager
            );
            report_generator.generate_from_results(game_data, &mission_results)?;
            info!("Report generation completed successfully");
            
            info!("Full pipeline completed successfully");
        },
    }
    
    Ok(())
} 