use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Options for the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowOptions {
    /// Extraction options
    pub extraction: ExtractionOptions,
    
    /// Processing options
    pub processing: ProcessingOptions,
    
    /// Reporting options
    pub reporting: ReportingOptions,
    
    /// Export options
    pub export: ExportOptions,
}

impl WorkflowOptions {
    /// Create new workflow options with defaults
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create workflow options with custom extraction settings
    pub fn with_extraction(mut self, extraction: ExtractionOptions) -> Self {
        self.extraction = extraction;
        self
    }
    
    /// Create workflow options with custom processing settings
    pub fn with_processing(mut self, processing: ProcessingOptions) -> Self {
        self.processing = processing;
        self
    }
    
    /// Create workflow options with custom reporting settings
    pub fn with_reporting(mut self, reporting: ReportingOptions) -> Self {
        self.reporting = reporting;
        self
    }
    
    /// Enable verbose mode for all options
    pub fn verbose(mut self) -> Self {
        self.processing.verbose = true;
        self
    }
    
    /// Enable force mode for extraction
    pub fn force_extraction(mut self) -> Self {
        self.extraction.force = true;
        self
    }
}

impl Default for WorkflowOptions {
    fn default() -> Self {
        Self {
            extraction: ExtractionOptions::default(),
            processing: ProcessingOptions::default(),
            reporting: ReportingOptions::default(),
            export: ExportOptions::default(),
        }
    }
}

/// Options for extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionOptions {
    /// Whether to use extractor
    pub use_extractor: bool,
    
    /// Specific directories to process
    pub directories: Option<Vec<String>>,
    
    /// Source directories for extraction
    pub source_directories: Vec<PathBuf>,
    
    /// Whether to force re-extraction
    pub force: bool,
}

impl ExtractionOptions {
    /// Create new extraction options
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set specific directories to extract
    pub fn with_directories(mut self, directories: Vec<String>) -> Self {
        self.directories = Some(directories);
        self
    }
    
    /// Enable force re-extraction
    pub fn with_force(mut self) -> Self {
        self.force = true;
        self
    }
    
    /// Disable extractor
    pub fn without_extractor(mut self) -> Self {
        self.use_extractor = false;
        self
    }
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            use_extractor: true,
            directories: None,
            source_directories: Vec::new(),
            force: false,
        }
    }
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
    
    /// Source directories for processing
    pub source_directories: Vec<PathBuf>,
    
    /// Enable verbose output
    pub verbose: bool,
}

impl ProcessingOptions {
    /// Create new processing options
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set maximum number of files per PBO
    pub fn with_max_files(mut self, max_files: usize) -> Self {
        self.max_files = max_files;
        self
    }
    
    /// Set maximum processing depth
    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }
    
    /// Set file extensions to process
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }
    
    /// Enable verbose output
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    
    /// Add a file extension to process
    pub fn add_extension(mut self, extension: String) -> Self {
        self.extensions.push(extension);
        self
    }
    
    /// Get common game data extensions
    pub fn game_data_extensions() -> Vec<String> {
        vec![
            "cpp".to_string(),
            "hpp".to_string(),
            "cfg".to_string(),
            "ext".to_string(),
        ]
    }
    
    /// Get common mission extensions
    pub fn mission_extensions() -> Vec<String> {
        vec![
            "sqf".to_string(),
            "sqm".to_string(),
            "hpp".to_string(),
            "ext".to_string(),
        ]
    }
}

impl Default for ProcessingOptions {
    fn default() -> Self {
        Self {
            max_files: 1000,
            max_depth: 10,
            extensions: vec![
                "cpp".to_string(),
                "hpp".to_string(),
                "sqf".to_string(),
                "sqm".to_string(),
                "cfg".to_string(),
                "ext".to_string(),
            ],
            source_directories: Vec::new(),
            verbose: false,
        }
    }
}

/// Format for reports
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReportFormat {
    PlainText,
    Markdown,
    Html,
    Json,
    Csv,
}

impl ReportFormat {
    /// Get file extension for this format
    pub fn file_extension(&self) -> &'static str {
        match self {
            ReportFormat::PlainText => "txt",
            ReportFormat::Markdown => "md",
            ReportFormat::Html => "html",
            ReportFormat::Json => "json",
            ReportFormat::Csv => "csv",
        }
    }
    
    /// Get all available formats
    pub fn all() -> Vec<ReportFormat> {
        vec![
            ReportFormat::PlainText,
            ReportFormat::Markdown,
            ReportFormat::Html,
            ReportFormat::Json,
            ReportFormat::Csv,
        ]
    }
}

impl std::fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportFormat::PlainText => write!(f, "Plain Text"),
            ReportFormat::Markdown => write!(f, "Markdown"),
            ReportFormat::Html => write!(f, "HTML"),
            ReportFormat::Json => write!(f, "JSON"),
            ReportFormat::Csv => write!(f, "CSV"),
        }
    }
}

/// Options for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingOptions {
    /// Report format
    pub format: ReportFormat,
    
    /// Output path (optional)
    pub output_path: Option<PathBuf>,
    
    /// Source directories for reporting
    pub source_directories: Vec<PathBuf>,
    
    /// Include mission dependencies in report
    pub include_mission_dependencies: bool,
    
    /// Include game data classes in report
    pub include_game_data_classes: bool,
    
    /// Include missing dependencies in report
    pub include_missing_dependencies: bool,
}

impl ReportingOptions {
    /// Create new reporting options
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set report format
    pub fn with_format(mut self, format: ReportFormat) -> Self {
        self.format = format;
        self
    }
    
    /// Set output path
    pub fn with_output_path(mut self, path: PathBuf) -> Self {
        self.output_path = Some(path);
        self
    }
    
    /// Include mission dependencies
    pub fn include_mission_dependencies(mut self) -> Self {
        self.include_mission_dependencies = true;
        self
    }
    
    /// Include game data classes
    pub fn include_game_data_classes(mut self) -> Self {
        self.include_game_data_classes = true;
        self
    }
    
    /// Include missing dependencies
    pub fn include_missing_dependencies(mut self) -> Self {
        self.include_missing_dependencies = true;
        self
    }
    
    /// Include all possible report data
    pub fn include_all(mut self) -> Self {
        self.include_mission_dependencies = true;
        self.include_game_data_classes = true;
        self.include_missing_dependencies = true;
        self
    }
    
    /// Generate output filename based on format
    pub fn generate_filename(&self, base_name: &str) -> String {
        format!("{}.{}", base_name, self.format.file_extension())
    }
}

impl Default for ReportingOptions {
    fn default() -> Self {
        Self {
            format: ReportFormat::Markdown,
            output_path: None,
            source_directories: Vec::new(),
            include_mission_dependencies: true,
            include_game_data_classes: false,
            include_missing_dependencies: true,
        }
    }
}

/// Options for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    /// Export format
    pub format: String,
    
    /// Output path (optional)
    pub output_path: Option<PathBuf>,
    
    /// Compression level (0-9, where applicable)
    pub compression_level: Option<u8>,
    
    /// Include metadata in export
    pub include_metadata: bool,
}

impl ExportOptions {
    /// Create new export options
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set export format
    pub fn with_format(mut self, format: String) -> Self {
        self.format = format;
        self
    }
    
    /// Set output path
    pub fn with_output_path(mut self, path: PathBuf) -> Self {
        self.output_path = Some(path);
        self
    }
    
    /// Set compression level
    pub fn with_compression(mut self, level: u8) -> Self {
        self.compression_level = Some(level);
        self
    }
    
    /// Include metadata in export
    pub fn include_metadata(mut self) -> Self {
        self.include_metadata = true;
        self
    }
    
    /// Get supported export formats
    pub fn supported_formats() -> Vec<String> {
        vec![
            "zip".to_string(),
            "tar".to_string(),
            "json".to_string(),
            "csv".to_string(),
        ]
    }
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: "zip".to_string(),
            output_path: None,
            compression_level: Some(6),
            include_metadata: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workflow_options_builder() {
        let options = WorkflowOptions::new()
            .verbose()
            .force_extraction();
        
        assert!(options.processing.verbose);
        assert!(options.extraction.force);
    }
    
    #[test]
    fn test_extraction_options_builder() {
        let options = ExtractionOptions::new()
            .with_directories(vec!["dir1".to_string(), "dir2".to_string()])
            .with_force()
            .without_extractor();
        
        assert_eq!(options.directories, Some(vec!["dir1".to_string(), "dir2".to_string()]));
        assert!(options.force);
        assert!(!options.use_extractor);
    }
    
    #[test]
    fn test_processing_options_builder() {
        let options = ProcessingOptions::new()
            .with_max_files(500)
            .with_max_depth(5)
            .verbose()
            .add_extension("test".to_string());
        
        assert_eq!(options.max_files, 500);
        assert_eq!(options.max_depth, 5);
        assert!(options.verbose);
        assert!(options.extensions.contains(&"test".to_string()));
    }
    
    #[test]
    fn test_reporting_options_builder() {
        let options = ReportingOptions::new()
            .with_format(ReportFormat::Json)
            .with_output_path(PathBuf::from("test.json"))
            .include_all();
        
        assert_eq!(options.format, ReportFormat::Json);
        assert_eq!(options.output_path, Some(PathBuf::from("test.json")));
        assert!(options.include_mission_dependencies);
        assert!(options.include_game_data_classes);
        assert!(options.include_missing_dependencies);
    }
    
    #[test]
    fn test_report_format_extensions() {
        assert_eq!(ReportFormat::PlainText.file_extension(), "txt");
        assert_eq!(ReportFormat::Markdown.file_extension(), "md");
        assert_eq!(ReportFormat::Html.file_extension(), "html");
        assert_eq!(ReportFormat::Json.file_extension(), "json");
        assert_eq!(ReportFormat::Csv.file_extension(), "csv");
    }
    
    #[test]
    fn test_filename_generation() {
        let options = ReportingOptions::new().with_format(ReportFormat::Json);
        assert_eq!(options.generate_filename("report"), "report.json");
        
        let options = ReportingOptions::new().with_format(ReportFormat::Markdown);
        assert_eq!(options.generate_filename("analysis"), "analysis.md");
    }
    
    #[test]
    fn test_default_extensions() {
        let game_data_ext = ProcessingOptions::game_data_extensions();
        assert!(game_data_ext.contains(&"cpp".to_string()));
        assert!(game_data_ext.contains(&"hpp".to_string()));
        
        let mission_ext = ProcessingOptions::mission_extensions();
        assert!(mission_ext.contains(&"sqf".to_string()));
        assert!(mission_ext.contains(&"sqm".to_string()));
    }
}