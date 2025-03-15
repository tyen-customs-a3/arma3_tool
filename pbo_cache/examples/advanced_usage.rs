use std::path::PathBuf;
use anyhow::{Result, Context};
use pbo_cache::{ExtractionConfig, ExtractionManager, PboType};
use log::{info, warn, debug, LevelFilter};
use clap::Parser;

/// Advanced PBO extraction example
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the cache directory
    #[clap(short, long, default_value = "./cache")]
    cache_dir: String,
    
    /// Game data directories (comma separated)
    #[clap(short, long)]
    game_dirs: Option<String>,
    
    /// Mission directories (comma separated)
    #[clap(short, long)]
    mission_dirs: Option<String>,
    
    /// File extensions for game data (comma separated)
    #[clap(short = 'e', long, default_value = "hpp,cpp,sqf")]
    game_extensions: String,
    
    /// File extensions for missions (comma separated)
    #[clap(short = 'x', long, default_value = "hpp,cpp,sqf,sqm")]
    mission_extensions: String,
    
    /// Number of threads to use
    #[clap(short, long)]
    threads: Option<usize>,
    
    /// Skip game data extraction
    #[clap(long)]
    skip_game_data: bool,
    
    /// Skip mission extraction
    #[clap(long)]
    skip_missions: bool,
    
    /// Clean cache before extraction
    #[clap(short, long)]
    clean: bool,
    
    /// Verbose output
    #[clap(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Initialize logging
    let log_level = if args.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    
    env_logger::Builder::new()
        .filter_level(log_level)
        .init();
    
    // Split comma-separated values
    let game_dirs = args.game_dirs
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect::<Vec<_>>();
        
    let mission_dirs = args.mission_dirs
        .unwrap_or_default()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .collect::<Vec<_>>();
        
    let game_extensions = args.game_extensions
        .split(',')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();
        
    let mission_extensions = args.mission_extensions
        .split(',')
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();
    
    // Create cache paths
    let cache_dir = PathBuf::from(&args.cache_dir);
    let game_data_cache_dir = cache_dir.join("gamedata");
    let mission_cache_dir = cache_dir.join("missions");
    
    // Display configuration
    info!("PBO Cache Configuration:");
    info!("  Cache directory: {}", cache_dir.display());
    info!("  Game data directories: {:?}", game_dirs);
    info!("  Mission directories: {:?}", mission_dirs);
    info!("  Game data extensions: {:?}", game_extensions);
    info!("  Mission extensions: {:?}", mission_extensions);
    info!("  Threads: {}", args.threads.unwrap_or_else(num_cpus::get));
    
    // Create extraction configuration
    let config = ExtractionConfig {
        cache_dir,
        game_data_cache_dir,
        mission_cache_dir,
        game_data_dirs: game_dirs,
        game_data_extensions,
        mission_dirs,
        mission_extensions,
        threads: args.threads.unwrap_or_else(num_cpus::get),
        timeout: 60,
        verbose: args.verbose,
    };
    
    // Create extraction manager
    let mut manager = ExtractionManager::new(config)?;
    
    // Clean cache if requested
    if args.clean {
        info!("Cleaning cache...");
        manager.cleanup_cache()?;
    }
    
    // Process game data
    if !args.skip_game_data {
        info!("Processing game data...");
        let start_time = std::time::Instant::now();
        let game_files = manager.process_game_data(false).await?;
        let elapsed = start_time.elapsed();
        
        info!("Game data processing complete in {:.2?}", elapsed);
        info!("Processed {} game data files", game_files.len());
    } else {
        info!("Skipping game data processing");
    }
    
    // Process missions
    if !args.skip_missions {
        info!("Processing missions...");
        let start_time = std::time::Instant::now();
        let mission_files = manager.process_all_missions(false).await?;
        let elapsed = start_time.elapsed();
        
        info!("Mission processing complete in {:.2?}", elapsed);
        info!("Processed {} missions with a total of {} files", 
            mission_files.len(),
            mission_files.values().map(|v| v.len()).sum::<usize>());
            
        // Display details about each mission
        if args.verbose {
            for (mission_path, files) in &mission_files {
                debug!("  Mission: {}", mission_path.display());
                debug!("    Files: {}", files.len());
            }
        }
    } else {
        info!("Skipping mission processing");
    }
    
    info!("All operations completed successfully");
    Ok(())
} 