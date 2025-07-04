use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;
use log::{info, debug};

use arma3_workflow::orchestrator::WorkflowOrchestrator;
use arma3_workflow::export::ExportWorkflowHandler;
use arma3_workflow::types::{Workflow, WorkflowType, ContentType};
use arma3_workflow::types::options::{WorkflowOptions, ExportOptions};
use arma3_database::DatabaseManager;
use crate::cli::adapters::Arma3ExporterAdapter;

pub async fn run_export(
    db_path: PathBuf,
    output: PathBuf,
    item_types: Option<String>,
    config_path: Option<PathBuf>,
    limit: usize,
) -> Result<()> {
    info!("Starting export workflow through WorkflowOrchestrator");
    
    // Create the database manager to ensure it exists
    let _db = DatabaseManager::new(&db_path)
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;
    
    // Create the export adapter
    let exporter = Box::new(Arma3ExporterAdapter::new(db_path));
    
    // Create the export workflow handler
    let export_handler = ExportWorkflowHandler::new(exporter);
    
    // Create the workflow orchestrator
    let orchestrator = WorkflowOrchestrator::new();
    
    // Determine export format from output file extension
    let export_format = output.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("csv")
        .to_string();
    
    // Create workflow options
    let export_options = ExportOptions::new()
        .with_format(export_format)
        .with_output_path(output.clone());
    
    let workflow_options = WorkflowOptions::new()
        .with_export(export_options);
    
    // Create the workflow
    let workflow = Workflow::new(
        "export".to_string(),
        WorkflowType::Export,
        ContentType::GameData,
        workflow_options,
    );
    
    // Create temporary work directory
    let temp_dir = tempfile::tempdir()
        .map_err(|e| anyhow::anyhow!("Failed to create temporary directory: {}", e))?;
    
    // Ensure output directory exists
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
    }
    
    debug!("Executing export workflow");
    
    // Execute the workflow
    let result = orchestrator.execute_workflow(
        workflow,
        temp_dir.path().to_path_buf(),
        output.parent().unwrap_or(&output).to_path_buf(),
        vec![Box::new(export_handler)],
    ).await.map_err(|e| anyhow::anyhow!("Export workflow failed: {}", e))?;
    
    info!("Export workflow completed successfully");
    debug!("Export workflow result: {:?}", result);
    
    Ok(())
}