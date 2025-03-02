use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;
use std::fs;

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
    pub fn write_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        self.base.ensure_output_dir()?;
        
        // Write individual mission reports in separate folders
        for result in results.iter() {
            let mission_name = sanitize_filename(&result.mission_name());
            self.write_mission_folder_reports(result, &mission_name)?;
        }
        
        // Write summary report
        self.write_summary_report(results)?;
        
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
        
        // Create mission folder
        let mission_folder = self.base.output_dir().join(&folder_name);
        fs::create_dir_all(&mission_folder)
            .context(format!("Failed to create mission folder: {}", mission_folder.display()))?;
        
        // Create a report writer for this mission folder
        let mission_writer = BaseReportWriter::new(&mission_folder, self.base.format);
        
        // Write main mission report
        let main_path = mission_writer.write_report(result, "mission_info")?;
        
        // Write equipment report
        let equipment = result.get_equipment();
        mission_writer.write_report(&equipment, "equipment_items")?;
        
        // Write missing dependencies report
        let dependencies = result.get_dependencies();
        mission_writer.write_report(&dependencies, "dependencies")?;
        
        debug!("Wrote mission reports for '{}' to {}", result.mission_name(), mission_folder.display());
        
        Ok(mission_folder)
    }
    
    /// Write a report for a single mission analysis result (legacy method)
    pub fn write_mission_report<T: Serialize + MissionName>(&self, result: &T, default_name: &str) -> Result<PathBuf> {
        let mission_name = sanitize_filename(&result.mission_name());
        let filename = if mission_name.is_empty() { default_name.to_string() } else { mission_name };
        
        let path = self.base.write_report(result, &filename)?;
        debug!("Wrote mission report for '{}' to {}", result.mission_name(), path.display());
        
        Ok(path)
    }
    
    /// Write a summary report for multiple mission analysis results
    pub fn write_summary_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<PathBuf> {
        // Create summary items
        let summary_items = results.iter().map(|result| {
            let equipment = result.get_equipment();
            let dependencies = result.get_dependencies();
            
            MissionSummaryItem {
                name: result.mission_name(),
                equipment_count: equipment.equipment.len(),
                vehicle_count: equipment.vehicles.len(),
                weapon_count: equipment.weapons.len(),
                magazine_count: equipment.magazines.len(),
                item_count: equipment.items.len(),
                backpack_count: equipment.backpacks.len(),
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
    pub equipment: Vec<EquipmentItemReport>,
    pub vehicles: Vec<EquipmentItemReport>,
    pub weapons: Vec<EquipmentItemReport>,
    pub magazines: Vec<EquipmentItemReport>,
    pub items: Vec<EquipmentItemReport>,
    pub backpacks: Vec<EquipmentItemReport>,
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
    pub equipment_count: usize,
    pub vehicle_count: usize,
    pub weapon_count: usize,
    pub magazine_count: usize,
    pub item_count: usize,
    pub backpack_count: usize,
    pub missing_dependencies_count: usize,
}

/// Mission summary for reporting
#[derive(Serialize)]
pub struct MissionSummary {
    pub total_missions: usize,
    pub missions: Vec<MissionSummaryItem>,
} 