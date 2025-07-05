use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use arma3_database::{
    DatabaseManager, ClassRepository, ClassModel
};
use arma3_types::types::PropertyValue;
use tempfile::tempdir;
use serde_json;

/// Test the CSV export functionality
#[test]
fn test_csv_export() {
    // Create a temporary directory for our test
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let db_path = temp_dir.path().join("test_export.db");
    let csv_path = temp_dir.path().join("classes_export.csv");
    
    // Create a new database
    let db = DatabaseManager::new(&db_path).expect("Failed to create database");
    let class_repo = ClassRepository::new(&db);
    
    // Populate with test data
    let test_classes = create_test_classes();
    for class in &test_classes {
        class_repo.create(class).expect("Failed to create class");
    }
    
    // Disable foreign key constraints temporarily for our test setup
    db.with_connection(|conn| {
        conn.execute("PRAGMA foreign_keys = OFF", [])?;
        
        // Create file index mapping for test data
        conn.execute(
            "INSERT INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
             VALUES (1, 'A3/Weapons_F/config.cpp', 'a3/weapons_f/config.cpp', 'A3/Weapons_F')",
            [],
        )?;
        
        conn.execute(
            "INSERT INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
             VALUES (2, 'A3/Soft_F/config.cpp', 'a3/soft_f/config.cpp', 'A3/Soft_F')",
            [],
        )?;
        
        // Re-enable foreign key constraints
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        Ok(())
    }).expect("Failed to set up file mapping");
    
    // Create a function to escape CSV values (duplicated from the bin)
    let escape_csv = |s: &str| -> String {
        if s.contains(',') || s.contains('"') || s.contains('\n') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    };
    
    // Create source path cache (simplified version from the binary)
    let mut source_paths = HashMap::new();
    db.with_connection(|conn| {
        let mut stmt = conn.prepare(
            "SELECT file_index, COALESCE(pbo_id, normalized_path) as source_path 
             FROM file_index_mapping"
        )?;
        
        let rows = stmt.query_map([], |row| {
            let idx: i64 = row.get(0)?;
            let path: String = row.get(1)?;
            Ok((idx as usize, path))
        })?;
        
        for row in rows {
            if let Ok((idx, path)) = row {
                source_paths.insert(idx, path);
            }
        }
        
        Ok(())
    }).expect("Failed to create source path cache");
    
    // Create output file
    let mut file = File::create(&csv_path).expect("Failed to create output file");
    
    // Write CSV header
    writeln!(file, "id,label,type,parent_id,container_class,source_path,properties")
        .expect("Failed to write header");
    
    // Get all classes
    let classes = class_repo.get_all().expect("Failed to get classes");
    println!("Found {} classes", classes.len());
    
    // Apply filters and process classes
    for class in classes {
        // Get source path if available
        let source_path = if let Some(idx) = class.source_file_index {
            source_paths.get(&idx).cloned().unwrap_or_default()
        } else {
            String::new()
        };
        
        // Create a label (use class ID if no display name is available)
        let display_name = class.properties.get("displayName")
            .map(|v| match v {
                PropertyValue::String(s) => s.as_str(),
                _ => class.id.as_str(),
            })
            .unwrap_or_else(|| class.id.as_str());
        
        // Convert properties to JSON
        let properties_json = serde_json::to_string(&class.properties)
            .expect("Failed to serialize properties");
        
        // Write CSV line
        writeln!(
            file,
            "{},{},{},{},{},{},\"{}\"",
            escape_csv(&class.id),
            escape_csv(&display_name),
            "class",
            escape_csv(&class.parent_id.unwrap_or_default()),
            escape_csv(&class.container_class.unwrap_or_default()),
            escape_csv(&source_path),
            escape_csv(&properties_json)
        ).expect("Failed to write CSV line");
    }
    
    // Read the CSV file
    let csv_content = std::fs::read_to_string(&csv_path)
        .expect("Failed to read CSV file");
    
    // Print the CSV content for debugging
    println!("CSV content:\n{}", csv_content);
    
    // Verify CSV header
    assert!(csv_content.starts_with("id,label,type,parent_id,container_class,source_path,properties"));
    
    // Verify all test classes are present
    for class in &test_classes {
        let expected_line_part = format!("{},", class.id);
        assert!(csv_content.contains(&expected_line_part), 
                "CSV does not contain class {}", class.id);
    }
    
    // Verify specific content for some classes
    assert!(csv_content.contains("CfgVehicles,CfgVehicles,class,,"));
    assert!(csv_content.contains("CfgWeapons,CfgWeapons,class,,"));
    
    // Verify class data with display names
    assert!(csv_content.contains("Car_F,Car,class"));
    assert!(csv_content.contains("Rifle_Base_F,Rifle Base,class"));
    assert!(csv_content.contains("arifle_MX_Base_F,MX Base,class"));
    
    // Verify source paths are included
    assert!(csv_content.contains("A3/Weapons_F"));
    assert!(csv_content.contains("A3/Soft_F"));
    
    // Verify properties are serialized as JSON
    assert!(csv_content.contains("\"displayName\""));
    assert!(csv_content.contains("Car\""));
    
    println!("Export test passed successfully!");
}

/// Create test classes that match the example format
fn create_test_classes() -> Vec<ClassModel> {
    vec![
        // Root classes
        ClassModel {
            id: "CfgVehicles".to_string(),
            parent_id: None,
            container_class: None,
            source_file_index: None,
            is_forward_declaration: false,
            properties: HashMap::new(),
        },
        
        ClassModel {
            id: "CfgWeapons".to_string(),
            parent_id: None,
            container_class: None,
            source_file_index: None,
            is_forward_declaration: false,
            properties: HashMap::new(),
        },
        
        // Base classes
        ClassModel {
            id: "Car_F".to_string(),
            parent_id: Some("CfgVehicles".to_string()),
            container_class: Some("CfgVehicles".to_string()),
            source_file_index: Some(2), // Maps to A3/Soft_F
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Car".to_string()));
                props.insert("simulation".to_string(), PropertyValue::String("car".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(0.0));
                props
            },
        },
        
        ClassModel {
            id: "Rifle_Base_F".to_string(),
            parent_id: Some("CfgWeapons".to_string()),
            container_class: Some("CfgWeapons".to_string()),
            source_file_index: Some(1), // Maps to A3/Weapons_F
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("Rifle Base".to_string()));
                props.insert("scope".to_string(), PropertyValue::Number(0.0));
                props.insert("magazines".to_string(), PropertyValue::Array(vec![]));
                props
            },
        },
        
        // Child classes
        ClassModel {
            id: "arifle_MX_Base_F".to_string(),
            parent_id: Some("Rifle_Base_F".to_string()),
            container_class: Some("CfgWeapons".to_string()),
            source_file_index: Some(1), // Maps to A3/Weapons_F
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String("MX Base".to_string()));
                props.insert("magazines".to_string(), PropertyValue::Array(vec![
                    PropertyValue::String("30Rnd_65x39_caseless_mag".to_string())
                ]));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        },
    ]
} 