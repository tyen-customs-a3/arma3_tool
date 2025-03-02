use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::mission::MissionReportWriter;
use crate::reporting::dependency::DependencyReportWriter;
use crate::reporting::mission::MissionName;
use crate::reporting::dependency::{
    MissingClassesReport, MissingClassDetail, 
    ClassUsageReport, ClassUsageDetail,
    MissionCompatibilityReport, MissionCompatibility,
    CategoryNeedsReport, CategoryNeedDetail
};
use super::analyzer::{MissionAnalysisResult, DependencyAnalysisResult};

// Implement MissionName trait for MissionAnalysisResult
impl MissionName for MissionAnalysisResult {
    fn mission_name(&self) -> String {
        self.mission_name.clone()
    }
}

// Implement MissionName trait for DependencyAnalysisResult
impl MissionName for DependencyAnalysisResult {
    fn mission_name(&self) -> String {
        self.mission_name.clone()
    }
}

/// Manager for mission reports
pub struct MissionReportManager {
    output_dir: PathBuf,
}

impl MissionReportManager {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
        }
    }
    
    pub fn write_reports(&self, results: &[MissionAnalysisResult]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the MissionReportWriter to write reports
        let report_writer = MissionReportWriter::new(&self.output_dir);
        report_writer.write_reports(results)?;
        
        info!("Wrote mission reports to {}", self.output_dir.display());
        
        Ok(())
    }
}

/// Manager for dependency reports
pub struct DependencyReportManager {
    output_dir: PathBuf,
}

impl DependencyReportManager {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
        }
    }
    
    pub fn write_dependency_report(&self, results: &[DependencyAnalysisResult]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the DependencyReportWriter to write reports
        let report_writer = DependencyReportWriter::new(&self.output_dir);
        
        // Write the main dependency reports
        report_writer.write_dependency_report(results)?;
        
        // Generate and write additional reports
        self.write_additional_reports(&report_writer, results)?;
        
        info!("Wrote dependency reports to {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Generate and write additional dependency reports
    fn write_additional_reports(&self, report_writer: &DependencyReportWriter, results: &[DependencyAnalysisResult]) -> Result<()> {
        // Generate missing classes report
        let missing_classes_report = self.generate_missing_classes_report(results);
        report_writer.write_missing_classes_report(&missing_classes_report)?;
        
        // Generate class usage report
        let class_usage_report = self.generate_class_usage_report(results);
        report_writer.write_class_usage_report(&class_usage_report)?;
        
        // Generate mission compatibility report
        let compatibility_report = self.generate_compatibility_report(results);
        report_writer.write_compatibility_report(&compatibility_report)?;
        
        // Generate category needs report
        let category_needs_report = self.generate_category_needs_report(results);
        report_writer.write_category_needs_report(&category_needs_report)?;
        
        Ok(())
    }
    
    /// Generate a report of missing classes across all missions
    fn generate_missing_classes_report(&self, results: &[DependencyAnalysisResult]) -> MissingClassesReport {
        let mut missing_class_map: HashMap<String, Vec<String>> = HashMap::new();
        
        // Collect missing classes from all missions
        for result in results {
            for class_name in &result.missing_classes {
                missing_class_map.entry(class_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result.mission_name.clone());
            }
        }
        
        // Convert to report format
        let missing_classes = missing_class_map.into_iter()
            .map(|(class_name, missions)| {
                MissingClassDetail {
                    class_name,
                    used_in_missions: missions.clone(),
                    usage_count: missions.len(),
                }
            })
            .collect::<Vec<_>>();
        
        MissingClassesReport {
            total_missing_classes: missing_classes.len(),
            missing_classes,
        }
    }
    
    /// Generate a report of class usage frequency across missions
    fn generate_class_usage_report(&self, results: &[DependencyAnalysisResult]) -> ClassUsageReport {
        let mut class_usage_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut available_classes: HashSet<String> = HashSet::new();
        
        // Collect all classes (both missing and available)
        for result in results {
            for class_name in &result.missing_classes {
                class_usage_map.entry(class_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result.mission_name.clone());
            }
            
            for class_name in &result.available_classes {
                available_classes.insert(class_name.clone());
                class_usage_map.entry(class_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result.mission_name.clone());
            }
        }
        
        // Convert to report format
        let class_usage = class_usage_map.into_iter()
            .map(|(class_name, missions)| {
                ClassUsageDetail {
                    class_name: class_name.clone(),
                    used_in_missions: missions.clone(),
                    usage_count: missions.len(),
                    is_available: available_classes.contains(&class_name),
                }
            })
            .collect::<Vec<_>>();
        
        ClassUsageReport {
            total_classes: class_usage.len(),
            class_usage,
        }
    }
    
    /// Generate a report of mission compatibility with available classes
    fn generate_compatibility_report(&self, results: &[DependencyAnalysisResult]) -> MissionCompatibilityReport {
        let missions = results.iter()
            .map(|result| {
                let total_required = result.total_equipment_count;
                let available = result.available_classes.len();
                let missing = result.missing_classes.len();
                
                // Calculate compatibility score (percentage of available classes)
                let compatibility_score = if total_required > 0 {
                    available as f64 / total_required as f64
                } else {
                    1.0 // If no classes required, compatibility is 100%
                };
                
                // Identify critical missing classes (top 5 most used)
                let critical_missing = result.missing_classes.iter()
                    .take(5)
                    .cloned()
                    .collect();
                
                MissionCompatibility {
                    mission_name: result.mission_name.clone(),
                    compatibility_score,
                    required_classes: total_required,
                    available_classes: available,
                    missing_classes: missing,
                    critical_missing_classes: critical_missing,
                }
            })
            .collect();
        
        MissionCompatibilityReport { missions }
    }
    
    /// Generate a report of class categories needed by missions
    fn generate_category_needs_report(&self, results: &[DependencyAnalysisResult]) -> CategoryNeedsReport {
        // This would require category information from class analysis
        // For now, we'll create a placeholder with basic categories
        let mut categories: HashMap<String, CategoryNeedDetail> = HashMap::new();
        
        // Initialize with common Arma 3 categories
        let category_names = vec!["Vehicle", "Weapon", "Magazine", "Item", "Backpack"];
        
        for category_name in category_names {
            categories.insert(category_name.to_string(), CategoryNeedDetail {
                category: category_name.to_string(),
                required_by_missions: Vec::new(),
                total_classes_needed: 0,
                available_classes: 0,
                missing_classes: 0,
            });
        }
        
        // In a real implementation, we would analyze the classes by category
        // and populate this report with actual data
        
        CategoryNeedsReport { categories }
    }
} 