use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::{BaseReportWriter, ReportConfig, ReportFormat, ReportWriter};
use crate::reporting::mission_report_writer::{MissionName, MissionEquipment, MissionDependencies};
use crate::scanning::classes::processor::ProcessedClass;
use crate::searching::class_search;

/// Detailed missing classes report
#[derive(Serialize, Clone)]
pub struct DetailedMissingClassesReport {
    pub total_missions: usize,
    pub total_missing_classes: usize,
    pub mission_reports: Vec<MissionMissingClassesReport>,
}

/// Missing classes report for a single mission
#[derive(Serialize, Clone)]
pub struct MissionMissingClassesReport {
    pub mission_name: String,
    pub total_classes: usize,
    pub missing_classes: Vec<MissingClassDetail>,
    pub missing_percentage: f64,
}

/// Detailed information about a missing class
#[derive(Serialize, Clone)]
pub struct MissingClassDetail {
    pub class_name: String,
    pub source_file: String,
    pub line_number: usize,
    pub context: String,
}

/// Missing classes report writer
pub struct MissingClassesReportWriter {
    base: BaseReportWriter,
}

impl MissingClassesReportWriter {
    /// Create a new MissingClassesReportWriter with default configuration
    pub fn new(output_dir: &Path) -> Self {
        Self {
            base: BaseReportWriter::new(output_dir),
        }
    }
    
    /// Create a new MissingClassesReportWriter with a specific format
    pub fn with_format(output_dir: &Path, format: ReportFormat) -> Self {
        Self {
            base: BaseReportWriter::with_config(output_dir, format, ReportConfig::new()),
        }
    }
    
    /// Create a new MissingClassesReportWriter with a specific configuration
    pub fn with_config(output_dir: &Path, format: ReportFormat, config: ReportConfig) -> Self {
        Self {
            base: BaseReportWriter::with_config(output_dir, format, config),
        }
    }
    
    /// Generate and write a detailed missing classes report
    pub fn write_detailed_missing_classes_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(
        &self,
        results: &[T],
        available_classes: &[ProcessedClass],
    ) -> Result<PathBuf> {
        // Ensure output directory exists
        self.base.ensure_output_dir()?;
        
        // Generate the detailed missing classes report
        let report = self.generate_detailed_missing_classes_report(results, available_classes);
        
        // Write the report
        let path = self.base.write_report(&report, "detailed_missing_classes")?;
        info!("Wrote detailed missing classes report to {}", path.display());
        
        Ok(path)
    }
    
    /// Generate a detailed missing classes report
    fn generate_detailed_missing_classes_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(
        &self,
        results: &[T],
        available_classes: &[ProcessedClass],
    ) -> DetailedMissingClassesReport {
        let mut mission_reports = Vec::with_capacity(results.len());
        let mut total_missing_classes = 0;
        
        for result in results {
            let mission_name = result.mission_name();
            let equipment = result.get_equipment();
            
            // Get class names for searching
            let class_names: Vec<String> = equipment.classes.iter()
                .map(|item| item.class_name.clone())
                .collect();
                
            // Search for classes using the class search module
            let search_results = class_search::search_classes_parallel(&class_names, available_classes);
            
            // Process search results to find missing classes
            let mut missing_classes = Vec::new();
            
            for (i, search_result) in search_results.iter().enumerate() {
                if !search_result.found {
                    // Find the corresponding equipment item
                    if i < equipment.classes.len() {
                        let item = &equipment.classes[i];
                        missing_classes.push(MissingClassDetail {
                            class_name: search_result.class_name.clone(),
                            source_file: item.source_file.clone(),
                            line_number: item.line_number,
                            context: item.context.clone(),
                        });
                    }
                }
            }
            
            // Calculate missing percentage
            let missing_percentage = if !class_names.is_empty() {
                (missing_classes.len() as f64 / class_names.len() as f64) * 100.0
            } else {
                0.0
            };
            
            // Create mission report
            let mission_report = MissionMissingClassesReport {
                mission_name,
                total_classes: class_names.len(),
                missing_classes: missing_classes.clone(),
                missing_percentage,
            };
            
            total_missing_classes += missing_classes.len();
            mission_reports.push(mission_report);
        }
        
        // Create the final report
        DetailedMissingClassesReport {
            total_missions: results.len(),
            total_missing_classes,
            mission_reports,
        }
    }
} 