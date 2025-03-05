use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use log::info;

use extraction::{extract_pbos, ExtractionConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory containing PBO files
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for extracted files
    #[arg(short, long)]
    output: PathBuf,

    /// File extensions to extract (comma-separated)
    #[arg(short, long, default_value = "")]
    extensions: String,

    /// Number of parallel threads to use
    #[arg(short, long, default_value_t = 4)]
    threads: usize,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Initialize logging with specified log level
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&args.log_level))
        .init();
    
    // Execute the appropriate command
    info!("Starting PBO extraction...");
    info!("Input directory: {}", args.input.display());
    info!("Output directory: {}", args.output.display());
    info!("Extensions filter: {}", args.extensions);
    info!("Using {} threads", args.threads);

    let config = ExtractionConfig {
        input_dir: &args.input,
        cache_dir: &args.output,
        extensions: &args.extensions,
        threads: args.threads,
        timeout: 30, // Default timeout of 30 seconds
    };

    extract_pbos(config).await?;
    info!("Extraction complete!");

    Ok(())
}
