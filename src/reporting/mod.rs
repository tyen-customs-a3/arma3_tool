use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;
use std::collections::HashSet;

pub mod mission_report_writer;
pub mod class_report_writer;
pub mod dependency_report_writer;
pub mod formats;
pub mod mission_reports;
pub mod class_report_manager;
pub mod missing_classes_report;
pub mod class_tree_report;

pub use formats::ReportFormat;
pub use mission_report_writer::MissionReportWriter;
pub use class_report_writer::ClassReportWriter;
pub use dependency_report_writer::DependencyReportWriter;
pub use mission_reports::MissionReportManager;
pub use mission_reports::DependencyReportManager;
pub use class_report_manager::ClassReportManager;
pub use class_tree_report::ClassTreeReportWriter;

/// Report configuration
#[derive(Debug, Clone)]
pub struct ReportConfig {
    /// Set of disabled report types
    disabled_reports: HashSet<String>,
    /// Whether all reports are enabled (default) or disabled
    default_enabled: bool,
}

impl ReportConfig {
    /// Create a new report configuration with all reports enabled by default
    pub fn new() -> Self {
        Self {
            disabled_reports: HashSet::new(),
            default_enabled: true,
        }
    }
    
    /// Create a new report configuration with all reports disabled by default
    pub fn all_disabled() -> Self {
        Self {
            disabled_reports: HashSet::new(),
            default_enabled: false,
        }
    }
    
    /// Disable a specific report type
    pub fn disable(&mut self, report_type: &str) -> &mut Self {
        self.disabled_reports.insert(report_type.to_string());
        self
    }
    
    /// Enable a specific report type
    pub fn enable(&mut self, report_type: &str) -> &mut Self {
        self.disabled_reports.remove(report_type);
        self
    }
    
    /// Check if a report type is enabled
    pub fn is_enabled(&self, report_type: &str) -> bool {
        if self.default_enabled {
            // By default, all reports are enabled unless explicitly disabled
            !self.disabled_reports.contains(report_type)
        } else {
            // By default, all reports are disabled unless explicitly enabled
            !self.disabled_reports.contains(report_type)
        }
    }
}

/// Trait for report writers
pub trait ReportWriter {
    /// Write a report to the specified output directory
    fn write_report<T: Serialize + ?Sized>(&self, data: &T, filename: &str) -> Result<PathBuf>;
}

/// Base report writer implementation
pub struct BaseReportWriter {
    output_dir: PathBuf,
    format: ReportFormat,
    config: ReportConfig,
}

impl BaseReportWriter {
    /// Create a new report writer with the specified output directory and format
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            format: ReportFormat::default(),
            config: ReportConfig::new(),
        }
    }
    
    /// Create a new report writer with the specified output directory, format, and configuration
    pub fn with_config(output_dir: &Path, format: ReportFormat, config: ReportConfig) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            format,
            config,
        }
    }
    
    /// Ensure the output directory exists
    pub fn ensure_output_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))
    }
    
    /// Get the output directory
    pub fn output_dir(&self) -> &Path {
        &self.output_dir
    }
    
    /// Get the report format
    pub fn format(&self) -> ReportFormat {
        self.format
    }
    
    /// Get the report configuration
    pub fn config(&self) -> &ReportConfig {
        &self.config
    }
    
    /// Get a mutable reference to the report configuration
    pub fn config_mut(&mut self) -> &mut ReportConfig {
        &mut self.config
    }
    
    /// Check if a report type is enabled
    pub fn is_report_enabled(&self, report_type: &str) -> bool {
        self.config.is_enabled(report_type)
    }
}

impl ReportWriter for BaseReportWriter {
    fn write_report<T: Serialize + ?Sized>(&self, data: &T, filename: &str) -> Result<PathBuf> {
        // Ensure the output directory exists
        self.ensure_output_dir()?;
        
        // Create the full path with the appropriate extension
        let mut path = self.output_dir.join(filename);
        path.set_extension(self.format.extension());
        
        // Serialize the data based on the format
        let serialized = match self.format {
            ReportFormat::Json => serde_json::to_string_pretty(data)
                .context(format!("Failed to serialize data to JSON for {}", path.display()))?,
            ReportFormat::Yaml => serde_yaml::to_string(data)
                .context(format!("Failed to serialize data to YAML for {}", path.display()))?,
            ReportFormat::Toml => toml::to_string(data)
                .context(format!("Failed to serialize data to TOML for {}", path.display()))?,
        };
        
        // Write the data to the file
        fs::write(&path, serialized)
            .context(format!("Failed to write report to {}", path.display()))?;
        
        debug!("Wrote report to {}", path.display());
        
        Ok(path)
    }
}

/// Sanitize a filename to be safe for use in a file path
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
} 