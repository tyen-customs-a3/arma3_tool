use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use log::{info, debug};
use serde_json::to_string_pretty;

use super::analyzer::{MissionAnalysisResult, DependencyAnalysisResult};

pub struct MissionReportWriter<'a> {
    output_dir: &'a Path,
}

impl<'a> MissionReportWriter<'a> {
    pub fn new(output_dir: &'a Path) -> Self {
        Self {
            output_dir,
        }
    }
    
    pub fn write_reports(&self, analysis_results: &[MissionAnalysisResult]) -> Result<()> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Write individual mission reports
        for result in analysis_results {
            self.write_mission_report(result)?;
        }
        
        // Write summary report
        self.write_summary_report(analysis_results)?;
        
        info!("Wrote {} mission reports to {}", analysis_results.len(), self.output_dir.display());
        
        Ok(())
    }
    
    fn write_mission_report(&self, result: &MissionAnalysisResult) -> Result<()> {
        let mission_name = sanitize_filename(&result.mission_name);
        let output_path = self.output_dir.join(format!("{}.json", mission_name));
        
        let json = to_string_pretty(result)
            .context("Failed to serialize mission analysis result")?;
        
        fs::write(&output_path, json)
            .context(format!("Failed to write mission report to {}", output_path.display()))?;
        
        debug!("Wrote mission report for '{}' to {}", result.mission_name, output_path.display());
        
        Ok(())
    }
    
    fn write_summary_report(&self, results: &[MissionAnalysisResult]) -> Result<()> {
        let output_path = self.output_dir.join("summary.json");
        
        // Create a summary structure
        let summary = MissionSummary {
            total_missions: results.len(),
            missions: results.iter().map(|r| MissionSummaryItem {
                name: r.mission_name.clone(),
                equipment_count: r.equipment.len(),
                vehicle_count: r.vehicles.len(),
                weapon_count: r.weapons.len(),
                magazine_count: r.magazines.len(),
                item_count: r.items.len(),
                backpack_count: r.backpacks.len(),
            }).collect(),
        };
        
        let json = to_string_pretty(&summary)
            .context("Failed to serialize mission summary")?;
        
        fs::write(&output_path, json)
            .context(format!("Failed to write summary report to {}", output_path.display()))?;
        
        info!("Wrote mission summary report to {}", output_path.display());
        
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MissionSummary {
    total_missions: usize,
    missions: Vec<MissionSummaryItem>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MissionSummaryItem {
    name: String,
    equipment_count: usize,
    vehicle_count: usize,
    weapon_count: usize,
    magazine_count: usize,
    item_count: usize,
    backpack_count: usize,
}

pub struct DependencyReportWriter<'a> {
    output_dir: &'a Path,
}

impl<'a> DependencyReportWriter<'a> {
    pub fn new(output_dir: &'a Path) -> Self {
        Self {
            output_dir,
        }
    }
    
    pub fn write_dependency_report(&self, results: &[DependencyAnalysisResult]) -> Result<()> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Write individual dependency reports
        for result in results {
            self.write_single_dependency_report(result)?;
        }
        
        // Write summary report
        self.write_dependency_summary(results)?;
        
        info!("Wrote {} dependency reports to {}", results.len(), self.output_dir.display());
        
        Ok(())
    }
    
    fn write_single_dependency_report(&self, result: &DependencyAnalysisResult) -> Result<()> {
        let mission_name = sanitize_filename(&result.mission_name);
        let output_path = self.output_dir.join(format!("{}_dependencies.json", mission_name));
        
        let json = to_string_pretty(result)
            .context("Failed to serialize dependency analysis result")?;
        
        fs::write(&output_path, json)
            .context(format!("Failed to write dependency report to {}", output_path.display()))?;
        
        debug!("Wrote dependency report for '{}' to {}", result.mission_name, output_path.display());
        
        Ok(())
    }
    
    fn write_dependency_summary(&self, results: &[DependencyAnalysisResult]) -> Result<()> {
        let output_path = self.output_dir.join("dependency_report.json");
        
        // Create a summary structure
        let summary = DependencySummary {
            total_missions: results.len(),
            missions: results.iter().map(|r| DependencySummaryItem {
                name: r.mission_name.clone(),
                total_equipment: r.total_equipment_count,
                missing_classes: r.missing_classes.len(),
                available_classes: r.available_classes.len(),
                missing_class_percentage: if r.total_equipment_count > 0 {
                    (r.missing_classes.len() as f64 / r.total_equipment_count as f64) * 100.0
                } else {
                    0.0
                },
            }).collect(),
        };
        
        let json = to_string_pretty(&summary)
            .context("Failed to serialize dependency summary")?;
        
        fs::write(&output_path, json)
            .context(format!("Failed to write dependency summary to {}", output_path.display()))?;
        
        info!("Wrote dependency summary report to {}", output_path.display());
        
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DependencySummary {
    total_missions: usize,
    missions: Vec<DependencySummaryItem>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DependencySummaryItem {
    name: String,
    total_equipment: usize,
    missing_classes: usize,
    available_classes: usize,
    missing_class_percentage: f64,
}

// Helper function to sanitize filenames
fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
} 