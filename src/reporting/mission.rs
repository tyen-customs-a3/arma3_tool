use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;
use std::fs;

use super::{BaseReportWriter, ReportWriter, ReportFormat, sanitize_filename, ReportConfig};

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
    
    pub fn with_config(output_dir: &Path, format: ReportFormat, config: ReportConfig) -> Self {
        Self {
            base: BaseReportWriter::with_config(output_dir, format, config),
        }
    }
    
    /// Get a reference to the report configuration
    pub fn config(&self) -> &ReportConfig {
        self.base.config()
    }
    
    /// Get a mutable reference to the report configuration
    pub fn config_mut(&mut self) -> &mut ReportConfig {
        self.base.config_mut()
    }
    
    /// Write reports for multiple mission analysis results
    pub fn write_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        self.base.ensure_output_dir()?;
        
        // Write individual mission reports in separate folders
        for result in results.iter() {
            let mission_name = sanitize_filename(&result.mission_name());
            self.write_mission_folder_reports(result, &mission_name)?;
        }
        
        // Write summary report
        if self.base.is_report_enabled("mission_summary") {
            self.write_summary_report(results)?;
        } else {
            debug!("Skipping mission summary report (disabled in configuration)");
        }
        
        info!("Wrote {} mission reports to {}", results.len(), self.base.output_dir().display());
        
        Ok(())
    }
    
    /// Write reports for a single mission in a dedicated folder
    pub fn write_mission_folder_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(
        &self, 
        result: &T, 
        default_name: &str
    ) -> Result<PathBuf> {
        let mission_name = sanitize_filename(&result.mission_name());
        let folder_name = if mission_name.is_empty() { default_name.to_string() } else { mission_name };
        
        // Check if mission reports are enabled
        let report_type = format!("mission_{}", folder_name);
        if !self.base.is_report_enabled(&report_type) {
            debug!("Skipping reports for mission '{}' (disabled in configuration)", result.mission_name());
            return Ok(PathBuf::new());
        }
        
        // Create mission folder
        let mission_folder = self.base.output_dir().join(&folder_name);
        fs::create_dir_all(&mission_folder)
            .context(format!("Failed to create mission folder: {}", mission_folder.display()))?;
        
        // Create a report writer for this mission folder
        let mission_writer = BaseReportWriter::with_config(&mission_folder, self.base.format(), self.base.config().clone());
        
        // Write main mission report
        if mission_writer.is_report_enabled("mission_info") {
            mission_writer.write_report(result, "mission_info")?;
        }
        
        // Write equipment report
        if mission_writer.is_report_enabled("equipment_items") {
            let equipment = result.get_equipment();
            mission_writer.write_report(&equipment, "equipment_items")?;
        }
        
        // Write missing dependencies report
        if mission_writer.is_report_enabled("dependencies") {
            let dependencies = result.get_dependencies();
            mission_writer.write_report(&dependencies, "dependencies")?;
        }
        
        debug!("Wrote mission reports for '{}' to {}", result.mission_name(), mission_folder.display());
        
        Ok(mission_folder)
    }
    
    /// Write a report for a single mission analysis result (legacy method)
    pub fn write_mission_report<T: Serialize + MissionName>(&self, result: &T, default_name: &str) -> Result<PathBuf> {
        let mission_name = sanitize_filename(&result.mission_name());
        let filename = if mission_name.is_empty() { default_name.to_string() } else { mission_name };
        
        let report_type = format!("mission_{}", filename);
        if !self.base.is_report_enabled(&report_type) {
            debug!("Skipping report for mission '{}' (disabled in configuration)", result.mission_name());
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(result, &filename)?;
        debug!("Wrote mission report for '{}' to {}", result.mission_name(), path.display());
        
        Ok(path)
    }
    
    /// Write a summary report for multiple mission analysis results
    pub fn write_summary_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<PathBuf> {
        if !self.base.is_report_enabled("mission_summary") {
            debug!("Skipping mission summary report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        // Create summary items
        let summary_items = results.iter().map(|result| {
            let equipment = result.get_equipment();
            let dependencies = result.get_dependencies();
            
            MissionSummaryItem {
                name: result.mission_name(),
                class_count: equipment.classes.len(),
                missing_dependencies_count: dependencies.missing_classes.len(),
            }
        }).collect::<Vec<_>>();
        
        let summary = MissionSummary {
            total_missions: results.len(),
            missions: summary_items,
        };
        
        let path = self.base.write_report(&summary, "summary")?;
        info!("Wrote mission summary report to {}", path.display());
        
        Ok(path)
    }
}

/// Trait for types that have a mission name
pub trait MissionName {
    fn mission_name(&self) -> String;
}

/// Trait for types that provide equipment items
pub trait MissionEquipment {
    fn get_equipment(&self) -> MissionEquipmentReport;
}

/// Trait for types that provide dependency information
pub trait MissionDependencies {
    fn get_dependencies(&self) -> MissionDependenciesReport;
}

/// Mission equipment report
#[derive(Serialize)]
pub struct MissionEquipmentReport {
    pub total_items: usize,
    pub classes: Vec<EquipmentItemReport>,
}

/// Equipment item for reporting
#[derive(Serialize)]
pub struct EquipmentItemReport {
    pub class_name: String,
    pub source_file: String,
    pub line_number: usize,
    pub context: String,
}

/// Mission dependencies report
#[derive(Serialize)]
pub struct MissionDependenciesReport {
    pub total_missing_classes: usize,
    pub missing_classes: Vec<String>,
}

/// Mission summary item for reporting
#[derive(Serialize)]
pub struct MissionSummaryItem {
    pub name: String,
    pub class_count: usize,
    pub missing_dependencies_count: usize,
}

/// Mission summary for reporting
#[derive(Serialize)]
pub struct MissionSummary {
    pub total_missions: usize,
    pub missions: Vec<MissionSummaryItem>,
} 