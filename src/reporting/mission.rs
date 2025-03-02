use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use super::{BaseReportWriter, ReportWriter, ReportFormat, sanitize_filename};

/// Writer for mission analysis reports
pub struct MissionReportWriter {
    base: BaseReportWriter,
}

impl MissionReportWriter {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            base: BaseReportWriter::new(output_dir, ReportFormat::Json),
        }
    }
    
    pub fn with_format(output_dir: &Path, format: ReportFormat) -> Self {
        Self {
            base: BaseReportWriter::new(output_dir, format),
        }
    }
    
    /// Write reports for multiple mission analysis results
    pub fn write_reports<T: Serialize + MissionName>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        self.base.ensure_output_dir()?;
        
        // Write individual mission reports
        for (i, result) in results.iter().enumerate() {
            self.write_mission_report(result, &format!("mission_{}", i))?;
        }
        
        // Write summary report
        self.write_summary_report(results)?;
        
        info!("Wrote {} mission reports to {}", results.len(), self.base.output_dir().display());
        
        Ok(())
    }
    
    /// Write a report for a single mission analysis result
    pub fn write_mission_report<T: Serialize + MissionName>(&self, result: &T, default_name: &str) -> Result<PathBuf> {
        let mission_name = sanitize_filename(&result.mission_name());
        let filename = if mission_name.is_empty() { default_name.to_string() } else { mission_name };
        
        let path = self.base.write_report(result, &filename)?;
        debug!("Wrote mission report for '{}' to {}", result.mission_name(), path.display());
        
        Ok(path)
    }
    
    /// Write a summary report for multiple mission analysis results
    pub fn write_summary_report<T: Serialize>(&self, results: &[T]) -> Result<PathBuf> {
        let path = self.base.write_report(results, "summary")?;
        info!("Wrote mission summary report to {}", path.display());
        
        Ok(path)
    }
}

/// Trait for types that have a mission name
pub trait MissionName {
    fn mission_name(&self) -> String;
}

/// Mission summary item for reporting
#[derive(Serialize)]
pub struct MissionSummaryItem {
    pub name: String,
    pub equipment_count: usize,
    pub vehicle_count: usize,
    pub weapon_count: usize,
    pub magazine_count: usize,
    pub item_count: usize,
    pub backpack_count: usize,
}

/// Mission summary for reporting
#[derive(Serialize)]
pub struct MissionSummary {
    pub total_missions: usize,
    pub missions: Vec<MissionSummaryItem>,
} 