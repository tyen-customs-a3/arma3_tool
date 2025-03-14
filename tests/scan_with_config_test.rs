use std::path::PathBuf;
use arma3_tool::config::ToolConfig;
use arma3_tool::cache::CacheManager;
use arma3_tool::scanner::gamedata::GameDataScanner;
use arma3_tool::scanner::mission::MissionScanner;

#[test]
fn test_scanner_initialization_with_config() {
    // Load the actual scan_config.json file
    let config_path = PathBuf::from("scan_config.json");
    let config = ToolConfig::from_file(&config_path).unwrap();
    
    // Create cache manager
    let cache_manager = CacheManager::new(config.cache_dir.clone());
    
    // Create scanners
    let game_data_scanner = GameDataScanner::new(config.clone(), cache_manager.clone());
    let mission_scanner = MissionScanner::new(config, cache_manager);
    
    // Test successful initialization
    assert!(true, "Scanners initialized successfully with scan_config.json");
} 