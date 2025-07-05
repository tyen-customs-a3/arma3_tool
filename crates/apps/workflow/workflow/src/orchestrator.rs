use std::time::{Duration, Instant};
use std::path::PathBuf;
use async_trait::async_trait;
use log::{info, warn, error};

use crate::error::{WorkflowError, Result};
use crate::types::{
    WorkflowType, Workflow,
    ExtractionSummary, ProcessingSummary, ReportingSummary
};

/// Represents the current state of a workflow execution
#[derive(Debug, Clone, PartialEq)]
pub enum WorkflowState {
    /// Workflow is ready to start
    Ready,
    /// Workflow is currently running
    Running,
    /// Workflow completed successfully
    Completed,
    /// Workflow failed with an error
    Failed,
    /// Workflow was cancelled
    Cancelled,
    /// Workflow paused (for future extensibility)
    Paused,
}

/// Progress information for a workflow
#[derive(Debug, Clone)]
pub struct WorkflowProgress {
    /// Current workflow state
    pub state: WorkflowState,
    /// Percentage complete (0-100)
    pub percentage: f64,
    /// Current stage description
    pub stage: String,
    /// Elapsed time since workflow started
    pub elapsed: Duration,
    /// Optional ETA for completion
    pub eta: Option<Duration>,
}

/// Context provided to workflow handlers
#[derive(Debug, Clone)]
pub struct WorkflowContext {
    /// Workflow configuration
    pub workflow: Workflow,
    /// Starting time for tracking duration
    pub start_time: Instant,
    /// Working directory for temporary files
    pub work_dir: PathBuf,
    /// Output directory for results
    pub output_dir: PathBuf,
    /// Whether cancellation has been requested
    pub is_cancelled: bool,
}

/// Result of a workflow execution
#[derive(Debug, Clone)]
pub struct WorkflowResult {
    /// Final workflow state
    pub state: WorkflowState,
    /// Total execution time
    pub duration: Duration,
    /// Extraction summary (if extraction was performed)
    pub extraction_summary: Option<ExtractionSummary>,
    /// Processing summary (if processing was performed)
    pub processing_summary: Option<ProcessingSummary>,
    /// Reporting summary (if reporting was performed)
    pub reporting_summary: Option<ReportingSummary>,
    /// Any errors that occurred
    pub errors: Vec<WorkflowError>,
    /// Output files produced
    pub output_files: Vec<PathBuf>,
}

/// Trait for components that can handle specific workflow stages
#[async_trait]
pub trait WorkflowHandler: Send + Sync {
    /// Get the name of this handler
    fn name(&self) -> &str;

    /// Check if this handler can process the given workflow type
    fn can_handle(&self, workflow_type: &WorkflowType) -> bool;

    /// Execute the workflow stage
    async fn execute(&self, context: &WorkflowContext) -> Result<WorkflowStageResult>;

    /// Validate the workflow configuration before execution
    async fn validate(&self, _workflow: &Workflow) -> Result<()> {
        // Default implementation does no validation
        Ok(())
    }

    /// Handle cancellation request
    async fn cancel(&self) -> Result<()> {
        // Default implementation does nothing
        Ok(())
    }
}

/// Result from a single workflow stage
#[derive(Debug, Clone)]
pub struct WorkflowStageResult {
    /// Stage that was executed
    pub stage: String,
    /// Whether the stage succeeded
    pub success: bool,
    /// Execution time for this stage
    pub duration: Duration,
    /// Stage-specific summary data
    pub summary: WorkflowStageSummary,
    /// Files produced by this stage
    pub output_files: Vec<PathBuf>,
    /// Any warnings generated
    pub warnings: Vec<String>,
}

/// Summary data specific to different workflow stages
#[derive(Debug, Clone)]
pub enum WorkflowStageSummary {
    Extraction(ExtractionSummary),
    Processing(ProcessingSummary),
    Reporting(ReportingSummary),
    Export {
        exported_files: usize,
        export_format: String,
    },
}

/// Progress callback for workflow execution
pub type ProgressCallback = Box<dyn Fn(WorkflowProgress) + Send + Sync>;

/// Main orchestrator for coordinating workflow execution
pub struct WorkflowOrchestrator {
    /// Registered workflow handlers
    handlers: Vec<Box<dyn WorkflowHandler>>,
    /// Progress callback
    progress_callback: Option<ProgressCallback>,
    /// Default timeout for workflows
    default_timeout: Duration,
}

impl WorkflowOrchestrator {
    /// Create a new workflow orchestrator
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            progress_callback: None,
            default_timeout: Duration::from_secs(3600), // 1 hour default
        }
    }

    /// Register a workflow handler
    pub fn register_handler(&mut self, handler: Box<dyn WorkflowHandler>) {
        info!("Registering workflow handler: {}", handler.name());
        self.handlers.push(handler);
    }

    /// Set progress callback
    pub fn set_progress_callback(&mut self, callback: ProgressCallback) {
        self.progress_callback = Some(callback);
    }

    /// Set default timeout for workflows
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }

    /// Execute a complete workflow
    pub async fn execute_workflow(
        &self,
        workflow: Workflow,
        output_dir: PathBuf,
    ) -> Result<WorkflowResult> {
        let start_time = Instant::now();
        let work_dir = tempfile::tempdir()
            .map_err(|e| WorkflowError::io_error(PathBuf::new(), e.to_string()))?
            .keep();

        let context = WorkflowContext {
            workflow: workflow.clone(),
            start_time,
            work_dir,
            output_dir,
            is_cancelled: false,
        };

        let mut result = WorkflowResult {
            state: WorkflowState::Running,
            duration: Duration::default(),
            extraction_summary: None,
            processing_summary: None,
            reporting_summary: None,
            errors: Vec::new(),
            output_files: Vec::new(),
        };

        // Report initial progress
        if let Some(ref callback) = self.progress_callback {
            callback(WorkflowProgress {
                state: WorkflowState::Running,
                percentage: 0.0,
                stage: "Starting workflow".to_string(),
                elapsed: Duration::default(),
                eta: Some(self.default_timeout),
            });
        }

        // Validate workflow configuration
        if let Err(e) = self.validate_workflow(&workflow).await {
            result.state = WorkflowState::Failed;
            result.errors.push(e);
            return Ok(result);
        }

        // Execute workflow stages based on type
        let stages = self.determine_workflow_stages(&workflow.workflow_type);
        let total_stages = stages.len();

        for (stage_index, stage_type) in stages.iter().enumerate() {
            // Check for cancellation
            if context.is_cancelled {
                result.state = WorkflowState::Cancelled;
                break;
            }

            // Find handler for this stage
            let handler = self.find_handler_for_stage(stage_type)?;

            // Report progress
            let progress_percentage = (stage_index as f64 / total_stages as f64) * 100.0;
            if let Some(ref callback) = self.progress_callback {
                callback(WorkflowProgress {
                    state: WorkflowState::Running,
                    percentage: progress_percentage,
                    stage: format!("Executing {}", handler.name()),
                    elapsed: start_time.elapsed(),
                    eta: None,
                });
            }

            // Execute stage
            info!("Executing workflow stage: {}", handler.name());
            match handler.execute(&context).await {
                Ok(stage_result) => {
                    info!("Stage {} completed successfully in {:?}", 
                          handler.name(), stage_result.duration);
                    
                    // Store stage results
                    match stage_result.summary {
                        WorkflowStageSummary::Extraction(summary) => {
                            result.extraction_summary = Some(summary);
                        }
                        WorkflowStageSummary::Processing(summary) => {
                            result.processing_summary = Some(summary);
                        }
                        WorkflowStageSummary::Reporting(summary) => {
                            result.reporting_summary = Some(summary);
                        }
                        WorkflowStageSummary::Export { .. } => {
                            // Export summary is handled separately
                        }
                    }

                    result.output_files.extend(stage_result.output_files);
                }
                Err(e) => {
                    error!("Stage {} failed: {}", handler.name(), e);
                    result.errors.push(e);
                    result.state = WorkflowState::Failed;
                    break;
                }
            }
        }

        // Finalize result
        result.duration = start_time.elapsed();
        if result.state == WorkflowState::Running {
            result.state = WorkflowState::Completed;
        }

        // Report final progress
        if let Some(ref callback) = self.progress_callback {
            callback(WorkflowProgress {
                state: result.state.clone(),
                percentage: 100.0,
                stage: "Workflow completed".to_string(),
                elapsed: result.duration,
                eta: None,
            });
        }

        info!("Workflow completed with state {:?} in {:?}", 
              result.state, result.duration);

        Ok(result)
    }

    /// Cancel a running workflow
    pub async fn cancel_workflow(&mut self) -> Result<()> {
        info!("Cancelling workflow execution");
        
        // Cancel all handlers
        for handler in &self.handlers {
            if let Err(e) = handler.cancel().await {
                warn!("Failed to cancel handler {}: {}", handler.name(), e);
            }
        }

        Ok(())
    }

    /// Validate workflow configuration
    async fn validate_workflow(&self, workflow: &Workflow) -> Result<()> {
        info!("Validating workflow configuration");

        // Basic validation
        if workflow.name.is_empty() {
            return Err(WorkflowError::validation_error("Workflow name cannot be empty"));
        }

        // Validate that we have handlers for all required stages
        let stages = self.determine_workflow_stages(&workflow.workflow_type);
        for stage in stages {
            if self.find_handler_for_stage(&stage).is_err() {
                return Err(WorkflowError::validation_error(
                    format!("No handler available for stage: {:?}", stage)
                ));
            }
        }

        // Validate with individual handlers
        for handler in &self.handlers {
            if handler.can_handle(&workflow.workflow_type) {
                handler.validate(workflow).await?;
            }
        }

        Ok(())
    }

    /// Determine which stages need to be executed for a workflow type
    fn determine_workflow_stages(&self, workflow_type: &WorkflowType) -> Vec<WorkflowStageType> {
        match workflow_type {
            WorkflowType::Extract => vec![WorkflowStageType::Extract],
            WorkflowType::Process => vec![WorkflowStageType::Process],
            WorkflowType::Report => vec![WorkflowStageType::Report],
            WorkflowType::Export => vec![WorkflowStageType::Export],
            WorkflowType::ExtractAndProcess => vec![
                WorkflowStageType::Extract,
                WorkflowStageType::Process,
            ],
            WorkflowType::ProcessAndReport => vec![
                WorkflowStageType::Process,
                WorkflowStageType::Report,
            ],
            WorkflowType::Complete => vec![
                WorkflowStageType::Extract,
                WorkflowStageType::Process,
                WorkflowStageType::Report,
                WorkflowStageType::Export,
            ],
        }
    }

    /// Find a handler capable of processing the given stage
    fn find_handler_for_stage(&self, stage: &WorkflowStageType) -> Result<&dyn WorkflowHandler> {
        // For now, map stage types to workflow types for handler lookup
        let workflow_type = match stage {
            WorkflowStageType::Extract => WorkflowType::Extract,
            WorkflowStageType::Process => WorkflowType::Process,
            WorkflowStageType::Report => WorkflowType::Report,
            WorkflowStageType::Export => WorkflowType::Complete, // Export is part of complete workflow
        };

        self.handlers
            .iter()
            .find(|h| h.can_handle(&workflow_type))
            .map(|h| h.as_ref())
            .ok_or_else(|| WorkflowError::configuration_error(
                format!("No handler found for stage: {:?}", stage)
            ))
    }
}

impl Default for WorkflowOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of workflow stages
#[derive(Debug, Clone, PartialEq)]
enum WorkflowStageType {
    Extract,
    Process,
    Report,
    Export,
}

impl WorkflowResult {
    /// Check if the workflow was successful
    pub fn is_successful(&self) -> bool {
        self.state == WorkflowState::Completed && self.errors.is_empty()
    }

    /// Get total number of errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get summary of all operations
    pub fn get_summary(&self) -> String {
        let mut summary = format!(
            "Workflow completed in {:?} with state: {:?}",
            self.duration, self.state
        );

        if let Some(ref extraction) = self.extraction_summary {
            summary.push_str(&format!(
                "\nExtraction: {} PBOs processed",
                extraction.extracted_pbos
            ));
        }

        if let Some(ref processing) = self.processing_summary {
            summary.push_str(&format!(
                "\nProcessing: {} files, {} entries found",
                processing.files_processed, processing.entries_found
            ));
        }

        if let Some(ref reporting) = self.reporting_summary {
            summary.push_str(&format!(
                "\nReporting: {} reports generated",
                reporting.reports_generated
            ));
        }

        if !self.errors.is_empty() {
            summary.push_str(&format!("\nErrors: {}", self.errors.len()));
        }

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{WorkflowOptions, ExtractionOptions, ProcessingOptions, ReportingOptions};
    use crate::ContentType;

    #[tokio::test]
    async fn test_orchestrator_creation() {
        let orchestrator = WorkflowOrchestrator::new();
        assert_eq!(orchestrator.handlers.len(), 0);
    }

    #[tokio::test]
    async fn test_workflow_validation() {
        let orchestrator = WorkflowOrchestrator::new();
        let workflow = Workflow::new(
            "test".to_string(),
            WorkflowType::Extract,
            ContentType::GameData,
            WorkflowOptions::default(),
        );

        // Should fail because no handlers are registered
        let result = orchestrator.validate_workflow(&workflow).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_workflow_result() {
        let result = WorkflowResult {
            state: WorkflowState::Completed,
            duration: Duration::from_secs(10),
            extraction_summary: None,
            processing_summary: None,
            reporting_summary: None,
            errors: Vec::new(),
            output_files: Vec::new(),
        };

        assert!(result.is_successful());
        assert_eq!(result.error_count(), 0);
    }

    #[test]
    fn test_workflow_stages() {
        let orchestrator = WorkflowOrchestrator::new();
        
        let extract_stages = orchestrator.determine_workflow_stages(&WorkflowType::Extract);
        assert_eq!(extract_stages.len(), 1);
        assert_eq!(extract_stages[0], WorkflowStageType::Extract);

        let complete_stages = orchestrator.determine_workflow_stages(&WorkflowType::Complete);
        assert_eq!(complete_stages.len(), 4);
    }
}