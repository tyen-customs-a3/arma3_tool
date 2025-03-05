use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use log::{info, warn};

use arma3_tool::class_commands::handle_scan_classes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input directory containing PBO files
    #[arg(short, long)]
    input_dir: PathBuf,

    /// Output directory for scan results and reports
    #[arg(short, long, default_value = "./class_reports")]
    output_dir: PathBuf,

    /// Cache directory for extracted files
    #[arg(short, long, default_value = "./cache")]
    cache_dir: PathBuf,

    /// File extensions to scan (comma-separated)
    #[arg(short, long, default_value = "cpp,hpp")]
    extensions: String,

    /// Maximum number of files to scan (for testing)
    #[arg(long)]
    max_files: Option<usize>,

    /// Timeout in seconds for PBO extraction operations
    #[arg(short = 'T', long, default_value = "30")]
    timeout: u32,

    /// Number of threads to use for processing
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

    // Execute the class scanning command
    info!("Starting class scanning...");
    info!("Input directory: {}", args.input_dir.display());
    info!("Output directory: {}", args.output_dir.display());
    info!("Cache directory: {}", args.cache_dir.display());
    info!("Extensions: {}", args.extensions);
    if let Some(max) = args.max_files {
        info!("Max files: {}", max);
    }

    // Verify input directory exists
    if !args.input_dir.exists() {
        return Err(anyhow::anyhow!("Input directory does not exist: {}", args.input_dir.display()));
    }

    // Convert to ClassScanArgs
    let scan_args = arma3_tool::commands::ClassScanArgs {
        input_dir: args.input_dir,
        output_dir: args.output_dir,
        cache_dir: args.cache_dir,
        extensions: args.extensions,
        max_files: args.max_files,
        timeout: args.timeout,
        threads: args.threads,
        log_level: args.log_level,
    };

    match handle_scan_classes(scan_args).await {
        Ok(_) => {
            info!("Class scanning completed successfully!");
            Ok(())
        },
        Err(e) => {
            warn!("Class scanning failed: {}", e);
            Err(e)
        }
    }
} 