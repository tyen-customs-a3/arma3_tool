use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

pub mod mission;
pub mod class;
pub mod dependency;
pub mod formats;

pub use mission::MissionReportWriter;
pub use class::ClassReportWriter;
pub use dependency::DependencyReportWriter;
pub use formats::ReportFormat;

/// Base trait for all report writers
pub trait ReportWriter {
    /// Write a report to the specified output directory
    fn write_report<T: Serialize + ?Sized>(&self, data: &T, filename: &str) -> Result<PathBuf>;
}

/// Base implementation for report writers
pub struct BaseReportWriter {
    output_dir: PathBuf,
    format: ReportFormat,
}

impl BaseReportWriter {
    pub fn new(output_dir: &Path, format: ReportFormat) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            format,
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