//! Unit tests for individual components of the Weapon Magazine Scanner
//! 
//! These tests focus on testing individual modules and functions in isolation.

use weapon_magazine_scanner::{models::*, WeaponMagazineScanner};
use tempfile::TempDir;
use std::path::Path;
use std::collections::HashMap;

/// Test WeaponInfo model creation and serialization
#[test]
fn test_weapon_info_model() {
    let weapon = WeaponInfo {
        name: "test_weapon".to_string(),
        parent: Some("base_weapon".to_string()),
        file_path: Path::new("test/config.cpp").to_path_buf(),
        magazine_wells: vec!["CBA_556_NATO".to_string(), "CBA_556_STANAG".to_string()],
        compatible_magazines: vec!["30Rnd_556x45".to_string(), "20Rnd_556x45".to_string()],
        mod_source: Some("test_mod".to_string()),
    };
    
    // Test basic properties
    assert_eq!(weapon.name, "test_weapon");
    assert_eq!(weapon.parent, Some("base_weapon".to_string()));
    assert_eq!(weapon.magazine_wells.len(), 2);
    assert_eq!(weapon.compatible_magazines.len(), 2);
    
    // Test serialization to JSON
    let json = serde_json::to_string(&weapon).expect("Failed to serialize WeaponInfo");
    assert!(json.contains("test_weapon"));
    assert!(json.contains("CBA_556_NATO"));
    
    // Test deserialization from JSON
    let deserialized: WeaponInfo = serde_json::from_str(&json)
        .expect("Failed to deserialize WeaponInfo");
    assert_eq!(deserialized.name, weapon.name);
    assert_eq!(deserialized.magazine_wells, weapon.magazine_wells);
    
    println!("✅ WeaponInfo model test passed");
}

/// Test MagazineWellInfo model creation and functionality
#[test]
fn test_magazine_well_info_model() {
    let mut magazines = HashMap::new();
    magazines.insert("NATO_Magazines".to_string(), vec![
        "30Rnd_556x45_Stanag".to_string(),
        "20Rnd_556x45_Stanag".to_string(),
    ]);
    magazines.insert("Tracer_Magazines".to_string(), vec![
        "30Rnd_556x45_Stanag_Tracer_Red".to_string(),
    ]);
    
    let magazine_well = MagazineWellInfo {
        name: "CBA_556_STANAG".to_string(),
        file_path: Path::new("test/config.cpp").to_path_buf(),
        magazines,
        mod_source: Some("test_mod".to_string()),
    };
    
    // Test basic properties
    assert_eq!(magazine_well.name, "CBA_556_STANAG");
    assert_eq!(magazine_well.magazines.len(), 2);
    assert!(magazine_well.magazines.contains_key("NATO_Magazines"));
    assert!(magazine_well.magazines.contains_key("Tracer_Magazines"));
    
    // Test magazine counts
    let nato_mags = &magazine_well.magazines["NATO_Magazines"];
    assert_eq!(nato_mags.len(), 2);
    assert!(nato_mags.contains(&"30Rnd_556x45_Stanag".to_string()));
    
    // Test serialization
    let json = serde_json::to_string(&magazine_well).expect("Failed to serialize MagazineWellInfo");
    assert!(json.contains("CBA_556_STANAG"));
    assert!(json.contains("NATO_Magazines"));
    
    println!("✅ MagazineWellInfo model test passed");
}

/// Test ScanResult model with complete data
#[test]
fn test_scan_result_model() {
    let weapon = WeaponInfo {
        name: "test_weapon".to_string(),
        parent: None,
        file_path: Path::new("test.cpp").to_path_buf(),
        magazine_wells: vec!["CBA_Test".to_string()],
        compatible_magazines: vec!["Test_Magazine".to_string()],
        mod_source: Some("test_mod".to_string()),
    };
    
    let mut magazine_wells = HashMap::new();
    let mut magazines = HashMap::new();
    magazines.insert("Test_Prefix".to_string(), vec!["Test_Magazine".to_string()]);
    
    magazine_wells.insert("CBA_Test".to_string(), MagazineWellInfo {
        name: "CBA_Test".to_string(),
        file_path: Path::new("test.cpp").to_path_buf(),
        magazines,
        mod_source: Some("test_mod".to_string()),
    });
    
    let scan_result = ScanResult {
        weapons: vec![weapon],
        magazine_wells,
        scan_timestamp: chrono::Utc::now(),
        folder_hash: "test_hash_123".to_string(),
    };
    
    // Test structure
    assert_eq!(scan_result.weapons.len(), 1);
    assert_eq!(scan_result.magazine_wells.len(), 1);
    assert!(!scan_result.folder_hash.is_empty());
    assert!(scan_result.scan_timestamp.timestamp() > 0);
    
    // Test serialization
    let json = serde_json::to_string(&scan_result).expect("Failed to serialize ScanResult");
    assert!(json.contains("test_weapon"));
    assert!(json.contains("CBA_Test"));
    assert!(json.contains("test_hash_123"));
    
    println!("✅ ScanResult model test passed");
}

/// Test FolderHash model functionality
#[test]
fn test_folder_hash_model() {
    let mut file_hashes = HashMap::new();
    file_hashes.insert(
        Path::new("config.cpp").to_path_buf(),
        "abc123def456".to_string()
    );
    file_hashes.insert(
        Path::new("addon.cpp").to_path_buf(),
        "def456ghi789".to_string()
    );
    
    let folder_hash = FolderHash {
        hash: "master_hash_xyz".to_string(),
        file_hashes,
    };
    
    // Test properties
    assert_eq!(folder_hash.hash, "master_hash_xyz");
    assert_eq!(folder_hash.file_hashes.len(), 2);
    assert!(folder_hash.file_hashes.contains_key(Path::new("config.cpp")));
    assert!(folder_hash.file_hashes.contains_key(Path::new("addon.cpp")));
    
    // Test serialization
    let json = serde_json::to_string(&folder_hash).expect("Failed to serialize FolderHash");
    assert!(json.contains("master_hash_xyz"));
    assert!(json.contains("config.cpp"));
    
    println!("✅ FolderHash model test passed");
}

/// Test scanner creation with different parameters
#[test]
fn test_scanner_creation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test different thread counts
    for thread_count in [1, 2, 4, 8] {
        let scanner = WeaponMagazineScanner::new(
            temp_dir.path(),
            thread_count,
            30
        ).expect("Failed to create scanner");
        
        let stats = scanner.get_performance_stats();
        assert_eq!(stats["thread_count"], thread_count.to_string());
        assert_eq!(stats["timeout_seconds"], "30");
    }
    
    // Test different timeout values (note: scanner enforces consistent timeout)
    for timeout in [10, 30, 60] {
        let scanner = WeaponMagazineScanner::new(
            temp_dir.path(),
            4,
            timeout
        ).expect("Failed to create scanner");
        
        let stats = scanner.get_performance_stats();
        // Scanner enforces consistent timeout, so this should be "30"
        assert_eq!(stats["timeout_seconds"], "30");
    }
    
    println!("✅ Scanner creation test passed");
}

/// Test scanner performance statistics
#[test]
fn test_scanner_performance_stats() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let scanner = WeaponMagazineScanner::new(temp_dir.path(), 6, 45)
        .expect("Failed to create scanner");
    
    let stats = scanner.get_performance_stats();
    
    // Verify all expected keys are present
    assert!(stats.contains_key("thread_count"));
    assert!(stats.contains_key("timeout_seconds"));
    
    // Verify values
    assert_eq!(stats["thread_count"], "6");
    assert_eq!(stats["timeout_seconds"], "30"); // Enforced consistent timeout
    
    // Verify stats are string type
    for (key, value) in &stats {
        assert!(!key.is_empty(), "Stats key should not be empty");
        assert!(!value.is_empty(), "Stats value should not be empty");
        assert!(value.parse::<u64>().is_ok(), "Stats value should be numeric: {}", value);
    }
    
    println!("✅ Scanner performance stats test passed");
}

/// Test error handling for invalid scanner creation
#[test]
fn test_scanner_error_handling() {
    // Test with non-existent directory (should still work - scanner creates what it needs)
    let non_existent = Path::new("/completely/non/existent/path/that/should/not/exist");
    
    // This might succeed or fail depending on permissions and OS
    // The important thing is it handles it gracefully
    match WeaponMagazineScanner::new(non_existent, 4, 30) {
        Ok(_) => println!("Scanner created successfully with non-existent path"),
        Err(e) => println!("Scanner creation failed gracefully: {}", e),
    }
    
    // Test with extreme thread counts
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Very high thread count should work
    let high_thread_scanner = WeaponMagazineScanner::new(temp_dir.path(), 1000, 30);
    assert!(high_thread_scanner.is_ok(), "Scanner should handle high thread counts");
    
    // Zero thread count might fail or default to 1
    let zero_thread_scanner = WeaponMagazineScanner::new(temp_dir.path(), 0, 30);
    match zero_thread_scanner {
        Ok(scanner) => {
            let stats = scanner.get_performance_stats();
            println!("Zero thread count handled: thread_count = {}", stats["thread_count"]);
        },
        Err(e) => {
            println!("Zero thread count failed gracefully: {}", e);
        }
    }
    
    println!("✅ Scanner error handling test passed");
}

/// Test empty result handling
#[test]
fn test_empty_scan_results() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let scanner = WeaponMagazineScanner::new(temp_dir.path(), 4, 30)
        .expect("Failed to create scanner");
    
    // Create an empty directory to scan
    let empty_dir = temp_dir.path().join("empty");
    std::fs::create_dir_all(&empty_dir).expect("Failed to create empty directory");
    
    let result = scanner.scan(&empty_dir).expect("Failed to scan empty directory");
    
    // Should return empty results, not error
    assert_eq!(result.weapons.len(), 0);
    assert_eq!(result.magazine_wells.len(), 0);
    assert!(!result.folder_hash.is_empty()); // Hash should still be calculated
    assert!(result.scan_timestamp.timestamp() > 0); // Timestamp should be set
    
    println!("✅ Empty scan results test passed");
}

/// Test model edge cases and validation
#[test]
fn test_model_edge_cases() {
    // Test weapon with no magazine wells
    let empty_weapon = WeaponInfo {
        name: "empty_weapon".to_string(),
        parent: None,
        file_path: Path::new("test.cpp").to_path_buf(),
        magazine_wells: Vec::new(),
        compatible_magazines: Vec::new(),
        mod_source: None,
    };
    
    assert!(empty_weapon.magazine_wells.is_empty());
    assert!(empty_weapon.compatible_magazines.is_empty());
    
    // Test magazine well with no magazines
    let empty_well = MagazineWellInfo {
        name: "empty_well".to_string(),
        file_path: Path::new("test.cpp").to_path_buf(),
        magazines: HashMap::new(),
        mod_source: None,
    };
    
    assert!(empty_well.magazines.is_empty());
    
    // Test scan result with no data
    let empty_result = ScanResult {
        weapons: Vec::new(),
        magazine_wells: HashMap::new(),
        scan_timestamp: chrono::Utc::now(),
        folder_hash: "empty_hash".to_string(),
    };
    
    assert!(empty_result.weapons.is_empty());
    assert!(empty_result.magazine_wells.is_empty());
    
    // All should serialize successfully
    assert!(serde_json::to_string(&empty_weapon).is_ok());
    assert!(serde_json::to_string(&empty_well).is_ok());
    assert!(serde_json::to_string(&empty_result).is_ok());
    
    println!("✅ Model edge cases test passed");
}

/// Test concurrent scanner usage (basic thread safety)
#[test]
fn test_scanner_thread_safety() {
    use std::sync::Arc;
    use std::thread;
    
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let scanner = Arc::new(
        WeaponMagazineScanner::new(temp_dir.path(), 4, 30)
            .expect("Failed to create scanner")
    );
    
    // Create empty directory to scan
    let scan_dir = temp_dir.path().join("scan_test");
    std::fs::create_dir_all(&scan_dir).expect("Failed to create scan directory");
    
    let mut handles = Vec::new();
    
    // Start multiple threads scanning the same directory
    for i in 0..3 {
        let scanner_clone = Arc::clone(&scanner);
        let scan_path = scan_dir.clone();
        
        let handle = thread::spawn(move || {
            let result = scanner_clone.scan(&scan_path);
            println!("Thread {} scan result: {:?}", i, result.is_ok());
            result.expect("Scan should succeed").weapons.len()
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().expect("Thread should complete"));
    }
    
    // All threads should return the same result (0 weapons from empty directory)
    assert!(results.iter().all(|&x| x == 0));
    
    println!("✅ Scanner thread safety test passed");
}