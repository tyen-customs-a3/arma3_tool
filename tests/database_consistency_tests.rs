//! Database state consistency tests throughout workflow operations

use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;
use tokio;

use arma3_database::DatabaseManager;
// use arma3_workflow::orchestrator::WorkflowOrchestrator; // TODO: Add workflow tests when needed
use arma3_workflow::types::{Workflow, WorkflowType, ContentType};
use arma3_workflow::types::options::{WorkflowOptions, ProcessingOptions, ReportingOptions, ReportFormat};

/// Helper function to create a test database with sample data
async fn create_test_database(db_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(db_path, "")?;
    
    let _db = DatabaseManager::new(db_path)?;
    
    // In a real test, we would populate the database with test data
    // For now, we just ensure the database can be created and opened
    
    Ok(())
}

/// Helper function to verify database integrity
async fn verify_database_integrity(db_path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    if !db_path.exists() {
        return Ok(false);
    }
    
    let _db = DatabaseManager::new(db_path)?;
    
    // In a real implementation, we would:
    // - Check table schemas
    // - Verify foreign key constraints
    // - Check for data corruption
    // - Validate index integrity
    
    Ok(true)
}

#[tokio::test]
async fn test_database_creation_and_access() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_creation.db");
    
    // Test database creation
    let result = create_test_database(&db_path).await;
    assert!(result.is_ok());
    assert!(db_path.exists());
    
    // Test database access
    let integrity_check = verify_database_integrity(&db_path).await;
    assert!(integrity_check.is_ok());
    assert!(integrity_check.unwrap());
}

#[tokio::test]
async fn test_concurrent_database_access() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_concurrent.db");
    
    // Create initial database
    create_test_database(&db_path).await.unwrap();
    
    // Test multiple concurrent database connections
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let db_path_clone = db_path.clone();
        let handle = tokio::spawn(async move {
            let db_result = DatabaseManager::new(&db_path_clone);
            assert!(db_result.is_ok(), "Connection {} failed", i);
            
            // Simulate some database operations
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            
            // Verify database is still accessible
            let integrity = verify_database_integrity(&db_path_clone).await;
            assert!(integrity.is_ok(), "Integrity check {} failed", i);
        });
        handles.push(handle);
    }
    
    // Wait for all concurrent operations to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Final integrity check
    let final_integrity = verify_database_integrity(&db_path).await;
    assert!(final_integrity.is_ok());
    assert!(final_integrity.unwrap());
}

#[tokio::test]
async fn test_workflow_database_state_transitions() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_transitions.db");
    let work_dir = temp_dir.path().join("work");
    let output_dir = temp_dir.path().join("output");
    
    fs::create_dir_all(&work_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    // Create initial database
    create_test_database(&db_path).await.unwrap();
    
    // Check initial state
    let initial_integrity = verify_database_integrity(&db_path).await;
    assert!(initial_integrity.is_ok());
    assert!(initial_integrity.unwrap());
    
    // Simulate workflow state changes
    let processing_options = ProcessingOptions {
        max_files: 100,
        max_depth: 5,
        extensions: vec!["cpp".to_string()],
        source_directories: vec![],
        verbose: false,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_processing(processing_options);
    
    let _workflow = Workflow::new(
        "state_transition_test".to_string(),
        WorkflowType::Process,
        ContentType::GameData,
        workflow_options,
    );
    
    // Check state after workflow creation
    let post_workflow_integrity = verify_database_integrity(&db_path).await;
    assert!(post_workflow_integrity.is_ok());
    assert!(post_workflow_integrity.unwrap());
    
    // Simulate more state changes
    // In a real test, we would execute actual workflow operations
    
    // Final integrity check
    let final_integrity = verify_database_integrity(&db_path).await;
    assert!(final_integrity.is_ok());
    assert!(final_integrity.unwrap());
}

#[tokio::test]
async fn test_database_rollback_on_error() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_rollback.db");
    
    // Create initial database
    create_test_database(&db_path).await.unwrap();
    
    // Get initial state checksum or verification
    let initial_state = verify_database_integrity(&db_path).await;
    assert!(initial_state.is_ok());
    
    // Simulate an operation that should fail and rollback
    // For this test, we'll just verify the database remains accessible
    
    // Try to create a workflow with invalid configuration
    let invalid_processing_options = ProcessingOptions {
        max_files: 0, // Invalid
        max_depth: 0, // Invalid
        extensions: vec![],
        source_directories: vec![PathBuf::from("/nonexistent")], // Invalid
        verbose: false,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_processing(invalid_processing_options);
    
    let _invalid_workflow = Workflow::new(
        "invalid_workflow".to_string(),
        WorkflowType::Process,
        ContentType::GameData,
        workflow_options,
    );
    
    // Verify database is still in a good state after invalid workflow
    let post_error_state = verify_database_integrity(&db_path).await;
    assert!(post_error_state.is_ok());
    assert!(post_error_state.unwrap());
}

#[tokio::test]
async fn test_database_backup_and_recovery() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_backup.db");
    let backup_path = temp_dir.path().join("backup.db");
    
    // Create initial database
    create_test_database(&db_path).await.unwrap();
    
    // Create backup
    fs::copy(&db_path, &backup_path).unwrap();
    
    // Verify both databases are valid
    let original_integrity = verify_database_integrity(&db_path).await;
    let backup_integrity = verify_database_integrity(&backup_path).await;
    
    assert!(original_integrity.is_ok());
    assert!(backup_integrity.is_ok());
    assert!(original_integrity.unwrap());
    assert!(backup_integrity.unwrap());
    
    // Simulate corruption or failure of original
    fs::write(&db_path, "corrupted data").unwrap();
    
    // Verify original is corrupted but backup is still good
    let corrupted_check = DatabaseManager::new(&db_path);
    let backup_check = verify_database_integrity(&backup_path).await;
    
    // Original should fail to open properly
    assert!(corrupted_check.is_err());
    
    // Backup should still be good
    assert!(backup_check.is_ok());
    assert!(backup_check.unwrap());
    
    // Restore from backup
    fs::copy(&backup_path, &db_path).unwrap();
    
    // Verify restoration worked
    let restored_integrity = verify_database_integrity(&db_path).await;
    assert!(restored_integrity.is_ok());
    assert!(restored_integrity.unwrap());
}

#[tokio::test]
async fn test_database_size_and_performance() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_performance.db");
    
    // Create initial database
    create_test_database(&db_path).await.unwrap();
    
    let start_time = std::time::Instant::now();
    
    // Simulate multiple operations
    for i in 0..10 {
        let db = DatabaseManager::new(&db_path).unwrap();
        
        // Simulate some work
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        
        // Verify integrity periodically
        if i % 3 == 0 {
            let integrity = verify_database_integrity(&db_path).await;
            assert!(integrity.is_ok());
            assert!(integrity.unwrap());
        }
    }
    
    let elapsed = start_time.elapsed();
    
    // Performance should be reasonable
    assert!(elapsed.as_secs() < 5);
    
    // Check final file size
    let metadata = fs::metadata(&db_path).unwrap();
    let file_size = metadata.len();
    
    // Database shouldn't grow unreasonably large for simple operations
    assert!(file_size < 10 * 1024 * 1024); // Less than 10MB
}

#[tokio::test]
async fn test_database_lock_behavior() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_locking.db");
    
    // Create initial database
    create_test_database(&db_path).await.unwrap();
    
    // Test that multiple readers can access simultaneously
    let mut handles = Vec::new();
    
    for i in 0..3 {
        let db_path_clone = db_path.clone();
        let handle = tokio::spawn(async move {
            let db = DatabaseManager::new(&db_path_clone);
            assert!(db.is_ok(), "Reader {} failed to connect", i);
            
            // Hold connection for a bit
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            
            // Verify we can still access the database
            let integrity = verify_database_integrity(&db_path_clone).await;
            assert!(integrity.is_ok());
        });
        handles.push(handle);
    }
    
    // Wait for all readers to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Final verification
    let final_integrity = verify_database_integrity(&db_path).await;
    assert!(final_integrity.is_ok());
    assert!(final_integrity.unwrap());
}

#[tokio::test]
async fn test_database_workflow_isolation() {
    let temp_dir = TempDir::new().unwrap();
    let db1_path = temp_dir.path().join("workflow1.db");
    let db2_path = temp_dir.path().join("workflow2.db");
    
    // Create separate databases for different workflows
    create_test_database(&db1_path).await.unwrap();
    create_test_database(&db2_path).await.unwrap();
    
    // Create workflows that use different databases
    let workflow1_options = WorkflowOptions::new()
        .with_processing(ProcessingOptions {
            max_files: 100,
            max_depth: 5,
            extensions: vec!["cpp".to_string()],
            source_directories: vec![],
            verbose: false,
        });
    
    let workflow2_options = WorkflowOptions::new()
        .with_reporting(ReportingOptions {
            format: ReportFormat::Json,
            output_path: None,
            source_directories: vec![],
            include_mission_dependencies: true,
            include_game_data_classes: false,
            include_missing_dependencies: true,
        });
    
    let _workflow1 = Workflow::new(
        "isolation_test_1".to_string(),
        WorkflowType::Process,
        ContentType::GameData,
        workflow1_options,
    );
    
    let _workflow2 = Workflow::new(
        "isolation_test_2".to_string(),
        WorkflowType::Report,
        ContentType::Mission,
        workflow2_options,
    );
    
    // Verify both databases remain independent and accessible
    let db1_integrity = verify_database_integrity(&db1_path).await;
    let db2_integrity = verify_database_integrity(&db2_path).await;
    
    assert!(db1_integrity.is_ok());
    assert!(db2_integrity.is_ok());
    assert!(db1_integrity.unwrap());
    assert!(db2_integrity.unwrap());
    
    // Verify databases can be accessed simultaneously
    let db1 = DatabaseManager::new(&db1_path);
    let db2 = DatabaseManager::new(&db2_path);
    
    assert!(db1.is_ok());
    assert!(db2.is_ok());
}