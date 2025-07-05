use arma3_pbo::core::{PboApi, PboApiOps};
use std::path::Path;
use tempfile::TempDir;
use std::fs;
use log::debug;
use std::sync::Once;

static INIT: Once = Once::new();

// Test fixture helper
fn setup() -> (PboApi, TempDir) {
    // Initialize logger
    INIT.call_once(|| {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init()
            .ok();
    });
    
    let temp_dir = TempDir::new().unwrap();
    let api = PboApi::builder()
        .with_timeout(30)
        .build();
    (api, temp_dir)
}

#[tokio::test]
async fn test_list_contents_integration() {
    let (api, _temp_dir) = setup();
    let test_pbo = Path::new("tests/data/mirrorform.pbo");
    
    let result = api.list_contents(test_pbo).await.unwrap();
    assert!(!result.is_empty());
    debug!("Found {} files in PBO", result.len());
    
    // Check that we have some expected file info
    for file in result {
        assert!(!file.file_path.is_empty());
        assert!(file.size > 0);
        debug!("File: {} ({} bytes)", file.file_path, file.size);
    }
}

#[tokio::test]
async fn test_extract_all_integration() {
    let (api, temp_dir) = setup();
    let test_pbo = Path::new("tests/data/mirrorform.pbo");
    let output_dir = temp_dir.path().join("extracted");
    
    api.extract_all(test_pbo, &output_dir).await.unwrap();
    assert!(output_dir.exists());
    
    // Verify that files were actually extracted
    let entries: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(!entries.is_empty());
    
    // Log the extracted files
    debug!("Extracted {} files:", entries.len());
    for entry in entries {
        debug!("  - {}", entry.path().display());
    }
}

#[tokio::test]
async fn test_extract_with_filter_integration() {
    let (api, temp_dir) = setup();
    let test_pbo = Path::new("tests/data/mirrorform.pbo");
    let output_dir = temp_dir.path().join("filtered");
    
    // Extract only .paa files
    api.extract_filtered(test_pbo, &output_dir, "*.paa").await.unwrap();
    assert!(output_dir.exists());
    
    // Debug: Print the output directory structure
    fn walk_dir(dir: &Path, depth: usize) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                let indent = "  ".repeat(depth);
                debug!("{}|- {}", indent, path.display());
                if path.is_dir() {
                    walk_dir(&path, depth + 1);
                }
            }
        }
    }
    
    debug!("Output directory contents:");
    walk_dir(&output_dir, 0);

    // Verify directory contains extracted files
    let entries: Vec<_> = walkdir::WalkDir::new(&output_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();
        
    assert!(!entries.is_empty(), "Output directory should not be empty");
    
    // Verify all extracted files match the filter or are $PBOPREFIX$.txt
    for entry in entries {
        let is_paa = entry.path().extension().map_or(false, |ext| ext == "paa");
        let is_pboprefix = entry.file_name().to_string_lossy() == "$PBOPREFIX$.txt";
        assert!(is_paa || is_pboprefix, 
            "Found file that doesn't match filter and isn't $PBOPREFIX$.txt: {}", 
            entry.path().display());
    }
}

#[tokio::test]
async fn test_get_properties_integration() {
    let (api, _temp_dir) = setup();
    let test_pbo = Path::new("tests/data/mirrorform.pbo");
    
    let properties = api.get_properties(test_pbo).await.unwrap();
    assert!(properties.file_count > 0);
    assert!(properties.total_size > 0);
    debug!("PBO has {} files, {} bytes total", properties.file_count, properties.total_size);
    
    // Check for common properties
    if let Some(prefix) = &properties.prefix {
        debug!("PBO prefix: {}", prefix);
        assert!(!prefix.is_empty());
    }
}

#[tokio::test]
async fn test_validate_pbo_integration() {
    let (api, _temp_dir) = setup();
    let test_pbo = Path::new("tests/data/mirrorform.pbo");
    
    let validation = api.validate_pbo(test_pbo).await.unwrap();
    debug!("PBO validation result: valid={}", validation.is_valid);
    
    if !validation.errors.is_empty() {
        debug!("Validation errors:");
        for error in &validation.errors {
            debug!("  - {}", error.message);
        }
    }
    
    if !validation.warnings.is_empty() {
        debug!("Validation warnings:");
        for warning in &validation.warnings {
            debug!("  - {}", warning.message);
        }
    }
}

#[tokio::test]
async fn test_headgear_pumpkin_integration() {
    let (api, temp_dir) = setup();
    let test_pbo = Path::new("tests/data/headgear_pumpkin.pbo");
    
    // First list the contents
    let files = api.list_contents(test_pbo).await.unwrap();
    assert!(!files.is_empty());
    debug!("PBO contents ({} files):", files.len());
    for file in &files {
        debug!("  - {} ({} bytes)", file.file_path, file.size);
    }
    
    // Now extract all files
    let output_dir = temp_dir.path().join("headgear_pumpkin");
    api.extract_all(test_pbo, &output_dir).await.unwrap();
    assert!(output_dir.exists());
    
    // Verify extracted files using WalkDir for recursive directory traversal
    let entries: Vec<_> = walkdir::WalkDir::new(&output_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();
    
    debug!("Extracted files:");
    for entry in &entries {
        debug!("  - {}", entry.path().display());
    }
    
    assert!(!entries.is_empty(), "Expected files to be extracted");
    
    // Check for expected file types (common Arma 3 asset files)
    let has_config = entries.iter().any(|e| 
        e.path().to_string_lossy().contains("config.cpp") || 
        e.path().to_string_lossy().contains("config.bin")
    );
    let has_paa = entries.iter().any(|e| 
        e.path().extension().map_or(false, |ext| ext == "paa")
    );
    
    assert!(has_config, "Expected config.cpp or config.bin file");
    assert!(has_paa, "Expected at least one .paa texture file");
}

#[tokio::test]
async fn test_headgear_pumpkin_extract_filtered() {
    let (api, temp_dir) = setup();
    let test_pbo = Path::new("tests/data/headgear_pumpkin.pbo");
    let output_dir = temp_dir.path().join("headgear_pumpkin_cpp");
    
    // Extract only config files
    api.extract_filtered(test_pbo, &output_dir, "config.*").await.unwrap();
    assert!(output_dir.exists());
    
    // Verify extracted files using WalkDir
    let entries: Vec<_> = walkdir::WalkDir::new(&output_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();
    
    debug!("Extracted files:");
    for entry in &entries {
        debug!("  - {}", entry.path().display());
    }
    
    // Should have config files extracted
    assert!(!entries.is_empty(), "Expected at least one file to be extracted");
    
    let has_config = entries.iter().any(|e| e.path().to_string_lossy().contains("config"));
    
    assert!(has_config, "Expected config file to be extracted");
}

#[tokio::test]
async fn test_read_file_content() {
    let (api, _temp_dir) = setup();
    let test_pbo = Path::new("tests/data/mirrorform.pbo");
    
    // Try to read a specific file from the PBO
    let files = api.list_contents(test_pbo).await.unwrap();
    if let Some(first_file) = files.first() {
        let content = api.read_file(test_pbo, &first_file.file_path).await.unwrap();
        assert!(!content.is_empty());
        debug!("Read {} bytes from file: {}", content.len(), first_file.file_path);
    }
}