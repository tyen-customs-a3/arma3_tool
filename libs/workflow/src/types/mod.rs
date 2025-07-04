pub mod workflow;
pub mod options;
pub mod summary;

// Re-export all types for convenience
pub use workflow::{WorkflowType, ContentType, Workflow};
pub use options::{WorkflowOptions, ExtractionOptions, ProcessingOptions, ReportingOptions, ReportFormat, ExportOptions};
pub use summary::{ExtractionSummary, ProcessingSummary, ReportingSummary, ExportSummary};