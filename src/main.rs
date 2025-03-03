mod commands;
mod scanning;
mod reporting;
mod manager;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use env_logger::Builder;
use log::{error, info, LevelFilter};
use commands::Commands;

#[derive(Copy, Clone, Debug, ValueEnum)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

/// Arma 3 analysis tool for scanning and analyzing game files, mods, and missions
/// 
/// USAGE:
///   arma3_tool [OPTIONS] <COMMAND>
///
/// Note: The --log-level parameter must be specified BEFORE the command:
///   CORRECT:   arma3_tool --log-level error full-analysis ...
///   INCORRECT: arma3_tool full-analysis --log-level error ...
///
/// Available commands:
///   scan-pbos                      - Scan and extract files from PBO archives
///   scan-classes                   - Scan and analyze class definitions in cpp files
///   scan-missions                  - Scan mission files, extract PBOs, and analyze equipment dependencies
///   mission-dependency-analysis    - Analyze mission dependencies against class definitions
///   full-analysis                  - Run a complete analysis pipeline for Arma 3 base game, mods, and missions
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set the logging level
    #[arg(short, long, value_enum, default_value = "info")]
    log_level: LogLevel,

    /// Reports to exclude globally (comma-separated)
    #[arg(long)]
    exclude_reports: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logger
    let mut builder = Builder::new();
    builder.filter_level(cli.log_level.into());
    builder.init();
    
    info!("Starting Arma 3 Tool");
    
    // Process the command
    if let Err(e) = process_command(cli.command, cli.exclude_reports).await {
        error!("Error: {}", e);
        return Err(e);
    }
    
    info!("Arma 3 Tool completed successfully");
    Ok(())
}

async fn process_command(command: Commands, global_exclude_reports: Option<String>) -> Result<()> {
    // Helper function to merge global and command-specific exclude reports
    let merge_exclude_reports = |cmd_exclude: Option<String>| -> Option<String> {
        match (global_exclude_reports.as_ref(), cmd_exclude.as_ref()) {
            (Some(global), Some(cmd)) => Some(format!("{},{}", global, cmd)),
            (Some(global), None) => Some(global.clone()),
            (None, Some(cmd)) => Some(cmd.clone()),
            (None, None) => None,
        }
    };

    match command {
        Commands::ScanPbos(mut args) => {
            // Merge global and command-specific exclude reports
            args.exclude_reports = merge_exclude_reports(args.exclude_reports);
            
            // Create a processing manager with default output and cache directories
            let mut manager = manager::ProcessingManager::new(
                &args.cache_dir.parent().unwrap_or(&std::path::PathBuf::from(".")),
                &args.cache_dir,
            );
            manager.process_pbo_files(&args).await?;
        },
        Commands::ScanClasses(mut args) => {
            // Merge global and command-specific exclude reports
            args.exclude_reports = merge_exclude_reports(args.exclude_reports);
            
            // Create a processing manager with the specified output directory
            let mut manager = manager::ProcessingManager::new(
                &args.output_dir,
                &std::path::PathBuf::from("./cache"),
            );
            // Process class files with default reports and exclusions
            manager.process_class_files(&args).await?;
        },
        Commands::ScanMissions(mut args) => {
            // Merge global and command-specific exclude reports
            args.exclude_reports = merge_exclude_reports(args.exclude_reports);
            
            // Create a processing manager with the specified output directory
            let mut manager = manager::ProcessingManager::new(
                &args.output_dir,
                &args.cache_dir,
            );
            // Process mission files with default reports and exclusions
            manager.process_mission_files(&args).await?;
        },
        Commands::MissionDependencyAnalysis(mut args) => {
            // Merge global and command-specific exclude reports
            args.exclude_reports = merge_exclude_reports(args.exclude_reports);
            
            // Create a processing manager with the specified output directory
            let mut manager = manager::ProcessingManager::new(
                &args.output_dir,
                &args.cache_dir,
            );
            
            // Process mission dependency analysis with the updated args
            manager.process_mission_dependency_analysis(&args).await?;
        },
        Commands::FullAnalysis(mut args) => {
            // Merge global and command-specific exclude reports
            args.exclude_reports = merge_exclude_reports(args.exclude_reports);
            
            // Create a processing manager with the specified output directory
            let mut manager = manager::ProcessingManager::new(
                &args.output_dir,
                &args.cache_dir,
            );
            
            // Run the full analysis pipeline
            manager.run_full_analysis(&args).await?;
        },
    }
    
    Ok(())
}
