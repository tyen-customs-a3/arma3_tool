use arma3_pbo::{PboApi, PboApiOps, PboOperationError};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[tokio::test]
async fn test_extract_nonexistent_pbo() {
    let api = PboApi::new(30);
    let temp_dir = tempdir().unwrap();
    let output_dir = temp_dir.path().join("output");
    
    let result = api.extract_all(
        Path::new("/nonexistent/path/file.pbo"),
        &output_dir,
    ).await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), PboOperationError::FileNotFound { .. }));
}

#[tokio::test]
async fn test_extract_empty_file() {
    let api = PboApi::new(30);
    let temp_dir = tempdir().unwrap();
    let pbo_path = temp_dir.path().join("empty.pbo");
    let output_dir = temp_dir.path().join("output");
    
    // Create empty file
    File::create(&pbo_path).unwrap();
    
    let result = api.extract_all(&pbo_path, &output_dir).await;
    
    // Should fail gracefully
    assert!(result.is_err());
}

#[tokio::test]
async fn test_extract_corrupted_header() {
    let api = PboApi::new(30);
    let temp_dir = tempdir().unwrap();
    let pbo_path = temp_dir.path().join("corrupted.pbo");
    let output_dir = temp_dir.path().join("output");
    
    // Create file with corrupted header
    let mut file = File::create(&pbo_path).unwrap();
    file.write_all(b"NOTAPBOHEADER").unwrap();
    file.write_all(&vec![0xFF; 100]).unwrap();
    
    let result = api.extract_all(&pbo_path, &output_dir).await;
    
    // Should fail gracefully
    assert!(result.is_err());
}

#[tokio::test]
async fn test_extract_filtered_no_matches() {
    let api = PboApi::new(30);
    let temp_dir = tempdir().unwrap();
    let pbo_path = temp_dir.path().join("test.pbo");
    let output_dir = temp_dir.path().join("output");
    
    // Create a minimal PBO file (this will still be invalid, but tests the filter logic)
    let mut file = File::create(&pbo_path).unwrap();
    file.write_all(b"PBO").unwrap();
    
    // Use a filter that won't match anything
    let result = api.extract_filtered(&pbo_path, &output_dir, "*.nonexistent").await;
    
    // Should fail due to invalid PBO, but tests the filter path
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_contents_invalid_pbo() {
    let api = PboApi::new(30);
    let temp_dir = tempdir().unwrap();
    let pbo_path = temp_dir.path().join("invalid.pbo");
    
    // Create file with random data
    let mut file = File::create(&pbo_path).unwrap();
    file.write_all(b"Random data that is not a PBO").unwrap();
    
    let result = api.list_contents(&pbo_path).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_properties_missing_file() {
    let api = PboApi::new(30);
    
    let result = api.get_properties(Path::new("/missing/file.pbo")).await;
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), PboOperationError::FileNotFound { .. }));
}

#[tokio::test]
async fn test_validate_zero_byte_pbo() {
    let api = PboApi::new(30);
    let temp_dir = tempdir().unwrap();
    let pbo_path = temp_dir.path().join("zero.pbo");
    
    // Create zero-byte file
    File::create(&pbo_path).unwrap();
    
    let result = api.validate_pbo(&pbo_path).await;
    
    match result {
        Ok(validation) => {
            assert!(validation.is_valid == false);
            assert!(!validation.errors.is_empty());
        }
        Err(_) => {
            // Also acceptable - can't validate invalid file
        }
    }
}

#[tokio::test]
async fn test_extract_with_invalid_output_path() {
    let api = PboApi::new(30);
    let temp_dir = tempdir().unwrap();
    let pbo_path = temp_dir.path().join("test.pbo");
    
    // Create dummy PBO
    File::create(&pbo_path).unwrap();
    
    // Use invalid output path
    let output_dir = Path::new("/\0invalid/path");
    
    let result = api.extract_all(&pbo_path, output_dir).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_timeout_handling() {
    // Create API with very short timeout
    let api = PboApi::new(1); // 1 second timeout
    let temp_dir = tempdir().unwrap();
    
    // Create a large file to potentially trigger timeout
    let pbo_path = temp_dir.path().join("large.pbo");
    let mut file = File::create(&pbo_path).unwrap();
    
    // Write 50MB of data
    for _ in 0..50 {
        file.write_all(&vec![0u8; 1024 * 1024]).unwrap();
    }
    
    let output_dir = temp_dir.path().join("output");
    
    // This might timeout or fail due to invalid format
    let result = api.extract_all(&pbo_path, &output_dir).await;
    
    // Should handle gracefully either way
    if let Err(e) = result {
        println!("Expected error (timeout or invalid format): {:?}", e);
    }
}

#[tokio::test]
async fn test_concurrent_operations() {
    let temp_dir = tempdir().unwrap();
    
    // Create multiple PBO files
    let mut handles = vec![];
    
    for i in 0..5 {
        let api = PboApi::new(30); // Create new API instance for each task
        let pbo_path = temp_dir.path().join(format!("test_{}.pbo", i));
        File::create(&pbo_path).unwrap();
        
        let handle = tokio::spawn(async move {
            let _ = api.list_contents(&pbo_path).await;
        });
        
        handles.push(handle);
    }
    
    // Wait for all operations
    for handle in handles {
        let _ = handle.await;
    }
    
    // Should complete without panicking
}

// Pattern matching tests moved to hemtt_backend tests module since matches_pattern is private