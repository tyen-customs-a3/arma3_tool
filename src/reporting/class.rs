use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;
use std::collections::HashMap;

use super::{BaseReportWriter, ReportWriter, ReportFormat, sanitize_filename, ReportConfig};

/// Writer for class analysis reports
pub struct ClassReportWriter {
    base: BaseReportWriter,
}

impl ClassReportWriter {
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
    
    pub fn with_config(output_dir: &Path, format: ReportFormat, config: ReportConfig) -> Self {
        Self {
            base: BaseReportWriter::with_config(output_dir, format, config),
        }
    }
    
    /// Get a reference to the report configuration
    pub fn config(&self) -> &ReportConfig {
        self.base.config()
    }
    
    /// Get a mutable reference to the report configuration
    pub fn config_mut(&mut self) -> &mut ReportConfig {
        self.base.config_mut()
    }
    
    /// Write all class reports
    pub fn write_all_reports<T: Serialize>(&self, classes: &[T]) -> Result<PathBuf> {
        // Ensure output directory exists
        self.base.ensure_output_dir()?;
        
        // Write JSON report
        let path = self.base.write_report(classes, "classes")?;
        info!("Wrote class report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for class statistics
    pub fn write_stats_report<T: Serialize>(&self, stats: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("class_stats") {
            debug!("Skipping class statistics report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(stats, "class_stats")?;
        info!("Wrote class statistics report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for classes categorized by type
    pub fn write_categorized_report<T: Serialize>(&self, categorized_classes: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("classes_by_category") {
            debug!("Skipping categorized class report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(categorized_classes, "classes_by_category")?;
        info!("Wrote categorized class report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for classes used in missions
    pub fn write_mission_usage_report<T: Serialize>(&self, usage_data: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("class_mission_usage") {
            debug!("Skipping class mission usage report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(usage_data, "class_mission_usage")?;
        info!("Wrote class mission usage report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for class inheritance hierarchy
    pub fn write_hierarchy_report<T: Serialize>(&self, hierarchy: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("class_hierarchy") {
            debug!("Skipping class hierarchy report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(hierarchy, "class_hierarchy")?;
        info!("Wrote class hierarchy report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for circular dependencies in class hierarchy
    pub fn write_circular_dependencies_report<T: Serialize>(&self, circular_deps: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("circular_dependencies") {
            debug!("Skipping circular dependencies report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(circular_deps, "circular_dependencies")?;
        info!("Wrote circular dependencies report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for a specific class category
    pub fn write_category_report<T: Serialize>(&self, category: &str, classes: &T) -> Result<PathBuf> {
        let report_type = format!("category_{}", sanitize_filename(category));
        if !self.base.is_report_enabled(&report_type) {
            debug!("Skipping report for category '{}' (disabled in configuration)", category);
            return Ok(PathBuf::new());
        }
        
        let filename = format!("category_{}", sanitize_filename(category));
        let path = self.base.write_report(classes, &filename)?;
        debug!("Wrote report for category '{}' to {}", category, path.display());
        
        Ok(path)
    }
}

/// Class statistics for reporting
#[derive(Serialize)]
pub struct ClassStats {
    pub total_classes: usize,
    pub total_files: usize,
    pub files_with_classes: usize,
    pub empty_files: usize,
    pub error_files: usize,
    pub class_categories: Vec<ClassCategoryCount>,
}

/// Class category count for reporting
#[derive(Serialize)]
pub struct ClassCategoryCount {
    pub category: String,
    pub count: usize,
}

/// Categorized classes for reporting
#[derive(Serialize)]
pub struct CategorizedClasses {
    pub categories: HashMap<String, Vec<String>>,
}

/// Class mission usage for reporting
#[derive(Serialize)]
pub struct ClassMissionUsage {
    pub class_name: String,
    pub used_in_missions: Vec<String>,
    pub usage_count: usize,
}

/// Class hierarchy for reporting
#[derive(Serialize)]
pub struct ClassHierarchy {
    pub root_classes: Vec<ClassNode>,
}

/// Node in class hierarchy
#[derive(Serialize, Clone)]
pub struct ClassNode {
    pub name: String,
    pub children: Vec<ClassNode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circular_ref: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub is_circular: bool,
}

/// Circular dependency information for reporting
#[derive(Serialize)]
pub struct CircularDependency {
    pub class_name: String,
    pub parent_class: String,
    pub cycle_path: Vec<String>,
} 