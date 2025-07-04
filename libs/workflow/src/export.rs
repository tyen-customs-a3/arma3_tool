use std::path::PathBuf;
use std::time::{Duration, Instant};
use async_trait::async_trait;
use log::{info, debug, error};

use crate::error::{WorkflowError, Result};
use crate::orchestrator::{
    WorkflowHandler, WorkflowContext, WorkflowStageResult, WorkflowStageSummary
};
use crate::types::WorkflowType;
use crate::types::options::ExportOptions;

/// Handler for export workflow stages
pub struct ExportWorkflowHandler {
    /// Exporter instance for handling data export
    exporter: Box<dyn ExporterInterface>,
}

/// Interface for data export operations
#[async_trait]
pub trait ExporterInterface: Send + Sync {
    /// Export data to various formats
    async fn export_data(&self, 
        source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ExportOptions,
    ) -> Result<ExportSummary>;
    
    /// Validate export configuration
    async fn validate_export_config(&self, options: &ExportOptions) -> Result<()>;
}

/// Summary of export operations
#[derive(Debug, Clone)]
pub struct ExportSummary {
    pub exported_files: usize,
    pub export_format: String,
    pub export_time: Duration,
    pub total_size: u64,
    pub output_files: Vec<PathBuf>,
}

impl ExportWorkflowHandler {
    /// Create a new export workflow handler
    pub fn new(exporter: Box<dyn ExporterInterface>) -> Self {
        Self { exporter }
    }
}

#[async_trait]
impl WorkflowHandler for ExportWorkflowHandler {
    fn name(&self) -> &str {
        "Export"
    }

    fn can_handle(&self, workflow_type: &WorkflowType) -> bool {
        matches!(workflow_type, WorkflowType::Complete)
    }

    async fn execute(&self, context: &WorkflowContext) -> Result<WorkflowStageResult> {
        info!("Starting export workflow");
        let start_time = Instant::now();
        
        // Get export options from workflow
        let export_options = &context.workflow.options.export;
        
        // Validate export configuration
        self.exporter.validate_export_config(export_options).await?;
        
        // Source directory is the output directory (where reports are generated)
        let source_dir = &context.output_dir;
        
        debug!("Exporting data from: {:?} to: {:?}", source_dir, context.output_dir);
        
        // Execute export
        let export_summary = self.exporter.export_data(
            source_dir,
            &context.output_dir,
            export_options,
        ).await.map_err(|e| WorkflowError::export_error(e.to_string()))?;
        
        let duration = start_time.elapsed();
        
        info!("Export completed in {:?}: {} files exported in {} format", 
              duration, export_summary.exported_files, export_summary.export_format);
        
        Ok(WorkflowStageResult {
            stage: self.name().to_string(),
            success: true,
            duration,
            summary: WorkflowStageSummary::Export {
                exported_files: export_summary.exported_files,
                export_format: export_summary.export_format,
            },
            output_files: export_summary.output_files,
            warnings: Vec::new(),
        })
    }

    async fn validate(&self, workflow: &crate::types::Workflow) -> Result<()> {
        debug!("Validating export workflow");
        
        // Get export options
        let export_options = &workflow.options.export;
        
        // Validate export format is supported
        if export_options.format.is_empty() {
            return Err(WorkflowError::validation_error("Export format not specified"));
        }
        
        // Validate with exporter
        self.exporter.validate_export_config(export_options).await?;
        
        Ok(())
    }

    async fn cancel(&self) -> Result<()> {
        info!("Cancelling export workflow");
        // TODO: Implement cancellation logic for exporter
        Ok(())
    }
}

/// Mock exporter for testing
#[cfg(test)]
pub struct MockExporter {
    pub should_fail: bool,
}

#[cfg(test)]
impl MockExporter {
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
impl ExporterInterface for MockExporter {
    async fn export_data(&self, 
        _source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ExportOptions,
    ) -> Result<ExportSummary> {
        if self.should_fail {
            return Err(WorkflowError::export_error("Mock export failed"));
        }
        
        Ok(ExportSummary {
            exported_files: 2,
            export_format: options.format.clone(),
            export_time: Duration::from_secs(2),
            total_size: 1024 * 1024 * 10, // 10MB
            output_files: vec![
                output_dir.join("export.zip"),
                output_dir.join("export.json"),
            ],
        })
    }
    
    async fn validate_export_config(&self, _options: &ExportOptions) -> Result<()> {
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
    async fn test_export_handler_creation() {
        let exporter = Box::new(MockExporter::new());
        let handler = ExportWorkflowHandler::new(exporter);
        assert_eq!(handler.name(), "Export");
    }

    #[tokio::test]
    async fn test_export_handler_can_handle() {
        let exporter = Box::new(MockExporter::new());
        let handler = ExportWorkflowHandler::new(exporter);
        
        assert!(handler.can_handle(&WorkflowType::Complete));
        assert!(!handler.can_handle(&WorkflowType::Extract));
        assert!(!handler.can_handle(&WorkflowType::Process));
        assert!(!handler.can_handle(&WorkflowType::Report));
        assert!(!handler.can_handle(&WorkflowType::ExtractAndProcess));
        assert!(!handler.can_handle(&WorkflowType::ProcessAndReport));
    }

    #[tokio::test]
    async fn test_export_handler_validation() {
        let exporter = Box::new(MockExporter::new());
        let handler = ExportWorkflowHandler::new(exporter);
        
        let mut options = WorkflowOptions::default();
        options.export.format = "zip".to_string();
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Complete,
            ContentType::GameData,
            options,
        );
        
        // Should pass validation
        assert!(handler.validate(&workflow).await.is_ok());
        
        // Test with empty format
        let mut bad_options = WorkflowOptions::default();
        bad_options.export.format = "".to_string();
        
        let bad_workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Complete,
            ContentType::GameData,
            bad_options,
        );
        
        // Should fail validation
        assert!(handler.validate(&bad_workflow).await.is_err());
    }

    #[tokio::test]
    async fn test_export_handler_execution() {
        let exporter = Box::new(MockExporter::new());
        let handler = ExportWorkflowHandler::new(exporter);
        
        // Create temporary directories for test
        let temp_dir = TempDir::new().unwrap();
        let work_dir = temp_dir.path().join("work");
        let output_dir = temp_dir.path().join("output");
        
        fs::create_dir_all(&work_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let mut options = WorkflowOptions::default();
        options.export.format = "zip".to_string();
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Complete,
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
        
        // Execute export
        let result = handler.execute(&context).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.stage, "Export");
        assert!(!result.output_files.is_empty());
        
        // Check summary
        if let WorkflowStageSummary::Export { exported_files, export_format } = result.summary {
            assert_eq!(exported_files, 2);
            assert_eq!(export_format, "zip");
        } else {
            panic!("Expected export summary");
        }
    }

    #[tokio::test]
    async fn test_export_handler_failure() {
        let exporter = Box::new(MockExporter::new().with_failure());
        let handler = ExportWorkflowHandler::new(exporter);
        
        // Create temporary directories for test
        let temp_dir = TempDir::new().unwrap();
        let work_dir = temp_dir.path().join("work");
        let output_dir = temp_dir.path().join("output");
        
        fs::create_dir_all(&work_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let mut options = WorkflowOptions::default();
        options.export.format = "zip".to_string();
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Complete,
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
        assert!(result.unwrap_err().to_string().contains("Mock export failed"));
    }
}