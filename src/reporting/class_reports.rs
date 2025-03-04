use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug, warn};
use serde::Serialize;
use rayon::prelude::*;

use crate::scanning::classes::processor::{ProcessedClass, ProcessingStats};
use crate::reporting::{ReportConfig, ReportFormat};
use crate::reporting::class::{
    ClassReportWriter, ClassValidationReport,
    ClassValidationResult
};
use crate::reporting::class_search::{self, MatchType};

/// Class report manager
pub struct ClassReportManager {
    output_dir: PathBuf,
    config: Option<ReportConfig>,
}

impl ClassReportManager {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: None,
        }
    }
    
    /// Create a new ClassReportManager with a specific configuration
    pub fn with_config(output_dir: &Path, config: ReportConfig) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: Some(config),
        }
    }

    /// Write all class reports - simplified version that just writes the raw class list
    pub fn write_all_reports(&self, classes: &[ProcessedClass]) -> Result<PathBuf> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the ClassReportWriter to write reports
        let report_writer = if let Some(config) = &self.config {
            ClassReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            ClassReportWriter::new(&self.output_dir)
        };
        
        // Write the main classes report
        let path = report_writer.write_all_reports(classes)?;
        
        info!("Wrote class list to {}", path.display());
        
        Ok(path)
    }

    /// Validate specific class names against processed classes
    pub fn validate_class_names(&mut self, classes: &[ProcessedClass], class_names: &[String]) -> Result<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the ClassReportWriter to write reports
        let report_writer = if let Some(config) = &self.config {
            ClassReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            ClassReportWriter::new(&self.output_dir)
        };

        // Use the shared class search functionality
        let results = class_search::search_classes_parallel(class_names, classes);

        // Convert ClassSearchResult to ClassValidationResult
        let validation_results: Vec<ClassValidationResult> = results.into_iter()
            .map(|result| ClassValidationResult {
                class_name: result.class_name,
                found: result.found,
                file_path: result.file_path,
                parent_class: result.parent_class,
                match_type: result.match_type,
                actual_class_name: result.actual_class_name,
                found_in_nested: result.found_in_nested,
                nested_parent: result.nested_parent,
            })
            .collect();

        // Separate into found and missing classes
        let (found_classes, missing_classes): (Vec<_>, Vec<_>) = validation_results.into_iter()
            .partition(|result| result.found);

        let missing_classes: Vec<String> = missing_classes.into_iter()
            .map(|result| result.class_name)
            .collect();

        // Create and write the validation report
        let validation_report = ClassValidationReport {
            total_classes_checked: class_names.len(),
            found_classes,
            missing_classes,
        };

        report_writer.write_validation_report(&validation_report)?;
        
        // Log summary of results
        for result in &validation_report.found_classes {
            let match_desc = match result.match_type {
                MatchType::ExactMatch => "exact match",
                MatchType::CaseInsensitiveMatch => "case-insensitive match",
                MatchType::PartialMatch => "partial match",
                MatchType::NotFound => "not found",
            };
            
            if let Some(actual_name) = &result.actual_class_name {
                info!("Found '{}' as {} (actual name: '{}')", result.class_name, match_desc, actual_name);
            } else {
                info!("Found '{}' as {}", result.class_name, match_desc);
            }
        }
        
        Ok(())
    }
} 