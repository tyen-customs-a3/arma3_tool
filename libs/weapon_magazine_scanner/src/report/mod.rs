//! Report generation module
//! 
//! This module provides multiple output formats for scan results:
//! - JSON for data interchange
//! - YAML for human-readable configuration
//! - CSV for spreadsheet analysis
//! - Text for simple viewing

pub mod generator;
pub mod formatters;
pub mod weapons_by_mod;

// Re-export main types
pub use generator::ReportGenerator;
pub use formatters::{JsonFormatter, YamlFormatter, CsvFormatter, TextFormatter, ReportFormatter};
pub use weapons_by_mod::{WeaponsByModExporter, ModStatistics};
