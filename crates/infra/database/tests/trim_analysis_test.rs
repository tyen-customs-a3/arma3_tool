#[cfg(feature = "trim_analysis")]
mod trim_analysis_tests {
    use std::path::PathBuf;
    use arma3_database::{DatabaseManager, ClassRepository, GraphQueryEngine};
    use arma3_database::models::class::ClassModel;
    use arma3_database::analysis::trim_analysis::analyze_pbo_impact;
    use tempfile::tempdir;

    /// Sets up a test database with a sample class hierarchy and PBO mappings
    fn setup_test_database() -> (tempfile::TempDir, PathBuf, DatabaseManager) {
        // Create a temporary directory for our test
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let db_path = temp_dir.path().join("test.db");
        
        // Create a new database
        let db = DatabaseManager::new(&db_path).expect("Failed to create database");
        let class_repo = ClassRepository::new(&db);
        
        // Add test classes with explicit type annotations
        let test_classes = vec![
            ClassModel::new("Root", None::<String>, None::<String>, None),
            ClassModel::new("Parent1", Some("Root".to_string()), None::<String>, Some(1)),
            ClassModel::new("Child1", Some("Parent1".to_string()), None::<String>, Some(1)),
            ClassModel::new("Child1_1", Some("Child1".to_string()), None::<String>, Some(1)),
            ClassModel::new("Parent2", Some("Root".to_string()), None::<String>, Some(2)),
            ClassModel::new("Child2", Some("Parent2".to_string()), None::<String>, Some(2)),
        ];
        
        for class in &test_classes {
            class_repo.create(class).expect("Failed to create class");
        }
        
        // Create file index mapping with foreign keys disabled
        db.with_connection(|conn| {
            // Disable foreign key constraints for test setup
            conn.execute("PRAGMA foreign_keys = OFF", [])?;
            
            conn.execute(
                "INSERT INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
                VALUES (1, 'PBO1/config.cpp', 'pbo1/config.cpp', 'PBO1')",
                [],
            )?;
            
            conn.execute(
                "INSERT INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
                VALUES (2, 'PBO2/config.cpp', 'pbo2/config.cpp', 'PBO2')",
                [],
            )?;
            
            // Re-enable foreign key constraints
            conn.execute("PRAGMA foreign_keys = ON", [])?;
            
            Ok(())
        }).expect("Failed to create file mapping");
        
        (temp_dir, db_path, db)
    }
    
    /// Tests the class impact analysis functionality
    /// Verifies that removing a parent class correctly identifies orphaned child classes
    #[test]
    fn test_class_impact_analysis() {
        // Setup test database
        let (_, _, db) = setup_test_database();
        
        // Create GraphQueryEngine
        let engine = GraphQueryEngine::new(&db);
        
        // Classes to remove
        let classes_to_remove = vec!["Parent1".to_string()];
        
        // Run impact analysis
        let impact = engine.impact_analysis(&classes_to_remove)
            .expect("Failed to run impact analysis");
        
        // Verify results
        assert_eq!(impact.removed_classes.len(), 1);
        assert_eq!(impact.orphaned_classes.len(), 1);
        assert!(impact.orphaned_classes.contains(&"Child1".to_string()));
    }
    
    /// Tests the PBO impact analysis functionality
    /// Verifies that removing classes correctly identifies PBOs that would become empty
    #[test]
    fn test_pbo_impact() {
        // Setup test database
        let (_, _, db) = setup_test_database();
        
        // Print out class-file mappings for debugging
        db.with_connection(|conn| {
            let query = "
                SELECT c.id, fim.pbo_id, c.source_file_index
                FROM classes c
                JOIN file_index_mapping fim ON c.source_file_index = fim.file_index
            ";
            
            let mut stmt = conn.prepare(query).unwrap();
            let rows = stmt.query_map([], |row| {
                Ok((
                    row.get::<_, String>(0).unwrap(), // class_id
                    row.get::<_, String>(1).unwrap(), // pbo_id
                    row.get::<_, i64>(2).unwrap(),    // source_file_index
                ))
            }).unwrap();
            
            println!("Class to PBO mappings:");
            for row in rows {
                let (class_id, pbo_id, source_idx) = row.unwrap();
                println!("  Class: {}, PBO: {}, Index: {}", class_id, pbo_id, source_idx);
            }
            
            Ok(())
        }).expect("Failed to query class mappings");
        
        // Classes to remove and their orphans
        let classes_to_remove = vec!["Parent1".to_string()];
        let orphaned_classes = vec!["Child1".to_string(), "Child1_1".to_string()];
        
        // Use the analyze_pbo_impact function from the analysis module
        let empty_pbos = analyze_pbo_impact(&db, &classes_to_remove, &orphaned_classes)
            .expect("Failed to analyze PBO impact");
        
        println!("Empty PBOs: {:?}", empty_pbos);
        
        // Verify results - PBO1 should be empty since all its classes would be removed
        assert_eq!(empty_pbos.len(), 1);
        assert!(empty_pbos.contains(&"PBO1".to_string()));
    }
} 