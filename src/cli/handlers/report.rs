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

/// Direct implementation of ReporterInterface using arma3_reporter library
pub struct DirectReporterImpl {
    db_path: PathBuf,
    ignore_classes_file: Option<PathBuf>,
}

impl DirectReporterImpl {
    pub fn new(db_path: PathBuf, ignore_classes_file: Option<PathBuf>) -> Self {
        Self { db_path, ignore_classes_file }
    }
}

#[async_trait]
impl ReporterInterface for DirectReporterImpl {
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
        
        // Generate reports - for now, use run_report which generates all available formats
        coordinator.run_report(output_dir)
            .map_err(|e| WorkflowError::reporting_error(e.to_string()))?;
        
        // Also generate additional reports
        coordinator.generate_class_graph(output_dir)
            .map_err(|e| WorkflowError::reporting_error(e.to_string()))?;
        
        coordinator.generate_mission_class_source_report(output_dir)
            .map_err(|e| WorkflowError::reporting_error(e.to_string()))?;
        
        let elapsed_time = start_time.elapsed();
        
        // Check for generated files
        let mut report_paths = Vec::new();
        let possible_files = [
            "dependency_report.html",
            "dependency_report.json", 
            "dependency_report.csv",
            "dependency_report.md",
            "dependency_report.txt",
            "class_hierarchy.csv",
            "mission_class_sources.csv"
        ];
        
        for file_name in &possible_files {
            let file_path = output_dir.join(file_name);
            if file_path.exists() {
                report_paths.push(file_path);
            }
        }
        
        // Look for timestamped dependency reports
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let file_name_lossy = entry.file_name().to_string_lossy().to_string();
                if file_name_lossy.starts_with("dependency_report_") {
                    report_paths.push(entry.path());
                }
            }
        }
        
        Ok(ReportingSummary {
            reports_generated: report_paths.len(),
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

pub async fn run_report(db_path: PathBuf, output_dir: PathBuf, config: &ScanConfig) -> Result<()> {
    info!("Starting reporting using workflow orchestration");
    
    // Create the direct reporter implementation
    let reporter = Box::new(DirectReporterImpl::new(
        db_path.clone(), 
        config.ignore_classes_file.clone()
    ));
    
    // Create the workflow handler
    let report_handler = ReportWorkflowHandler::new(reporter);
    
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