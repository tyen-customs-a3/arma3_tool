use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::{ReportConfig, ReportFormat};
use crate::reporting::mission::{
    MissionReportWriter, MissionName, MissionEquipment, MissionDependencies,
    MissionSummary, MissionSummaryItem
};
use crate::reporting::dependency::{
    DependencyReportWriter, MissingClassesReport, ClassUsageReport, MissionCompatibilityReport,
    CategoryNeedsReport, ClassInheritanceReport, CompatibilityDiagnosticsReport,
    MissingClassDetail, ClassUsageDetail, MissionCompatibility, CategoryNeedDetail,
    InheritanceRelationship, MissionDiagnostics, ClassDiagnostic
};
use crate::scanning::classes::processor::ProcessedClass;
use crate::reporting::class_search;
use crate::reporting::missing_classes_report::MissingClassesReportWriter;

/// Mission report manager
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
        
        // Write all mission reports
        report_writer.write_reports(results)?;
        
        info!("Wrote mission reports to {}", self.output_dir.display());
        
        Ok(())
    }
}

/// Dependency report manager
pub struct DependencyReportManager {
    output_dir: PathBuf,
    config: Option<ReportConfig>,
    available_classes: Vec<ProcessedClass>,
}

impl DependencyReportManager {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: None,
            available_classes: Vec::new(),
        }
    }
    
    /// Create a new DependencyReportManager with a specific configuration
    pub fn with_config(output_dir: &Path, config: ReportConfig) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: Some(config),
            available_classes: Vec::new(),
        }
    }

    /// Set the available classes for dependency checking
    pub fn set_available_classes(&mut self, classes: Vec<ProcessedClass>) {
        self.available_classes = classes;
    }
    
    /// Write class existence report
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
        
        // Write the class existence report
        let path = report_writer.write_class_existence_report(report)?;
        
        info!("Wrote class existence report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write empty dependency reports (when no data is available)
    pub fn write_empty_report(&self) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the DependencyReportWriter to write empty reports
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
    
    /// Write a detailed missing classes report
    pub fn write_detailed_missing_classes_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the MissingClassesReportWriter to write the report
        let report_writer = if let Some(config) = &self.config {
            MissingClassesReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            MissingClassesReportWriter::new(&self.output_dir)
        };
        
        // Write the detailed missing classes report
        report_writer.write_detailed_missing_classes_report(results, &self.available_classes)?;
        
        info!("Wrote detailed missing classes report to {}", self.output_dir.display());
        
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
        report_writer.write_dependency_report(results, &self.available_classes)?;
        
        // Write additional reports
        self.write_additional_reports(results)?;
        
        // Write the detailed missing classes report
        self.write_detailed_missing_classes_report(results)?;
        
        info!("Wrote dependency reports to {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Write additional dependency reports
    fn write_additional_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the DependencyReportWriter to write reports
        let report_writer = if let Some(config) = &self.config {
            DependencyReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            DependencyReportWriter::new(&self.output_dir)
        };
        
        // Generate and write missing classes report
        let missing_classes_report = self.generate_missing_classes_report(results);
        report_writer.write_missing_classes_report(&missing_classes_report)?;
        
        // Generate and write class usage report
        let class_usage_report = self.generate_class_usage_report(results);
        report_writer.write_class_usage_report(&class_usage_report)?;
        
        // Generate and write compatibility report
        let compatibility_report = self.generate_compatibility_report(results);
        report_writer.write_compatibility_report(&compatibility_report)?;
        
        // Generate and write category needs report
        let category_needs_report = self.generate_category_needs_report(results);
        report_writer.write_category_needs_report(&category_needs_report)?;
        
        info!("Wrote additional dependency reports to {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Generate missing classes report
    fn generate_missing_classes_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> MissingClassesReport {
        // Collect all missing classes from all missions
        let mut missing_classes = HashMap::new();
        
        for result in results {
            let mission_name = result.mission_name();
            let equipment = result.get_equipment();
            
            // Get class names for searching
            let class_names: Vec<String> = equipment.classes.iter()
                .map(|item| item.class_name.clone())
                .collect();
                
            // Search for classes using the class search module
            let search_results = class_search::search_classes_parallel(&class_names, &self.available_classes);
            
            // Process search results
            for search_result in search_results {
                if !search_result.found {
                    let entry = missing_classes.entry(search_result.class_name.clone())
                        .or_insert_with(|| MissingClassDetail {
                            class_name: search_result.class_name.clone(),
                            used_in_missions: Vec::new(),
                            usage_count: 0,
                            possible_alternatives: Vec::new(),
                            inheritance_path: search_result.parent_class.map(|parent| vec![parent]),
                        });
                    
                    if !entry.used_in_missions.contains(&mission_name) {
                        entry.used_in_missions.push(mission_name.clone());
                    }
                    entry.usage_count += 1;
                    
                    // If we found a case-insensitive match, add it as a possible alternative
                    if let Some(actual_name) = search_result.actual_class_name {
                        if !entry.possible_alternatives.contains(&actual_name) {
                            entry.possible_alternatives.push(actual_name);
                        }
                    }
                }
            }
        }
        
        let missing_class_details = missing_classes.into_values().collect::<Vec<_>>();
        
        MissingClassesReport {
            total_missing_classes: missing_class_details.len(),
            missing_classes: missing_class_details,
        }
    }
    
    /// Generate class usage report
    fn generate_class_usage_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> ClassUsageReport {
        // Collect all classes used in missions
        let mut class_usage = HashMap::new();
        
        for result in results {
            let mission_name = result.mission_name();
            let equipment = result.get_equipment();
            
            for item in &equipment.classes {
                let entry = class_usage.entry(item.class_name.clone())
                    .or_insert_with(|| ClassUsageDetail {
                        class_name: item.class_name.clone(),
                        used_in_missions: Vec::new(),
                        usage_count: 0,
                        is_available: true, // Assume available by default
                        available_through_inheritance: false,
                        parent_classes: None,
                    });
                
                if !entry.used_in_missions.contains(&mission_name) {
                    entry.used_in_missions.push(mission_name.clone());
                }
                entry.usage_count += 1;
            }
        }
        
        let class_usage_details = class_usage.into_values().collect::<Vec<_>>();
        
        ClassUsageReport {
            total_classes: class_usage_details.len(),
            class_usage: class_usage_details,
        }
    }
    
    /// Generate compatibility report
    fn generate_compatibility_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> MissionCompatibilityReport {
        // Create compatibility reports for each mission
        let missions = results.iter().map(|result| {
            let mission_name = result.mission_name();
            let equipment = result.get_equipment();
            let dependencies = result.get_dependencies();
            
            let required_classes = equipment.classes.len();
            let missing_classes = dependencies.missing_classes.len();
            let available_classes = required_classes - missing_classes;
            
            // Calculate compatibility score (0-100%)
            let compatibility_score = if required_classes > 0 {
                (available_classes as f64 / required_classes as f64) * 100.0
            } else {
                100.0 // If no classes required, it's 100% compatible
            };
            
            // Determine compatibility level
            let compatibility_level = if compatibility_score >= 90.0 {
                crate::reporting::dependency::CompatibilityLevel::High
            } else if compatibility_score >= 70.0 {
                crate::reporting::dependency::CompatibilityLevel::Medium
            } else if compatibility_score >= 50.0 {
                crate::reporting::dependency::CompatibilityLevel::Low
            } else {
                crate::reporting::dependency::CompatibilityLevel::Incompatible
            };
            
            MissionCompatibility {
                mission_name,
                compatibility_score,
                required_classes,
                available_classes,
                available_through_inheritance: 0, // We don't have this information
                missing_classes,
                critical_missing_classes: dependencies.missing_classes.clone(),
                compatibility_level,
            }
        }).collect();
        
        MissionCompatibilityReport { missions }
    }
    
    /// Generate category needs report
    fn generate_category_needs_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, _results: &[T]) -> CategoryNeedsReport {
        // This is a simplified implementation
        // In a real implementation, you'd analyze the classes by category
        CategoryNeedsReport {
            categories: HashMap::new(),
        }
    }
}

/// Sanitize a mission name for use in filenames
fn sanitize_mission_name(name: &str) -> String {
    crate::reporting::sanitize_filename(name)
} 