//! Report formatters for different output formats

pub mod json_formatter;
pub mod yaml_formatter;
pub mod csv_formatter;
pub mod text_formatter;

// Re-export formatters
pub use json_formatter::JsonFormatter;
pub use yaml_formatter::YamlFormatter;
pub use csv_formatter::CsvFormatter;
pub use text_formatter::TextFormatter;

use anyhow::Result;
use crate::models::ScanResult;

/// Trait for report formatters
pub trait ReportFormatter {
    fn format(&self, result: &ScanResult) -> Result<String>;
    fn format_name(&self) -> &'static str;
    fn file_extension(&self) -> &'static str;
}
