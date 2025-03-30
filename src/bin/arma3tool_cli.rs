use anyhow::{anyhow, Context, Result};
use arma3_tool::config::ScanConfig;
use clap::{Parser, Subcommand};
use log::info;
use arma3_tool_pbo_cache::{
    extract_game_data as pbo_extract_game_data, 
    ExtractionConfig, ExtractionManager
};
use std::path::PathBuf;
use arma3_db::models::CacheConfig;
use arma3_db::DatabaseManager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the configuration file
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    config_path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract game data and missions from PBOs
    Extract {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
    },
    
    /// Process extracted game data and missions to create asset database
    Process {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override database file path
        #[arg(long)]
        db_path: Option<PathBuf>,
    },
    
    /// Generate dependency reports from asset database
    Report {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override database file path
        #[arg(long)]
        db_path: Option<PathBuf>,
        
        /// Override output directory from config
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    
    /// Run all operations (extract, process, and report)
    All {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override database file path
        #[arg(long)]
        db_path: Option<PathBuf>,
        
        /// Override output directory from config
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    
    /// Run diagnostic scan on game data files
    Diagnostic {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
    },
}

/// Get the default cache file path
fn get_default_cache_path(cache_dir: &PathBuf) -> PathBuf {
    cache_dir.join("arma3.db")
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    // Load the scan configuration
    let config_path = cli.config_path.unwrap_or_else(|| PathBuf::from("scan_config.json"));
    let config = ScanConfig::load(&config_path.to_string_lossy())
        .map_err(|e| anyhow!(e))
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
    
    match cli.command {
        Commands::Extract { cache_dir } => {
            // Configure extraction
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir.clone();
                extraction_config.game_data_cache_dir = cache_dir.join("gamedata");
                extraction_config.mission_cache_dir = cache_dir.join("missions");
            }
            
            run_extract(extraction_config).await?;
        },
        
        Commands::Process { cache_dir, db_path } => {
            // Configure processing
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir.clone();
                extraction_config.game_data_cache_dir = cache_dir.join("gamedata");
                extraction_config.mission_cache_dir = cache_dir.join("missions");
            }
            
            let db_path = db_path.unwrap_or_else(|| {
                get_default_cache_path(&extraction_config.cache_dir)
            });
            
            run_process(extraction_config, db_path).await?;
        },
        
        Commands::Report { cache_dir, db_path, output_dir } => {
            // Configure reporting
            let cache_dir = cache_dir.unwrap_or_else(|| config.cache_dir.clone());
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());
            println!("cache_dir: {:?}", cache_dir);
            println!("output_dir: {:?}", output_dir);
            // Determine database file path
            let db_path = db_path.unwrap_or_else(|| {
                get_default_cache_path(&cache_dir)
            });

            println!("db_path: {:?}", db_path);
            
            //run_report(db_path, output_dir)?;
        },
        
        Commands::All { cache_dir, db_path, output_dir } => {
            // Configure for all operations
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir.clone();
                extraction_config.game_data_cache_dir = cache_dir.join("gamedata");
                extraction_config.mission_cache_dir = cache_dir.join("missions");
            }
            
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());
            let db_path = db_path.unwrap_or_else(|| get_default_cache_path(&extraction_config.cache_dir));
            println!("output_dir: {:?}", output_dir);
            println!("db_path: {:?}", db_path);
            
            // Run all operations in sequence
            run_extract(extraction_config.clone()).await?;
            run_process(extraction_config, db_path.clone()).await?;
        },
        
        Commands::Diagnostic { cache_dir } => {
            // Configure diagnostic scan
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir.clone();
                extraction_config.game_data_cache_dir = cache_dir.join("gamedata");
                extraction_config.mission_cache_dir = cache_dir.join("missions");
            }
            
            run_diagnostic(extraction_config).await?;
        }
    }
    
    Ok(())
}

// Extract PBO files
async fn run_extract(config: ExtractionConfig) -> Result<()> {
    // Save paths for logging later
    let game_data_cache_dir = config.game_data_cache_dir.clone();
    let mission_cache_dir = config.mission_cache_dir.clone();
    
    // Extract game data
    info!("Extracting game data...");
    let paths = pbo_extract_game_data(config.clone()).await?;
    info!("Game data extraction complete: {} paths processed", paths.len());

    // Extract missions using multithreaded extraction
    info!("Extracting missions...");
    
    // Create extractor manager
    let mut manager = ExtractionManager::new(config)?;
    
    // Process all missions in parallel
    let mission_results = manager.process_all_missions(false).await?;
    let total_missions = mission_results.len();
    let total_files: usize = mission_results.values().map(|files| files.len()).sum();
    
    info!("Mission extraction complete: {} missions processed with {} total files", 
          total_missions, total_files);
    
    info!("Extraction complete");
    info!("Game data extracted to: {}", game_data_cache_dir.display());
    info!("Missions extracted to: {}", mission_cache_dir.display());
    Ok(())
}

// Process extracted files into asset database
async fn run_process(config: ExtractionConfig, db_path: PathBuf) -> Result<()> {
    
    // Process extracted files
    info!("Processing extracted files...");
    
    // Create database manager
    let db_config = CacheConfig::with_cache_dirs(
        db_path.clone(),
        config.cache_dir.clone(),
        config.game_data_cache_dir.clone(),
        config.mission_cache_dir.clone()
    );
    let db_manager = DatabaseManager::with_config(db_config)
        .map_err(|e| anyhow::anyhow!("Failed to create database manager: {}", e))?;
    
    // Process extracted game data
    info!("Processing game data files from {}...", config.game_data_cache_dir.display());
    let scanner = arma3_tool::scanner::gamedata::GameDataScanner::with_database(config.clone(), db_manager.clone())?;
    let game_data = scanner.scan(false).await?;
    info!("Processed {} game data classes", game_data.classes.len());
    
    // Save game data to database
    scanner.save_to_database(&game_data)?;
    info!("Saved game data to database");
    
    // Process extracted missions
    info!("Processing mission files from {}...", config.mission_cache_dir.display());
    let scanner = arma3_tool::scanner::mission::MissionScanner::with_database(config.clone(), db_manager)?;
    let mission_data = scanner.scan().await?;
    info!("Processed {} missions", mission_data.missions.len());
    
    Ok(())
}

// Run diagnostic scan on game data
async fn run_diagnostic(config: ExtractionConfig) -> Result<()> {
    // Create a path for the diagnostic database
    let diagnostics_db_path = config.cache_dir.join("diagnostics.db");
    
    // Create database manager
    let db_config = CacheConfig::with_cache_dirs(
        diagnostics_db_path.clone(),
        config.cache_dir.clone(),
        config.game_data_cache_dir.clone(),
        config.mission_cache_dir.clone()
    );
    let db_manager = DatabaseManager::with_config(db_config)
        .map_err(|e| anyhow::anyhow!("Failed to create database manager: {}", e))?;
    
    // First, extract game data files
    info!("Extracting game data for diagnostic scan...");
    info!("Game data will be extracted to: {}", config.game_data_cache_dir.display());
    let paths = pbo_extract_game_data(config.clone()).await?;
    info!("Game data extraction complete: {} paths processed", paths.len());
    
    // Run diagnostic scan on game data files
    info!("Running diagnostic scan on game data files...");
    let scanner = arma3_tool::scanner::gamedata::GameDataScanner::with_database(config, db_manager)?;
    let game_data = scanner.scan(true).await?;
    
    // Don't save diagnostic data to database
    info!("Diagnostic scan found {} classes", game_data.classes.len());
    info!("Diagnostic scan complete");
    
    Ok(())
}