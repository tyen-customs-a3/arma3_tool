use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;
use std::collections::HashSet;

pub mod mission;
pub mod class;
pub mod dependency;
pub mod formats;
pub mod examples;
pub mod mission_reports;
pub mod class_reports;

pub use mission::MissionReportWriter;
pub use class::ClassReportWriter;
pub use dependency::DependencyReportWriter;
pub use formats::ReportFormat;
pub use examples::*;
pub use mission_reports::MissionReportManager;
pub use mission_reports::DependencyReportManager;
pub use class_reports::ClassReportManager;

/// Configuration for controlling which reports are generated
#[derive(Debug, Clone, Default)]
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
            !self.disabled_reports.contains(report_type)
        } else {
            self.disabled_reports.contains(report_type)
        }
    }
}

/// Base trait for all report writers
pub trait ReportWriter {
    /// Write a report to the specified output directory
    fn write_report<T: Serialize + ?Sized>(&self, data: &T, filename: &str) -> Result<PathBuf>;
}

/// Base implementation for report writers
pub struct BaseReportWriter {
    output_dir: PathBuf,
    format: ReportFormat,
    config: ReportConfig,
}

impl BaseReportWriter {
    pub fn new(output_dir: &Path, format: ReportFormat) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            format,
            config: ReportConfig::new(),
        }
    }
    
    /// Create a new BaseReportWriter with a specific configuration
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
    
    /// Get a reference to the report configuration
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
        // Ensure output directory exists
        self.ensure_output_dir()?;
        
        // Create the full output path with appropriate extension
        let output_path = self.output_dir.join(format!("{}.{}", 
            filename, 
            self.format.extension()));
        
        // Serialize the data based on the format
        let content = match self.format {
            ReportFormat::Json => serde_json::to_string_pretty(data)
                .context("Failed to serialize data to JSON")?,
            ReportFormat::Yaml => serde_yaml::to_string(data)
                .context("Failed to serialize data to YAML")?,
            ReportFormat::Toml => toml::to_string_pretty(data)
                .context("Failed to serialize data to TOML")?,
        };
        
        // Write the file
        fs::write(&output_path, content)
            .context(format!("Failed to write report to {}", output_path.display()))?;
        
        debug!("Wrote report to {}", output_path.display());
        
        Ok(output_path)
    }
}

// Helper function to sanitize filenames for reports
pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
} 