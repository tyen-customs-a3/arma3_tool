use anyhow::{anyhow, Context, Result};
use arma3_tool::config::ScanConfig;
use arma3_tool_report_writer::{
    DependencyReportWriter, ComparisonReportWriter, FuzzySearchReportWriter, GraphReportWriter,
    ensure_dir_exists, MissionDependencyBuilder, ScanReport
};
use arma3_tool_shared_models::{GameDataClasses, MissionData};
use clap::{Parser, Subcommand};
use log::{error, info, warn};
use arma3_tool_pbo_cache::{
    extract_game_data as pbo_extract_game_data, 
    extract_mission as pbo_extract_mission, 
    ExtractionConfig, ExtractionManager
};
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use chrono::Utc;

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
    cache_dir.clone()
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
                extraction_config.cache_dir = cache_dir;
            }
            
            run_extract(extraction_config).await?;
        },
        
        Commands::Process { cache_dir, db_path } => {
            // Configure processing
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir;
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
            
            // Determine database file path
            let db_path = db_path.unwrap_or_else(|| {
                get_default_cache_path(&cache_dir)
            });
            
            //run_report(db_path, output_dir)?;
        },
        
        Commands::All { cache_dir, db_path, output_dir } => {
            // Configure for all operations
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir;
            }
            
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());
            let db_path = db_path.unwrap_or_else(|| get_default_cache_path(&extraction_config.cache_dir).join("arma3.db"));
            
            // Run all operations in sequence
            run_extract(extraction_config.clone()).await?;
            run_process(extraction_config, db_path.clone()).await?;
        },
        
        Commands::Diagnostic { cache_dir } => {
            // Configure diagnostic scan
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir;
            }
            
            run_diagnostic(extraction_config).await?;
        }
    }
    
    Ok(())
}

// Extract PBO files
async fn run_extract(config: ExtractionConfig) -> Result<()> {
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
    Ok(())
}

// Process extracted files into asset database
async fn run_process(config: ExtractionConfig, db_path: PathBuf) -> Result<()> {
    
    // Process extracted files
    info!("Processing extracted files...");
    
    // Process extracted game data
    info!("Processing game data files...");
    let scanner = arma3_tool::scanner::gamedata::GameDataScanner::with_database(config.clone(), db_path.clone())?;
    let game_data = scanner.scan_only(false).await?;
    info!("Processed {} game data classes", game_data.classes.len());
    
    // Save game data to database
    scanner.save_to_database(&game_data)?;
    info!("Saved game data to database");
    
    // Get PBO metadata from the extractor
    let mut extractor = ExtractionManager::new(config.clone())?;
    // Get the CacheIndex from IndexManager and then get the game data metadata
    let pbo_metadata = extractor.get_index().get_index().get_game_data_metadata();
    
    // Process extracted missions
    info!("Processing mission files...");
    let scanner = arma3_tool::scanner::mission::MissionScanner::new(config.clone())?;
    let mission_data = scanner.scan_only(None).await?;
    info!("Processed {} missions", mission_data.missions.len());
    
    // TODO: Save missions to database
    
    Ok(())
}

// Run diagnostic scan on game data
async fn run_diagnostic(config: ExtractionConfig) -> Result<()> {
    // Create a path for the diagnostic database
    let diagnostics_db_path = config.cache_dir.join("diagnostics.db");
    
    // Run diagnostic scan on game data files
    info!("Running diagnostic scan on game data files...");
    let scanner = arma3_tool::scanner::gamedata::GameDataScanner::with_database(config, diagnostics_db_path)?;
    let game_data = scanner.scan_only(true).await?;
    
    // Don't save diagnostic data to database
    info!("Diagnostic scan found {} classes", game_data.classes.len());
    info!("Diagnostic scan complete");
    
    Ok(())
}