use anyhow::Result;
use crate::config::ScanConfig;
use arma3_workflow::{
    WorkflowOrchestrator, ReportWorkflowHandler, Workflow, WorkflowType, 
    ContentType, WorkflowOptions, ReportingOptions, ReportFormat
};
use crate::cli::adapters::Arma3ReporterAdapter;
use std::path::PathBuf;
use log::info;

pub async fn run_report(db_path: PathBuf, output_dir: PathBuf, config: &ScanConfig) -> Result<()> {
    info!("Starting reporting using workflow orchestration");
    
    // Create the reporter adapter
    let reporter_adapter = Box::new(Arma3ReporterAdapter::new(
        db_path.clone(), 
        config.ignore_classes_file.clone()
    ));
    
    // Create the workflow handler
    let report_handler = ReportWorkflowHandler::new(reporter_adapter);
    
    // Create the workflow orchestrator
    let mut orchestrator = WorkflowOrchestrator::new();
    orchestrator.register_handler(Box::new(report_handler));
    
    // Convert ScanConfig to workflow options
    let reporting_options = ReportingOptions {
        format: ReportFormat::Html, // Default to HTML format
        output_path: Some(output_dir.clone()),
        source_directories: vec![db_path.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf()],
        include_mission_dependencies: true,
        include_game_data_classes: true,
        include_missing_dependencies: true,
    };
    
    let workflow_options = WorkflowOptions::default()
        .with_reporting(reporting_options);
    
    // Create the workflow
    let workflow = Workflow::new(
        "report".to_string(),
        WorkflowType::Report,
        ContentType::GameData,
        workflow_options,
    );
    
    // Execute the workflow
    let result = orchestrator.execute_workflow(workflow, output_dir).await
        .map_err(|e| anyhow::anyhow!("Workflow execution failed: {}", e))?;
    
    // Report results
    if let Some(reporting_summary) = result.reporting_summary {
        info!(
            "Reporting completed successfully: {} reports generated in {:?}",
            reporting_summary.reports_generated,
            result.duration
        );
        
        if !reporting_summary.errors.is_empty() {
            info!("Reporting completed with {} warnings/errors:", reporting_summary.errors.len());
            for error in &reporting_summary.errors {
                log::warn!("  {}", error);
            }
        }
        
        // Log generated report paths
        for report_path in &reporting_summary.report_paths {
            info!("Generated report: {}", report_path.display());
        }
    }
    
    if !result.errors.is_empty() {
        for error in &result.errors {
            log::error!("Workflow error: {}", error);
        }
        return Err(anyhow::anyhow!("Workflow completed with {} errors", result.errors.len()));
    }
    
    info!("Reporting workflow completed successfully");
    Ok(())
}