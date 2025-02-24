mod commands;
mod scanning;

use clap::Parser;
use env_logger;
use log::{error, info};
use commands::Commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    info!("Starting Arma 3 Tool");
    
    if let Err(e) = process_command(cli.command).await {
        error!("Error: {}", e);
        std::process::exit(1);
    }
    
    Ok(())
}

async fn process_command(command: Commands) -> anyhow::Result<()> {
    match command {
        Commands::ScanPbos(args) => {
            scanning::scan_pbos(args).await?;
        }
    }
    Ok(())
}
