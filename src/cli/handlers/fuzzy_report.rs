use anyhow::Result;
use crate::config::ScanConfig;
use arma3_workflow::{
    WorkflowOrchestrator, ReportWorkflowHandler, Workflow, WorkflowType, 
    ContentType, WorkflowOptions, ReportingOptions, ReportFormat
};
use crate::cli::adapters::FuzzyReporterAdapter;
use std::path::PathBuf;
use log::info;

pub async fn run_fuzzy_report(
    db_path: PathBuf,
    output_dir: PathBuf,
    config: &ScanConfig,
) -> Result<()> {
    info!("Starting fuzzy report using workflow orchestration");
    
    // Create the fuzzy reporter adapter
    let fuzzy_reporter_adapter = Box::new(FuzzyReporterAdapter::new(
        db_path.clone(), 
        config.ignore_classes_file.clone()
    ));
    
    // Create the workflow handler for fuzzy reports
    let report_handler = ReportWorkflowHandler::new(fuzzy_reporter_adapter);
    
    // Create the workflow orchestrator
    let mut orchestrator = WorkflowOrchestrator::new();
    orchestrator.register_handler(Box::new(report_handler));
    
    // Convert ScanConfig to workflow options for fuzzy reporting
    let reporting_options = ReportingOptions {
        format: ReportFormat::Csv, // Fuzzy reports are CSV format
        output_path: Some(output_dir.clone()),
        source_directories: vec![db_path.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf()],
        include_mission_dependencies: false,
        include_game_data_classes: false,
        include_missing_dependencies: false, // Fuzzy reports handle missing classes differently
    };
    
    let workflow_options = WorkflowOptions::default()
        .with_reporting(reporting_options);
    
    // Create the fuzzy workflow
    let workflow = Workflow::new(
        "fuzzy_report".to_string(),
        WorkflowType::Report,
        ContentType::GameData,
        workflow_options,
    );
    
    // Execute the workflow
    let result = orchestrator.execute_workflow(workflow, output_dir).await
        .map_err(|e| anyhow::anyhow!("Fuzzy workflow execution failed: {}", e))?;
    
    // Report results
    if let Some(reporting_summary) = result.reporting_summary {
        info!(
            "Fuzzy reporting completed successfully: {} reports generated in {:?}",
            reporting_summary.reports_generated,
            result.duration
        );
        
        if !reporting_summary.errors.is_empty() {
            info!("Fuzzy reporting completed with {} warnings/errors:", reporting_summary.errors.len());
            for error in &reporting_summary.errors {
                log::warn!("  {}", error);
            }
        }
        
        // Log generated report paths
        for report_path in &reporting_summary.report_paths {
            info!("Generated fuzzy report: {}", report_path.display());
        }
    }
    
    if !result.errors.is_empty() {
        for error in &result.errors {
            log::error!("Fuzzy workflow error: {}", error);
        }
        return Err(anyhow::anyhow!("Fuzzy workflow completed with {} errors", result.errors.len()));
    }
    
    info!("Fuzzy reporting workflow completed successfully");
    Ok(())
}