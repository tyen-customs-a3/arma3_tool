use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::mission::MissionReportWriter;
use crate::reporting::dependency::DependencyReportWriter;
use crate::reporting::mission::{
    MissionName, MissionEquipment, MissionDependencies,
    MissionEquipmentReport, EquipmentItemReport, MissionDependenciesReport
};
use crate::reporting::dependency::{
    MissingClassesReport, MissingClassDetail, 
    ClassUsageReport, ClassUsageDetail,
    MissionCompatibilityReport, MissionCompatibility, CompatibilityLevel,
    CategoryNeedsReport, CategoryNeedDetail,
    ClassInheritanceReport, InheritanceRelationship,
    CompatibilityDiagnosticsReport, MissionDiagnostics, ClassDiagnostic
};
use super::analyzer::MissionDependencyResult;

// Implement MissionName trait for MissionDependencyResult
impl MissionName for MissionDependencyResult {
    fn mission_name(&self) -> String {
        self.mission_name.clone()
    }
}

// Implement MissionEquipment trait for MissionDependencyResult
impl MissionEquipment for MissionDependencyResult {
    fn get_equipment(&self) -> MissionEquipmentReport {
        // Since MissionDependencyResult doesn't use the equipment categories, create a simple report
        MissionEquipmentReport {
            total_items: self.class_dependencies.len(),
            equipment: Vec::new(),
            vehicles: Vec::new(),
            weapons: Vec::new(),
            magazines: Vec::new(),
            items: Vec::new(),
            backpacks: Vec::new(),
        }
    }
}

// Implement MissionDependencies trait for MissionDependencyResult
impl MissionDependencies for MissionDependencyResult {
    fn get_dependencies(&self) -> MissionDependenciesReport {
        // Convert our unique class names to the format expected by MissionDependenciesReport
        let class_names: Vec<String> = self.unique_class_names.iter().cloned().collect();
        
        MissionDependenciesReport {
            total_missing_classes: class_names.len(),
            missing_classes: class_names, // All class dependencies
        }
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
    
    pub fn write_reports(&self, results: &[MissionDependencyResult]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the MissionReportWriter to write reports
        let report_writer = MissionReportWriter::new(&self.output_dir);
        
        // Write individual mission reports in folders
        for result in results {
            report_writer.write_mission_folder_reports(result, &format!("mission_{}", result.mission_name))?;
        }
        
        // Write summary report
        report_writer.write_summary_report(results)?;
        
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
    
    /// Write an empty dependency report when no mission data is available
    pub fn write_empty_report(&self) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the DependencyReportWriter to write reports
        let report_writer = DependencyReportWriter::new(&self.output_dir);
        
        // Create empty reports
        let empty_missing_classes = MissingClassesReport {
            total_missing_classes: 0,
            missing_classes: Vec::new(),
        };
        
        let empty_class_usage = ClassUsageReport {
            total_classes: 0,
            class_usage: Vec::new(),
        };
        
        let empty_compatibility = MissionCompatibilityReport {
            missions: Vec::new(),
        };
        
        let empty_category_needs = CategoryNeedsReport {
            categories: HashMap::new(),
        };
        
        let empty_inheritance = ClassInheritanceReport {
            inheritance_relationships: Vec::new(),
        };
        
        let empty_diagnostics = CompatibilityDiagnosticsReport {
            mission_diagnostics: Vec::new(),
        };
        
        // Write empty reports
        report_writer.write_missing_classes_report(&empty_missing_classes)?;
        report_writer.write_class_usage_report(&empty_class_usage)?;
        report_writer.write_compatibility_report(&empty_compatibility)?;
        report_writer.write_category_needs_report(&empty_category_needs)?;
        report_writer.write_inheritance_report(&empty_inheritance)?;
        report_writer.write_compatibility_diagnostics_report(&empty_diagnostics)?;
        
        info!("Wrote empty dependency reports to {}", self.output_dir.display());
        
        Ok(())
    }
    
    // Updated to use MissionDependencyResult instead of DependencyAnalysisResult
    pub fn write_dependency_report(&self, results: &[MissionDependencyResult]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the MissionReportWriter to write folder-based reports
        let report_writer = MissionReportWriter::new(&self.output_dir);
        
        // Write individual mission reports in folders
        for result in results {
            let mission_name = sanitize_mission_name(&result.mission_name);
            report_writer.write_mission_folder_reports(result, &mission_name)?;
        }
        
        // Write summary report
        report_writer.write_summary_report(results)?;
        
        // Generate and write additional reports
        self.write_additional_reports(results)?;
        
        info!("Wrote dependency reports to {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Generate and write additional dependency reports
    fn write_additional_reports(&self, results: &[MissionDependencyResult]) -> Result<()> {
        // Use the DependencyReportWriter for summary reports
        let report_writer = DependencyReportWriter::new(&self.output_dir);
        
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
    fn generate_missing_classes_report(&self, results: &[MissionDependencyResult]) -> MissingClassesReport {
        let mut missing_class_map: HashMap<String, Vec<String>> = HashMap::new();
        
        // Collect classes from all missions
        for result in results {
            for class_name in &result.unique_class_names {
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
                    possible_alternatives: Vec::new(), // Would be populated with real alternatives in a full implementation
                    inheritance_path: None,
                }
            })
            .collect::<Vec<_>>();
        
        MissingClassesReport {
            total_missing_classes: missing_classes.len(),
            missing_classes,
        }
    }
    
    /// Generate a report of class usage frequency across missions
    fn generate_class_usage_report(&self, results: &[MissionDependencyResult]) -> ClassUsageReport {
        let mut class_usage_map: HashMap<String, Vec<String>> = HashMap::new();
        
        // Collect all classes 
        for result in results {
            for class_name in &result.unique_class_names {
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
                    is_available: false, // We don't track this in the new structure
                    available_through_inheritance: false, // Same here
                    parent_classes: None,
                }
            })
            .collect::<Vec<_>>();
        
        ClassUsageReport {
            total_classes: class_usage.len(),
            class_usage,
        }
    }
    
    /// Generate a report of mission compatibility with available classes
    fn generate_compatibility_report(&self, results: &[MissionDependencyResult]) -> MissionCompatibilityReport {
        let missions = results.iter()
            .map(|result| {
                let total_required = result.unique_class_names.len();
                
                // Calculate compatibility score (simplistic version)
                let compatibility_score = 0.5; // Default mid-level score
                
                MissionCompatibility {
                    mission_name: result.mission_name.clone(),
                    compatibility_score,
                    required_classes: total_required,
                    available_classes: 0, // We don't track this in the new structure
                    available_through_inheritance: 0, // Same here
                    missing_classes: total_required, // Assume all classes are missing for simplicity
                    critical_missing_classes: result.unique_class_names.iter()
                        .take(5)
                        .cloned()
                        .collect(),
                    compatibility_level: CompatibilityLevel::Medium, // Default level
                }
            })
            .collect();
        
        MissionCompatibilityReport { missions }
    }
    
    /// Generate a report of class categories needed by missions
    fn generate_category_needs_report(&self, results: &[MissionDependencyResult]) -> CategoryNeedsReport {
        // This is a placeholder implementation
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
                available_through_inheritance: 0,
            });
        }
        
        CategoryNeedsReport { categories }
    }
}

// Helper function to sanitize mission names for folder creation
fn sanitize_mission_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}