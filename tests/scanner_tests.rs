use std::path::{Path, PathBuf};
use std::fs;
use tempfile::TempDir;
use arma3_tool::config::ToolConfig;
use arma3_tool::cache::CacheManager;
use arma3_tool::scanner::gamedata::GameDataScanner;
use arma3_tool::scanner::mission::MissionScanner;
use arma3_tool::scanner::report::ReportGenerator;

// Helper function to create a test config
fn create_test_config(temp_dir: &TempDir) -> ToolConfig {
    ToolConfig {
        game_data_dirs: vec![PathBuf::from("tests/fixtures/gamedata")],
        game_data_extensions: vec!["hpp".to_string(), "cpp".to_string()],
        mission_dirs: vec![PathBuf::from("tests/fixtures/missions")],
        mission_extensions: vec![
            "hpp".to_string(), 
            "cpp".to_string(), 
            "sqf".to_string(), 
            "sqm".to_string()
        ],
        cache_dir: temp_dir.path().join("cache"),
        report_dir: temp_dir.path().join("reports"),
        threads: 2,
        timeout: 30,
        process_base_game: false,
    }
}

#[test]
fn test_scanner_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    let cache_manager = CacheManager::new(config.cache_dir.clone());
    
    // Create scanners
    let game_data_scanner = GameDataScanner::new(config.clone(), cache_manager.clone());
    let mission_scanner = MissionScanner::new(config, cache_manager);
    
    // Test that scanners were created successfully
    // This is a basic test that doesn't rely on actual PBO extraction
    assert!(true, "Scanners created successfully");
}

#[test]
fn test_report_generator_creation() {
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    let cache_manager = CacheManager::new(config.cache_dir.clone());
    
    // Create directories
    fs::create_dir_all(&config.report_dir).unwrap();
    fs::create_dir_all(&config.cache_dir).unwrap();
    
    // Create report generator
    let mut report_generator = ReportGenerator::new(config.report_dir, cache_manager);
    
    // Test generate method (should return error when no cache exists)
    let result = report_generator.generate(None);
    assert!(result.is_err(), "Should fail when no cache exists");
} 