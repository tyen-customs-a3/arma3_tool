use anyhow::{anyhow, Context, Result};
use arma3_tool::config::ScanConfig;
use arma3_tool_report_writer::{
    DependencyReportWriter, ComparisonReportWriter, FuzzySearchReportWriter,
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
use arma3_tool_cache_storage::{StorageManager, CacheData, PboCache};
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
    
    /// Compare two sets of processed data to find missing dependencies
    Compare {
        /// Cache directory containing the old (reference) data
        #[arg(long)]
        cache_dir_a: PathBuf,
        
        /// Cache directory containing the new data to compare
        #[arg(long)]
        cache_dir_b: PathBuf,
        
        /// Override output directory from config
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    
    /// Find similar class names for missing dependencies using fuzzy matching
    FuzzySearch {
        /// Cache directory containing the data to analyze
        #[arg(long)]
        cache_dir: PathBuf,
        
        /// Override output directory
        #[arg(long)]
        output_dir: Option<PathBuf>,

        /// Similarity threshold (0.0 to 1.0, default: 0.8)
        #[arg(long, default_value = "0.8")]
        threshold: f64,
    },
    
    /// Run all operations (extract, process, and report)
    All {
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
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
            
            run_report(db_path, output_dir)?;
        },
        
        Commands::Compare { cache_dir_a, cache_dir_b, output_dir } => {
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());
            run_compare(cache_dir_a, cache_dir_b, output_dir)?;
        },
        
        Commands::FuzzySearch { cache_dir, output_dir, threshold } => {
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());
            run_fuzzy_search(cache_dir, output_dir, threshold)?;
        },
        
        Commands::All { cache_dir, output_dir } => {
            // Configure for all operations
            let mut extraction_config = config.to_pbo_cache_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir;
            }
            
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());
            let db_path = get_default_cache_path(&extraction_config.cache_dir);
            
            // Run all operations in sequence
            run_extract(extraction_config.clone()).await?;
            run_process(extraction_config, db_path.clone()).await?;
            run_report(db_path, output_dir)?;
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
    // Create storage manager
    let storage = StorageManager::new(&config.cache_dir);
    
    // Process extracted files
    info!("Processing extracted files...");
    
    // Process extracted game data
    info!("Processing game data files...");
    let scanner = arma3_tool::scanner::gamedata::GameDataScanner::new(config.clone())?;
    let game_data = scanner.scan_only(false).await?;
    info!("Processed {} game data classes", game_data.classes.len());
    
    // Get PBO metadata from the extractor
    let mut extractor = ExtractionManager::new(config.clone())?;
    // Get the CacheIndex from IndexManager and then get the game data metadata
    let pbo_metadata = extractor.get_index().get_index().get_game_data_metadata();
    
    // Create PBO cache from metadata
    let mut pbo_cache = PboCache { game_data: HashMap::new() };
    for metadata in pbo_metadata {
        let path = metadata.path.clone();
        let entry = arma3_tool_cache_storage::PboCacheEntry {
            path: path.clone(),
            last_modified: chrono::DateTime::<Utc>::from(metadata.extraction_time),
            file_size: metadata.file_size,
            extraction_time: chrono::DateTime::<Utc>::from(metadata.extraction_time),
            extracted_files: metadata.extracted_files,
            used_extensions: metadata.used_extensions,
            pbo_type: match metadata.pbo_type {
                arma3_tool_pbo_cache::PboType::GameData => "GameData".to_string(),
                arma3_tool_pbo_cache::PboType::Mission => "Mission".to_string(),
            },
        };
        pbo_cache.game_data.insert(path, entry);
    }
    
    // Process extracted missions
    info!("Processing mission files...");
    let scanner = arma3_tool::scanner::mission::MissionScanner::new(config.clone())?;
    let mission_data = scanner.scan_only(None).await?;
    info!("Processed {} missions", mission_data.missions.len());
    
    // Create and save cache data
    let cache_data = CacheData::new(game_data, mission_data, pbo_cache);
    
    // Save the database
    info!("Saving cache data to {}...", config.cache_dir.display());
    storage.save(&cache_data)?;
    info!("Cache data created successfully");
    
    Ok(())
}

// Generate reports from asset database
fn run_report(db_path: PathBuf, output_dir: PathBuf) -> Result<()> {
    // Ensure db_path is a directory by stripping arma3_cache.mpack if present
    let cache_dir = if db_path.ends_with("arma3_cache.mpack") {
        db_path.parent().unwrap_or(&db_path).to_path_buf()
    } else {
        db_path
    };
    
    // Load the database
    info!("Loading cache data from {}...", cache_dir.display());
    let storage = StorageManager::new(&cache_dir);
    let cache_data = storage.load()?;
    
    // Create scanner
    let scanner = MissionDependencyBuilder::new(&cache_data.game_data);
    
    // Scan missions
    info!("Analyzing mission dependencies...");
    let scan_report = scanner.scan_missions(&cache_data.mission_data);
    
    // Write reports
    info!("Writing reports to {}...", output_dir.display());
    ensure_dir_exists(&output_dir)?;
    let writer = DependencyReportWriter::new(&scan_report);
    writer.write_report(&output_dir)?;
    
    if !scan_report.missing.is_empty() {
        warn!(
            "Found {} missing dependencies across {} missions",
            scan_report.missing.len(),
            scan_report.total_missions_scanned
        );
    } else {
        info!(
            "No missing dependencies found across {} missions",
            scan_report.total_missions_scanned
        );
    }
    
    info!("Report generation complete");
    Ok(())
}

// Compare two sets of data
fn run_compare(cache_dir_a: PathBuf, cache_dir_b: PathBuf, output_dir: PathBuf) -> Result<()> {
    info!("Comparing data between {} and {}", 
         cache_dir_a.display(), cache_dir_b.display());
    
    // Ensure cache dirs are directories by stripping arma3_cache.mpack if present
    let dir_a = if cache_dir_a.ends_with("arma3_cache.mpack") {
        cache_dir_a.parent().unwrap_or(&cache_dir_a).to_path_buf()
    } else {
        cache_dir_a
    };
    
    let dir_b = if cache_dir_b.ends_with("arma3_cache.mpack") {
        cache_dir_b.parent().unwrap_or(&cache_dir_b).to_path_buf()
    } else {
        cache_dir_b
    };
    
    // Create storage and load data for both sets
    let storage_a = StorageManager::new(&dir_a);
    let storage_b = StorageManager::new(&dir_b);
    
    // Load cache data
    let cache_data_a = storage_a.load()?;
    let cache_data_b = storage_b.load()?;
    
    info!(
        "Loaded set A with {} classes and {} missions",
        cache_data_a.game_data.classes.len(),
        cache_data_a.mission_data.missions.len()
    );
    
    info!(
        "Loaded set B with {} classes and {} missions",
        cache_data_b.game_data.classes.len(),
        cache_data_b.mission_data.missions.len()
    );
    
    // Create scanners for both datasets
    let scanner_a = MissionDependencyBuilder::new(&cache_data_a.game_data);
    let scanner_b = MissionDependencyBuilder::new(&cache_data_b.game_data);
    
    // Scan missions for both sets
    info!("Analyzing mission dependencies for set A...");
    let scan_report_a = scanner_a.scan_missions(&cache_data_a.mission_data);
    
    info!("Analyzing mission dependencies for set B...");
    let scan_report_b = scanner_b.scan_missions(&cache_data_b.mission_data);
    
    // Generate comparison report
    info!("Generating comparison report...");
    ensure_dir_exists(&output_dir)?;
    let writer = ComparisonReportWriter::new(
        &scan_report_a, 
        &scan_report_b,
        &cache_data_a.game_data
    );
    writer.write_report(&output_dir)?;
    
    info!("Comparison report complete");
    Ok(())
}

// Run fuzzy search for similar class names
fn run_fuzzy_search(cache_dir: PathBuf, output_dir: PathBuf, threshold: f64) -> Result<()> {
    info!("Running fuzzy search analysis on {}", cache_dir.display());
    
    // Ensure cache_dir is a directory by stripping arma3_cache.mpack if present
    let dir = if cache_dir.ends_with("arma3_cache.mpack") {
        cache_dir.parent().unwrap_or(&cache_dir).to_path_buf()
    } else {
        cache_dir
    };
    
    // Create storage and load data
    let storage = StorageManager::new(&dir);
    
    // Load cache data
    let cache_data = storage.load()?;
    
    info!(
        "Loaded {} classes and {} missions",
        cache_data.game_data.classes.len(),
        cache_data.mission_data.missions.len()
    );
    
    // Create scanner
    let scanner = MissionDependencyBuilder::new(&cache_data.game_data);
    
    // Scan missions
    info!("Analyzing mission dependencies...");
    let scan_report = scanner.scan_missions(&cache_data.mission_data);
    
    if scan_report.missing.is_empty() {
        info!("No missing dependencies found to analyze.");
        return Ok(());
    }

    // Create and write fuzzy search report
    info!("Performing fuzzy search analysis with threshold {:.2}...", threshold);
    let writer = FuzzySearchReportWriter::new(&scan_report, &cache_data.game_data);
    let fuzzy_report = writer.write_report(&output_dir)?;
    
    info!(
        "Analyzed {} missing classes and found similar matches",
        fuzzy_report.missing_classes.len()
    );
    
    info!("Fuzzy search report complete");
    Ok(())
}

// Run diagnostic scan on game data
async fn run_diagnostic(config: ExtractionConfig) -> Result<()> {
    // Run diagnostic scan on game data files
    info!("Running diagnostic scan on game data files...");
    let scanner = arma3_tool::scanner::gamedata::GameDataScanner::new(config)?;
    let _game_data = scanner.scan_only(true).await?;
    
    info!("Diagnostic scan complete");
    Ok(())
} 