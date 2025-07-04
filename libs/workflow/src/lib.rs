pub mod types;
pub mod error;
pub mod orchestrator;
pub mod extract;
pub mod process;
pub mod report;
pub mod export;

// Re-export key types for convenience
pub use types::workflow::{WorkflowType, ContentType, Workflow};
pub use types::options::{WorkflowOptions, ExtractionOptions, ProcessingOptions, ReportingOptions, ReportFormat, ExportOptions};
pub use types::summary::{ExtractionSummary, ProcessingSummary, ReportingSummary, ExportSummary};

// Re-export orchestrator types
pub use orchestrator::{
    WorkflowOrchestrator, WorkflowHandler, WorkflowContext, WorkflowResult, 
    WorkflowProgress, WorkflowState, WorkflowStageResult, WorkflowStageSummary
};

// Re-export error types
pub use error::{WorkflowError, Result};

// Re-export handler types
pub use extract::{ExtractWorkflowHandler, ExtractorInterface};
pub use process::{ProcessWorkflowHandler, ScannerInterface};
pub use report::{ReportWorkflowHandler, ReporterInterface};
pub use export::{ExportWorkflowHandler, ExporterInterface};