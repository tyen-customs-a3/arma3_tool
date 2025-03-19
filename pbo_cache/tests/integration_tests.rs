use std::path::{Path, PathBuf};
use anyhow::Result;
use tempfile::tempdir;
use std::fs::{self, File};
use std::io::Write;

use arma3_tool_pbo_cache::{
    ExtractionConfig,
    ExtractionManager,
    PboType,
    PboMetadata,
};

/// Utility test to setup a basic configuration
fn create_test_config() -> (tempfile::TempDir, ExtractionConfig) {
    // Create temp directory for cache
    let temp_dir = tempdir().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();
    let game_data_cache_dir = cache_dir.join("gamedata");
    let mission_cache_dir = cache_dir.join("missions");
    
    // Create a test config
    let config = ExtractionConfig {
        cache_dir,
        game_data_cache_dir,
        mission_cache_dir,
        game_data_dirs: Vec::new(),
        game_data_extensions: vec!["hpp".to_string(), "cpp".to_string()],
        mission_dirs: Vec::new(),
        mission_extensions: vec!["hpp".to_string(), "sqf".to_string()],
        threads: 1,
        timeout: 30,
        verbose: false,
    };
    
    (temp_dir, config)
}

/// Create a mock cached file structure
fn create_mock_cache(cache_dir: &Path) -> Vec<PathBuf> {
    let mut created_files = Vec::new();
    
    // Create some mock cached files
    let game_data_file = cache_dir.join("gamedata").join("test.hpp");
    let mission_file = cache_dir.join("missions").join("test_mission").join("mission.sqf");
    
    // Create parent directories
    fs::create_dir_all(game_data_file.parent().unwrap()).unwrap();
    fs::create_dir_all(mission_file.parent().unwrap()).unwrap();
    
    // Create files with some content
    File::create(&game_data_file).unwrap().write_all(b"test content").unwrap();
    File::create(&mission_file).unwrap().write_all(b"mission content").unwrap();
    
    created_files.push(game_data_file);
    created_files.push(mission_file);
    
    created_files
}

/// Create a mock PBO file
fn create_mock_pbo(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)?.write_all(b"mock pbo content")?;
    Ok(())
}

#[test]
fn test_manager_creation() -> Result<()> {
    let (temp_dir, config) = create_test_config();
    let _manager = ExtractionManager::new(config)?;
    
    // Verify cache directories were created
    assert!(temp_dir.path().join("gamedata").exists());
    assert!(temp_dir.path().join("missions").exists());
    
    Ok(())
}

#[test]
fn test_cleanup_cache() -> Result<()> {
    let (temp_dir, config) = create_test_config();
    let mut manager = ExtractionManager::new(config)?;
    
    // Create mock cached files
    let created_files = create_mock_cache(temp_dir.path());
    
    // Verify files exist
    for file in &created_files {
        assert!(file.exists(), "Mock cache file should exist: {}", file.display());
    }
    
    // Create mock PBO files
    let game_data_pbo = temp_dir.path().join("game_data.pbo");
    let mission_pbo = temp_dir.path().join("mission.pbo");
    create_mock_pbo(&game_data_pbo)?;
    create_mock_pbo(&mission_pbo)?;
    
    // Add files to the index
    let mut game_data_metadata = PboMetadata::new(
        game_data_pbo.clone(),
        PboType::GameData,
        vec!["hpp".to_string(), "cpp".to_string()],
    )?;
    game_data_metadata.extracted_files = vec![created_files[0].clone()];
    manager.update_metadata(game_data_metadata)?;
    
    let mut mission_metadata = PboMetadata::new(
        mission_pbo.clone(),
        PboType::Mission,
        vec!["hpp".to_string(), "sqf".to_string()],
    )?;
    mission_metadata.extracted_files = vec![created_files[1].clone()];
    manager.update_metadata(mission_metadata)?;
    
    // Save the index
    manager.save_index()?;
    
    // Delete the PBO files to simulate them being removed
    fs::remove_file(game_data_pbo)?;
    fs::remove_file(mission_pbo)?;
    
    // Run cleanup (should remove files since the PBOs don't exist)
    manager.cleanup_cache()?;
    
    // Verify files were removed
    for file in &created_files {
        assert!(!file.exists(), "Cache file should be removed: {}", file.display());
    }
    
    // Verify mission directory was removed (should be empty)
    let mission_dir = temp_dir.path().join("missions").join("test_mission");
    assert!(!mission_dir.exists(), "Empty mission directory should be removed");
    
    Ok(())
}