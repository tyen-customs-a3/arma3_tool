use anyhow::Result;
use std::path::Path;
use std::fs;
use crate::models::ScanResult;
use super::formatters::{JsonFormatter, YamlFormatter, CsvFormatter, TextFormatter, ReportFormatter};

/// Main report generator that coordinates different output formats
pub struct ReportGenerator {
    json_formatter: JsonFormatter,
    yaml_formatter: YamlFormatter,
    csv_formatter: CsvFormatter,
    text_formatter: TextFormatter,
}

impl ReportGenerator {
    pub fn new() -> Self {
        Self {
            json_formatter: JsonFormatter::new(),
            yaml_formatter: YamlFormatter::new(),
            csv_formatter: CsvFormatter::new(),
            text_formatter: TextFormatter::new(),
        }
    }

    /// Generate a report in the specified format
    pub fn generate(&self, result: &ScanResult, output: &Path, format: &str) -> Result<()> {
        // Create parent directory if needed
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = match format.to_lowercase().as_str() {
            "json" => self.json_formatter.format(result)?,
            "yaml" | "yml" => self.yaml_formatter.format(result)?,
            "csv" => self.csv_formatter.format(result)?,
            "txt" | "text" => self.text_formatter.format(result)?,
            _ => anyhow::bail!("Unsupported format: {}", format),
        };

        fs::write(output, content)?;
        log::info!("Report generated: {} ({})", output.display(), format);
        Ok(())
    }

    /// Generate a report in the specified format with project root context
    pub fn generate_with_project_root(&self, result: &ScanResult, output: &Path, format: &str, project_root: &Path) -> Result<()> {
        // Create parent directory if needed
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = match format.to_lowercase().as_str() {
            "json" => self.json_formatter.format_with_project_root(result, project_root)?,
            "yaml" | "yml" => self.yaml_formatter.format(result)?,
            "csv" => self.csv_formatter.format(result)?,
            "txt" | "text" => self.text_formatter.format(result)?,
            _ => anyhow::bail!("Unsupported format: {}", format),
        };

        fs::write(output, content)?;
        log::info!("Report generated: {} ({})", output.display(), format);
        Ok(())
    }

    /// Generate multiple formats at once
    pub fn generate_multiple(&self, result: &ScanResult, base_path: &Path, formats: &[&str]) -> Result<()> {
        for format in formats {
            let extension = match *format {
                "yaml" | "yml" => "yaml",
                "txt" | "text" => "txt",
                other => other,
            };
            
            let output_path = base_path.with_extension(extension);
            self.generate(result, &output_path, format)?;
        }
        Ok(())
    }

    /// Get available formats
    pub fn available_formats() -> Vec<&'static str> {
        vec!["json", "yaml", "csv", "text"]
    }

    /// Validate format
    pub fn is_valid_format(format: &str) -> bool {
        Self::available_formats().contains(&format.to_lowercase().as_str())
    }

    /// Get recommended format based on use case
    pub fn recommend_format(use_case: &str) -> &'static str {
        match use_case.to_lowercase().as_str() {
            "api" | "data" | "interchange" => "json",
            "config" | "human" | "readable" => "yaml",
            "spreadsheet" | "excel" | "analysis" => "csv",
            "simple" | "plain" | "console" => "text",
            _ => "json", // Default
        }
    }
}
