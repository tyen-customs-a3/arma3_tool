use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;
use tokio;
use serde_json;

use arma3_tool::cli::run_export;
use arma3_config::ItemFilterConfig;
use arma3_database::{DatabaseManager, ClassRepository, ClassModel};
use arma3_models::types::PropertyValue;

/// Test data for creating inheritance hierarchies
fn create_inheritance_test_data() -> Vec<ClassModel> {
    vec![
        // Base weapon classes
        ClassModel {
            id: "CfgWeapons".to_string(),
            parent_id: None,
            container_class: None,
            source_file_index: None,
            is_forward_declaration: false,
            properties: HashMap::new(),
        },
        
        ClassModel {
            id: "Rifle_Base_F".to_string(),
            parent_id: Some("CfgWeapons".to_string()),
            container_class: Some("CfgWeapons".to_string()),
            source_file_index: Some(1),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Rifle Base".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        ClassModel {
            id: "Pistol_Base_F".to_string(),
            parent_id: Some("CfgWeapons".to_string()),
            container_class: Some("CfgWeapons".to_string()),
            source_file_index: Some(1),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Pistol Base".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        ClassModel {
            id: "Launcher_Base_F".to_string(),
            parent_id: Some("CfgWeapons".to_string()),
            container_class: Some("CfgWeapons".to_string()),
            source_file_index: Some(1),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Launcher Base".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        // Specific weapon instances
        ClassModel {
            id: "arifle_MX_F".to_string(),
            parent_id: Some("Rifle_Base_F".to_string()),
            container_class: Some("CfgWeapons".to_string()),
            source_file_index: Some(1),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("MX 6.5 mm".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        ClassModel {
            id: "hgun_P07_F".to_string(),
            parent_id: Some("Pistol_Base_F".to_string()),
            container_class: Some("CfgWeapons".to_string()),
            source_file_index: Some(1),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("P07 9 mm".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        ClassModel {
            id: "launch_RPG32_F".to_string(),
            parent_id: Some("Launcher_Base_F".to_string()),
            container_class: Some("CfgWeapons".to_string()),
            source_file_index: Some(1),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("RPG-32".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        // Base uniform classes
        ClassModel {
            id: "CfgVehicles".to_string(),
            parent_id: None,
            container_class: None,
            source_file_index: None,
            is_forward_declaration: false,
            properties: HashMap::new(),
        },
        
        ClassModel {
            id: "Uniform_Base".to_string(),
            parent_id: Some("CfgVehicles".to_string()),
            container_class: Some("CfgVehicles".to_string()),
            source_file_index: Some(2),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Uniform Base".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        ClassModel {
            id: "U_BasicBody".to_string(),
            parent_id: Some("Uniform_Base".to_string()),
            container_class: Some("CfgVehicles".to_string()),
            source_file_index: Some(2),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Basic Body".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        // Base vest classes
        ClassModel {
            id: "Vest_Base".to_string(),
            parent_id: Some("CfgVehicles".to_string()),
            container_class: Some("CfgVehicles".to_string()),
            source_file_index: Some(3),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Vest Base".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        ClassModel {
            id: "V_PlateCarrier1_rgr".to_string(),
            parent_id: Some("Vest_Base".to_string()),
            container_class: Some("CfgVehicles".to_string()),
            source_file_index: Some(3),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Carrier Rig (Green)".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        // Base backpack classes
        ClassModel {
            id: "Backpack_Base".to_string(),
            parent_id: Some("CfgVehicles".to_string()),
            container_class: Some("CfgVehicles".to_string()),
            source_file_index: Some(4),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Backpack Base".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        ClassModel {
            id: "B_AssaultPack_rgr".to_string(),
            parent_id: Some("Backpack_Base".to_string()),
            container_class: Some("CfgVehicles".to_string()),
            source_file_index: Some(4),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Assault Pack (Green)".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
        
        // Some classes that should be excluded (scope != 2)
        ClassModel {
            id: "arifle_MX_Base_F".to_string(),
            parent_id: Some("Rifle_Base_F".to_string()),
            container_class: Some("CfgWeapons".to_string()),
            source_file_index: Some(1),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("MX Base".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(0.0)); // Should be excluded
                props
            },
        },
        
        // Some classes with excluded prefixes
        ClassModel {
            id: "B_soldier_F".to_string(),
            parent_id: Some("CfgVehicles".to_string()),
            container_class: Some("CfgVehicles".to_string()),
            source_file_index: Some(2),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Soldier".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
    ]
}

/// Create a test configuration file
fn create_test_config() -> ItemFilterConfig {
    use arma3_config::{ItemTypeConfig, ExclusionRules};
    
    ItemFilterConfig {
        version: "1.0".to_string(),
        item_types: {
            let mut types = HashMap::new();
            types.insert("weapons".to_string(), ItemTypeConfig {
                base_classes: vec![
                    "Rifle_Base_F".to_string(),
                    "Pistol_Base_F".to_string(),
                    "Launcher_Base_F".to_string(),
                ],
            });
            types.insert("uniforms".to_string(), ItemTypeConfig {
                base_classes: vec![
                    "Uniform_Base".to_string(),
                ],
            });
            types.insert("vests".to_string(), ItemTypeConfig {
                base_classes: vec![
                    "Vest_Base".to_string(),
                ],
            });
            types.insert("backpacks".to_string(), ItemTypeConfig {
                base_classes: vec![
                    "Backpack_Base".to_string(),
                ],
            });
            types
        },
        exclusion_rules: ExclusionRules {
            max_scope: 1,
            excluded_prefixes: vec!["B_soldier_f".to_string()],
        },
    }
}

/// Setup a test database with known class hierarchies
async fn setup_test_database(db_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let db = DatabaseManager::new(db_path)?;
    let class_repo = ClassRepository::new(&db);
    
    // Create test classes
    let test_classes = create_inheritance_test_data();
    for class in &test_classes {
        class_repo.create(class)?;
    }
    
    // Setup file index mapping
    db.with_connection(|conn| {
        conn.execute("PRAGMA foreign_keys = OFF", [])?;
        
        // Create file index mappings
        conn.execute(
            "INSERT INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
             VALUES (1, 'A3/Weapons_F/config.cpp', 'a3/weapons_f/config.cpp', 'A3/Weapons_F')",
            [],
        )?;
        
        conn.execute(
            "INSERT INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
             VALUES (2, 'A3/Characters_F/config.cpp', 'a3/characters_f/config.cpp', 'A3/Characters_F')",
            [],
        )?;
        
        conn.execute(
            "INSERT INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
             VALUES (3, 'A3/Vests_F/config.cpp', 'a3/vests_f/config.cpp', 'A3/Vests_F')",
            [],
        )?;
        
        conn.execute(
            "INSERT INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
             VALUES (4, 'A3/Backpacks_F/config.cpp', 'a3/backpacks_f/config.cpp', 'A3/Backpacks_F')",
            [],
        )?;
        
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(())
    })?;
    
    Ok(())
}

#[tokio::test]
async fn test_export_integration_all_types() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test_export_integration.db");
    let config_path = temp_dir.path().join("test_item_filter_config.json");
    let output_path = temp_dir.path().join("test_export_output.csv");
    
    // Setup test database
    setup_test_database(&db_path).await?;
    
    // Create test configuration
    let config = create_test_config();
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json)?;
    
    // Run export with all types
    run_export(
        db_path.clone(),
        output_path.clone(),
        None, // Export all types
        Some(config_path.clone()),
        0, // No limit
    ).await?;
    
    // Verify output file exists
    assert!(output_path.exists(), "Export output file should exist");
    
    // Read and verify CSV content
    let csv_content = fs::read_to_string(&output_path)?;
    println!("CSV content:\n{}", csv_content);
    
    // Verify header
    assert!(csv_content.starts_with("id,label,type,parent_id,container_class,source_path,properties"));
    
    // Verify that weapons are exported
    assert!(csv_content.contains("arifle_MX_F"), "Should contain MX rifle");
    assert!(csv_content.contains("hgun_P07_F"), "Should contain P07 pistol");
    assert!(csv_content.contains("launch_RPG32_F"), "Should contain RPG launcher");
    
    // Verify that uniforms are exported
    assert!(csv_content.contains("U_BasicBody"), "Should contain uniform");
    
    // Verify that vests are exported
    assert!(csv_content.contains("V_PlateCarrier1_rgr"), "Should contain vest");
    
    // Verify that backpacks are exported
    assert!(csv_content.contains("B_AssaultPack_rgr"), "Should contain backpack");
    
    // Verify exclusions work
    assert!(!csv_content.contains("arifle_MX_Base_F"), "Should exclude scope 0 items");
    assert!(!csv_content.contains("B_soldier_F"), "Should exclude items with excluded prefixes");
    
    // Count lines to verify we have the expected number of items
    let lines: Vec<&str> = csv_content.lines().collect();
    assert!(lines.len() > 1, "Should have at least header + data lines");
    
    Ok(())
}

#[tokio::test]
async fn test_export_integration_specific_types() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test_export_specific.db");
    let config_path = temp_dir.path().join("test_item_filter_config.json");
    let output_path = temp_dir.path().join("test_export_weapons_only.csv");
    
    // Setup test database
    setup_test_database(&db_path).await?;
    
    // Create test configuration
    let config = create_test_config();
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json)?;
    
    // Run export with only weapons
    run_export(
        db_path.clone(),
        output_path.clone(),
        Some("weapons".to_string()),
        Some(config_path.clone()),
        0, // No limit
    ).await?;
    
    // Verify output file exists
    assert!(output_path.exists(), "Export output file should exist");
    
    // Read and verify CSV content
    let csv_content = fs::read_to_string(&output_path)?;
    println!("Weapons-only CSV content:\n{}", csv_content);
    
    // Verify that weapons are exported
    assert!(csv_content.contains("arifle_MX_F"), "Should contain MX rifle");
    assert!(csv_content.contains("hgun_P07_F"), "Should contain P07 pistol");
    assert!(csv_content.contains("launch_RPG32_F"), "Should contain RPG launcher");
    
    // Verify that other types are NOT exported
    assert!(!csv_content.contains("U_BasicBody"), "Should not contain uniform");
    assert!(!csv_content.contains("V_PlateCarrier1_rgr"), "Should not contain vest");
    assert!(!csv_content.contains("B_AssaultPack_rgr"), "Should not contain backpack");
    
    Ok(())
}

#[tokio::test]
async fn test_export_integration_with_limit() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test_export_limit.db");
    let config_path = temp_dir.path().join("test_item_filter_config.json");
    let output_path = temp_dir.path().join("test_export_limited.csv");
    
    // Setup test database
    setup_test_database(&db_path).await?;
    
    // Create test configuration
    let config = create_test_config();
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json)?;
    
    // Run export with limit
    run_export(
        db_path.clone(),
        output_path.clone(),
        None, // Export all types
        Some(config_path.clone()),
        3, // Limit to 3 items
    ).await?;
    
    // Verify output file exists
    assert!(output_path.exists(), "Export output file should exist");
    
    // Read and verify CSV content
    let csv_content = fs::read_to_string(&output_path)?;
    println!("Limited CSV content:\n{}", csv_content);
    
    // Count lines (header + 3 data lines = 4 total)
    let lines: Vec<&str> = csv_content.lines().collect();
    assert_eq!(lines.len(), 4, "Should have exactly 4 lines (header + 3 data)");
    
    Ok(())
}

#[tokio::test]
async fn test_export_integration_multiple_types() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test_export_multi.db");
    let config_path = temp_dir.path().join("test_item_filter_config.json");
    let output_path = temp_dir.path().join("test_export_multi_types.csv");
    
    // Setup test database
    setup_test_database(&db_path).await?;
    
    // Create test configuration
    let config = create_test_config();
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json)?;
    
    // Run export with multiple types
    run_export(
        db_path.clone(),
        output_path.clone(),
        Some("weapons,vests".to_string()),
        Some(config_path.clone()),
        0, // No limit
    ).await?;
    
    // Verify output file exists
    assert!(output_path.exists(), "Export output file should exist");
    
    // Read and verify CSV content
    let csv_content = fs::read_to_string(&output_path)?;
    println!("Multi-type CSV content:\n{}", csv_content);
    
    // Verify that weapons are exported
    assert!(csv_content.contains("arifle_MX_F"), "Should contain MX rifle");
    assert!(csv_content.contains("hgun_P07_F"), "Should contain P07 pistol");
    assert!(csv_content.contains("launch_RPG32_F"), "Should contain RPG launcher");
    
    // Verify that vests are exported
    assert!(csv_content.contains("V_PlateCarrier1_rgr"), "Should contain vest");
    
    // Verify that other types are NOT exported
    assert!(!csv_content.contains("U_BasicBody"), "Should not contain uniform");
    assert!(!csv_content.contains("B_AssaultPack_rgr"), "Should not contain backpack");
    
    Ok(())
}

#[tokio::test]
async fn test_export_integration_invalid_config() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("test_export_invalid.db");
    let config_path = temp_dir.path().join("invalid_config.json");
    let output_path = temp_dir.path().join("test_export_invalid.csv");
    
    // Setup test database
    setup_test_database(&db_path).await?;
    
    // Create invalid configuration
    fs::write(&config_path, "{ invalid json }")?;
    
    // Run export should fail
    let result = run_export(
        db_path.clone(),
        output_path.clone(),
        None,
        Some(config_path.clone()),
        0,
    ).await;
    
    // Verify it fails
    assert!(result.is_err(), "Should fail with invalid configuration");
    
    Ok(())
}

#[tokio::test]
async fn test_export_integration_missing_database() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let db_path = temp_dir.path().join("nonexistent.db");
    let config_path = temp_dir.path().join("test_item_filter_config.json");
    let output_path = temp_dir.path().join("test_export_missing_db.csv");
    
    // Create test configuration
    let config = create_test_config();
    let config_json = serde_json::to_string_pretty(&config)?;
    fs::write(&config_path, config_json)?;
    
    // Run export should fail
    let result = run_export(
        db_path.clone(),
        output_path.clone(),
        None,
        Some(config_path.clone()),
        0,
    ).await;
    
    // Verify it fails
    assert!(result.is_err(), "Should fail with missing database");
    
    Ok(())
}