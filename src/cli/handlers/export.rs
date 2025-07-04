use anyhow::Result;
use std::path::PathBuf;
use std::time::Instant;
use log::{info, debug};

use arma3_workflow::{
    WorkflowOrchestrator, ExportWorkflowHandler, ExporterInterface,
    Workflow, WorkflowType, ContentType, WorkflowOptions, 
    ExportOptions, ExportSummary, WorkflowError
};
use arma3_database::DatabaseManager;
use async_trait::async_trait;

/// Direct implementation of ExporterInterface using database operations
pub struct DirectExporterImpl {
    db_path: PathBuf,
}

impl DirectExporterImpl {
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }
}

#[async_trait]
impl ExporterInterface for DirectExporterImpl {
    async fn export_data(
        &self,
        _source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ExportOptions,
    ) -> arma3_workflow::Result<ExportSummary> {
        let start_time = Instant::now();
        
        // Create database manager
        let _db = DatabaseManager::new(&self.db_path)
            .map_err(|e| WorkflowError::export_error(format!("Failed to open database: {}", e)))?;
        
        // For now, create a simple placeholder export
        // TODO: Implement actual export functionality
        let export_filename = match options.format.as_str() {
            "json" => "export.json",
            "csv" => "export.csv", 
            "zip" => "export.zip",
            "tar" => "export.tar",
            _ => "export.csv", // Default to CSV
        };
        
        let export_path = if let Some(output_path) = &options.output_path {
            output_path.clone()
        } else {
            output_dir.join(export_filename)
        };
        
        // Ensure output directory exists
        if let Some(parent) = export_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| WorkflowError::export_error(format!("Failed to create output directory: {}", e)))?;
        }
        
        // Create a placeholder export file
        std::fs::write(&export_path, "# Export placeholder - TODO: Implement actual export\n")
            .map_err(|e| WorkflowError::export_error(format!("Failed to write export file: {}", e)))?;
        
        let elapsed_time = start_time.elapsed();
        
        let mut summary = ExportSummary::new();
        summary.add_generated_export(export_path);
        summary.set_elapsed_time(elapsed_time);
        
        Ok(summary)
    }
    
    async fn validate_export_config(&self, options: &ExportOptions) -> arma3_workflow::Result<()> {
        // Check database exists
        if !self.db_path.exists() {
            return Err(WorkflowError::validation_error(
                format!("Database file does not exist: {:?}", self.db_path)
            ));
        }
        
        // Check format is supported
        if options.format.is_empty() {
            return Err(WorkflowError::validation_error("Export format not specified"));
        }
        
        let supported_formats = ["zip", "tar", "json", "csv"];
        if !supported_formats.contains(&options.format.as_str()) {
            return Err(WorkflowError::validation_error(
                format!("Unsupported export format: {}. Supported formats: {:?}", 
                       options.format, supported_formats)
            ));
        }
        
        // Check output directory parent exists
        if let Some(output_path) = &options.output_path {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    return Err(WorkflowError::validation_error(
                        format!("Output directory parent does not exist: {:?}", parent)
                    ));
                }
            }
        }
        
        Ok(())
    }
}

pub async fn run_export(
    db_path: PathBuf,
    output: PathBuf,
    _item_types: Option<String>,
    _config_path: Option<PathBuf>,
    _limit: usize,
) -> Result<()> {
    info!("Starting export workflow through WorkflowOrchestrator");
    
    // Create the database manager to ensure it exists
    let _db = DatabaseManager::new(&db_path)
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;
    
    // Create the direct exporter implementation
    let exporter = Box::new(DirectExporterImpl::new(db_path));
    
    // Create the export workflow handler
    let export_handler = ExportWorkflowHandler::new(exporter);
    
    // Create the workflow orchestrator
    let mut orchestrator = WorkflowOrchestrator::new();
    orchestrator.register_handler(Box::new(export_handler));
    
    // Determine export format from output file extension
    let export_format = output.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("csv")
        .to_string();
    
    // Create workflow options
    let export_options = ExportOptions {
        format: export_format,
        output_path: Some(output.clone()),
        ..Default::default()
    };
    
    let workflow_options = WorkflowOptions::default()
        .with_export(export_options);
    
    // Create the workflow
    let workflow = Workflow::new(
        "export".to_string(),
        WorkflowType::Export,
        ContentType::GameData,
        workflow_options,
    );
    
    // Ensure output directory exists
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;
    }
    
    debug!("Executing export workflow");
    
    // Execute the workflow
    let result = orchestrator.execute_workflow(
        workflow,
        output.parent().unwrap_or(&output).to_path_buf(),
    ).await.map_err(|e| anyhow::anyhow!("Export workflow failed: {}", e))?;
    
    // Report results
    info!(
        "Export completed successfully in {:?}",
        result.duration
    );
    
    // Log generated export files
    for output_file in &result.output_files {
        info!("Generated export: {}", output_file.display());
    }
    
    if !result.errors.is_empty() {
        for error in &result.errors {
            log::error!("Workflow error: {}", error);
        }
        return Err(anyhow::anyhow!("Workflow completed with {} errors", result.errors.len()));
    }
    
    info!("Export workflow completed successfully");
    debug!("Export workflow result: {:?}", result);
    
    Ok(())
}