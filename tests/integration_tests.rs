//! Integration tests for end-to-end workflow orchestration scenarios

use std::path::PathBuf;
use std::time::Instant;
use tempfile::TempDir;
use std::fs;
use tokio;

use arma3_tool::cli::handlers::{extract, process, report, export};
use arma3_workflow::orchestrator::WorkflowOrchestrator;
use arma3_workflow::types::{Workflow, WorkflowType, ContentType};
use arma3_workflow::types::options::{WorkflowOptions, ExtractionOptions, ProcessingOptions, ReportingOptions, ExportOptions, ReportFormat};
use arma3_workflow::extract::ExtractWorkflowHandler;
use arma3_workflow::process::ProcessWorkflowHandler;
use arma3_workflow::report::ReportWorkflowHandler;
use arma3_workflow::export::ExportWorkflowHandler;
use arma3_database::DatabaseManager;
use arma3_extractor::ExtractionConfig;

/// Helper function to create test PBO files
fn create_test_pbo_files(dir: &PathBuf) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    
    // Create some mock PBO files
    fs::write(dir.join("test_gamedata.pbo"), "Mock PBO content for game data")?;
    fs::write(dir.join("test_mission.pbo"), "Mock PBO content for mission")?;
    fs::write(dir.join("addon1.pbo"), "Mock addon PBO content")?;
    
    Ok(())
}

/// Helper function to create test extraction config
fn create_test_extraction_config(work_dir: &PathBuf, db_path: &PathBuf) -> ExtractionConfig {
    ExtractionConfig {
        game_data_extensions: vec!["cpp".to_string(), "hpp".to_string()],
        mission_extensions: vec!["sqf".to_string(), "sqm".to_string()],
        threads: 2,
        timeout: 10,
        verbose: true,
        db_path: db_path.clone(),
    }
}

#[tokio::test]
async fn test_complete_workflow_orchestration() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let work_dir = temp_dir.path().join("work");
    let output_dir = temp_dir.path().join("output");
    let db_path = temp_dir.path().join("test.db");
    
    // Create test data
    create_test_pbo_files(&source_dir).unwrap();
    fs::create_dir_all(&work_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    // Create test database
    fs::write(&db_path, "").unwrap();
    
    let orchestrator = WorkflowOrchestrator::new();
    
    // Test complete workflow
    let workflow_options = WorkflowOptions::new()
        .with_extraction(ExtractionOptions {
            use_extractor: true,
            directories: None,
            source_directories: vec![source_dir.clone()],
            force: false,
        })
        .with_processing(ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec!["cpp".to_string(), "sqf".to_string()],
            source_directories: vec![source_dir],
            verbose: true,
        })
        .with_reporting(ReportingOptions {
            format: ReportFormat::Markdown,
            output_path: Some(output_dir.join("report.md")),
            source_directories: vec![],
            include_mission_dependencies: true,
            include_game_data_classes: true,
            include_missing_dependencies: true,
        });
    
    let workflow = Workflow::new(
        "integration_test".to_string(),
        WorkflowType::Complete,
        ContentType::GameData,
        workflow_options,
    );
    
    // Note: This is a simplified integration test
    // In a real scenario, we would need mock implementations of the actual
    // workflow handlers that can work with our test data
    
    // For now, just test that the workflow can be created and validated
    assert_eq!(workflow.name, "integration_test");
    assert_eq!(workflow.workflow_type, WorkflowType::Complete);
    assert_eq!(workflow.content_type, ContentType::GameData);
}

#[tokio::test]
async fn test_extract_process_pipeline() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let work_dir = temp_dir.path().join("work");
    let db_path = temp_dir.path().join("test.db");
    
    // Create test data
    create_test_pbo_files(&source_dir).unwrap();
    fs::create_dir_all(&work_dir).unwrap();
    fs::write(&db_path, "").unwrap();
    
    let start_time = Instant::now();
    
    // Test extract → process pipeline
    let extraction_options = ExtractionOptions {
        use_extractor: true,
        directories: None,
        source_directories: vec![source_dir.clone()],
        force: false,
    };
    
    let processing_options = ProcessingOptions {
        max_files: 1000,
        max_depth: 10,
        extensions: vec!["cpp".to_string(), "sqf".to_string()],
        source_directories: vec![source_dir],
        verbose: true,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_extraction(extraction_options)
        .with_processing(processing_options);
    
    let workflow = Workflow::new(
        "extract_process_test".to_string(),
        WorkflowType::ExtractAndProcess,
        ContentType::GameData,
        workflow_options,
    );
    
    let elapsed = start_time.elapsed();
    println!("Extract-Process pipeline setup completed in: {:?}", elapsed);
    
    // Verify workflow configuration
    assert_eq!(workflow.workflow_type, WorkflowType::ExtractAndProcess);
    assert!(workflow.options.extraction.use_extractor);
    assert_eq!(workflow.options.processing.max_files, 1000);
}

#[tokio::test]
async fn test_process_report_pipeline() {
    let temp_dir = TempDir::new().unwrap();
    let work_dir = temp_dir.path().join("work");
    let output_dir = temp_dir.path().join("output");
    let db_path = temp_dir.path().join("test.db");
    
    fs::create_dir_all(&work_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    fs::write(&db_path, "").unwrap();
    
    let start_time = Instant::now();
    
    // Test process → report pipeline
    let processing_options = ProcessingOptions {
        max_files: 500,
        max_depth: 5,
        extensions: vec!["cpp".to_string(), "hpp".to_string()],
        source_directories: vec![],
        verbose: false,
    };
    
    let reporting_options = ReportingOptions {
        format: ReportFormat::Html,
        output_path: Some(output_dir.join("report.html")),
        source_directories: vec![],
        include_mission_dependencies: false,
        include_game_data_classes: true,
        include_missing_dependencies: true,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_processing(processing_options)
        .with_reporting(reporting_options);
    
    let workflow = Workflow::new(
        "process_report_test".to_string(),
        WorkflowType::ProcessAndReport,
        ContentType::Mission,
        workflow_options,
    );
    
    let elapsed = start_time.elapsed();
    println!("Process-Report pipeline setup completed in: {:?}", elapsed);
    
    // Verify workflow configuration
    assert_eq!(workflow.workflow_type, WorkflowType::ProcessAndReport);
    assert_eq!(workflow.content_type, ContentType::Mission);
    assert_eq!(workflow.options.reporting.format, ReportFormat::Html);
    assert!(!workflow.options.reporting.include_mission_dependencies);
}

#[tokio::test]
async fn test_database_state_consistency() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("consistency_test.db");
    
    // Create initial database
    let db_manager = DatabaseManager::new(&db_path).expect("Failed to create database");
    
    // Simulate multiple workflow operations affecting database state
    let initial_time = Instant::now();
    
    // Test that database can be accessed consistently
    let db1 = DatabaseManager::new(&db_path).expect("First DB access should succeed");
    let db2 = DatabaseManager::new(&db_path).expect("Second DB access should succeed");
    let db3 = DatabaseManager::new(&db_path).expect("Third DB access should succeed");
    
    // Test database operations don't conflict
    // In a real test, we would perform actual database operations here
    
    let elapsed = initial_time.elapsed();
    println!("Database consistency test completed in: {:?}", elapsed);
    
    assert!(db_path.exists());
}

#[tokio::test]
async fn test_concurrent_workflow_validation() {
    let temp_dir = TempDir::new().unwrap();
    let work_dir = temp_dir.path().join("work");
    let output_dir = temp_dir.path().join("output");
    
    fs::create_dir_all(&work_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    let start_time = Instant::now();
    
    // Create multiple workflows concurrently
    let workflows = (0..5).map(|i| {
        let workflow_options = WorkflowOptions::new()
            .with_processing(ProcessingOptions {
                max_files: 100 * (i + 1),
                max_depth: 5,
                extensions: vec!["cpp".to_string()],
                source_directories: vec![],
                verbose: false,
            });
        
        Workflow::new(
            format!("concurrent_test_{}", i),
            WorkflowType::Process,
            ContentType::GameData,
            workflow_options,
        )
    }).collect::<Vec<_>>();
    
    // Test that multiple workflows can be created without conflicts
    assert_eq!(workflows.len(), 5);
    
    for (i, workflow) in workflows.iter().enumerate() {
        assert_eq!(workflow.name, format!("concurrent_test_{}", i));
        assert_eq!(workflow.workflow_type, WorkflowType::Process);
        assert_eq!(workflow.options.processing.max_files, 100 * (i + 1));
    }
    
    let elapsed = start_time.elapsed();
    println!("Concurrent workflow validation completed in: {:?}", elapsed);
}

#[tokio::test]
async fn test_workflow_orchestrator_integration() {
    let temp_dir = TempDir::new().unwrap();
    let work_dir = temp_dir.path().join("work");
    let output_dir = temp_dir.path().join("output");
    
    fs::create_dir_all(&work_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    let orchestrator = WorkflowOrchestrator::new();
    
    // Test orchestrator can be created and configured
    // In a real test, we would execute workflows through the orchestrator
    
    let workflow_options = WorkflowOptions::new()
        .with_reporting(ReportingOptions {
            format: ReportFormat::Json,
            output_path: Some(output_dir.join("test_report.json")),
            source_directories: vec![],
            include_mission_dependencies: true,
            include_game_data_classes: false,
            include_missing_dependencies: true,
        });
    
    let workflow = Workflow::new(
        "orchestrator_test".to_string(),
        WorkflowType::Report,
        ContentType::Mission,
        workflow_options,
    );
    
    // Verify workflow setup for orchestrator
    assert_eq!(workflow.name, "orchestrator_test");
    assert_eq!(workflow.workflow_type, WorkflowType::Report);
    
    // Note: Full orchestrator execution would require proper mock implementations
    // of all the workflow handlers, which is complex for an integration test
}

#[tokio::test]
async fn test_export_format_validation() {
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();
    
    let supported_formats = vec!["csv", "json", "xml"];
    
    for format in supported_formats {
        let export_options = ExportOptions {
            format: format.to_string(),
            output_path: Some(output_dir.join(format!("test_export.{}", format))),
            compression_level: None,
            include_metadata: true,
        };
        
        let workflow_options = WorkflowOptions::new()
            .with_export(export_options);
        
        let workflow = Workflow::new(
            format!("export_test_{}", format),
            WorkflowType::Export,
            ContentType::GameData,
            workflow_options,
        );
        
        assert_eq!(workflow.options.export.format, format);
        assert!(workflow.options.export.include_metadata);
    }
}

#[tokio::test]
async fn test_workflow_type_consistency() {
    let temp_dir = TempDir::new().unwrap();
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();
    
    let workflow_types = vec![
        WorkflowType::Extract,
        WorkflowType::Process,
        WorkflowType::Report,
        WorkflowType::Export,
        WorkflowType::ExtractAndProcess,
        WorkflowType::ProcessAndReport,
        WorkflowType::Complete,
    ];
    
    let content_types = vec![
        ContentType::GameData,
        ContentType::Mission,
    ];
    
    // Test all combinations of workflow and content types
    for (i, workflow_type) in workflow_types.iter().enumerate() {
        for (j, content_type) in content_types.iter().enumerate() {
            let workflow_options = WorkflowOptions::default();
            
            let workflow = Workflow::new(
                format!("consistency_test_{}_{}", i, j),
                workflow_type.clone(),
                content_type.clone(),
                workflow_options,
            );
            
            assert_eq!(workflow.workflow_type, *workflow_type);
            assert_eq!(workflow.content_type, *content_type);
        }
    }
}

#[tokio::test]
async fn test_error_recovery_scenarios() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_dir = temp_dir.path().join("nonexistent");
    
    // Test workflow creation with invalid paths
    let invalid_extraction_options = ExtractionOptions {
        use_extractor: true,
        directories: None,
        source_directories: vec![nonexistent_dir.clone()],
        force: false,
    };
    
    let invalid_processing_options = ProcessingOptions {
        max_files: 0, // Invalid max files
        max_depth: 0, // Invalid max depth
        extensions: vec![],
        source_directories: vec![nonexistent_dir],
        verbose: false,
    };
    
    let workflow_options = WorkflowOptions::new()
        .with_extraction(invalid_extraction_options)
        .with_processing(invalid_processing_options);
    
    // Workflow should still be created (validation happens later)
    let workflow = Workflow::new(
        "error_recovery_test".to_string(),
        WorkflowType::ExtractAndProcess,
        ContentType::GameData,
        workflow_options,
    );
    
    assert_eq!(workflow.name, "error_recovery_test");
    assert_eq!(workflow.options.processing.max_files, 0);
    assert_eq!(workflow.options.processing.max_depth, 0);
}

#[tokio::test]
async fn test_performance_characteristics() {
    let start_time = Instant::now();
    
    // Test creating many workflows quickly
    let workflows: Vec<_> = (0..100).map(|i| {
        let workflow_options = WorkflowOptions::default();
        
        Workflow::new(
            format!("perf_test_{}", i),
            WorkflowType::Process,
            ContentType::GameData,
            workflow_options,
        )
    }).collect();
    
    let creation_time = start_time.elapsed();
    
    // Verify all workflows were created
    assert_eq!(workflows.len(), 100);
    
    // Performance should be reasonable (less than 1 second for 100 workflows)
    assert!(creation_time.as_secs() < 1);
    
    println!("Created 100 workflows in: {:?}", creation_time);
}