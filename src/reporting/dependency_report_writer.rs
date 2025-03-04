use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::{BaseReportWriter, ReportConfig, ReportFormat, ReportWriter};
use crate::reporting::mission_report_writer::{MissionName, MissionEquipment, MissionDependencies};
use crate::scanning::classes::processor::ProcessedClass;
use crate::searching::class_search::{self, ClassSearchResult, MatchType};

/// Mission dependencies report
#[derive(Serialize)]
pub struct MissionDependenciesReport {
    pub total_missing_classes: usize,
    pub missing_classes: Vec<String>,
    pub class_search_results: Vec<ClassSearchResult>,
    pub total_exact_matches: usize,
    pub total_case_insensitive_matches: usize,
    pub total_partial_matches: usize,
}

/// Dependency report writer
pub struct DependencyReportWriter {
    base: BaseReportWriter,
}

impl DependencyReportWriter {
    /// Create a new DependencyReportWriter with default configuration
    pub fn new(output_dir: &Path) -> Self {
        Self {
            base: BaseReportWriter::new(output_dir),
        }
    }
    
    /// Create a new DependencyReportWriter with a specific format
    pub fn with_format(output_dir: &Path, format: ReportFormat) -> Self {
        Self {
            base: BaseReportWriter::with_config(output_dir, format, ReportConfig::new()),
        }
    }
    
    /// Create a new DependencyReportWriter with a specific configuration
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
    
    /// Write dependency reports for multiple missions
    pub fn write_dependency_report<T: Serialize + MissionName + MissionEquipment + MissionDependencies>(&self, results: &[T], available_classes: &[ProcessedClass]) -> Result<()> {
        if !self.base.is_report_enabled("dependency_report") {
            debug!("Skipping dependency report (disabled in configuration)");
            return Ok(());
        }
        
        // Ensure output directory exists
        self.base.ensure_output_dir()?;
        
        // Write individual dependency reports for each mission
        for result in results {
            // Get all required classes from the mission
            let equipment = result.get_equipment();
            let class_names: Vec<String> = equipment.classes.iter()
                .map(|item| item.class_name.clone())
                .collect();

            // Search for classes using our shared search functionality
            let search_results = class_search::search_classes_parallel(&class_names, available_classes);

            // Count different types of matches
            let mut total_exact_matches = 0;
            let mut total_case_insensitive_matches = 0;
            let mut total_partial_matches = 0;

            for result in &search_results {
                match result.match_type {
                    MatchType::ExactMatch => total_exact_matches += 1,
                    MatchType::CaseInsensitiveMatch => total_case_insensitive_matches += 1,
                    MatchType::PartialMatch => total_partial_matches += 1,
                    _ => {}
                }
            }

            // Create dependency report based on search results
            let missing_classes: Vec<String> = search_results.iter()
                .filter(|result| !result.found)
                .map(|result| result.class_name.clone())
                .collect();

            let dependencies = MissionDependenciesReport {
                total_missing_classes: missing_classes.len(),
                missing_classes,
                class_search_results: search_results,
                total_exact_matches,
                total_case_insensitive_matches,
                total_partial_matches,
            };

            // Write the report for this mission
            let mission_name = result.mission_name();
            let sanitized_name = crate::reporting::sanitize_filename(&mission_name);
            let filename = format!("dependency_{}", sanitized_name);
            self.base.write_report(&dependencies, &filename)?;
            debug!("Wrote dependency report for '{}' to {}", mission_name, self.base.output_dir().display());
        }
        
        info!("Wrote dependency reports to {}", self.base.output_dir().display());
        
        Ok(())
    }
    
    /// Write a dependency summary report
    pub fn write_dependency_summary<T: Serialize>(&self, results: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("dependency_summary") {
            debug!("Skipping dependency summary report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(results, "dependency_summary")?;
        debug!("Wrote dependency summary report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for missing classes
    pub fn write_missing_classes_report<T: Serialize>(&self, missing_classes: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("missing_classes") {
            debug!("Skipping missing classes report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(missing_classes, "missing_classes")?;
        debug!("Wrote missing classes report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for class usage frequency
    pub fn write_class_usage_report<T: Serialize>(&self, usage_data: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("class_usage") {
            debug!("Skipping class usage report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(usage_data, "class_usage")?;
        debug!("Wrote class usage report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for mission compatibility
    pub fn write_compatibility_report<T: Serialize>(&self, compatibility_data: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("mission_compatibility") {
            debug!("Skipping mission compatibility report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(compatibility_data, "mission_compatibility")?;
        debug!("Wrote mission compatibility report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for category needs
    pub fn write_category_needs_report<T: Serialize>(&self, category_needs: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("category_needs") {
            debug!("Skipping category needs report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(category_needs, "category_needs")?;
        debug!("Wrote category needs report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for class inheritance
    pub fn write_inheritance_report<T: Serialize>(&self, inheritance_data: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("class_inheritance") {
            debug!("Skipping class inheritance report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(inheritance_data, "class_inheritance")?;
        debug!("Wrote class inheritance report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for compatibility diagnostics
    pub fn write_compatibility_diagnostics_report<T: Serialize>(&self, diagnostics_data: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("compatibility_diagnostics") {
            debug!("Skipping compatibility diagnostics report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(diagnostics_data, "compatibility_diagnostics")?;
        debug!("Wrote compatibility diagnostics report to {}", path.display());
        
        Ok(path)
    }
    
    /// Write a report for class existence validation
    pub fn write_class_existence_report<T: Serialize>(&self, existence_data: &T) -> Result<PathBuf> {
        if !self.base.is_report_enabled("class_existence") {
            debug!("Skipping class existence report (disabled in configuration)");
            return Ok(PathBuf::new());
        }
        
        let path = self.base.write_report(existence_data, "class_existence")?;
        debug!("Wrote class existence report to {}", path.display());
        
        Ok(path)
    }
}

/// Dependency summary item
#[derive(Serialize)]
pub struct DependencySummaryItem {
    pub name: String,
    pub total_equipment: usize,
    pub missing_classes: usize,
    pub available_classes: usize,
    pub missing_class_percentage: f64,
    pub inheritance_matches: usize,
}

/// Dependency summary report
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

/// Missing class detail
#[derive(Serialize)]
pub struct MissingClassDetail {
    pub class_name: String,
    pub used_in_missions: Vec<String>,
    pub usage_count: usize,
    pub possible_alternatives: Vec<String>,
    pub inheritance_path: Option<Vec<String>>,
}

/// Class usage report
#[derive(Serialize)]
pub struct ClassUsageReport {
    pub total_classes: usize,
    pub class_usage: Vec<ClassUsageDetail>,
}

/// Class usage detail
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

/// Mission compatibility
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

/// Compatibility level
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

/// Category need detail
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

/// Inheritance relationship
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

/// Mission diagnostics
#[derive(Serialize)]
pub struct MissionDiagnostics {
    pub mission_name: String,
    pub class_diagnostics: Vec<ClassDiagnostic>,
}

/// Class diagnostic
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

/// Class existence report
#[derive(Serialize)]
pub struct ClassExistenceReport {
    pub total_missions: usize,
    pub total_unique_classes: usize,
    pub existing_classes: usize,
    pub missing_classes: usize,
    pub existence_percentage: f64,
    pub mission_reports: Vec<MissionClassExistenceReport>,
}

/// Mission class existence report
#[derive(Serialize)]
pub struct MissionClassExistenceReport {
    pub mission_name: String,
    pub total_classes: usize,
    pub existing_classes: usize,
    pub missing_classes: usize,
    pub existence_percentage: f64,
    pub missing_class_list: Vec<MissingClassInfo>,
}

/// Missing class info
#[derive(Serialize)]
pub struct MissingClassInfo {
    pub class_name: String,
    pub reference_count: usize,
    pub reference_locations: Vec<String>,
    pub suggested_alternatives: Vec<String>,
} 