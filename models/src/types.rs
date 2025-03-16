use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::time::Duration;

/// Type for processing workflow
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WorkflowType {
    /// Extract only
    Extract,
    /// Process only
    Process,
    /// Generate reports only
    Report,
    /// Extract and process
    ExtractAndProcess,
    /// Process and report
    ProcessAndReport,
    /// Complete workflow (extract, process, report)
    Complete,
}

/// Type of content to work with
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ContentType {
    /// Game data
    GameData,
    /// Mission data
    Mission,
    /// Both game data and mission data
    Both,
}

/// Workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Name of the workflow
    pub name: String,
    
    /// Type of workflow
    pub workflow_type: WorkflowType,
    
    /// Type of content to process
    pub content_type: ContentType,
    
    /// Processing options
    pub options: WorkflowOptions,
}

/// Options for the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowOptions {
    /// Extraction options
    pub extraction: ExtractionOptions,
    
    /// Processing options
    pub processing: ProcessingOptions,
    
    /// Reporting options
    pub reporting: ReportingOptions,
}

/// Options for extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionOptions {
    /// Whether to use pbo_cache
    pub use_pbo_cache: bool,
    
    /// Specific directories to process
    pub directories: Option<Vec<String>>,
    
    /// Whether to force re-extraction
    pub force: bool,
}

/// Options for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingOptions {
    /// Maximum number of files to process per PBO
    pub max_files: usize,
    
    /// Maximum depth to recursively process directories
    pub max_depth: usize,
    
    /// File extensions to process
    pub extensions: Vec<String>,
    
    /// Enable verbose output
    pub verbose: bool,
}

/// Options for reporting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReportFormat {
    PlainText,
    Markdown,
    Html,
    Json,
    Csv,
}

/// Options for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingOptions {
    /// Report format
    pub format: ReportFormat,
    
    /// Output path (optional)
    pub output_path: Option<PathBuf>,
    
    /// Include mission dependencies in report
    pub include_mission_dependencies: bool,
    
    /// Include game data classes in report
    pub include_game_data_classes: bool,
    
    /// Include missing dependencies in report
    pub include_missing_dependencies: bool,
}

/// Summary of a workflow execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowSummary {
    /// Workflow that was executed
    pub workflow: Workflow,
    
    /// Extraction summary
    pub extraction: Option<ExtractionSummary>,
    
    /// Processing summary
    pub processing: Option<ProcessingSummary>,
    
    /// Reporting summary
    pub reporting: Option<ReportingSummary>,
    
    /// Total elapsed time
    pub elapsed_time: Duration,
    
    /// Errors encountered during workflow execution
    pub errors: Vec<String>,
}

/// Summary of extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionSummary {
    /// Number of PBOs extracted
    pub extracted_pbos: usize,
    
    /// Extraction paths
    pub extraction_paths: Vec<PathBuf>,
    
    /// Elapsed time
    pub elapsed_time: Duration,
    
    /// Errors encountered during extraction
    pub errors: Vec<String>,
}

/// Summary of processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingSummary {
    /// Number of PBOs processed
    pub processed_pbos: usize,
    
    /// Number of files processed
    pub files_processed: usize,
    
    /// Number of entries found
    pub entries_found: usize,
    
    /// Elapsed time
    pub elapsed_time: Duration,
    
    /// Errors encountered during processing
    pub errors: Vec<String>,
}

/// Summary of reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingSummary {
    /// Number of reports generated
    pub reports_generated: usize,
    
    /// Report paths
    pub report_paths: Vec<PathBuf>,
    
    /// Elapsed time
    pub elapsed_time: Duration,
    
    /// Errors encountered during reporting
    pub errors: Vec<String>,
}

/// Property value for game data and mission components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<PropertyValue>),
    Object(HashMap<String, PropertyValue>),
} 