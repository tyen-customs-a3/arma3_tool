#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::fs;
    use anyhow::Result;
    
    use crate::code_scanner::class::{
        ClassProcessor,
        types::{ClassScanOptions, ProcessedClass},
    };
    use crate::code_scanner::database::{
        ClassDatabase,
        DatabaseOperations,
        QueryOptions,
    };
    
    // Helper function to create a temporary test directory
    fn create_test_dir() -> Result<PathBuf> {
        let test_dir = std::env::temp_dir().join("arma3_tool_tests").join(format!("test_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
        fs::create_dir_all(&test_dir)?;
        Ok(test_dir)
    }
    
    // Helper function to create a test class file
    fn create_test_class_file(dir: &Path, name: &str, content: &str) -> Result<PathBuf> {
        let file_path = dir.join(name);
        fs::write(&file_path, content)?;
        Ok(file_path)
    }
    
    #[test]
    fn test_scan_single_class() -> Result<()> {
        // Create a temporary test directory
        let test_dir = create_test_dir()?;
        let output_dir = test_dir.join("output");
        
        // Create a test class file
        let class_content = r#"
        class TestVehicle {
            scope = 2;
            displayName = "Test Vehicle";
            model = "\A3\Test\TestVehicle.p3d";
            
            class Turrets {
                class MainTurret {
                    gunnerAction = "gunner_stance";
                    weapons[] = {"HMG_127"};
                    magazines[] = {"100Rnd_127x99_mag", "100Rnd_127x99_mag"};
                };
            };
        };
        "#;
        
        let class_file = create_test_class_file(&test_dir, "test_vehicle.hpp", class_content)?;
        
        // Create a processor with default options
        let processor = ClassProcessor::with_defaults(&output_dir);
        
        // Process the file
        let result = processor.scan_specific_files(&[class_file])?;
        
        // Verify the results
        assert_eq!(result.stats.total_files, 1);
        assert_eq!(result.stats.files_with_classes, 1);
        assert_eq!(result.stats.total_classes, 2); // TestVehicle and MainTurret
        
        // Verify the main class
        let main_class = result.classes.iter().find(|c| c.name == "TestVehicle").expect("Main class not found");
        assert_eq!(main_class.parent, None);
        
        // Verify properties
        let scope_prop = main_class.properties.iter().find(|(name, _)| name == "scope").expect("scope property not found");
        assert_eq!(scope_prop.1, "2");
        
        let display_name_prop = main_class.properties.iter().find(|(name, _)| name == "displayName").expect("displayName property not found");
        assert_eq!(display_name_prop.1, "\"Test Vehicle\"");
        
        // Verify nested class
        let turret_class = result.classes.iter().find(|c| c.name == "MainTurret").expect("Turret class not found");
        
        // Verify array property
        let weapons_prop = turret_class.properties.iter().find(|(name, _)| name == "weapons").expect("weapons property not found");
        assert_eq!(weapons_prop.1, "[\"HMG_127\"]");
        
        Ok(())
    }
    
    #[test]
    fn test_scan_multiple_classes() -> Result<()> {
        // Create a temporary test directory
        let test_dir = create_test_dir()?;
        let output_dir = test_dir.join("output");
        
        // Create test class files
        let class1_content = r#"
        class Soldier_Base_F {
            scope = 1;
            displayName = "Soldier Base";
            
            class Inventory {
                uniform = "U_B_CombatUniform_mcam";
                vest = "V_PlateCarrier1_rgr";
                backpack = "B_AssaultPack_mcamo";
            };
        };
        "#;
        
        let class2_content = r#"
        class B_Soldier_F: Soldier_Base_F {
            scope = 2;
            displayName = "Rifleman";
            weapons[] = {"arifle_MX_F", "hgun_P07_F"};
            respawnWeapons[] = {"arifle_MX_F", "hgun_P07_F"};
        };
        "#;
        
        let class1_file = create_test_class_file(&test_dir, "soldier_base.hpp", class1_content)?;
        let class2_file = create_test_class_file(&test_dir, "b_soldier.hpp", class2_content)?;
        
        // Create a processor with default options
        let processor = ClassProcessor::with_defaults(&output_dir);
        
        // Process the files
        let result = processor.scan_specific_files(&[class1_file, class2_file])?;
        
        // Verify the results
        assert_eq!(result.stats.total_files, 2);
        assert_eq!(result.stats.files_with_classes, 2);
        assert_eq!(result.stats.total_classes, 3); // Soldier_Base_F, Inventory, and B_Soldier_F
        
        // Verify inheritance
        let derived_class = result.classes.iter().find(|c| c.name == "B_Soldier_F").expect("Derived class not found");
        assert_eq!(derived_class.parent, Some("Soldier_Base_F".to_string()));
        
        Ok(())
    }
    
    #[test]
    fn test_database_operations() -> Result<()> {
        // Create a temporary test directory
        let test_dir = create_test_dir()?;
        let output_dir = test_dir.join("output");
        let db_path = test_dir.join("class_db.json");
        
        // Create test class files
        let class1_content = r#"
        class Vehicle_Base_F {
            scope = 1;
            displayName = "Vehicle Base";
        };
        
        class Car_F: Vehicle_Base_F {
            scope = 1;
            displayName = "Car Base";
        };
        "#;
        
        let class2_content = r#"
        class B_MRAP_01_F: Car_F {
            scope = 2;
            displayName = "Hunter";
            crew = "B_Soldier_F";
            faction = "BLU_F";
        };
        "#;
        
        let class1_file = create_test_class_file(&test_dir, "vehicle_base.hpp", class1_content)?;
        let class2_file = create_test_class_file(&test_dir, "b_mrap.hpp", class2_content)?;
        
        // Create a processor with default options
        let processor = ClassProcessor::with_defaults(&output_dir);
        
        // Process the files
        let result = processor.scan_specific_files(&[class1_file, class2_file])?;
        
        // Create a database operations instance
        let mut db_ops = DatabaseOperations::new(&db_path)?;
        
        // Update the database with scan results
        let update_stats = db_ops.update_with_scan_results(result)?;
        
        // Verify update stats
        assert_eq!(update_stats.added_classes, 3); // Vehicle_Base_F, Car_F, and B_MRAP_01_F
        assert_eq!(update_stats.total_classes, 3);
        
        // Query the database
        let query_results = db_ops.query(&QueryOptions {
            parent: Some("Car_F".to_string()),
            ..Default::default()
        });
        
        // Verify query results
        assert_eq!(query_results.len(), 1);
        assert_eq!(query_results[0].class.name, "B_MRAP_01_F");
        
        // Save the database
        db_ops.save()?;
        
        // Verify the database file exists
        assert!(db_path.exists());
        
        Ok(())
    }
} 