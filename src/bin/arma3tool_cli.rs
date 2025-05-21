use anyhow::{anyhow, Context, Result};
use arma3_tool::cli::{Cli, Commands};
use arma3_tool::config::ScanConfig;
use clap::Parser;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    // Load the scan configuration
    let config_path = cli
        .config_path
        .unwrap_or_else(|| PathBuf::from("scan_config.json"));
    
    // Make config mutable to allow overriding paths from CLI args
    let mut config: ScanConfig = ScanConfig::load(&config_path.to_string_lossy())
        .map_err(|e| anyhow!(e))
        .with_context(|| format!("Failed to load config from {}", config_path.display()))?;

    match cli.command {
        Commands::Extract {
            cache_dir,
            extractor_db_path,
        } => {
            // Apply CLI overrides to config before creating ExtractionConfig
            if let Some(cd) = cache_dir {
                config.cache_dir = cd;
            }
            if let Some(edb) = extractor_db_path {
                config.extractor_database_path = Some(edb);
            }

            // Create extractor config using potentially updated ScanConfig
            let extraction_config = config.to_extractor_config();

            arma3_tool::cli::run_extract(extraction_config).await?;
        }

        Commands::Process {
            cache_dir,
            analysis_db_path,
        } => {
            // Apply CLI overrides
            if let Some(cd) = cache_dir {
                config.cache_dir = cd;
            }
            if let Some(adb) = analysis_db_path {
                config.analysis_database_path = Some(adb);
            }

            // Get analysis DB path (applies default if needed)
            let final_analysis_db_path = config.get_analysis_db_path();

            // Create extractor config (needed by run_process, uses its own DB path from config)
            let extraction_config = config.to_extractor_config();

            arma3_tool::cli::run_process(extraction_config, final_analysis_db_path).await?;
        }

        Commands::Report {
            cache_dir,
            analysis_db_path,
            output_dir,
        } => {
            // Apply CLI overrides
            if let Some(cd) = cache_dir {
                config.cache_dir = cd;
            }
            if let Some(adb) = analysis_db_path {
                config.analysis_database_path = Some(adb);
            }
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());

            // Determine final analysis database file path
            let final_analysis_db_path = config.get_analysis_db_path();
            println!("Using Analysis DB: {}", final_analysis_db_path.display());
            println!("Using Report Output Dir: {}", output_dir.display());

            arma3_tool::cli::run_report(final_analysis_db_path, output_dir).await?;
        }

        Commands::All {
            cache_dir,
            extractor_db_path,
            analysis_db_path,
            output_dir,
        } => {
            // Apply CLI overrides to config
            if let Some(cd) = cache_dir {
                config.cache_dir = cd;
            }
            if let Some(edb) = extractor_db_path {
                config.extractor_database_path = Some(edb);
            }
            if let Some(adb) = analysis_db_path {
                config.analysis_database_path = Some(adb);
            }
            let output_dir = output_dir.unwrap_or_else(|| config.report_dir.clone());

            // Create extractor config using potentially updated ScanConfig
            let extraction_config = config.to_extractor_config();
            // Determine final analysis database file path
            let final_analysis_db_path = config.get_analysis_db_path();

            println!(
                "Using Extractor DB: {}",
                extraction_config.db_path.display()
            );
            println!("Using Analysis DB: {}", final_analysis_db_path.display());
            println!("Using Report Output Dir: {}", output_dir.display());

            // Run all operations in sequence
            arma3_tool::cli::run_extract(extraction_config.clone()).await?; // Pass config for extract
                                                                            // Pass the same extraction_config (contains cache paths) but the separate analysis DB path for process
            arma3_tool::cli::run_process(extraction_config, final_analysis_db_path.clone()).await?;
            arma3_tool::cli::run_report(final_analysis_db_path, output_dir).await?;
            // Pass analysis DB for report
        }
    }

    Ok(())
}
