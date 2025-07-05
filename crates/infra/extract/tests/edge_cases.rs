use anyhow::Result;
use tempfile::tempdir;
use std::fs::{self, File};
use std::io::Write;
use arma3_extract::{ExtractionConfig, ExtractionManager};

/// Helper to create test configuration
fn create_test_config() -> (tempfile::TempDir, ExtractionConfig) {
    let temp_dir = tempdir().unwrap();
    let cache_dir = temp_dir.path().to_path_buf();
    let game_data_cache_dir = cache_dir.join("gamedata");
    let mission_cache_dir = cache_dir.join("missions");
    
    let config = ExtractionConfig {
        cache_dir: cache_dir.clone(),
        game_data_cache_dir,
        mission_cache_dir,
        game_data_dirs: Vec::new(),
        game_data_extensions: vec!["hpp".to_string(), "cpp".to_string()],
        mission_dirs: Vec::new(),
        mission_extensions: vec!["hpp".to_string(), "sqf".to_string()],
        threads: 1,
        timeout: 5, // Short timeout for edge case testing
        verbose: true,
        db_path: cache_dir.join("cache.db"),
    };
    
    (temp_dir, config)
}

#[tokio::test]
async fn test_empty_pbo_extraction() -> Result<()> {
    let (temp_dir, mut config) = create_test_config();
    
    // Create an empty PBO file (just minimal header)
    let pbo_path = temp_dir.path().join("empty.pbo");
    let mut file = File::create(&pbo_path)?;
    
    // Write minimal PBO header (empty PBO)
    file.write_all(b"\x00\x00\x00\x00")?; // Empty header
    file.write_all(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00")?;
    file.write_all(b"\x00\x00\x00\x00")?; // End marker
    
    config.game_data_dirs = vec![temp_dir.path().to_path_buf()];
    
    let mut manager = ExtractionManager::new(config)?;
    
    // Should handle empty PBO gracefully
    let result = manager.process_game_data(false).await;
    
    // The extraction might fail, but shouldn't panic
    match result {
        Ok(files) => {
            assert_eq!(files.len(), 0, "Empty PBO should extract 0 files");
        }
        Err(e) => {
            // This is expected for malformed PBO
            println!("Expected error for empty PBO: {}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_corrupted_pbo_extraction() -> Result<()> {
    let (temp_dir, mut config) = create_test_config();
    
    // Create a corrupted PBO file
    let pbo_path = temp_dir.path().join("corrupted.pbo");
    let mut file = File::create(&pbo_path)?;
    
    // Write random garbage data
    file.write_all(b"This is not a valid PBO file at all!")?;
    file.write_all(&vec![0xFF; 1024])?; // Random bytes
    
    config.game_data_dirs = vec![temp_dir.path().to_path_buf()];
    
    let mut manager = ExtractionManager::new(config)?;
    
    // Should handle corrupted PBO gracefully
    let result = manager.process_game_data(false).await;
    
    match result {
        Ok(files) => {
            assert_eq!(files.len(), 0, "Corrupted PBO should extract 0 files");
        }
        Err(e) => {
            // This is expected for corrupted PBO
            println!("Expected error for corrupted PBO: {}", e);
        }
    }
    
    // Check that the failed extraction was recorded
    let db_manager = manager.get_db_manager();
    let failed = db_manager.is_failed_extraction(&pbo_path)?;
    assert!(failed.is_some(), "Failed extraction should be recorded in database");
    
    Ok(())
}

#[tokio::test]
async fn test_zero_byte_file_extraction() -> Result<()> {
    let (temp_dir, mut config) = create_test_config();
    
    // Create a zero-byte PBO file
    let pbo_path = temp_dir.path().join("zerobyte.pbo");
    File::create(&pbo_path)?; // Creates empty file
    
    config.game_data_dirs = vec![temp_dir.path().to_path_buf()];
    
    let mut manager = ExtractionManager::new(config)?;
    
    // Should handle zero-byte file gracefully
    let result = manager.process_game_data(false).await;
    
    match result {
        Ok(files) => {
            assert_eq!(files.len(), 0, "Zero-byte PBO should extract 0 files");
        }
        Err(e) => {
            // This is expected
            println!("Expected error for zero-byte PBO: {}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_pbo_with_invalid_paths() -> Result<()> {
    let (temp_dir, mut config) = create_test_config();
    
    // This test would need a specially crafted PBO with invalid paths
    // For now, we'll test path validation in our code
    
    // Create a PBO path with invalid characters
    let invalid_pbo_path = temp_dir.path().join("test\0invalid.pbo");
    
    config.game_data_dirs = vec![temp_dir.path().to_path_buf()];
    
    let mut manager = ExtractionManager::new(config)?;
    
    // Try to process a PBO with null bytes in the path
    // This should fail at the filesystem level
    let result = manager.process_mission(&invalid_pbo_path, false).await;
    
    assert!(result.is_err(), "Should fail to process PBO with invalid path");
    
    Ok(())
}

#[tokio::test]
async fn test_concurrent_extraction_stress() -> Result<()> {
    let (temp_dir, mut config) = create_test_config();
    
    // Create multiple small PBO files
    let pbo_count = 10;
    for i in 0..pbo_count {
        let pbo_path = temp_dir.path().join(format!("test_{}.pbo", i));
        let mut file = File::create(&pbo_path)?;
        // Write minimal invalid PBO data
        file.write_all(&format!("PBO{}", i).as_bytes())?;
    }
    
    config.game_data_dirs = vec![temp_dir.path().to_path_buf()];
    config.threads = 4; // Use multiple threads for stress test
    
    let mut manager = ExtractionManager::new(config)?;
    
    // Should handle multiple concurrent extractions
    let start = std::time::Instant::now();
    let result = manager.process_game_data(false).await;
    let elapsed = start.elapsed();
    
    println!("Processed {} PBOs in {:?}", pbo_count, elapsed);
    
    // All should fail gracefully
    match result {
        Ok(files) => {
            assert_eq!(files.len(), 0, "Invalid PBOs should extract 0 files");
        }
        Err(e) => {
            println!("Expected error for invalid PBOs: {}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_extraction_with_permission_denied() -> Result<()> {
    // Skip this test on Windows as permission handling is different
    if cfg!(windows) {
        return Ok(());
    }
    
    let (temp_dir, mut config) = create_test_config();
    
    // Create a directory with no write permissions
    let readonly_dir = temp_dir.path().join("readonly");
    fs::create_dir(&readonly_dir)?;
    
    // Create a dummy PBO
    let pbo_path = temp_dir.path().join("test.pbo");
    File::create(&pbo_path)?;
    
    // Set cache directory to read-only location
    config.game_data_cache_dir = readonly_dir.clone();
    config.game_data_dirs = vec![temp_dir.path().to_path_buf()];
    
    // Remove write permissions
    let mut perms = fs::metadata(&readonly_dir)?.permissions();
    use std::os::unix::fs::PermissionsExt;
    perms.set_mode(0o555); // Read and execute only
    fs::set_permissions(&readonly_dir, perms)?;
    
    let mut manager = ExtractionManager::new(config)?;
    
    // Should handle permission denied gracefully
    let result = manager.process_game_data(false).await;
    
    // Restore permissions for cleanup
    let mut perms = fs::metadata(&readonly_dir)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&readonly_dir, perms)?;
    
    assert!(result.is_err() || result.unwrap().is_empty(), 
            "Should fail or extract nothing when cache dir is read-only");
    
    Ok(())
}

#[tokio::test]
async fn test_extraction_timeout() -> Result<()> {
    let (temp_dir, mut config) = create_test_config();
    
    // Create a large fake PBO to potentially trigger timeout
    let pbo_path = temp_dir.path().join("large.pbo");
    let mut file = File::create(&pbo_path)?;
    
    // Write large amount of data
    let large_data = vec![0u8; 10 * 1024 * 1024]; // 10MB
    file.write_all(&large_data)?;
    
    config.game_data_dirs = vec![temp_dir.path().to_path_buf()];
    config.timeout = 1; // Very short timeout
    
    let mut manager = ExtractionManager::new(config)?;
    
    // Should handle timeout gracefully
    let result = manager.process_game_data(false).await;
    
    // Either succeeds quickly or fails due to invalid format/timeout
    match result {
        Ok(files) => {
            println!("Extraction completed with {} files", files.len());
        }
        Err(e) => {
            println!("Expected error (timeout or invalid format): {}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_pbo_already_being_extracted() -> Result<()> {
    let (temp_dir, mut config) = create_test_config();
    
    // Create a PBO file
    let pbo_path = temp_dir.path().join("concurrent.pbo");
    File::create(&pbo_path)?;
    
    config.mission_dirs = vec![temp_dir.path().to_path_buf()];
    
    // Test concurrent access to same PBO
    let _manager1 = ExtractionManager::new(config.clone())?;
    let _manager2 = ExtractionManager::new(config)?;
    
    // Both managers try to extract the same PBO
    // This should be handled gracefully by the system
    
    // In practice, the database locking should prevent issues
    // but we can't easily test true concurrent access in a unit test
    
    Ok(())
}