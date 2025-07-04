use std::path::PathBuf;
use std::time::{Duration, Instant};
use async_trait::async_trait;
use log::{info, debug, error};

use crate::error::{WorkflowError, Result};
use crate::orchestrator::{
    WorkflowHandler, WorkflowContext, WorkflowStageResult, WorkflowStageSummary
};
use crate::types::{WorkflowType, ExtractionSummary, ExtractionOptions};

/// Handler for extraction workflow stages
pub struct ExtractWorkflowHandler {
    /// Extractor instance for handling PBO extraction
    extractor: Box<dyn ExtractorInterface>,
}

/// Interface for PBO extraction operations
#[async_trait]
pub trait ExtractorInterface: Send + Sync {
    /// Extract PBO files from the specified directory
    async fn extract_pbos(&self, 
        source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ExtractionOptions,
    ) -> Result<ExtractionSummary>;
    
    /// Validate extraction configuration
    async fn validate_extraction_config(&self, options: &ExtractionOptions) -> Result<()>;
}

impl ExtractWorkflowHandler {
    /// Create a new extract workflow handler
    pub fn new(extractor: Box<dyn ExtractorInterface>) -> Self {
        Self { extractor }
    }
}

#[async_trait]
impl WorkflowHandler for ExtractWorkflowHandler {
    fn name(&self) -> &str {
        "Extract"
    }

    fn can_handle(&self, workflow_type: &WorkflowType) -> bool {
        matches!(workflow_type, 
            WorkflowType::Extract | 
            WorkflowType::ExtractAndProcess | 
            WorkflowType::Complete
        )
    }

    async fn execute(&self, context: &WorkflowContext) -> Result<WorkflowStageResult> {
        info!("Starting extraction workflow");
        let start_time = Instant::now();
        
        // Get extraction options from workflow
        let extraction_options = &context.workflow.options.extraction;
        
        // Validate extraction configuration
        self.extractor.validate_extraction_config(extraction_options).await?;
        
        // Determine source directory from workflow
        let source_dir = extraction_options.source_directories.first()
            .ok_or_else(|| WorkflowError::validation_error("No source directory specified for extraction"))?;
        
        debug!("Extracting PBOs from: {:?} to: {:?}", source_dir, context.work_dir);
        
        // Execute extraction
        let extraction_summary = self.extractor.extract_pbos(
            source_dir,
            &context.work_dir,
            extraction_options,
        ).await.map_err(|e| WorkflowError::extraction_error(e.to_string()))?;
        
        let duration = start_time.elapsed();
        
        info!("Extraction completed in {:?}: {} PBOs extracted", 
              duration, extraction_summary.extracted_pbos);
        
        Ok(WorkflowStageResult {
            stage: self.name().to_string(),
            success: true,
            duration,
            summary: WorkflowStageSummary::Extraction(extraction_summary),
            output_files: vec![context.work_dir.clone()],
            warnings: Vec::new(),
        })
    }

    async fn validate(&self, workflow: &crate::types::Workflow) -> Result<()> {
        debug!("Validating extraction workflow");
        
        // Validate extraction options
        let extraction_options = &workflow.options.extraction;
        
        // Check source directories are specified
        if extraction_options.source_directories.is_empty() {
            return Err(WorkflowError::validation_error("No source directories specified for extraction"));
        }
        
        // Check source directories exist
        for source_dir in &extraction_options.source_directories {
            if !source_dir.exists() {
                return Err(WorkflowError::validation_error(
                    format!("Source directory does not exist: {:?}", source_dir)
                ));
            }
        }
        
        // Validate with extractor
        self.extractor.validate_extraction_config(extraction_options).await?;
        
        Ok(())
    }

    async fn cancel(&self) -> Result<()> {
        info!("Cancelling extraction workflow");
        // TODO: Implement cancellation logic for extractor
        Ok(())
    }
}

/// Mock extractor for testing
#[cfg(test)]
pub struct MockExtractor {
    pub should_fail: bool,
}

#[cfg(test)]
impl MockExtractor {
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
impl ExtractorInterface for MockExtractor {
    async fn extract_pbos(&self, 
        _source_dir: &PathBuf,
        _output_dir: &PathBuf,
        _options: &ExtractionOptions,
    ) -> Result<ExtractionSummary> {
        if self.should_fail {
            return Err(WorkflowError::extraction_error("Mock extraction failed"));
        }
        
        Ok(ExtractionSummary {
            extracted_pbos: 5,
            extraction_paths: vec![
                PathBuf::from("test1.pbo"),
                PathBuf::from("test2.pbo"),
                PathBuf::from("test3.pbo"),
                PathBuf::from("test4.pbo"),
                PathBuf::from("test5.pbo"),
            ],
            elapsed_time: Duration::from_secs(10),
            errors: Vec::new(),
        })
    }
    
    async fn validate_extraction_config(&self, _options: &ExtractionOptions) -> Result<()> {
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
    async fn test_extract_handler_creation() {
        let extractor = Box::new(MockExtractor::new());
        let handler = ExtractWorkflowHandler::new(extractor);
        assert_eq!(handler.name(), "Extract");
    }

    #[tokio::test]
    async fn test_extract_handler_can_handle() {
        let extractor = Box::new(MockExtractor::new());
        let handler = ExtractWorkflowHandler::new(extractor);
        
        assert!(handler.can_handle(&WorkflowType::Extract));
        assert!(handler.can_handle(&WorkflowType::ExtractAndProcess));
        assert!(handler.can_handle(&WorkflowType::Complete));
        assert!(!handler.can_handle(&WorkflowType::Process));
        assert!(!handler.can_handle(&WorkflowType::Report));
    }

    #[tokio::test]
    async fn test_extract_handler_validation() {
        let extractor = Box::new(MockExtractor::new());
        let handler = ExtractWorkflowHandler::new(extractor);
        
        // Create temporary directory for test
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().to_path_buf();
        
        let mut options = WorkflowOptions::default();
        options.extraction.source_directories = vec![source_dir];
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Extract,
            ContentType::GameData,
            options,
        );
        
        // Should pass validation
        assert!(handler.validate(&workflow).await.is_ok());
        
        // Test with non-existent directory
        let mut bad_options = WorkflowOptions::default();
        bad_options.extraction.source_directories = vec![PathBuf::from("/nonexistent")];
        
        let bad_workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Extract,
            ContentType::GameData,
            bad_options,
        );
        
        // Should fail validation
        assert!(handler.validate(&bad_workflow).await.is_err());
    }

    #[tokio::test]
    async fn test_extract_handler_execution() {
        let extractor = Box::new(MockExtractor::new());
        let handler = ExtractWorkflowHandler::new(extractor);
        
        // Create temporary directories for test
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let work_dir = temp_dir.path().join("work");
        let output_dir = temp_dir.path().join("output");
        
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&work_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let mut options = WorkflowOptions::default();
        options.extraction.source_directories = vec![source_dir];
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Extract,
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
        
        // Execute extraction
        let result = handler.execute(&context).await.unwrap();
        
        assert!(result.success);
        assert_eq!(result.stage, "Extract");
        assert!(!result.output_files.is_empty());
        
        // Check summary
        if let WorkflowStageSummary::Extraction(summary) = result.summary {
            assert_eq!(summary.extracted_pbos, 5);
            assert_eq!(summary.extraction_paths.len(), 5);
        } else {
            panic!("Expected extraction summary");
        }
    }

    #[tokio::test]
    async fn test_extract_handler_failure() {
        let extractor = Box::new(MockExtractor::new().with_failure());
        let handler = ExtractWorkflowHandler::new(extractor);
        
        // Create temporary directories for test
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let work_dir = temp_dir.path().join("work");
        let output_dir = temp_dir.path().join("output");
        
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&work_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let mut options = WorkflowOptions::default();
        options.extraction.source_directories = vec![source_dir];
        
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Extract,
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
        assert!(result.unwrap_err().to_string().contains("Mock extraction failed"));
    }
}