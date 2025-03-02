use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;
use std::collections::HashMap;

use super::{BaseReportWriter, ReportWriter, ReportFormat, sanitize_filename};
use super::mission::MissionName;

/// Writer for dependency analysis reports
pub struct DependencyReportWriter {
    base: BaseReportWriter,
}

impl DependencyReportWriter {
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
    
    /// Write dependency reports for multiple results
    pub fn write_dependency_report<T: Serialize + MissionName>(&self, results: &[T]) -> Result<()> {
        // Create output directory if it doesn't exist
        self.base.ensure_output_dir()?;
        
        // Write individual dependency reports
        for result in results {
            self.write_single_dependency_report(result)?;
        }
        
        // Write summary report
        self.write_dependency_summary(results)?;
        
        info!("Wrote {} dependency reports to {}", results.len(), self.base.output_dir().display());
        
        Ok(())
    }
    
    /// Write a report for a single dependency analysis result
    pub fn write_single_dependency_report<T: Serialize + MissionName>(&self, result: &T) -> Result<PathBuf> {
        let mission_name = sanitize_filename(&result.mission_name());
        let output_path = self.base.write_report(
            result, 
            &format!("{}_dependencies", mission_name)
        )?;
        
        debug!("Wrote dependency report for '{}' to {}", result.mission_name(), output_path.display());
        
        Ok(output_path)
    }
    
    /// Write a summary report for dependency analysis
    pub fn write_dependency_summary<T: Serialize>(&self, results: &[T]) -> Result<PathBuf> {
        let output_path = self.base.write_report(results, "dependency_report")?;
        
        info!("Wrote dependency summary report to {}", output_path.display());
        
        Ok(output_path)
    }
    
    /// Write a report for missing classes across all missions
    pub fn write_missing_classes_report<T: Serialize>(&self, missing_classes: &T) -> Result<PathBuf> {
        let output_path = self.base.write_report(missing_classes, "missing_classes")?;
        
        info!("Wrote missing classes report to {}", output_path.display());
        
        Ok(output_path)
    }
    
    /// Write a report for class usage frequency across missions
    pub fn write_class_usage_report<T: Serialize>(&self, usage_data: &T) -> Result<PathBuf> {
        let output_path = self.base.write_report(usage_data, "class_usage_frequency")?;
        
        info!("Wrote class usage frequency report to {}", output_path.display());
        
        Ok(output_path)
    }
    
    /// Write a report for mission compatibility with available classes
    pub fn write_compatibility_report<T: Serialize>(&self, compatibility_data: &T) -> Result<PathBuf> {
        let output_path = self.base.write_report(compatibility_data, "mission_compatibility")?;
        
        info!("Wrote mission compatibility report to {}", output_path.display());
        
        Ok(output_path)
    }
    
    /// Write a report for class categories needed by missions
    pub fn write_category_needs_report<T: Serialize>(&self, category_needs: &T) -> Result<PathBuf> {
        let output_path = self.base.write_report(category_needs, "category_needs")?;
        
        info!("Wrote category needs report to {}", output_path.display());
        
        Ok(output_path)
    }
    
    /// Write a report for class inheritance relationships relevant to missions
    pub fn write_inheritance_report<T: Serialize>(&self, inheritance_data: &T) -> Result<PathBuf> {
        let output_path = self.base.write_report(inheritance_data, "class_inheritance")?;
        
        info!("Wrote class inheritance report to {}", output_path.display());
        
        Ok(output_path)
    }
    
    /// Write a report for class compatibility issues with detailed diagnostics
    pub fn write_compatibility_diagnostics_report<T: Serialize>(&self, diagnostics_data: &T) -> Result<PathBuf> {
        let output_path = self.base.write_report(diagnostics_data, "compatibility_diagnostics")?;
        
        info!("Wrote compatibility diagnostics report to {}", output_path.display());
        
        Ok(output_path)
    }
}

/// Dependency summary item for reporting
#[derive(Serialize)]
pub struct DependencySummaryItem {
    pub name: String,
    pub total_equipment: usize,
    pub missing_classes: usize,
    pub available_classes: usize,
    pub missing_class_percentage: f64,
    pub inheritance_matches: usize,
}

/// Dependency summary for reporting
#[derive(Serialize)]
pub struct DependencySummary {
    pub total_missions: usize,
    pub missions: Vec<DependencySummaryItem>,
    pub overall_compatibility: f64,
}

/// Missing classes report
#[derive(Serialize)]
pub struct MissingClassesReport {
    pub total_missing_classes: usize,
    pub missing_classes: Vec<MissingClassDetail>,
}

/// Detail for a missing class
#[derive(Serialize)]
pub struct MissingClassDetail {
    pub class_name: String,
    pub used_in_missions: Vec<String>,
    pub usage_count: usize,
    pub possible_alternatives: Vec<String>,
    pub inheritance_path: Option<Vec<String>>,
}

/// Class usage frequency report
#[derive(Serialize)]
pub struct ClassUsageReport {
    pub total_classes: usize,
    pub class_usage: Vec<ClassUsageDetail>,
}

/// Detail for class usage
#[derive(Serialize)]
pub struct ClassUsageDetail {
    pub class_name: String,
    pub used_in_missions: Vec<String>,
    pub usage_count: usize,
    pub is_available: bool,
    pub available_through_inheritance: bool,
    pub parent_classes: Option<Vec<String>>,
}

/// Mission compatibility report
#[derive(Serialize)]
pub struct MissionCompatibilityReport {
    pub missions: Vec<MissionCompatibility>,
}

/// Mission compatibility details
#[derive(Serialize)]
pub struct MissionCompatibility {
    pub mission_name: String,
    pub compatibility_score: f64,
    pub required_classes: usize,
    pub available_classes: usize,
    pub available_through_inheritance: usize,
    pub missing_classes: usize,
    pub critical_missing_classes: Vec<String>,
    pub compatibility_level: CompatibilityLevel,
}

/// Compatibility level enum
#[derive(Serialize)]
pub enum CompatibilityLevel {
    #[serde(rename = "high")]
    High,    // 90-100% compatibility
    #[serde(rename = "medium")]
    Medium,  // 70-90% compatibility
    #[serde(rename = "low")]
    Low,     // 50-70% compatibility
    #[serde(rename = "incompatible")]
    Incompatible, // <50% compatibility
}

/// Category needs report
#[derive(Serialize)]
pub struct CategoryNeedsReport {
    pub categories: HashMap<String, CategoryNeedDetail>,
}

/// Category need details
#[derive(Serialize)]
pub struct CategoryNeedDetail {
    pub category: String,
    pub required_by_missions: Vec<String>,
    pub total_classes_needed: usize,
    pub available_classes: usize,
    pub missing_classes: usize,
    pub available_through_inheritance: usize,
}

/// Class inheritance report
#[derive(Serialize)]
pub struct ClassInheritanceReport {
    pub inheritance_relationships: Vec<InheritanceRelationship>,
}

/// Inheritance relationship details
#[derive(Serialize)]
pub struct InheritanceRelationship {
    pub class_name: String,
    pub parent_classes: Vec<String>,
    pub used_in_missions: Vec<String>,
    pub is_available: bool,
}

/// Compatibility diagnostics report
#[derive(Serialize)]
pub struct CompatibilityDiagnosticsReport {
    pub mission_diagnostics: Vec<MissionDiagnostics>,
}

/// Mission diagnostics details
#[derive(Serialize)]
pub struct MissionDiagnostics {
    pub mission_name: String,
    pub class_diagnostics: Vec<ClassDiagnostic>,
}

/// Class diagnostic details
#[derive(Serialize)]
pub struct ClassDiagnostic {
    pub class_name: String,
    pub is_available: bool,
    pub available_through_inheritance: bool,
    pub inheritance_path: Option<Vec<String>>,
    pub source_file: Option<String>,
    pub line_number: Option<usize>,
    pub context: Option<String>,
    pub suggested_alternatives: Vec<String>,
} 