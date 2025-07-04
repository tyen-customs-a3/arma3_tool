use anyhow::Result;
use arma3_extractor::ExtractionConfig;
use arma3_workflow::{
    WorkflowOrchestrator, ProcessWorkflowHandler, Workflow, WorkflowType, 
    ContentType, WorkflowOptions, ProcessingOptions
};
use crate::cli::adapters::Arma3ScannerAdapter;
use std::path::PathBuf;
use log::info;

pub async fn run_process(config: ExtractionConfig, db_path: PathBuf) -> Result<()> {
    info!("Starting processing using workflow orchestration");
    
    // Create the scanner adapter
    let scanner_adapter = Box::new(Arma3ScannerAdapter::new(config.clone(), db_path.clone()));
    
    // Create the workflow handler
    let process_handler = ProcessWorkflowHandler::new(scanner_adapter);
    
    // Create the workflow orchestrator
    let mut orchestrator = WorkflowOrchestrator::new();
    orchestrator.register_handler(Box::new(process_handler));
    
    // Convert ExtractionConfig to workflow options
    let processing_options = ProcessingOptions {
        max_files: 1000, // Default reasonable limit
        max_depth: 10,   // Default reasonable depth
        extensions: config.game_data_extensions.clone(),
        source_directories: vec![config.game_data_cache_dir.clone(), config.mission_cache_dir.clone()],
        verbose: false,  // Could be made configurable
    };
    
    let workflow_options = WorkflowOptions::default()
        .with_processing(processing_options);
    
    // Create the workflow
    let workflow = Workflow::new(
        "process".to_string(),
        WorkflowType::Process,
        ContentType::GameData,
        workflow_options,
    );
    
    // Execute the workflow
    let output_dir = db_path.parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();
    
    let result = orchestrator.execute_workflow(workflow, output_dir).await
        .map_err(|e| anyhow::anyhow!("Workflow execution failed: {}", e))?;
    
    // Report results
    if let Some(processing_summary) = result.processing_summary {
        info!(
            "Processing completed successfully: {} files processed, {} entries found in {:?}",
            processing_summary.files_processed,
            processing_summary.entries_found,
            result.duration
        );
        
        if !processing_summary.errors.is_empty() {
            info!("Processing completed with {} warnings/errors:", processing_summary.errors.len());
            for error in &processing_summary.errors {
                log::warn!("  {}", error);
            }
        }
    }
    
    if !result.errors.is_empty() {
        for error in &result.errors {
            log::error!("Workflow error: {}", error);
        }
        return Err(anyhow::anyhow!("Workflow completed with {} errors", result.errors.len()));
    }
    
    info!("Processing workflow completed successfully");
    Ok(())
}