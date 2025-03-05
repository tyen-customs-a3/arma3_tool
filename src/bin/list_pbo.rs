use std::path::PathBuf;
use clap::Parser;
use pbo_tools::core::api::{PboApi, PboApiOps};
use pbo_tools::extract::ExtractOptions;

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
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();
    
    let pbo_path = args.input;
    
    println!("Listing PBO: {}", pbo_path.display());
    println!("Using {} threads", args.threads);
    
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
            println!("Files in PBO:");
            for file in result.get_file_list() {
                println!("  {}", file);
            }
            if let Some(prefix) = result.get_prefix() {
                println!("\nPBO Prefix: {}", prefix);
            }
        }
        Err(e) => {
            eprintln!("Error listing PBO contents: {}", e);
            std::process::exit(1);
        }
    }
} 