use anyhow::{anyhow, Context, Result};
use arma3_tool::config::ScanConfig;
use arma3_tool::cli::{Cli, Commands, get_default_cache_path};
use clap::Parser;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    // Load the scan configuration
    let config_path = cli.config_path.unwrap_or_else(|| PathBuf::from("scan_config.json"));
    let config: ScanConfig = ScanConfig::load(&config_path.to_string_lossy())
        .map_err(|e| anyhow!(e))
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;
    
    match cli.command {
        Commands::Extract { cache_dir } => {
            // Configure extraction
            let mut extraction_config = config.to_extractor_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir.clone();
                extraction_config.game_data_cache_dir = cache_dir.join("gamedata");
                extraction_config.mission_cache_dir = cache_dir.join("missions");
            }
            
            arma3_tool::cli::run_extract(extraction_config).await?;
        },
        
        Commands::Process { cache_dir, db_path } => {
            // Configure processing
            let mut extraction_config = config.to_extractor_config();
            if let Some(cache_dir) = cache_dir {
                extraction_config.cache_dir = cache_dir.clone();
                extraction_config.game_data_cache_dir = cache_dir.join("gamedata");
                extraction_config.mission_cache_dir = cache_dir.join("missions");
            }
            
            let db_path = db_path.unwrap_or_else(|| {
                get_default_cache_path(&extraction_config.cache_dir)
            });
            
            arma3_tool::cli::run_process(extraction_config, db_path).await?;
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
            
            arma3_tool::cli::run_report(db_path, output_dir).await?;
        },
        
        Commands::All { cache_dir, db_path, output_dir } => {
            // Configure for all operations
            let mut extraction_config = config.to_extractor_config();
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
            arma3_tool::cli::run_extract(extraction_config.clone()).await?;
            arma3_tool::cli::run_process(extraction_config, db_path.clone()).await?;
            arma3_tool::cli::run_report(db_path, output_dir).await?;
        },
    }
    
    Ok(())
} 