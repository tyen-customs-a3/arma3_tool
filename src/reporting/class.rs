use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;
use std::collections::HashMap;

use super::{BaseReportWriter, ReportWriter, ReportFormat, sanitize_filename};

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
        let path = self.base.write_report(stats, "class_stats")?;
        info!("Wrote class statistics report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for classes categorized by type
    pub fn write_categorized_report<T: Serialize>(&self, categorized_classes: &T) -> Result<PathBuf> {
        let path = self.base.write_report(categorized_classes, "classes_by_category")?;
        info!("Wrote categorized class report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for classes used in missions
    pub fn write_mission_usage_report<T: Serialize>(&self, usage_data: &T) -> Result<PathBuf> {
        let path = self.base.write_report(usage_data, "class_mission_usage")?;
        info!("Wrote class mission usage report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for class inheritance hierarchy
    pub fn write_hierarchy_report<T: Serialize>(&self, hierarchy: &T) -> Result<PathBuf> {
        let path = self.base.write_report(hierarchy, "class_hierarchy")?;
        info!("Wrote class hierarchy report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for a specific class category
    pub fn write_category_report<T: Serialize>(&self, category: &str, classes: &T) -> Result<PathBuf> {
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
#[derive(Serialize)]
pub struct ClassNode {
    pub name: String,
    pub children: Vec<ClassNode>,
} 