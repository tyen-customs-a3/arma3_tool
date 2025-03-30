// Arma 3 Tool - CLI and UI entry point
use std::env;
use arma3_tool::config::ScanConfig;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    let config_path = if args.len() > 2 {
        args[2].clone()
    } else {
        "scan_config.json".to_string()
    };
    
    // Load scan config
    match ScanConfig::load(&config_path) {
        Ok(config) => {
            // Run the cache builder
            if let Err(e) = arma3_tool::bin::cache_builder::build_cache(config).await {
                eprintln!("Error building cache: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error loading configuration from {}: {}", config_path, e);
            std::process::exit(1);
        }
    }
} 