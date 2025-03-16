use anyhow::{anyhow, Context, Result};
use arma3_tool::config::ScanConfig;
use arma3_tool_cache_storage::StorageManager;
use arma3_tool_dependency_scanner::DependencyScanner;
use arma3_tool_report_writer::ReportWriter;
use clap::{Parser, Subcommand};
use log::{error, info, warn};
use pbo_cache::{
    extract_game_data as pbo_extract_game_data, 
    extract_mission as pbo_extract_mission, 
    ExtractionConfig, ExtractionManager
};
use std::path::PathBuf;
use std::fs;
use walkdir::WalkDir;

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
    
    /// Process extracted game data and missions to create mpack database
    Process {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
    },
    
    /// Generate dependency reports from mpack data
    Report {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override output directory from config
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    
    /// Run all operations (extract, process, and report)
    All {
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override output directory from config
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    // Load the scan configuration
    let config_path = cli.config_path.unwrap_or_else(|| PathBuf::from("scan_config.json"));
    let config = ScanConfig::load(&config_path.to_string_lossy())
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
    
    match cli.command {
        Commands::Extract { cache_dir } => {
            // Configure extraction
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir;
            }
            
            // Extract game data
            info!("Extracting game data...");
            let paths = pbo_extract_game_data(extraction_config.clone()).await?;
            info!("Game data extraction complete: {} paths processed", paths.len());

            // Extract missions
            info!("Extracting missions...");
            let mut total_missions = 0;

            // We need to find and extract mission PBOs from each mission directory
            for mission_dir_str in &config.mission_dirs {
                let mission_dir = PathBuf::from(mission_dir_str);
                info!("Scanning mission directory: {}", mission_dir_str);
                
                // Walk directory to find PBO files
                let entries = WalkDir::new(&mission_dir)
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path().extension().map_or(false, |ext| ext == "pbo")
                    })
                    .collect::<Vec<_>>();
                
                info!("Found {} mission PBO files in {}", entries.len(), mission_dir_str);
                
                // Extract each mission PBO
                for entry in entries {
                    let pbo_path = entry.path();
                    info!("Extracting mission: {}", pbo_path.display());
                    let extracted_files = pbo_extract_mission(extraction_config.clone(), pbo_path).await?;
                    total_missions += 1;
                    info!("Extracted {} files from {}", extracted_files.len(), pbo_path.display());
                }
                
                info!("Mission extraction complete: {} missions processed", total_missions);
            }
            
            info!("Extraction complete");
        },
        
        Commands::Process { cache_dir } => {
            // Configure processing
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir;
            }
            
            // Create storage
            let storage = StorageManager::new(&extraction_config.cache_dir);
            
            // Process extracted files
            info!("Processing extracted files...");
            
            // Process extracted game data if requested
            info!("Processing game data files...");
            // Use the appropriate scanner to process game data
            let scanner = arma3_tool::scanner::gamedata::GameDataScanner::new(extraction_config.clone())?;
            let mut game_data = Some(scanner.scan_only().await?);
            info!("Processed {} game data classes", game_data.as_ref().unwrap().classes.len());
            
            // Process extracted missions if requested
            info!("Processing mission files...");
            let scanner = arma3_tool::scanner::mission::MissionScanner::new(extraction_config.clone())?;
            let mut mission_data = Some(scanner.scan_only(None).await?);
            info!("Processed {} missions", mission_data.as_ref().unwrap().missions.len());
            
            // Create and save the database if either was processed
            if game_data.is_some() || mission_data.is_some() {
                info!("Creating mpack database...");
                let cache_data = arma3_tool_cache_storage::CacheData::new(
                    game_data.unwrap_or_else(|| arma3_tool_models::GameDataClasses::new()),
                    mission_data.unwrap_or_else(|| arma3_tool_models::MissionData::new())
                );
                storage.save(&cache_data)?;
                info!("mpack database created successfully");
            } else {
                info!("No data processed. Please specify what to process.");
            }
        },
        
        Commands::Report { cache_dir, output_dir } => {
            // Configure reporting
            let cache_dir = cache_dir.unwrap_or_else(|| config.cache_dir.clone());
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());
            
            // Create storage and load data
            let storage = StorageManager::new(&cache_dir);
            
            // Load cache data
            let cache_data = storage.load()?;
            
            // Create scanner
            let scanner = DependencyScanner::new(&cache_data.game_data);
            
            // Scan missions
            info!("Analyzing mission dependencies...");
            let scan_report = scanner.scan_missions(&cache_data.mission_data);
            
            // Write reports
            info!("Writing reports to {}...", output_dir.display());
            let writer = ReportWriter::new(output_dir.clone(), ());
            writer.write_report(&scan_report)?;
            
            if !scan_report.missing_dependencies.is_empty() {
                warn!(
                    "Found {} missing dependencies across {} missions",
                    scan_report.missing_dependencies.len(),
                    scan_report.total_missions_scanned
                );
            } else {
                info!(
                    "No missing dependencies found across {} missions",
                    scan_report.total_missions_scanned
                );
            }
            
            info!("Reports complete");
        },
        
        Commands::All { cache_dir, output_dir } => {
            // Configure for all operations
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir;
            }
            
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());
            
            // STEP 1: Extract
            // Extract game data
            info!("Extracting game data...");
            let paths = pbo_extract_game_data(extraction_config.clone()).await?;
            info!("Game data extraction complete: {} paths processed", paths.len());
            
            // Extract missions
            info!("Extracting missions...");
            let mut total_missions = 0;
            
            // We need to find and extract mission PBOs from each mission directory
            for mission_dir_str in &config.mission_dirs {
                let mission_dir = PathBuf::from(mission_dir_str);
                info!("Scanning mission directory: {}", mission_dir_str);
                
                // Walk directory to find PBO files
                let entries = WalkDir::new(&mission_dir)
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path().extension().map_or(false, |ext| ext == "pbo")
                    })
                    .collect::<Vec<_>>();
                
                info!("Found {} mission PBO files in {}", entries.len(), mission_dir_str);
                
                // Extract each mission PBO
                for entry in entries {
                    let pbo_path = entry.path();
                    info!("Extracting mission: {}", pbo_path.display());
                    let extracted_files = pbo_extract_mission(extraction_config.clone(), pbo_path).await?;
                    total_missions += 1;
                    info!("Extracted {} files from {}", extracted_files.len(), pbo_path.display());
                }
                
                info!("Mission extraction complete: {} missions processed", total_missions);
            }
            
            // STEP 2: Process
            // Create storage
            let storage = StorageManager::new(&extraction_config.cache_dir);
            
            // Process extracted game data if requested
            info!("Processing game data files...");
            let scanner = arma3_tool::scanner::gamedata::GameDataScanner::new(extraction_config.clone())?;
            let mut game_data = Some(scanner.scan_only().await?);
            info!("Processed {} game data classes", game_data.as_ref().unwrap().classes.len());
            
            // Process extracted missions if requested
            info!("Processing mission files...");
            let scanner = arma3_tool::scanner::mission::MissionScanner::new(extraction_config.clone())?;
            let mut mission_data = Some(scanner.scan_only(None).await?);
            info!("Processed {} missions", mission_data.as_ref().unwrap().missions.len());
            
            // Create and save the database if either was processed
            if game_data.is_some() || mission_data.is_some() {
                info!("Creating mpack database...");
                let cache_data = arma3_tool_cache_storage::CacheData::new(
                    game_data.unwrap_or_else(|| arma3_tool_models::GameDataClasses::new()),
                    mission_data.unwrap_or_else(|| arma3_tool_models::MissionData::new())
                );
                storage.save(&cache_data)?;
                info!("mpack database created successfully");
            }
            
            // STEP 3: Report
            // Load cache data (we just created it, but reload to be sure)
            let cache_data = storage.load()?;
            
            // Create scanner
            let scanner = DependencyScanner::new(&cache_data.game_data);
            
            // Scan missions
            info!("Analyzing mission dependencies...");
            let scan_report = scanner.scan_missions(&cache_data.mission_data);
            
            // Write reports
            info!("Writing reports to {}...", output_dir.display());
            let writer = ReportWriter::new(output_dir.clone(), ());
            writer.write_report(&scan_report)?;
            
            if !scan_report.missing_dependencies.is_empty() {
                warn!(
                    "Found {} missing dependencies across {} missions",
                    scan_report.missing_dependencies.len(),
                    scan_report.total_missions_scanned
                );
            } else {
                info!(
                    "No missing dependencies found across {} missions",
                    scan_report.total_missions_scanned
                );
            }
            
            info!("All operations complete");
        }
    }
    
    Ok(())
} 