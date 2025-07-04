use std::path::PathBuf;
use std::time::Instant;
use async_trait::async_trait;
use log::{info, debug};

use crate::error::{WorkflowError, Result};
use crate::orchestrator::{
    WorkflowHandler, WorkflowContext, WorkflowStageResult, WorkflowStageSummary
};
use crate::types::{WorkflowType, ReportingSummary, ReportingOptions};

/// Handler for reporting workflow stages
pub struct ReportWorkflowHandler {
    /// Reporter instance for handling report generation
    reporter: Box<dyn ReporterInterface>,
}

/// Interface for report generation operations
#[async_trait]
pub trait ReporterInterface: Send + Sync {
    /// Generate reports from processed data
    async fn generate_reports(&self, 
        source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ReportingOptions,
    ) -> Result<ReportingSummary>;
    
    /// Validate reporting configuration
    async fn validate_reporting_config(&self, options: &ReportingOptions) -> Result<()>;
}

impl ReportWorkflowHandler {
    /// Create a new report workflow handler
    pub fn new(reporter: Box<dyn ReporterInterface>) -> Self {
        Self { reporter }
    }
}

#[async_trait]
impl WorkflowHandler for ReportWorkflowHandler {
    fn name(&self) -> &str {
        "Report"
    }

    fn can_handle(&self, workflow_type: &WorkflowType) -> bool {
        matches!(workflow_type, 
            WorkflowType::Report | 
            WorkflowType::ProcessAndReport |
            WorkflowType::Complete
        )
    }

    async fn execute(&self, context: &WorkflowContext) -> Result<WorkflowStageResult> {
        info!("Starting reporting workflow");
        let start_time = Instant::now();
        
        // Get reporting options from workflow
        let reporting_options = &context.workflow.options.reporting;
        
        // Validate reporting configuration
        self.reporter.validate_reporting_config(reporting_options).await?;
        
        // Determine source directory - use output_dir if this follows processing
        let source_dir = if context.workflow.workflow_type.includes_processing() {
            &context.output_dir
        } else {
            reporting_options.source_directories.first()
                .ok_or_else(|| WorkflowError::validation_error("No source directory specified for reporting"))?
        };
        
        debug!("Generating reports from: {:?} to: {:?}", source_dir, context.output_dir);
        
        // Execute reporting
        let reporting_summary = self.reporter.generate_reports(
            source_dir,
            &context.output_dir,
            reporting_options,
        ).await.map_err(|e| WorkflowError::reporting_error(e.to_string()))?;
        
        let duration = start_time.elapsed();
        
        info!("Reporting completed in {:?}: {} reports generated", 
              duration, reporting_summary.reports_generated);
        
        Ok(WorkflowStageResult {
            stage: self.name().to_string(),
            success: true,
            duration,
            summary: WorkflowStageSummary::Reporting(reporting_summary),
            output_files: vec![context.output_dir.clone()],
            warnings: Vec::new(),
        })
    }

    async fn validate(&self, workflow: &crate::types::Workflow) -> Result<()> {
        debug!("Validating reporting workflow");
        
        // Get reporting options
        let reporting_options = &workflow.options.reporting;
        
        // If this is a standalone report workflow, check source directories
        if !workflow.workflow_type.includes_processing() {
            if reporting_options.source_directories.is_empty() {
                return Err(WorkflowError::validation_error("No source directories specified for reporting"));
            }
            
            // Check source directories exist
            for source_dir in &reporting_options.source_directories {
                if !source_dir.exists() {
                    return Err(WorkflowError::validation_error(
                        format!("Source directory does not exist: {:?}", source_dir)
                    ));
                }
            }
        }
        
        // Validate with reporter
        self.reporter.validate_reporting_config(reporting_options).await?;
        
        Ok(())
    }

    async fn cancel(&self) -> Result<()> {
        info!("Cancelling reporting workflow");
        // TODO: Implement cancellation logic for reporter
        Ok(())
    }
}

/// Mock reporter for testing
#[cfg(test)]
pub struct MockReporter {
    pub should_fail: bool,
}

#[cfg(test)]
impl MockReporter {
    pub fn new() -> Self {
        Self { should_fail: false }
    }
    
    pub fn with_failure(mut self) -> Self {
        self.should_fail = true;
        self
    }
}

#[cfg(test)]
#[async_trait]
impl ReporterInterface for MockReporter {
    async fn generate_reports(&self, 
        _source_dir: &PathBuf,
        _output_dir: &PathBuf,
        _options: &ReportingOptions,
    ) -> Result<ReportingSummary> {
        if self.should_fail {
            return Err(WorkflowError::reporting_error("Mock reporting failed"));
        }
        
        Ok(ReportingSummary {
            reports_generated: 3,
            report_paths: vec![
                PathBuf::from("report.html"),
                PathBuf::from("report.json"),
                PathBuf::from("report.csv"),
            ],
            elapsed_time: std::time::Duration::from_secs(5),
            errors: Vec::new(),
        })
    }
    
    async fn validate_reporting_config(&self, _options: &ReportingOptions) -> Result<()> {
        if self.should_fail {
            return Err(WorkflowError::validation_error("Mock validation failed"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Workflow, WorkflowOptions, ContentType};
    use tempfile::TempDir;
    use std::fs;

    #[tokio::test]
    async fn test_report_handler_creation() {
        let reporter = Box::new(MockReporter::new());
        let handler = ReportWorkflowHandler::new(reporter);
        assert_eq!(handler.name(), "Report");
    }

    #[tokio::test]
    async fn test_report_handler_can_handle() {
        let reporter = Box::new(MockReporter::new());
        let handler = ReportWorkflowHandler::new(reporter);
        
        assert!(handler.can_handle(&WorkflowType::Report));
        assert!(handler.can_handle(&WorkflowType::ProcessAndReport));
        assert!(handler.can_handle(&WorkflowType::Complete));
        assert!(!handler.can_handle(&WorkflowType::Extract));
        assert!(!handler.can_handle(&WorkflowType::Process));
        assert!(!handler.can_handle(&WorkflowType::ExtractAndProcess));
    }

    #[tokio::test]
    async fn test_report_handler_validation_standalone() {
        let reporter = Box::new(MockReporter::new());
        let handler = ReportWorkflowHandler::new(reporter);
        
        // Create temporary directory for test
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().to_path_buf();
        
        let mut options = WorkflowOptions::default();
        options.reporting.source_directories = vec![source_dir];
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Report,
            ContentType::GameData,
            options,
        );
        
        // Should pass validation
        assert!(handler.validate(&workflow).await.is_ok());
        
        // Test with non-existent directory
        let mut bad_options = WorkflowOptions::default();
        bad_options.reporting.source_directories = vec![PathBuf::from("/nonexistent")];
        
        let bad_workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Report,
            ContentType::GameData,
            bad_options,
        );
        
        // Should fail validation
        assert!(handler.validate(&bad_workflow).await.is_err());
    }

    #[tokio::test]
    async fn test_report_handler_validation_with_processing() {
        let reporter = Box::new(MockReporter::new());
        let handler = ReportWorkflowHandler::new(reporter);
        
        // For workflows that include processing, we don't need source directories
        let options = WorkflowOptions::default();
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::ProcessAndReport,
            ContentType::GameData,
            options,
        );
        
        // Should pass validation even without source directories
        assert!(handler.validate(&workflow).await.is_ok());
    }

    #[tokio::test]
    async fn test_report_handler_execution() {
        let reporter = Box::new(MockReporter::new());
        let handler = ReportWorkflowHandler::new(reporter);
        
        // Create temporary directories for test
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let work_dir = temp_dir.path().join("work");
        let output_dir = temp_dir.path().join("output");
        
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&work_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let mut options = WorkflowOptions::default();
        options.reporting.source_directories = vec![source_dir];
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Report,
            ContentType::GameData,
            options,
        );
        
        let context = WorkflowContext {
            workflow,
            start_time: Instant::now(),
            work_dir,
            output_dir,
            is_cancelled: false,
        };
        
        // Execute reporting
        let result = handler.execute(&context).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.stage, "Report");
        assert!(!result.output_files.is_empty());
        
        // Check summary
        if let WorkflowStageSummary::Reporting(summary) = result.summary {
            assert_eq!(summary.reports_generated, 3);
            assert_eq!(summary.report_paths.len(), 3);
        } else {
            panic!("Expected reporting summary");
        }
    }

    #[tokio::test]
    async fn test_report_handler_failure() {
        let reporter = Box::new(MockReporter::new().with_failure());
        let handler = ReportWorkflowHandler::new(reporter);
        
        // Create temporary directories for test
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let work_dir = temp_dir.path().join("work");
        let output_dir = temp_dir.path().join("output");
        
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&work_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let mut options = WorkflowOptions::default();
        options.reporting.source_directories = vec![source_dir];
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Report,
            ContentType::GameData,
            options,
        );
        
        let context = WorkflowContext {
            workflow,
            start_time: Instant::now(),
            work_dir,
            output_dir,
            is_cancelled: false,
        };
        
        // Should fail execution
        let result = handler.execute(&context).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Mock reporting failed"));
    }
}