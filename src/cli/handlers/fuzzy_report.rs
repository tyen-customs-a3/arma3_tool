use anyhow::Result;
use arma3_config::ScanConfig;
use arma3_reporter::ReportCoordinator;
use arma3_database::DatabaseManager;
use arma3_workflow::{
    WorkflowOrchestrator, ReportWorkflowHandler, ReporterInterface,
    Workflow, WorkflowType, ContentType, WorkflowOptions, ReportingOptions, ReportFormat,
    ReportingSummary, WorkflowError
};
use async_trait::async_trait;
use std::path::PathBuf;
use std::time::Instant;
use log::info;

/// Direct implementation of ReporterInterface for fuzzy reporting using arma3_reporter library
pub struct DirectFuzzyReporterImpl {
    db_path: PathBuf,
    ignore_classes_file: Option<PathBuf>,
}

impl DirectFuzzyReporterImpl {
    pub fn new(db_path: PathBuf, ignore_classes_file: Option<PathBuf>) -> Self {
        Self { db_path, ignore_classes_file }
    }
}

#[async_trait]
impl ReporterInterface for DirectFuzzyReporterImpl {
    async fn generate_reports(
        &self,
        _source_dir: &PathBuf,
        output_dir: &PathBuf,
        _options: &ReportingOptions,
    ) -> arma3_workflow::Result<ReportingSummary> {
        let start_time = Instant::now();
        
        // Create database manager
        let db = DatabaseManager::new(&self.db_path)
            .map_err(|e| WorkflowError::reporting_error(format!("Failed to open database: {}", e)))?;
        
        // Create reporter coordinator
        let coordinator = ReportCoordinator::new(&db, self.ignore_classes_file.clone());
        
        // Generate fuzzy report (always CSV for fuzzy reports)
        coordinator.generate_fuzzy_missing_class_report(output_dir)
            .map_err(|e| WorkflowError::reporting_error(e.to_string()))?;
        
        let elapsed_time = start_time.elapsed();
        
        // Look for the generated fuzzy report file
        let report_paths = if let Ok(entries) = std::fs::read_dir(output_dir) {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry.file_name().to_string_lossy().starts_with("fuzzy_missing_class_report_")
                })
                .map(|entry| entry.path())
                .collect()
        } else {
            Vec::new()
        };
        
        Ok(ReportingSummary {
            reports_generated: if report_paths.is_empty() { 0 } else { 1 },
            report_paths,
            elapsed_time,
            errors: Vec::new(), // TODO: Capture any warnings from reporter
        })
    }
    
    async fn validate_reporting_config(&self, options: &ReportingOptions) -> arma3_workflow::Result<()> {
        // Check database exists
        if !self.db_path.exists() {
            return Err(WorkflowError::validation_error(
                format!("Database file does not exist: {:?}", self.db_path)
            ));
        }
        
        // Check ignore classes file exists if specified
        if let Some(ignore_file) = &self.ignore_classes_file {
            if !ignore_file.exists() {
                return Err(WorkflowError::validation_error(
                    format!("Ignore classes file does not exist: {:?}", ignore_file)
                ));
            }
        }
        
        // Check output directory is writable
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

pub async fn run_fuzzy_report(
    db_path: PathBuf,
    output_dir: PathBuf,
    config: &ScanConfig,
) -> Result<()> {
    info!("Starting fuzzy report using workflow orchestration");
    
    // Create the direct fuzzy reporter implementation
    let fuzzy_reporter = Box::new(DirectFuzzyReporterImpl::new(
        db_path.clone(), 
        config.ignore_classes_file.clone()
    ));
    
    // Create the workflow handler for fuzzy reports
    let report_handler = ReportWorkflowHandler::new(fuzzy_reporter);
    
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