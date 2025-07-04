use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tempfile::tempdir;
use tokio;

use arma3_tool::cli::run_export;
use arma3_config::{ItemFilterConfig, ItemTypeConfig, ExclusionRules};
use arma3_database::{DatabaseManager, ClassRepository, ClassModel};
use arma3_models::types::PropertyValue;

/// Create a large test dataset for performance testing
fn create_large_test_dataset(size: usize) -> Vec<ClassModel> {
    let mut classes = Vec::new();
    
    // Add root classes
    classes.push(ClassModel {
        id: "CfgWeapons".to_string(),
        parent_id: None,
        container_class: None,
        source_file_index: None,
        is_forward_declaration: false,
        properties: HashMap::new(),
    });
    
    classes.push(ClassModel {
        id: "CfgVehicles".to_string(),
        parent_id: None,
        container_class: None,
        source_file_index: None,
        is_forward_declaration: false,
        properties: HashMap::new(),
    });
    
    // Add base classes
    let base_classes = vec![
        ("Rifle_Base_F", "CfgWeapons"),
        ("Pistol_Base_F", "CfgWeapons"),
        ("Launcher_Base_F", "CfgWeapons"),
        ("Uniform_Base", "CfgVehicles"),
        ("Vest_Base", "CfgVehicles"),
        ("Backpack_Base", "CfgVehicles"),
    ];
    
    for (base_name, parent) in &base_classes {
        classes.push(ClassModel {
            id: base_name.to_string(),
            parent_id: Some(parent.to_string()),
            container_class: Some(parent.to_string()),
            source_file_index: Some(1),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String(format!("{} Base", base_name)));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        });
    }
    
    // Generate a large number of child classes
    let items_per_base = size / base_classes.len();
    
    for (i, (base_name, parent)) in base_classes.iter().enumerate() {
        for j in 0..items_per_base {
            let class_id = format!("{}_{:04}", base_name.replace("_Base_F", "").replace("_Base", ""), j);
            
            classes.push(ClassModel {
                id: class_id.clone(),
                parent_id: Some(base_name.to_string()),
                container_class: Some(parent.to_string()),
                source_file_index: Some((i % 5) + 1), // Rotate through different source files
                is_forward_declaration: false,
                properties: {
                    let mut props = HashMap::new();
                    props.insert("displayName".to_string(), PropertyValue::String(format!("Test Item {}", j)));
                    props.insert("scope".to_string(), PropertyValue::Number(2.0));
                    if i < 3 { // Only add magazines to weapons
                        props.insert("magazines".to_string(), PropertyValue::Array(vec![
                            PropertyValue::String("30Rnd_mag".to_string())
                        ]));
                    }
                    props
                },
            });
        }
    }
    
    // Add some noise classes that shouldn't match
    for k in 0..(size / 10) {
        classes.push(ClassModel {
            id: format!("NoiseClass_{:04}", k),
            parent_id: Some("CfgVehicles".to_string()),
            container_class: Some("CfgVehicles".to_string()),
            source_file_index: Some(10),
            is_forward_declaration: false,
            properties: {
                let mut props = HashMap::new();
                props.insert("displayName".to_string(), PropertyValue::String(format!("Noise {}", k)));
                props.insert("scope".to_string(), PropertyValue::Number(2.0));
                props
            },
        });
    }
    
    classes
}

/// Setup a large test database
async fn setup_large_test_database(db_path: &PathBuf, size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let db = DatabaseManager::new(db_path)?;
    let class_repo = ClassRepository::new(&db);
    
    let start = Instant::now();
    println!("Creating {} test classes...", size);
    
    let test_classes = create_large_test_dataset(size);
    
    for class in &test_classes {
        class_repo.create(class)?;
    }
    
    // Setup file index mapping
    db.with_connection(|conn| {
        conn.execute("PRAGMA foreign_keys = OFF", [])?;
        
        for i in 1..=10 {
            let file_path = format!("A3/Test_{}/config.cpp", i);
            let normalized_path = format!("a3/test_{}/config.cpp", i);
            let pbo_id = format!("A3/Test_{}", i);
            
            conn.execute(
                "INSERT INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
                 VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![i, file_path, normalized_path, pbo_id],
            )?;
        }
        
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(())
    })?;
    
    let duration = start.elapsed();
    println!("Database setup completed in {:?} with {} classes", duration, test_classes.len());
    
    Ok(())
}

/// Create test configuration
fn create_benchmark_config() -> ItemFilterConfig {
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

/// Benchmark export with different database sizes
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let sizes = vec![100, 1000, 5000, 10000];
    
    for size in sizes {
        println!("\n=== Benchmarking with {} classes ===", size);
        
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("benchmark.db");
        let config_path = temp_dir.path().join("benchmark_config.json");
        let output_path = temp_dir.path().join("benchmark_output.csv");
        
        // Setup database
        setup_large_test_database(&db_path, size).await?;
        
        // Create configuration
        let config = create_benchmark_config();
        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(&config_path, config_json)?;
        
        // Benchmark export
        let start = Instant::now();
        
        run_export(
            db_path.clone(),
            output_path.clone(),
            None, // Export all types
            Some(config_path.clone()),
            0, // No limit
        ).await?;
        
        let duration = start.elapsed();
        
        // Analyze results
        let csv_content = fs::read_to_string(&output_path)?;
        let line_count = csv_content.lines().count() - 1; // Subtract header
        
        println!("Export completed:");
        println!("  Time: {:?}", duration);
        println!("  Exported items: {}", line_count);
        println!("  Rate: {:.2} items/second", line_count as f64 / duration.as_secs_f64());
        println!("  Time per item: {:.2} ms", duration.as_millis() as f64 / line_count as f64);
        
        // Memory usage estimation
        let db_size = fs::metadata(&db_path)?.len();
        let output_size = fs::metadata(&output_path)?.len();
        println!("  Database size: {} KB", db_size / 1024);
        println!("  Output size: {} KB", output_size / 1024);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_benchmark_small_dataset() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("small_benchmark.db");
        let config_path = temp_dir.path().join("small_config.json");
        let output_path = temp_dir.path().join("small_output.csv");
        
        // Setup small database
        setup_large_test_database(&db_path, 100).await?;
        
        // Create configuration
        let config = create_benchmark_config();
        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(&config_path, config_json)?;
        
        // Run export and measure time
        let start = Instant::now();
        
        run_export(
            db_path,
            output_path.clone(),
            None,
            Some(config_path),
            0,
        ).await?;
        
        let duration = start.elapsed();
        
        // Should complete reasonably quickly
        assert!(duration.as_secs() < 10, "Export took too long: {:?}", duration);
        
        // Should have produced output
        assert!(output_path.exists());
        let csv_content = fs::read_to_string(&output_path)?;
        let line_count = csv_content.lines().count();
        assert!(line_count > 1, "Should have header + data lines");
        
        println!("Small benchmark completed in {:?} with {} lines", duration, line_count);
        
        Ok(())
    }
    
    #[test]
    fn test_large_dataset_creation() {
        let dataset = create_large_test_dataset(1000);
        
        // Should have root classes + base classes + generated classes + noise
        assert!(dataset.len() >= 1000, "Dataset should have at least requested size");
        
        // Should have the expected base classes
        assert!(dataset.iter().any(|c| c.id == "Rifle_Base_F"));
        assert!(dataset.iter().any(|c| c.id == "Pistol_Base_F"));
        assert!(dataset.iter().any(|c| c.id == "Uniform_Base"));
        
        // Should have generated items
        assert!(dataset.iter().any(|c| c.id.starts_with("Rifle_")));
        assert!(dataset.iter().any(|c| c.id.starts_with("Pistol_")));
        
        // Should have noise classes
        assert!(dataset.iter().any(|c| c.id.starts_with("NoiseClass_")));
    }
}