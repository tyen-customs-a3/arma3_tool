use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tempfile::tempdir;
use tokio;

use arma3_tool::cli::{run_export, run_export_optimized};
use arma3_config::{ItemFilterConfig, ItemTypeConfig, ExclusionRules};
use arma3_database::{DatabaseManager, ClassRepository, ClassModel};
use arma3_models::types::PropertyValue;

/// Create a test dataset for comparison
fn create_comparison_test_dataset(size: usize) -> Vec<ClassModel> {
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
    
    // Generate child classes
    let items_per_base = size / base_classes.len();
    
    for (i, (base_name, parent)) in base_classes.iter().enumerate() {
        for j in 0..items_per_base {
            let class_id = format!("{}_{:04}", base_name.replace("_Base_F", "").replace("_Base", ""), j);
            
            classes.push(ClassModel {
                id: class_id.clone(),
                parent_id: Some(base_name.to_string()),
                container_class: Some(parent.to_string()),
                source_file_index: Some((i % 5) + 1),
                is_forward_declaration: false,
                properties: {
                    let mut props = HashMap::new();
                    props.insert("displayName".to_string(), PropertyValue::String(format!("Test Item {}", j)));
                    props.insert("scope".to_string(), PropertyValue::Number(2.0));
                    props
                },
            });
        }
    }
    
    // Add some noise classes
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

/// Setup a test database
async fn setup_comparison_database(db_path: &PathBuf, size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let db = DatabaseManager::new(db_path)?;
    let class_repo = ClassRepository::new(&db);
    
    let test_classes = create_comparison_test_dataset(size);
    
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
    
    Ok(())
}

/// Create test configuration
fn create_comparison_config() -> ItemFilterConfig {
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

/// Compare performance of original vs optimized export
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let sizes = vec![100, 500, 1000];
    
    for size in sizes {
        println!("\n=== Comparing performance with {} classes ===", size);
        
        // Setup for original export
        let temp_dir_orig = tempdir()?;
        let db_path_orig = temp_dir_orig.path().join("comparison_orig.db");
        let config_path_orig = temp_dir_orig.path().join("comparison_config_orig.json");
        let output_path_orig = temp_dir_orig.path().join("comparison_output_orig.csv");
        
        // Setup for optimized export  
        let temp_dir_opt = tempdir()?;
        let db_path_opt = temp_dir_opt.path().join("comparison_opt.db");
        let config_path_opt = temp_dir_opt.path().join("comparison_config_opt.json");
        let output_path_opt = temp_dir_opt.path().join("comparison_output_opt.csv");
        
        // Setup databases (identical data)
        setup_comparison_database(&db_path_orig, size).await?;
        setup_comparison_database(&db_path_opt, size).await?;
        
        // Create configurations (identical)
        let config = create_comparison_config();
        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(&config_path_orig, &config_json)?;
        fs::write(&config_path_opt, &config_json)?;
        
        // Benchmark original export
        println!("Running original export...");
        let start_orig = Instant::now();
        
        run_export(
            db_path_orig.clone(),
            output_path_orig.clone(),
            None,
            Some(config_path_orig.clone()),
            0,
        ).await?;
        
        let duration_orig = start_orig.elapsed();
        
        // Analyze original results
        let csv_content_orig = fs::read_to_string(&output_path_orig)?;
        let line_count_orig = csv_content_orig.lines().count() - 1;
        
        // Benchmark optimized export
        println!("Running optimized export...");
        let start_opt = Instant::now();
        
        run_export_optimized(
            db_path_opt.clone(),
            output_path_opt.clone(),
            None,
            Some(config_path_opt.clone()),
            0,
        ).await?;
        
        let duration_opt = start_opt.elapsed();
        
        // Analyze optimized results
        let csv_content_opt = fs::read_to_string(&output_path_opt)?;
        let line_count_opt = csv_content_opt.lines().count() - 1;
        
        // Print comparison
        println!("\nOriginal Export:");
        println!("  Time: {:?}", duration_orig);
        println!("  Exported items: {}", line_count_orig);
        println!("  Rate: {:.2} items/second", line_count_orig as f64 / duration_orig.as_secs_f64());
        
        println!("\nOptimized Export:");
        println!("  Time: {:?}", duration_opt);
        println!("  Exported items: {}", line_count_opt);
        println!("  Rate: {:.2} items/second", line_count_opt as f64 / duration_opt.as_secs_f64());
        
        // Calculate speedup
        let speedup = duration_orig.as_secs_f64() / duration_opt.as_secs_f64();
        println!("\nPerformance Improvement:");
        println!("  Speedup: {:.2}x", speedup);
        println!("  Time reduction: {:.1}%", (1.0 - (duration_opt.as_secs_f64() / duration_orig.as_secs_f64())) * 100.0);
        
        // Verify results are identical
        assert_eq!(line_count_orig, line_count_opt, "Both exports should produce the same number of results");
        
        // Basic content verification (both should have same classes, order might differ)
        let orig_lines: std::collections::HashSet<_> = csv_content_orig.lines().skip(1).collect();
        let opt_lines: std::collections::HashSet<_> = csv_content_opt.lines().skip(1).collect();
        assert_eq!(orig_lines, opt_lines, "Both exports should produce the same content");
        
        println!("✅ Results verified - both exports produce identical content");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_comparison_small_dataset() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("small_comparison.db");
        
        // Setup small database
        setup_comparison_database(&db_path, 50).await?;
        
        // Should complete without errors
        Ok(())
    }
}