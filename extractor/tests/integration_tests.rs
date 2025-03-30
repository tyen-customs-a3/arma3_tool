use anyhow::Result;
use tempfile::tempdir;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use arma3_extractor::{
    ExtractionConfig,
    ExtractionManager,
    PboMetadata,
    PboType,
};

/// Helper function to create a mock PBO file
fn create_mock_pbo(dir: impl AsRef<std::path::Path>, name: &str) -> PathBuf {
    let path = dir.as_ref().join(name);
    let mut file = File::create(&path).unwrap();
    file.write_all(b"Mock PBO content").unwrap();
    path
}

/// Helper function to create a mock PBO file with specific content
fn create_mock_pbo_with_content(dir: impl AsRef<std::path::Path>, name: &str, content: &[u8]) -> PathBuf {
    let path = dir.as_ref().join(name);
    let mut file = File::create(&path).unwrap();
    file.write_all(content).unwrap();
    path
}

/// Utility test to setup a basic configuration
fn create_test_config() -> (tempfile::TempDir, ExtractionConfig) {
    // Create temp directory for cache
    let temp_dir = tempdir().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();
    let game_data_cache_dir = cache_dir.join("gamedata");
    let mission_cache_dir = cache_dir.join("missions");
    
    // Create a test config
    let config = ExtractionConfig {
        cache_dir: cache_dir.clone(),
        game_data_cache_dir,
        mission_cache_dir,
        game_data_dirs: Vec::new(),
        game_data_extensions: vec!["hpp".to_string(), "cpp".to_string()],
        mission_dirs: Vec::new(),
        mission_extensions: vec!["hpp".to_string(), "sqf".to_string()],
        threads: 1,
        timeout: 30,
        verbose: false,
        db_path: cache_dir.join("cache.db"),
    };
    
    (temp_dir, config)
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
fn test_db_manager_integration() -> Result<()> {
    // Setup test environment
    let (temp_dir, config) = create_test_config();
    
    // Create the extraction manager
    let manager = ExtractionManager::new(config.clone())?;
    
    // Get the db_manager through the extraction manager
    let db_manager = manager.get_db_manager();
    
    // Create a mock PBO file
    let pbo_path = create_mock_pbo(temp_dir.path(), "test_integration.pbo");
    
    // Check if it needs extraction (it should since it's not in the database)
    let needs_extraction = db_manager.needs_extraction(
        &pbo_path,
        PboType::GameData,
        &config.game_data_extensions,
    )?;
    
    assert!(needs_extraction, "New PBO should need extraction");
    
    // Create a metadata object
    let mut metadata = PboMetadata::new(
        pbo_path.clone(),
        PboType::GameData,
        config.game_data_extensions.clone(),
    )?;
    
    // Add some extracted files
    metadata.extracted_files = vec![
        PathBuf::from("test_integration/file1.hpp"),
        PathBuf::from("test_integration/file2.cpp"),
    ];
    
    // Update the metadata
    db_manager.update_metadata(metadata.clone())?;
    
    // Now check if it needs extraction (it shouldn't)
    let needs_extraction = db_manager.needs_extraction(
        &pbo_path,
        PboType::GameData,
        &config.game_data_extensions,
    )?;
    
    assert!(!needs_extraction, "PBO with metadata should not need extraction");
    
    // Check database contains the PBO
    let game_data = db_manager.get_game_data_metadata()?;
    assert_eq!(game_data.len(), 1);
    
    // Record a failed extraction and check it's recorded
    db_manager.add_failed_extraction(&pbo_path, "Test integration failure".to_string())?;
    
    let failed_result = db_manager.is_failed_extraction(&pbo_path)?;
    assert!(failed_result.is_some());
    
    // The timestamp will be dynamic, but we can check the error message
    let (_, error_message) = failed_result.unwrap();
    assert_eq!(error_message, "Test integration failure");
    
    Ok(())
}

#[test]
fn test_file_modification_triggers_extraction() -> Result<()> {
    // Setup test environment
    let (temp_dir, config) = create_test_config();
    
    // Create the extraction manager
    let manager = ExtractionManager::new(config.clone())?;
    let db_manager = manager.get_db_manager();
    
    // Create a small mock PBO file
    let pbo_path = create_mock_pbo_with_content(temp_dir.path(), "size_change.pbo", b"Small content");
    
    // Create and add metadata
    let mut metadata = PboMetadata::new(
        pbo_path.clone(),
        PboType::GameData,
        config.game_data_extensions.clone(),
    )?;
    
    metadata.extracted_files = vec![
        PathBuf::from("size_change/file1.hpp"),
    ];
    
    // Update the metadata
    db_manager.update_metadata(metadata.clone())?;
    
    // Verify it doesn't need extraction
    let needs_extraction = db_manager.needs_extraction(
        &pbo_path,
        PboType::GameData,
        &config.game_data_extensions,
    )?;
    assert!(!needs_extraction, "PBO should not need extraction initially");
    
    // Now replace the file with a larger one
    // Sleep briefly to ensure file system timestamp will change
    thread::sleep(Duration::from_millis(100));
    let _pbo_path = create_mock_pbo_with_content(
        temp_dir.path(), 
        "size_change.pbo", 
        b"This is a much larger content that will change the file size"
    );
    
    // Now it should need extraction due to size change
    let needs_extraction = db_manager.needs_extraction(
        &pbo_path,
        PboType::GameData,
        &config.game_data_extensions,
    )?;
    
    assert!(needs_extraction, "PBO with changed file size should need extraction");
    
    Ok(())
}

#[test]
fn test_extension_changes_trigger_extraction() -> Result<()> {
    // Setup test environment
    let (temp_dir, config) = create_test_config();
    
    // Create the extraction manager
    let manager = ExtractionManager::new(config.clone())?;
    let db_manager = manager.get_db_manager();
    
    // Create a mock PBO file
    let pbo_path = create_mock_pbo(temp_dir.path(), "extension_test.pbo");
    
    // Initial extensions
    let initial_extensions = vec!["hpp".to_string(), "cpp".to_string()];
    
    // Create and add metadata
    let mut metadata = PboMetadata::new(
        pbo_path.clone(),
        PboType::GameData,
        initial_extensions.clone(),
    )?;
    
    metadata.extracted_files = vec![
        PathBuf::from("extension_test/file1.hpp"),
        PathBuf::from("extension_test/file2.cpp"),
    ];
    
    // Update the metadata
    db_manager.update_metadata(metadata.clone())?;
    
    // Verify it doesn't need extraction with the same extensions
    let needs_extraction = db_manager.needs_extraction(
        &pbo_path,
        PboType::GameData,
        &initial_extensions,
    )?;
    assert!(!needs_extraction, "PBO should not need extraction with same extensions");
    
    // Now check with different extensions
    let new_extensions = vec!["hpp".to_string(), "cpp".to_string(), "sqf".to_string()];
    let needs_extraction = db_manager.needs_extraction(
        &pbo_path,
        PboType::GameData,
        &new_extensions,
    )?;
    
    assert!(needs_extraction, "PBO with changed extensions should need extraction");
    
    Ok(())
}

#[test]
fn test_extension_order_doesnt_matter() -> Result<()> {
    // Setup test environment
    let (temp_dir, config) = create_test_config();
    
    // Create the extraction manager
    let manager = ExtractionManager::new(config.clone())?;
    let db_manager = manager.get_db_manager();
    
    // Create a mock PBO file
    let pbo_path = create_mock_pbo(temp_dir.path(), "extension_order.pbo");
    
    // Initial extensions - order shouldn't matter for comparison
    let initial_extensions = vec!["hpp".to_string(), "sqf".to_string(), "cpp".to_string()];
    
    // Create and add metadata
    let mut metadata = PboMetadata::new(
        pbo_path.clone(),
        PboType::GameData,
        initial_extensions.clone(),
    )?;
    
    metadata.extracted_files = vec![
        PathBuf::from("extension_order/file1.hpp"),
        PathBuf::from("extension_order/file2.sqf"),
        PathBuf::from("extension_order/file3.cpp"),
    ];
    
    // Update the metadata
    db_manager.update_metadata(metadata.clone())?;
    
    // Verify it doesn't need extraction with the same extensions in different order
    let reordered_extensions = vec!["cpp".to_string(), "hpp".to_string(), "sqf".to_string()];
    let needs_extraction = db_manager.needs_extraction(
        &pbo_path,
        PboType::GameData,
        &reordered_extensions,
    )?;
    
    assert!(!needs_extraction, "PBO should not need extraction just because extension order changed");
    
    Ok(())
}

#[test]
fn test_pbo_model_validation() -> Result<()> {
    // Setup test environment
    let (temp_dir, config) = create_test_config();
    
    // Create the extraction manager just to access DbManager
    let manager = ExtractionManager::new(config.clone())?;
    let db_manager = manager.get_db_manager();
    
    // Create valid metadata and update
    let pbo_path = create_mock_pbo(temp_dir.path(), "valid_model.pbo");
    let extensions = vec!["hpp".to_string(), "sqf".to_string()];
    
    let mut valid_metadata = PboMetadata::new(
        pbo_path.clone(),
        PboType::GameData,
        extensions.clone(),
    )?;
    
    valid_metadata.extracted_files = vec![
        PathBuf::from("valid_model/file1.hpp"),
    ];
    
    // This should succeed because the model is valid
    assert!(db_manager.update_metadata(valid_metadata.clone()).is_ok());
    
    // Now create invalid metadata with an empty path
    // We can't easily create truly invalid metadata through the PboMetadata API
    // since it has proper validation, but we can test our DbManager's handling
    
    // Create a PBO path that will be deleted
    let temp_pbo_path = create_mock_pbo(temp_dir.path(), "temp_to_delete.pbo");
    let mut invalid_metadata = PboMetadata::new(
        temp_pbo_path.clone(),
        PboType::GameData,
        extensions.clone(),
    )?;
    
    // Remove the file - this makes the metadata invalid for database use
    std::fs::remove_file(&temp_pbo_path)?;
    
    // The update should still work because DbManager doesn't validate metadata fully
    // but downstream systems should handle the missing file correctly
    invalid_metadata.extracted_files = vec![
        PathBuf::from("invalid_path/file1.hpp"),
    ];
    
    assert!(db_manager.update_metadata(invalid_metadata.clone()).is_ok());
    
    // When we check if it needs extraction, it should return true since the file doesn't exist
    let needs_extraction = db_manager.needs_extraction(
        &temp_pbo_path,
        PboType::GameData,
        &extensions,
    )?;
    
    assert!(needs_extraction, "Non-existent PBO should need extraction");
    
    Ok(())
}
