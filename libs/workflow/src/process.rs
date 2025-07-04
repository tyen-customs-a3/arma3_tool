use std::path::PathBuf;
use std::time::Instant;
use async_trait::async_trait;
use log::{info, debug};

use crate::error::{WorkflowError, Result};
use crate::orchestrator::{
    WorkflowHandler, WorkflowContext, WorkflowStageResult, WorkflowStageSummary
};
use crate::types::{WorkflowType, ProcessingSummary, ProcessingOptions};

/// Handler for processing workflow stages
pub struct ProcessWorkflowHandler {
    /// Scanner instance for handling file processing
    scanner: Box<dyn ScannerInterface>,
}

/// Interface for file scanning operations
#[async_trait]
pub trait ScannerInterface: Send + Sync {
    /// Scan files in the specified directory
    async fn scan_files(&self, 
        source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ProcessingOptions,
    ) -> Result<ProcessingSummary>;
    
    /// Validate processing configuration
    async fn validate_processing_config(&self, options: &ProcessingOptions) -> Result<()>;
}

impl ProcessWorkflowHandler {
    /// Create a new process workflow handler
    pub fn new(scanner: Box<dyn ScannerInterface>) -> Self {
        Self { scanner }
    }
}

#[async_trait]
impl WorkflowHandler for ProcessWorkflowHandler {
    fn name(&self) -> &str {
        "Process"
    }

    fn can_handle(&self, workflow_type: &WorkflowType) -> bool {
        matches!(workflow_type, 
            WorkflowType::Process | 
            WorkflowType::ExtractAndProcess | 
            WorkflowType::ProcessAndReport |
            WorkflowType::Complete
        )
    }

    async fn execute(&self, context: &WorkflowContext) -> Result<WorkflowStageResult> {
        info!("Starting processing workflow");
        let start_time = Instant::now();
        
        // Get processing options from workflow
        let processing_options = &context.workflow.options.processing;
        
        // Validate processing configuration
        self.scanner.validate_processing_config(processing_options).await?;
        
        // Determine source directory - use work_dir if this follows extraction
        let source_dir = if context.workflow.workflow_type.includes_extraction() {
            &context.work_dir
        } else {
            processing_options.source_directories.first()
                .ok_or_else(|| WorkflowError::validation_error("No source directory specified for processing"))?
        };
        
        debug!("Processing files from: {:?} to: {:?}", source_dir, context.output_dir);
        
        // Execute processing
        let processing_summary = self.scanner.scan_files(
            source_dir,
            &context.output_dir,
            processing_options,
        ).await.map_err(|e| WorkflowError::processing_error(e.to_string()))?;
        
        let duration = start_time.elapsed();
        
        info!("Processing completed in {:?}: {} files processed, {} entries found", 
              duration, processing_summary.files_processed, processing_summary.entries_found);
        
        Ok(WorkflowStageResult {
            stage: self.name().to_string(),
            success: true,
            duration,
            summary: WorkflowStageSummary::Processing(processing_summary),
            output_files: vec![context.output_dir.clone()],
            warnings: Vec::new(),
        })
    }

    async fn validate(&self, workflow: &crate::types::Workflow) -> Result<()> {
        debug!("Validating processing workflow");
        
        // Get processing options
        let processing_options = &workflow.options.processing;
        
        // If this is a standalone process workflow, check source directories
        if !workflow.workflow_type.includes_extraction() {
            if processing_options.source_directories.is_empty() {
                return Err(WorkflowError::validation_error("No source directories specified for processing"));
            }
            
            // Check source directories exist
            for source_dir in &processing_options.source_directories {
                if !source_dir.exists() {
                    return Err(WorkflowError::validation_error(
                        format!("Source directory does not exist: {:?}", source_dir)
                    ));
                }
            }
        }
        
        // Validate with scanner
        self.scanner.validate_processing_config(processing_options).await?;
        
        Ok(())
    }

    async fn cancel(&self) -> Result<()> {
        info!("Cancelling processing workflow");
        // TODO: Implement cancellation logic for scanner
        Ok(())
    }
}

/// Mock scanner for testing
#[cfg(test)]
pub struct MockScanner {
    pub should_fail: bool,
}

#[cfg(test)]
impl MockScanner {
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
impl ScannerInterface for MockScanner {
    async fn scan_files(&self, 
        _source_dir: &PathBuf,
        _output_dir: &PathBuf,
        _options: &ProcessingOptions,
    ) -> Result<ProcessingSummary> {
        if self.should_fail {
            return Err(WorkflowError::processing_error("Mock scanning failed"));
        }
        
        Ok(ProcessingSummary {
            processed_pbos: 3,
            files_processed: 25,
            entries_found: 150,
            elapsed_time: std::time::Duration::from_secs(30),
            errors: Vec::new(),
        })
    }
    
    async fn validate_processing_config(&self, _options: &ProcessingOptions) -> Result<()> {
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
    async fn test_process_handler_creation() {
        let scanner = Box::new(MockScanner::new());
        let handler = ProcessWorkflowHandler::new(scanner);
        assert_eq!(handler.name(), "Process");
    }

    #[tokio::test]
    async fn test_process_handler_can_handle() {
        let scanner = Box::new(MockScanner::new());
        let handler = ProcessWorkflowHandler::new(scanner);
        
        assert!(handler.can_handle(&WorkflowType::Process));
        assert!(handler.can_handle(&WorkflowType::ExtractAndProcess));
        assert!(handler.can_handle(&WorkflowType::ProcessAndReport));
        assert!(handler.can_handle(&WorkflowType::Complete));
        assert!(!handler.can_handle(&WorkflowType::Extract));
        assert!(!handler.can_handle(&WorkflowType::Report));
    }

    #[tokio::test]
    async fn test_process_handler_validation_standalone() {
        let scanner = Box::new(MockScanner::new());
        let handler = ProcessWorkflowHandler::new(scanner);
        
        // Create temporary directory for test
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().to_path_buf();
        
        let mut options = WorkflowOptions::default();
        options.processing.source_directories = vec![source_dir];
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Process,
            ContentType::GameData,
            options,
        );
        
        // Should pass validation
        assert!(handler.validate(&workflow).await.is_ok());
        
        // Test with non-existent directory
        let mut bad_options = WorkflowOptions::default();
        bad_options.processing.source_directories = vec![PathBuf::from("/nonexistent")];
        
        let bad_workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Process,
            ContentType::GameData,
            bad_options,
        );
        
        // Should fail validation
        assert!(handler.validate(&bad_workflow).await.is_err());
    }

    #[tokio::test]
    async fn test_process_handler_validation_with_extraction() {
        let scanner = Box::new(MockScanner::new());
        let handler = ProcessWorkflowHandler::new(scanner);
        
        // For workflows that include extraction, we don't need source directories
        let options = WorkflowOptions::default();
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::ExtractAndProcess,
            ContentType::GameData,
            options,
        );
        
        // Should pass validation even without source directories
        assert!(handler.validate(&workflow).await.is_ok());
    }

    #[tokio::test]
    async fn test_process_handler_execution() {
        let scanner = Box::new(MockScanner::new());
        let handler = ProcessWorkflowHandler::new(scanner);
        
        // Create temporary directories for test
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let work_dir = temp_dir.path().join("work");
        let output_dir = temp_dir.path().join("output");
        
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&work_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let mut options = WorkflowOptions::default();
        options.processing.source_directories = vec![source_dir];
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Process,
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
        
        // Execute processing
        let result = handler.execute(&context).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.stage, "Process");
        assert!(!result.output_files.is_empty());
        
        // Check summary
        if let WorkflowStageSummary::Processing(summary) = result.summary {
            assert_eq!(summary.files_processed, 25);
            assert_eq!(summary.entries_found, 150);
        } else {
            panic!("Expected processing summary");
        }
    }

    #[tokio::test]
    async fn test_process_handler_failure() {
        let scanner = Box::new(MockScanner::new().with_failure());
        let handler = ProcessWorkflowHandler::new(scanner);
        
        // Create temporary directories for test
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let work_dir = temp_dir.path().join("work");
        let output_dir = temp_dir.path().join("output");
        
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&work_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let mut options = WorkflowOptions::default();
        options.processing.source_directories = vec![source_dir];
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Process,
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
        assert!(result.unwrap_err().to_string().contains("Mock scanning failed"));
    }
}