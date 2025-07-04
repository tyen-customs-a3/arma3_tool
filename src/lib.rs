pub mod cli;
pub mod config;
pub mod error;
pub mod scanner;

// Re-export configuration from arma3_config
pub use arma3_config::{ScanConfig, ItemFilterConfig, VisualizationConfig};

// Re-export workflow types from arma3_workflow
pub use arma3_workflow::{
    WorkflowType, ContentType, Workflow,
    WorkflowOptions, ExtractionOptions, ProcessingOptions, ReportingOptions, ReportFormat,
    ExtractionSummary, ProcessingSummary, ReportingSummary,
};

// Re-export reporter functionality from arma3_reporter
pub use arma3_reporter::{
    ReportCoordinator, DependencyAnalyzer, ReportWriter, ClassHierarchyWriter,
    DependencyAnalysis, DependencyReport, FuzzyMissingClassReport, MissingClassMatch, PotentialMatch,
    ReporterError, Result as ReporterResult,
};

pub use error::{Result, ToolError};