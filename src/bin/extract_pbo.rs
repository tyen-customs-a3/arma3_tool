use std::path::PathBuf;
use std::fs;
use clap::Parser;
use pbo_tools::core::api::{PboApi, PboApiOps};
use pbo_tools::extract::ExtractOptions;
use pbo_tools::core::config::PboConfig;
use log;
use env_logger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input PBO file to extract
    #[arg(short, long)]
    input: PathBuf,

    /// Output directory for extracted files
    #[arg(short, long)]
    output: PathBuf,
    
    /// File extensions to extract (comma-separated)
    #[arg(short, long, default_value = "")]
    extensions: String,
    
    /// Timeout in seconds for extraction
    #[arg(short, long, default_value = "30")]
    timeout: u32,

    /// Number of threads to use for extraction
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
    let output_dir = args.output;
    
    println!("Extracting PBO: {}", pbo_path.display());
    println!("Output directory: {}", output_dir.display());
    
    // Remove output directory if it exists
    if output_dir.exists() {
        println!("Removing existing output directory: {}", output_dir.display());
        fs::remove_dir_all(&output_dir).expect("Failed to remove existing output directory");
    }
    // Create output directory
    println!("Creating output directory: {}", output_dir.display());
    std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    // First try to list contents
    let api = PboApi::builder()
        .with_timeout(args.timeout)
        .build();

    println!("Using {} threads for extraction", args.threads);

    match api.list_contents(&pbo_path) {
        Ok(list_result) => {
            println!("PBO contents:");
            for line in list_result.stdout.lines() {
                println!("  {}", line);
            }
            if let Some(prefix) = list_result.get_prefix() {
                println!("\nPBO Prefix: {}", prefix);
            }
        }
        Err(e) => {
            eprintln!("Error listing PBO contents: {}", e);
            std::process::exit(1);
        }
    }

    // Try different extraction approaches
    let attempts: Vec<(&str, Box<dyn Fn() -> Result<_, _>>)> = vec![
        ("Simple extract", Box::new(|| {
            api.extract_files(&pbo_path, &output_dir, None)
        })),
        ("With binary mappings", Box::new(|| {
            let config = PboConfig::builder()
                .add_bin_mapping("config.bin", "cpp")
                .add_bin_mapping("script.bin", "cpp")
                .build();
            let api = PboApi::builder()
                .with_config(config)
                .with_timeout(args.timeout)
                .build();
            api.extract_files(&pbo_path, &output_dir, None)
        })),
        ("With options", Box::new(|| {
            let options = ExtractOptions {
                no_pause: true,
                warnings_as_errors: false,
                verbose: true,
                file_filter: if args.extensions.is_empty() { 
                    Some("hpp,cpp,xml".to_string()) 
                } else { 
                    Some(args.extensions.clone()) 
                },
                ..Default::default()
            };
            api.extract_with_options(&pbo_path, &output_dir, options)
        })),
        ("With permissive options", Box::new(|| {
            let options = ExtractOptions {
                no_pause: true,
                warnings_as_errors: false,
                verbose: true,
                file_filter: None,
                ..Default::default()
            };
            api.extract_with_options(&pbo_path, &output_dir, options)
        })),
    ];

    for (desc, attempt) in attempts.iter() {
        println!("\nTrying extraction with {}", desc);
        match attempt() {
            Ok(result) => {
                println!("Extraction successful!");
                
                // Check if files were actually extracted
                let extracted_files = fs::read_dir(&output_dir)
                    .map(|entries| entries.count())
                    .unwrap_or(0);
                
                if extracted_files == 0 {
                    println!("Warning: No files were extracted to the output directory!");
                } else {
                    println!("Extracted {} files to {}", extracted_files, output_dir.display());
                }
                
                println!("Extracted files reported by PBO tool:");
                for file in result.get_file_list() {
                    println!("  {}", file);
                }
                if let Some(prefix) = result.get_prefix() {
                    println!("\nPBO Prefix: {}", prefix);
                }
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Error with {}: {}", desc, e);
            }
        }
    }

    eprintln!("All extraction attempts failed");
    std::process::exit(1);
} 