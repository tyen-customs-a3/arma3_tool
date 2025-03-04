use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::{BaseReportWriter, ReportConfig, ReportFormat, ReportWriter};
use crate::reporting::mission_report_writer::{MissionName, MissionEquipment, MissionDependencies};
use crate::scanning::classes::processor::ProcessedClass;
use crate::searching::class_search::{self, ClassSearchResult, MatchType};

/// Class tree report for a mission
#[derive(Serialize, Clone)]
pub struct ClassTreeReport {
    pub mission_name: String,
    pub total_classes: usize,
    pub found_classes: usize,
    pub missing_classes: usize,
    pub source_files: Vec<SourceFileClassTree>,
}

/// Source file with its classes
#[derive(Serialize, Clone)]
pub struct SourceFileClassTree {
    pub file_path: String,
    pub classes: Vec<ClassTreeItem>,
}

/// Class tree item
#[derive(Serialize, Clone)]
pub struct ClassTreeItem {
    pub class_name: String,
    pub line_number: usize,
    pub context: String,
    pub found: bool,
    pub found_in: Option<String>,
    pub match_type: String,
    pub actual_class_name: Option<String>,
    pub found_in_nested: bool,
    pub nested_parent: Option<String>,
}

/// Class tree report writer
pub struct ClassTreeReportWriter {
    base: BaseReportWriter,
}

impl ClassTreeReportWriter {
    /// Create a new ClassTreeReportWriter with default configuration
    pub fn new(output_dir: &Path) -> Self {
        Self {
            base: BaseReportWriter::new(output_dir),
        }
    }
    
    /// Create a new ClassTreeReportWriter with a specific format
    pub fn with_format(output_dir: &Path, format: ReportFormat) -> Self {
        Self {
            base: BaseReportWriter::with_config(output_dir, format, ReportConfig::new()),
        }
    }
    
    /// Create a new ClassTreeReportWriter with a specific configuration
    pub fn with_config(output_dir: &Path, format: ReportFormat, config: ReportConfig) -> Self {
        Self {
            base: BaseReportWriter::with_config(output_dir, format, config),
        }
    }
    
    /// Generate and write a class tree report
    pub fn write_class_tree_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(
        &self,
        results: &[T],
        available_classes: &[ProcessedClass],
    ) -> Result<PathBuf> {
        // Ensure output directory exists
        self.base.ensure_output_dir()?;
        
        // Generate class tree reports for each mission
        let mut all_reports = Vec::with_capacity(results.len());
        
        for result in results {
            let report = self.generate_class_tree_report(result, available_classes);
            all_reports.push(report);
        }
        
        // Write the combined report
        let path = self.base.write_report(&all_reports, "class_tree_report")?;
        info!("Wrote class tree report to {}", path.display());
        
        Ok(path)
    }
    
    /// Generate a class tree report for a single mission
    pub fn generate_class_tree_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(
        &self,
        result: &T,
        available_classes: &[ProcessedClass],
    ) -> ClassTreeReport {
        let mission_name = result.mission_name();
        let equipment = result.get_equipment();
        
        // Group classes by source file
        let mut file_classes: HashMap<String, Vec<(usize, &str, String)>> = HashMap::new();
        
        for item in &equipment.classes {
            file_classes
                .entry(item.source_file.clone())
                .or_default()
                .push((item.line_number, &item.class_name, item.context.clone()));
        }
        
        // Get class names for searching
        let class_names: Vec<String> = equipment.classes.iter()
            .map(|item| item.class_name.clone())
            .collect();
            
        // Search for classes using the class search module
        let search_results = class_search::search_classes_parallel(&class_names, available_classes);
        
        // Create a map of class name to search result for quick lookup
        let mut class_results: HashMap<String, ClassSearchResult> = HashMap::new();
        for result in search_results {
            class_results.insert(result.class_name.clone(), result);
        }
        
        // Create source file trees
        let mut source_files = Vec::new();
        let mut found_count = 0;
        
        for (file_path, classes) in file_classes {
            let mut class_items = Vec::new();
            
            for (line_number, class_name, context) in classes {
                if let Some(search_result) = class_results.get(class_name) {
                    if search_result.found {
                        found_count += 1;
                    }
                    
                    // Convert match type to string
                    let match_type = match search_result.match_type {
                        MatchType::ExactMatch => "exact_match",
                        MatchType::CaseInsensitiveMatch => "case_insensitive_match",
                        MatchType::PartialMatch => "partial_match",
                        MatchType::NotFound => "not_found",
                    };
                    
                    class_items.push(ClassTreeItem {
                        class_name: class_name.to_string(),
                        line_number,
                        context,
                        found: search_result.found,
                        found_in: search_result.file_path.clone(),
                        match_type: match_type.to_string(),
                        actual_class_name: search_result.actual_class_name.clone(),
                        found_in_nested: search_result.found_in_nested,
                        nested_parent: search_result.nested_parent.clone(),
                    });
                }
            }
            
            // Sort classes by line number
            class_items.sort_by_key(|item| item.line_number);
            
            source_files.push(SourceFileClassTree {
                file_path,
                classes: class_items,
            });
        }
        
        // Sort source files by path
        source_files.sort_by(|a, b| a.file_path.cmp(&b.file_path));
        
        ClassTreeReport {
            mission_name,
            total_classes: class_names.len(),
            found_classes: found_count,
            missing_classes: class_names.len() - found_count,
            source_files,
        }
    }
    
    /// Write a class tree report for a single mission
    pub fn write_mission_class_tree_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(
        &self,
        result: &T,
        available_classes: &[ProcessedClass],
    ) -> Result<PathBuf> {
        // Ensure output directory exists
        self.base.ensure_output_dir()?;
        
        // Generate the class tree report
        let report = self.generate_class_tree_report(result, available_classes);
        
        // Create a sanitized filename
        let sanitized_name = crate::reporting::sanitize_filename(&report.mission_name);
        let filename = format!("{}_class_tree", sanitized_name);
        
        // Write the report
        let path = self.base.write_report(&report, &filename)?;
        debug!("Wrote class tree report for '{}' to {}", report.mission_name, path.display());
        
        Ok(path)
    }
} 