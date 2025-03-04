use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::{ReportConfig, ReportFormat, BaseReportWriter, ReportWriter};
use crate::reporting::mission_report_writer::{
    MissionReportWriter, MissionName, MissionEquipment, MissionDependencies,
    MissionSummary, MissionSummaryItem
};
use crate::reporting::dependency_report_writer::{
    DependencyReportWriter, MissingClassesReport, ClassUsageReport, MissionCompatibilityReport,
    CategoryNeedsReport, ClassInheritanceReport, CompatibilityDiagnosticsReport,
    MissingClassDetail, ClassUsageDetail, MissionCompatibility, CategoryNeedDetail,
    InheritanceRelationship, MissionDiagnostics, ClassDiagnostic
};
use crate::scanning::classes::processor::ProcessedClass;
use crate::reporting::missing_classes_report::MissingClassesReportWriter;
use crate::reporting::class_tree_report::ClassTreeReportWriter;
use crate::searching::class_search;

/// Mission report manager
pub struct MissionReportManager {
    output_dir: PathBuf,
    config: Option<ReportConfig>,
    available_classes: Vec<ProcessedClass>,
}

impl MissionReportManager {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: None,
            available_classes: Vec::new(),
        }
    }
    
    /// Create a new MissionReportManager with a specific configuration
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
    
    /// Get mission directory path
    fn get_mission_dir(&self, mission_name: &str) -> PathBuf {
        let sanitized_name = sanitize_mission_name(mission_name);
        self.output_dir.join("missions").join(sanitized_name)
    }
    
    /// Write mission reports
    pub fn write_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Create missions directory
        let missions_dir = self.output_dir.join("missions");
        std::fs::create_dir_all(&missions_dir)
            .context(format!("Failed to create missions directory: {}", missions_dir.display()))?;
        
        // Write summary report in the root directory
        self.write_summary_report(results)?;
        
        // Write individual mission reports in their own directories
        for result in results {
            self.write_mission_reports(result)?;
        }
        
        // Write class tree report if we have available classes
        if !self.available_classes.is_empty() {
            self.write_class_tree_report(results)?;
        }
        
        info!("Wrote mission reports to {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Write summary report for all missions
    fn write_summary_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<PathBuf> {
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
        
        // Use the BaseReportWriter to write the summary report
        let report_writer = if let Some(config) = &self.config {
            BaseReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            BaseReportWriter::new(&self.output_dir)
        };
        
        // Write the summary report
        let path = report_writer.write_report(&summary, "mission_summary")?;
        info!("Wrote mission summary report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write reports for a single mission
    fn write_mission_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, result: &T) -> Result<()> {
        let mission_name = result.mission_name();
        let mission_dir = self.get_mission_dir(&mission_name);
        
        // Create the mission directory
        std::fs::create_dir_all(&mission_dir)
            .context(format!("Failed to create mission directory: {}", mission_dir.display()))?;
        
        // Use the BaseReportWriter to write mission-specific reports
        let report_writer = if let Some(config) = &self.config {
            BaseReportWriter::with_config(&mission_dir, ReportFormat::Json, config.clone())
        } else {
            BaseReportWriter::new(&mission_dir)
        };
        
        // Write mission info report
        report_writer.write_report(result, "info")?;
        
        // Write equipment items report
        let equipment = result.get_equipment();
        report_writer.write_report(&equipment, "equipment")?;
        
        // Write dependencies report
        let dependencies = result.get_dependencies();
        report_writer.write_report(&dependencies, "dependencies")?;
        
        // Write class tree report if we have available classes
        if !self.available_classes.is_empty() {
            self.write_mission_class_tree_report(result, &mission_dir)?;
        }
        
        debug!("Wrote reports for mission '{}' to {}", mission_name, mission_dir.display());
        
        Ok(())
    }
    
    /// Write class tree report
    pub fn write_class_tree_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the ClassTreeReportWriter to write the combined report
        let report_writer = if let Some(config) = &self.config {
            ClassTreeReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            ClassTreeReportWriter::new(&self.output_dir)
        };
        
        // Write the combined class tree report
        report_writer.write_class_tree_report(results, &self.available_classes)?;
        
        info!("Wrote combined class tree report to {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Write class tree report for a single mission
    fn write_mission_class_tree_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(
        &self,
        result: &T,
        mission_dir: &Path,
    ) -> Result<PathBuf> {
        // Create a mission-specific report writer
        let mission_report_writer = if let Some(config) = &self.config {
            ClassTreeReportWriter::with_config(mission_dir, ReportFormat::Json, config.clone())
        } else {
            ClassTreeReportWriter::new(mission_dir)
        };
        
        // Write the mission-specific class tree report
        let path = mission_report_writer.write_mission_class_tree_report(result, &self.available_classes)?;
        
        Ok(path)
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
    
    /// Get mission directory path
    fn get_mission_dir(&self, mission_name: &str) -> PathBuf {
        let sanitized_name = sanitize_mission_name(mission_name);
        self.output_dir.join("missions").join(sanitized_name)
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
    
    /// Write class tree report
    pub fn write_class_tree_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Create missions directory
        let missions_dir = self.output_dir.join("missions");
        std::fs::create_dir_all(&missions_dir)
            .context(format!("Failed to create missions directory: {}", missions_dir.display()))?;
        
        // Use the ClassTreeReportWriter to write the combined report
        let report_writer = if let Some(config) = &self.config {
            ClassTreeReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            ClassTreeReportWriter::new(&self.output_dir)
        };
        
        // Write the combined class tree report
        report_writer.write_class_tree_report(results, &self.available_classes)?;
        
        // Write individual mission class tree reports
        for result in results {
            let mission_name = result.mission_name();
            let mission_dir = self.get_mission_dir(&mission_name);
            
            // Create the directory
            std::fs::create_dir_all(&mission_dir)
                .context(format!("Failed to create mission directory: {}", mission_dir.display()))?;
            
            // Write the mission-specific class tree report
            self.write_mission_class_tree_report(result, &mission_dir)?;
        }
        
        info!("Wrote class tree reports to {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Write class tree report for a single mission
    fn write_mission_class_tree_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(
        &self,
        result: &T,
        mission_dir: &Path,
    ) -> Result<PathBuf> {
        // Create a mission-specific report writer
        let mission_report_writer = if let Some(config) = &self.config {
            ClassTreeReportWriter::with_config(mission_dir, ReportFormat::Json, config.clone())
        } else {
            ClassTreeReportWriter::new(mission_dir)
        };
        
        // Write the mission-specific class tree report
        let path = mission_report_writer.write_mission_class_tree_report(result, &self.available_classes)?;
        
        Ok(path)
    }
    
    /// Write dependency reports
    pub fn write_dependency_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Create missions directory
        let missions_dir = self.output_dir.join("missions");
        std::fs::create_dir_all(&missions_dir)
            .context(format!("Failed to create missions directory: {}", missions_dir.display()))?;
        
        // Use the DependencyReportWriter to write the combined reports
        let report_writer = if let Some(config) = &self.config {
            DependencyReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            DependencyReportWriter::new(&self.output_dir)
        };
        
        // Write the combined dependency reports
        report_writer.write_dependency_report(results, &self.available_classes)?;
        
        // Write additional reports
        self.write_additional_reports(results)?;
        
        // Write the detailed missing classes report
        self.write_detailed_missing_classes_report(results)?;
        
        // Write the class tree report
        self.write_class_tree_report(results)?;
        
        // Write individual mission dependency reports
        for result in results {
            self.write_mission_dependency_reports(result)?;
        }
        
        info!("Wrote dependency reports to {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Write dependency reports for a single mission
    fn write_mission_dependency_reports<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, result: &T) -> Result<()> {
        let mission_name = result.mission_name();
        let mission_dir = self.get_mission_dir(&mission_name);
        
        // Create the mission directory
        std::fs::create_dir_all(&mission_dir)
            .context(format!("Failed to create mission directory: {}", mission_dir.display()))?;
        
        // Use the DependencyReportWriter to write mission-specific reports
        let report_writer = if let Some(config) = &self.config {
            DependencyReportWriter::with_config(&mission_dir, ReportFormat::Json, config.clone())
        } else {
            DependencyReportWriter::new(&mission_dir)
        };
        
        // Generate mission-specific reports
        let equipment = result.get_equipment();
        let dependencies = result.get_dependencies();
        
        // Write mission info report
        report_writer.write_class_existence_report(result)?;
        
        // Write equipment items report
        report_writer.write_class_existence_report(&equipment)?;
        
        // Write dependencies report
        report_writer.write_class_existence_report(&dependencies)?;
        
        debug!("Wrote dependency reports for mission '{}' to {}", mission_name, mission_dir.display());
        
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
                crate::reporting::dependency_report_writer::CompatibilityLevel::High
            } else if compatibility_score >= 70.0 {
                crate::reporting::dependency_report_writer::CompatibilityLevel::Medium
            } else if compatibility_score >= 50.0 {
                crate::reporting::dependency_report_writer::CompatibilityLevel::Low
            } else {
                crate::reporting::dependency_report_writer::CompatibilityLevel::Incompatible
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