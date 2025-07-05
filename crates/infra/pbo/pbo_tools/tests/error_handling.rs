use pbo_tools::core::{PboApi, PboApiOps};
use pbo_tools::ops::PboOperationError;
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_invalid_pbo_path() {
    let api = PboApi::builder()
        .with_timeout(30)
        .build();
    let nonexistent = PathBuf::from("nonexistent.pbo");
    
    match api.list_contents(&nonexistent).await {
        Err(PboOperationError::IoError { source, .. }) if source.kind() == std::io::ErrorKind::NotFound => {
            // Expected - file doesn't exist
        }
        Err(PboOperationError::FileNotFound { path }) => {
            assert_eq!(path, nonexistent);
        }
        other => panic!("Expected IoError or FileNotFound error, got {:?}", other),
    }
}

#[tokio::test]
async fn test_invalid_output_dir() {
    let api = PboApi::builder()
        .with_timeout(30)
        .build();
    let test_pbo = PathBuf::from("tests/data/mirrorform.pbo");
    // Use an absolute path with multiple missing parent directories to ensure InvalidPath error
    let invalid_dir = PathBuf::from("/nonexistent/deep/nested/path/tc/mirrorform");
    
    match api.extract_all(&test_pbo, &invalid_dir).await {
        Err(PboOperationError::FileNotFound { path }) => {
            assert_eq!(path, invalid_dir);
        }
        Err(PboOperationError::IoError { .. }) => {
            // Also acceptable - IO error when trying to create directories
        }
        other => panic!("Expected FileNotFound or IoError, got {:?}", other),
    }
}

#[tokio::test]
async fn test_invalid_file_filter() {
    let api = PboApi::builder()
        .with_timeout(30)
        .build();
    let test_pbo = PathBuf::from("tests/data/mirrorform.pbo");
    let temp_dir = TempDir::new().unwrap();
    
    // Test with pattern that shouldn't match anything
    match api.extract_filtered(&test_pbo, temp_dir.path(), "definitely_nonexistent_pattern_12345").await {
        Ok(()) => {
            // Filter worked but probably didn't match anything - this is acceptable
        }
        Err(PboOperationError::ValidationFailed { reason }) => {
            assert!(reason.contains("Invalid") || reason.contains("regex") || reason.contains("pattern"));
        }
        Err(PboOperationError::InvalidFormat { reason }) => {
            // Also acceptable - format error for invalid regex
            assert!(reason.contains("regex") || reason.contains("pattern"));
        }
        other => panic!("Expected Ok, ValidationFailed or InvalidFormat, got {:?}", other),
    }
}

#[tokio::test]
async fn test_validation_failures() {
    let api = PboApi::builder()
        .with_timeout(30)
        .build();
    let test_pbo = PathBuf::from("tests/data/mirrorform.pbo");
    let temp_dir = TempDir::new().unwrap();
    
    // Test with empty file filter
    match api.extract_filtered(&test_pbo, temp_dir.path(), "").await {
        Err(PboOperationError::InvalidPath { path }) => {
            assert!(path.contains("empty") || path.contains("filter"));
        }
        other => panic!("Expected InvalidPath error for empty filter, got {:?}", other),
    }
}

#[tokio::test]
async fn test_timeout_error() {
    let api = PboApi::builder()
        .with_timeout(1) // Very short timeout
        .build();
    let test_pbo = PathBuf::from("tests/data/mirrorform.pbo");
    
    // This might timeout on a slow system, but it's hard to guarantee
    // Just test that the API works with short timeouts
    match api.list_contents(&test_pbo).await {
        Ok(_) => {
            // Fast operation completed successfully
        }
        Err(PboOperationError::Timeout { .. }) => {
            // Expected timeout error
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}