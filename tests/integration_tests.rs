use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};
use arma3_tool::config::ToolConfig;
use arma3_tool::cache::CacheManager;
use arma3_tool::scanner::gamedata::GameDataScanner;
use arma3_tool::scanner::mission::MissionScanner;
use arma3_tool::scanner::report::ReportGenerator;

// Helper function to create a test config directly
fn create_test_config(temp_dir: &TempDir) -> ToolConfig {
    let cache_dir = temp_dir.path().join("cache");
    let report_dir = temp_dir.path().join("reports");
    
    // Create directories
    fs::create_dir_all(&cache_dir).unwrap();
    fs::create_dir_all(&report_dir).unwrap();
    
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
        cache_dir,
        report_dir,
        threads: 2,
        timeout: 30,
        process_base_game: false,
    }
}

#[test]
fn test_full_pipeline_setup() {
    // Create temp directory and config
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    
    // Initialize cache manager
    let cache_manager = CacheManager::new(config.cache_dir.clone());
    
    // Create scanners
    let game_data_scanner = GameDataScanner::new(config.clone(), cache_manager.clone());
    let mission_scanner = MissionScanner::new(config.clone(), cache_manager);
    
    // Create report generator
    let report_generator = ReportGenerator::new(config.report_dir.clone());
    
    // Test that all components were created successfully
    assert!(true, "Full pipeline setup successful");
}

#[test]
fn test_cache_directory_creation() {
    // Create temp directory and config
    let temp_dir = TempDir::new().unwrap();
    let config = create_test_config(&temp_dir);
    
    // Initialize cache manager
    let cache_manager = CacheManager::new(config.cache_dir.clone());
    
    // Check that cache directories were created
    assert!(config.cache_dir.join("game_data").exists());
    assert!(config.cache_dir.join("missions").exists());
} 