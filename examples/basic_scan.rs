use std::path::PathBuf;
use arma3_tool::config::ToolConfig;
use arma3_tool::cache::CacheManager;
use arma3_tool::scanner::gamedata::GameDataScanner;
use arma3_tool::scanner::mission::MissionScanner;
use arma3_tool::scanner::report::ReportGenerator;
use log::{info, error};

fn main() {
    // Initialize logger
    env_logger::init();
    
    // Load configuration
    let config_path = PathBuf::from("scan_config.json");
    let config = match ToolConfig::from_file(&config_path) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return;
        }
    };
    
    // Initialize cache manager
    let cache_manager = CacheManager::new(config.cache_dir.clone());
    
    // Scan game data
    info!("Scanning game data...");
    let game_data_scanner = GameDataScanner::new(config.clone(), cache_manager.clone());
    let game_data = match game_data_scanner.scan(None) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to scan game data: {}", e);
            return;
        }
    };
    info!("Game data scanning completed successfully");
    
    // Scan missions
    info!("Scanning missions...");
    let mission_scanner = MissionScanner::new(config.clone(), cache_manager);
    let mission_results = match mission_scanner.scan(None) {
        Ok(results) => results,
        Err(e) => {
            error!("Failed to scan missions: {}", e);
            return;
        }
    };
    info!("Mission scanning completed successfully");
    
    // Generate reports
    info!("Generating reports...");
    let report_generator = ReportGenerator::new(config.report_dir.clone());
    match report_generator.generate_from_results(&game_data, &mission_results) {
        Ok(_) => info!("Report generation completed successfully"),
        Err(e) => error!("Failed to generate reports: {}", e),
    }
} 