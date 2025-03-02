use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::mission::MissionReportWriter;
use crate::reporting::dependency::DependencyReportWriter;
use crate::reporting::{ReportFormat, ReportConfig};
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
    CompatibilityDiagnosticsReport, MissionDiagnostics, ClassDiagnostic,
    ClassExistenceReport, MissionClassExistenceReport, MissingClassInfo
};

/// Manager for mission reports
pub struct MissionReportManager {
    output_dir: PathBuf,
    config: Option<ReportConfig>,
}

impl MissionReportManager {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: None,
        }
    }
    
    /// Create a new MissionReportManager with a specific configuration
    pub fn with_config(output_dir: &Path, config: ReportConfig) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: Some(config),
        }
    }
    
    /// Write mission reports
    pub fn write_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the MissionReportWriter to write reports
        let report_writer = if let Some(config) = &self.config {
            MissionReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            MissionReportWriter::new(&self.output_dir)
        };
        
        // Write the mission reports
        report_writer.write_reports(results)?;
        
        info!("Wrote mission reports to {}", self.output_dir.display());
        
        Ok(())
    }
}

/// Manager for dependency reports
pub struct DependencyReportManager {
    output_dir: PathBuf,
    config: Option<ReportConfig>,
}

impl DependencyReportManager {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: None,
        }
    }
    
    /// Create a new DependencyReportManager with a specific configuration
    pub fn with_config(output_dir: &Path, config: ReportConfig) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: Some(config),
        }
    }
    
    /// Write class existence validation report
    pub fn write_class_existence_report<T: Serialize>(&self, report: &T) -> Result<PathBuf> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the DependencyReportWriter to write the report
        let report_writer = if let Some(config) = &self.config {
            DependencyReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            DependencyReportWriter::new(&self.output_dir)
        };
        
        let path = report_writer.write_class_existence_report(report)?;
        
        info!("Wrote class existence validation report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write an empty dependency report when no mission data is available
    pub fn write_empty_report(&self) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the DependencyReportWriter to write reports
        let report_writer = if let Some(config) = &self.config {
            DependencyReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            DependencyReportWriter::new(&self.output_dir)
        };
        
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
    
    /// Write dependency reports
    pub fn write_dependency_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the DependencyReportWriter to write reports
        let report_writer = if let Some(config) = &self.config {
            DependencyReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            DependencyReportWriter::new(&self.output_dir)
        };
        
        // Write the dependency reports
        report_writer.write_dependency_report(results)?;
        
        // Write additional reports
        self.write_additional_reports(results)?;
        
        info!("Wrote dependency reports to {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Write additional dependency reports
    fn write_additional_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Use the DependencyReportWriter to write additional reports
        let report_writer = if let Some(config) = &self.config {
            DependencyReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            DependencyReportWriter::new(&self.output_dir)
        };
        
        // Generate and write missing classes report
        let missing_classes = self.generate_missing_classes_report(results);
        report_writer.write_missing_classes_report(&missing_classes)?;
        
        // Generate and write class usage report
        let class_usage = self.generate_class_usage_report(results);
        report_writer.write_class_usage_report(&class_usage)?;
        
        // Generate and write compatibility report
        let compatibility = self.generate_compatibility_report(results);
        report_writer.write_compatibility_report(&compatibility)?;
        
        // Generate and write category needs report
        let category_needs = self.generate_category_needs_report(results);
        report_writer.write_category_needs_report(&category_needs)?;
        
        Ok(())
    }
    
    /// Generate a report of missing classes across all missions
    fn generate_missing_classes_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> MissingClassesReport {
        let mut missing_class_map: HashMap<String, Vec<String>> = HashMap::new();
        
        // Collect classes from all missions
        for result in results {
            let dependencies = result.get_dependencies();
            for class_name in &dependencies.missing_classes {
                missing_class_map.entry(class_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result.mission_name());
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
    fn generate_class_usage_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> ClassUsageReport {
        let mut class_usage_map: HashMap<String, Vec<String>> = HashMap::new();
        
        // Collect all classes 
        for result in results {
            let dependencies = result.get_dependencies();
            for class_name in &dependencies.missing_classes {
                class_usage_map.entry(class_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result.mission_name());
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
    fn generate_compatibility_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> MissionCompatibilityReport {
        let missions = results.iter()
            .map(|result| {
                let dependencies = result.get_dependencies();
                let total_required = dependencies.missing_classes.len();
                
                // Calculate compatibility score (simplistic version)
                let compatibility_score = 0.5; // Default mid-level score
                
                MissionCompatibility {
                    mission_name: result.mission_name(),
                    compatibility_score,
                    required_classes: total_required,
                    available_classes: 0, // We don't track this in the new structure
                    available_through_inheritance: 0, // Same here
                    missing_classes: total_required, // Assume all classes are missing for simplicity
                    critical_missing_classes: dependencies.missing_classes.iter()
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
    fn generate_category_needs_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, _results: &[T]) -> CategoryNeedsReport {
        // This is a simplified implementation with a single category
        let mut categories: HashMap<String, CategoryNeedDetail> = HashMap::new();
        
        // Initialize with a single "All" category
        categories.insert("All".to_string(), CategoryNeedDetail {
            category: "All".to_string(),
            required_by_missions: Vec::new(),
            total_classes_needed: 0,
            available_classes: 0,
            missing_classes: 0,
            available_through_inheritance: 0,
        });
        
        CategoryNeedsReport { categories }
    }
}

// Helper function to sanitize mission names for folder creation
fn sanitize_mission_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
} 