mod commands;
mod scanning;

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
///   analyze-mission-dependencies   - Run a complete analysis pipeline: extract PBOs, scan missions, and verify class dependencies
///   full-analysis                  - Run a complete analysis pipeline for Arma 3 base game, mods, and missions
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Set the logging level
    #[arg(short, long, value_enum, default_value = "info")]
    log_level: LogLevel,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Configure logging based on the specified level
    Builder::new()
        .filter_level(cli.log_level.into())
        .format_timestamp_millis()
        .init();
    
    info!("Starting Arma3 Tool");
    
    if let Err(e) = process_command(cli.command).await {
        error!("Error: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}

async fn process_command(command: Commands) -> Result<()> {
    match command {
        Commands::ScanPbos(args) => {
            scanning::scan_pbos(args).await?;
        }
        Commands::ScanClasses(args) => {
            scanning::scan_classes(args).await?;
        }
        Commands::ScanMissions(args) => {
            scanning::scan_missions(args).await?;
        }
        Commands::AnalyzeMissionDependencies(args) => {
            scanning::analyze_mission_dependencies(args).await?;
        }
        Commands::FullAnalysis(args) => {
            scanning::full_analysis(args).await?;
        }
    }
    Ok(())
}
