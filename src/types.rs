// This file previously contained workflow types that have been moved to libs/workflow
// Re-export workflow types for backward compatibility
pub use arma3_workflow::{
    WorkflowType, ContentType, Workflow,
    WorkflowOptions, ExtractionOptions, ProcessingOptions, ReportingOptions, ReportFormat,
    ExtractionSummary, ProcessingSummary, ReportingSummary,
};

// Note: This file can be removed once all code has been updated to import
// workflow types directly from arma3_workflow instead of crate::types