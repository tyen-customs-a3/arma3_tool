use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::{BaseReportWriter, ReportWriter, ReportFormat, ReportConfig};

/// Mission report writer
pub struct MissionReportWriter {
    base: BaseReportWriter,
}

impl MissionReportWriter {
    /// Create a new MissionReportWriter with default configuration
    pub fn new(output_dir: &Path) -> Self {
        Self {
            base: BaseReportWriter::new(output_dir),
        }
    }
    
    /// Create a new MissionReportWriter with a specific format
    pub fn with_format(output_dir: &Path, format: ReportFormat) -> Self {
        Self {
            base: BaseReportWriter::with_config(output_dir, format, ReportConfig::new()),
        }
    }
    
    /// Create a new MissionReportWriter with a specific configuration
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
    
    /// Write all mission reports
    pub fn write_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Ensure output directory exists
        self.base.ensure_output_dir()?;
        
        // Write summary report
        if self.base.is_report_enabled("mission_summary") {
            self.write_summary_report(results)?;
        }
        
        // Write individual mission reports
        for result in results {
            self.write_mission_folder_reports(result, &result.mission_name())?;
        }
        
        info!("Wrote mission reports to {}", self.base.output_dir().display());
        
        Ok(())
    }
    
    /// Write reports for a single mission in a dedicated folder
    pub fn write_mission_folder_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(
        &self, 
        result: &T, 
        default_name: &str
    ) -> Result<PathBuf> {
        // Create a mission-specific folder
        let mission_name = result.mission_name();
        let sanitized_name = crate::reporting::sanitize_filename(&mission_name);
        let mission_dir = self.base.output_dir().join(&sanitized_name);
        
        // Create the directory
        std::fs::create_dir_all(&mission_dir)
            .context(format!("Failed to create mission directory: {}", mission_dir.display()))?;
        
        // Write mission info report
        if self.base.is_report_enabled("mission_info") {
            let mission_writer = MissionReportWriter::with_config(&mission_dir, self.base.format(), self.base.config().clone());
            mission_writer.write_mission_report(result, "info")?;
        }
        
        // Write equipment items report
        if self.base.is_report_enabled("equipment_items") {
            let equipment = result.get_equipment();
            let mission_writer = MissionReportWriter::with_config(&mission_dir, self.base.format(), self.base.config().clone());
            mission_writer.base.write_report(&equipment, "equipment")?;
        }
        
        // Write dependencies report
        if self.base.is_report_enabled("dependencies") {
            let dependencies = result.get_dependencies();
            let mission_writer = MissionReportWriter::with_config(&mission_dir, self.base.format(), self.base.config().clone());
            mission_writer.base.write_report(&dependencies, "dependencies")?;
        }
        
        debug!("Wrote mission reports for '{}' to {}", mission_name, mission_dir.display());
        
        Ok(mission_dir)
    }
    
    /// Write a basic mission report
    pub fn write_mission_report<T: Serialize + MissionName>(&self, result: &T, default_name: &str) -> Result<PathBuf> {
        // Create a sanitized filename
        let mission_name = result.mission_name();
        let sanitized_name = if mission_name.is_empty() {
            default_name.to_string()
        } else {
            crate::reporting::sanitize_filename(&mission_name)
        };
        
        let path = self.base.write_report(result, &sanitized_name)?;
        debug!("Wrote mission report for '{}' to {}", mission_name, path.display());
        
        Ok(path)
    }
    
    /// Write a summary report for all missions
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
        debug!("Wrote mission summary report to {}", path.display());
        
        Ok(path)
    }
}

/// Trait for types that have a mission name
pub trait MissionName {
    /// Get the mission name
    fn mission_name(&self) -> String;
}

/// Trait for types that provide equipment items
pub trait MissionEquipment {
    /// Get the mission equipment report
    fn get_equipment(&self) -> MissionEquipmentReport;
}

/// Trait for types that provide dependency information
pub trait MissionDependencies {
    /// Get the mission dependencies report
    fn get_dependencies(&self) -> MissionDependenciesReport;
}

/// Mission equipment report
#[derive(Serialize)]
pub struct MissionEquipmentReport {
    pub total_items: usize,
    pub classes: Vec<EquipmentItemReport>,
}

/// Equipment item report
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

/// Mission summary item
#[derive(Serialize)]
pub struct MissionSummaryItem {
    pub name: String,
    pub class_count: usize,
    pub missing_dependencies_count: usize,
}

/// Mission summary report
#[derive(Serialize)]
pub struct MissionSummary {
    pub total_missions: usize,
    pub missions: Vec<MissionSummaryItem>,
} 