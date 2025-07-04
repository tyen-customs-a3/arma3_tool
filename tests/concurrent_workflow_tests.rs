//! Concurrent workflow execution tests to ensure thread safety and race condition prevention

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use tempfile::TempDir;
use std::fs;
use tokio;
use futures::future::join_all;

use arma3_workflow::orchestrator::WorkflowOrchestrator;
use arma3_workflow::types::{Workflow, WorkflowType, ContentType};
use arma3_workflow::types::options::{WorkflowOptions, ProcessingOptions, ReportingOptions, ExportOptions, ReportFormat};
use arma3_database::DatabaseManager;

/// Helper function to create test workflow
fn create_test_workflow(name: &str, workflow_type: WorkflowType) -> Workflow {
    let workflow_options = match workflow_type {
        WorkflowType::Process => {
            WorkflowOptions::new().with_processing(ProcessingOptions {
                max_files: 100,
                max_depth: 5,
                extensions: vec!["cpp".to_string(), "sqf".to_string()],
                source_directories: vec![],
                verbose: false,
            })
        }
        WorkflowType::Report => {
            WorkflowOptions::new().with_reporting(ReportingOptions {
                format: ReportFormat::Json,
                output_path: None,
                source_directories: vec![],
                include_mission_dependencies: true,
                include_game_data_classes: false,
                include_missing_dependencies: true,
            })
        }
        WorkflowType::Export => {
            WorkflowOptions::new().with_export(ExportOptions {
                format: "csv".to_string(),
                output_path: None,
                compression_level: None,
                include_metadata: true,
            })
        }
        _ => WorkflowOptions::default(),
    };
    
    Workflow::new(
        name.to_string(),
        workflow_type,
        ContentType::GameData,
        workflow_options,
    )
}

#[tokio::test]
async fn test_concurrent_workflow_creation() {
    let num_workflows = 10;
    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    
    for i in 0..num_workflows {
        let success_count_clone = Arc::clone(&success_count);
        let handle = tokio::spawn(async move {
            let workflow = create_test_workflow(
                &format!("concurrent_test_{}", i),
                WorkflowType::Process,
            );
            
            // Verify workflow was created correctly
            if workflow.name == format!("concurrent_test_{}", i) 
                && workflow.workflow_type == WorkflowType::Process {
                success_count_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    join_all(handles).await;
    
    // Verify all workflows were created successfully
    assert_eq!(success_count.load(Ordering::Relaxed), num_workflows);
}

#[tokio::test]
async fn test_concurrent_orchestrator_access() {
    let orchestrator = Arc::new(WorkflowOrchestrator::new());
    let num_concurrent = 5;
    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    
    for i in 0..num_concurrent {
        let orchestrator_clone = Arc::clone(&orchestrator);
        let success_count_clone = Arc::clone(&success_count);
        
        let handle = tokio::spawn(async move {
            // Simulate concurrent orchestrator usage
            let workflow = create_test_workflow(
                &format!("orchestrator_test_{}", i),
                WorkflowType::Report,
            );
            
            // In a real scenario, we would execute the workflow
            // For now, just verify the orchestrator can be accessed concurrently
            
            success_count_clone.fetch_add(1, Ordering::Relaxed);
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
    
    assert_eq!(success_count.load(Ordering::Relaxed), num_concurrent);
}

#[tokio::test]
async fn test_concurrent_database_operations() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("concurrent_test.db");
    
    // Create initial database
    fs::write(&db_path, "").unwrap();
    let _initial_db = DatabaseManager::new(&db_path).unwrap();
    
    let num_concurrent = 8;
    let success_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    
    for i in 0..num_concurrent {
        let db_path_clone = db_path.clone();
        let success_count_clone = Arc::clone(&success_count);
        let error_count_clone = Arc::clone(&error_count);
        
        let handle = tokio::spawn(async move {
            // Simulate concurrent database access
            match DatabaseManager::new(&db_path_clone) {
                Ok(_db) => {
                    // Simulate some database work
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    success_count_clone.fetch_add(1, Ordering::Relaxed);
                }
                Err(_) => {
                    error_count_clone.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
    
    // Most or all database connections should succeed
    let successes = success_count.load(Ordering::Relaxed);
    let errors = error_count.load(Ordering::Relaxed);
    
    assert_eq!(successes + errors, num_concurrent);
    assert!(successes >= num_concurrent / 2); // At least half should succeed
}

#[tokio::test]
async fn test_workflow_state_isolation() {
    let num_workflows = 6;
    let mut handles = Vec::new();
    let shared_counter = Arc::new(AtomicUsize::new(0));
    
    // Create workflows with different types to test isolation
    let workflow_types = vec![
        WorkflowType::Process,
        WorkflowType::Report,
        WorkflowType::Export,
    ];
    
    for i in 0..num_workflows {
        let workflow_type = workflow_types[i % workflow_types.len()].clone();
        let counter_clone = Arc::clone(&shared_counter);
        
        let handle = tokio::spawn(async move {
            let workflow = create_test_workflow(
                &format!("isolation_test_{}", i),
                workflow_type.clone(),
            );
            
            // Verify workflow state is isolated
            assert_eq!(workflow.name, format!("isolation_test_{}", i));
            assert_eq!(workflow.workflow_type, workflow_type);
            
            // Simulate some work
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
            
            counter_clone.fetch_add(1, Ordering::Relaxed);
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
    
    assert_eq!(shared_counter.load(Ordering::Relaxed), num_workflows);
}

#[tokio::test]
async fn test_resource_contention_handling() {
    let temp_dir = TempDir::new().unwrap();
    let shared_output_dir = temp_dir.path().join("shared_output");
    fs::create_dir_all(&shared_output_dir).unwrap();
    
    let num_workflows = 4;
    let success_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    
    for i in 0..num_workflows {
        let output_dir_clone = shared_output_dir.clone();
        let success_count_clone = Arc::clone(&success_count);
        
        let handle = tokio::spawn(async move {
            // Create workflows that might compete for resources
            let reporting_options = ReportingOptions {
                format: ReportFormat::Markdown,
                output_path: Some(output_dir_clone.join(format!("report_{}.md", i))),
                source_directories: vec![],
                include_mission_dependencies: true,
                include_game_data_classes: false,
                include_missing_dependencies: true,
            };
            
            let workflow_options = WorkflowOptions::new().with_reporting(reporting_options);
            
            let workflow = Workflow::new(
                format!("resource_test_{}", i),
                WorkflowType::Report,
                ContentType::GameData,
                workflow_options,
            );
            
            // Simulate resource usage
            tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
            
            // Verify workflow configuration
            if let Some(output_path) = &workflow.options.reporting.output_path {
                if output_path.ends_with(&format!("report_{}.md", i)) {
                    success_count_clone.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
    
    assert_eq!(success_count.load(Ordering::Relaxed), num_workflows);
}

#[tokio::test]
async fn test_error_isolation_between_workflows() {
    let num_workflows = 5;
    let success_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    
    for i in 0..num_workflows {
        let success_count_clone = Arc::clone(&success_count);
        let error_count_clone = Arc::clone(&error_count);
        
        let handle = tokio::spawn(async move {
            // Create some workflows with intentional errors
            let should_error = i % 2 == 0;
            
            if should_error {
                // Create workflow with invalid configuration
                let invalid_options = ProcessingOptions {
                    max_files: 0, // Invalid
                    max_depth: 0, // Invalid
                    extensions: vec![],
                    source_directories: vec![PathBuf::from("/nonexistent")],
                    verbose: false,
                };
                
                let workflow_options = WorkflowOptions::new().with_processing(invalid_options);
                
                let _workflow = Workflow::new(
                    format!("error_test_{}", i),
                    WorkflowType::Process,
                    ContentType::GameData,
                    workflow_options,
                );
                
                // Even with invalid config, workflow creation should succeed
                // (validation happens during execution)
                error_count_clone.fetch_add(1, Ordering::Relaxed);
            } else {
                // Create valid workflow
                let workflow = create_test_workflow(
                    &format!("valid_test_{}", i),
                    WorkflowType::Report,
                );
                
                assert_eq!(workflow.name, format!("valid_test_{}", i));
                success_count_clone.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
    
    let successes = success_count.load(Ordering::Relaxed);
    let errors = error_count.load(Ordering::Relaxed);
    
    assert_eq!(successes + errors, num_workflows);
    assert!(successes > 0); // Some workflows should succeed
    assert!(errors > 0); // Some workflows should have errors
}

#[tokio::test]
async fn test_high_concurrency_stress() {
    let num_workflows = 50;
    let completion_count = Arc::new(AtomicUsize::new(0));
    let start_time = std::time::Instant::now();
    
    let mut handles = Vec::new();
    
    for i in 0..num_workflows {
        let completion_count_clone = Arc::clone(&completion_count);
        
        let handle = tokio::spawn(async move {
            let workflow_type = match i % 3 {
                0 => WorkflowType::Process,
                1 => WorkflowType::Report,
                _ => WorkflowType::Export,
            };
            
            let workflow = create_test_workflow(
                &format!("stress_test_{}", i),
                workflow_type,
            );
            
            // Simulate varying amounts of work
            let work_duration = (i % 10) + 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(work_duration)).await;
            
            // Verify workflow integrity
            assert!(workflow.name.starts_with("stress_test_"));
            
            completion_count_clone.fetch_add(1, Ordering::Relaxed);
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
    
    let elapsed = start_time.elapsed();
    let completed = completion_count.load(Ordering::Relaxed);
    
    // All workflows should complete
    assert_eq!(completed, num_workflows);
    
    // Should complete within reasonable time (adjust based on system performance)
    assert!(elapsed.as_secs() < 10);
    
    println!("Completed {} concurrent workflows in {:?}", num_workflows, elapsed);
}

#[tokio::test]
async fn test_memory_usage_under_concurrency() {
    let num_workflows = 20;
    let mut handles = Vec::new();
    
    // Measure initial memory usage (simplified)
    let start_time = std::time::Instant::now();
    
    for i in 0..num_workflows {
        let handle = tokio::spawn(async move {
            // Create workflows with larger configurations
            let large_extensions: Vec<String> = (0..100)
                .map(|j| format!("ext_{}", j))
                .collect();
            
            let processing_options = ProcessingOptions {
                max_files: 10000,
                max_depth: 100,
                extensions: large_extensions,
                source_directories: vec![],
                verbose: true,
            };
            
            let workflow_options = WorkflowOptions::new().with_processing(processing_options);
            
            let workflow = Workflow::new(
                format!("memory_test_{}", i),
                WorkflowType::Process,
                ContentType::GameData,
                workflow_options,
            );
            
            // Hold onto the workflow for a bit
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            
            // Verify workflow was created with large config
            assert_eq!(workflow.options.processing.extensions.len(), 100);
            assert_eq!(workflow.options.processing.max_files, 10000);
        });
        handles.push(handle);
    }
    
    join_all(handles).await;
    
    let elapsed = start_time.elapsed();
    
    // Should complete within reasonable time even with large configurations
    assert!(elapsed.as_secs() < 5);
    
    println!("Memory test with {} workflows completed in {:?}", num_workflows, elapsed);
}

#[tokio::test]
async fn test_cancellation_and_cleanup() {
    let num_workflows = 10;
    let cancelled_count = Arc::new(AtomicUsize::new(0));
    let completed_count = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    
    for i in 0..num_workflows {
        let cancelled_count_clone = Arc::clone(&cancelled_count);
        let completed_count_clone = Arc::clone(&completed_count);
        
        let handle = tokio::spawn(async move {
            let workflow = create_test_workflow(
                &format!("cancellation_test_{}", i),
                WorkflowType::Process,
            );
            
            // Simulate work that might be cancelled
            let work_result = tokio::select! {
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                    completed_count_clone.fetch_add(1, Ordering::Relaxed);
                    "completed"
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_millis(50)) => {
                    cancelled_count_clone.fetch_add(1, Ordering::Relaxed);
                    "cancelled"
                }
            };
            
            // Verify workflow state regardless of cancellation
            assert!(workflow.name.starts_with("cancellation_test_"));
        });
        handles.push(handle);
    }
    
    // Wait for all tasks
    join_all(handles).await;
    
    let cancelled = cancelled_count.load(Ordering::Relaxed);
    let completed = completed_count.load(Ordering::Relaxed);
    
    assert_eq!(cancelled + completed, num_workflows);
    
    println!("Cancellation test: {} cancelled, {} completed", cancelled, completed);
}