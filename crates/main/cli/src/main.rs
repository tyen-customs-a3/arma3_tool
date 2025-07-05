//! Unified CLI interface for Arma 3 tools
//! 
//! This is the main entry point that provides a unified command-line interface
//! for all Arma 3 tool functionality, routing commands to appropriate application
//! and service layer crates.

use clap::{Parser, Subcommand};
use log::{error, info};
use std::process;

mod commands;

use commands::{
    workflow::WorkflowCommands,
    scan::ScanCommands,
    analyze::AnalyzeCommands,
    extract::ExtractCommands,
    database::DatabaseCommands,
};

#[derive(Parser)]
#[command(name = "arma3-cli")]
#[command(about = "Unified CLI interface for Arma 3 tools")]
#[command(long_about = "
A comprehensive tool suite for working with Arma 3 mods, missions, and content.
Provides scanning, analysis, extraction, and workflow management capabilities.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    verbose: bool,
    
    /// Enable debug logging
    #[arg(short, long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Workflow management and automation
    #[command(subcommand)]
    Workflow(WorkflowCommands),
    
    /// Scanning operations for gamedata, missions, and weapons
    #[command(subcommand)]
    Scan(ScanCommands),
    
    /// Analysis operations for SQF scripts and dependencies
    #[command(subcommand)]
    Analyze(AnalyzeCommands),
    
    /// PBO extraction and file operations
    #[command(subcommand)]
    Extract(ExtractCommands),
    
    /// Database operations and queries
    #[command(subcommand)]
    Database(DatabaseCommands),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    // Initialize logging based on CLI flags
    let log_level = if cli.debug {
        log::LevelFilter::Debug
    } else if cli.verbose {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Warn
    };
    
    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();
    
    info!("Starting Arma 3 CLI tool");
    
    // Route to appropriate command handler
    let result = match cli.command {
        Commands::Workflow(cmd) => commands::workflow::handle_command(cmd).await,
        Commands::Scan(cmd) => commands::scan::handle_command(cmd).await,
        Commands::Analyze(cmd) => commands::analyze::handle_command(cmd).await,
        Commands::Extract(cmd) => commands::extract::handle_command(cmd).await,
        Commands::Database(cmd) => commands::database::handle_command(cmd).await,
    };
    
    // Handle command results
    match result {
        Ok(()) => {
            info!("Command completed successfully");
        }
        Err(e) => {
            error!("Command failed: {}", e);
            process::exit(1);
        }
    }
}