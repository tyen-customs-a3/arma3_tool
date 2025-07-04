pub mod cli;
pub mod error;

// Re-export configuration from arma3_config
pub use arma3_config::{ScanConfig, ItemFilterConfig, VisualizationConfig};

// Re-export workflow types from arma3_workflow
pub use arma3_workflow::{
    WorkflowType, ContentType, Workflow,
    WorkflowOptions, ExtractionOptions, ProcessingOptions, ReportingOptions, ReportFormat,
    ExtractionSummary, ProcessingSummary, ReportingSummary,
    WorkflowError, Result as WorkflowResult,
};

pub use error::Result;