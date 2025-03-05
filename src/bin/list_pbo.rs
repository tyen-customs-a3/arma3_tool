use std::path::PathBuf;
use clap::Parser;
use pbo_tools::core::api::{PboApi, PboApiOps};
use pbo_tools::extract::ExtractOptions;
use log::{self, debug, info, trace};
use env_logger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input PBO file to list
    #[arg(short, long)]
    input: PathBuf,
    
    /// Timeout in seconds for listing
    #[arg(short, long, default_value = "30")]
    timeout: u32,
    
    /// Number of threads to use
    #[arg(short, long, default_value_t = 4)]
    threads: usize,
    
    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();
    
    // Initialize logging with specified log level
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&args.log_level))
        .init();
    
    let pbo_path = args.input;
    
    info!("Listing PBO: {}", pbo_path.display());
    debug!("Using {} threads", args.threads);
    
    let api = PboApi::builder()
        .with_timeout(args.timeout)
        .build();

    let options = ExtractOptions {
        no_pause: true,
        warnings_as_errors: false,
        verbose: true,
        ..Default::default()
    };

    match api.list_with_options(&pbo_path, options) {
        Ok(result) => {
            trace!("Files in PBO:");
            for file in result.get_file_list() {
                trace!("  {}", file);
            }
            if let Some(prefix) = result.get_prefix() {
                trace!("\nPBO Prefix: {}", prefix);
            }
        }
        Err(e) => {
            eprintln!("Error listing PBO contents: {}", e);
            std::process::exit(1);
        }
    }
} 