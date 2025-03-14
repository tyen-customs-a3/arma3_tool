use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use std::collections::{HashMap, HashSet};
use log::{debug, info, warn, error};
use serde::{Serialize, Deserialize};
use crate::error::{Result, ToolError};
use crate::scanner::gamedata::{GameDataClasses, ClassDefinition, PboSource};
use mission_scanner::MissionResults;

/// Report generator for mission dependencies
pub struct ReportGenerator {
    /// Directory for output reports
    report_dir: PathBuf,
}

/// Represents a dependency analysis for a mission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyAnalysis {
    /// Name of the mission
    pub mission_name: String,
    
    /// Path to the mission
    pub mission_path: PathBuf,
    
    /// Required mods
    pub required_mods: Vec<ModRequirement>,
    
    /// Missing dependencies
    pub missing_dependencies: Vec<MissingDependency>,
    
    /// Class usage statistics
    pub class_usage: HashMap<String, usize>,
}

/// Represents a mod requirement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModRequirement {
    /// Name of the mod
    pub name: String,
    
    /// Number of classes used from this mod
    pub class_count: usize,
    
    /// List of classes used from this mod
    pub classes: Vec<String>,
}

/// Represents a missing dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingDependency {
    /// Name of the missing class
    pub class_name: String,
    
    /// Context where the class is referenced
    pub context: String,
    
    /// Type of reference
    pub reference_type: String,
}

impl ReportGenerator {
    /// Create a new report generator
    pub fn new(report_dir: PathBuf) -> Self {
        Self {
            report_dir,
        }
    }
    
    /// Generate reports for specific missions
    pub fn generate(&self, missions: Option<Vec<String>>) -> Result<()> {
        // This is a placeholder for the actual implementation
        // In a real implementation, we would load cached scan results and generate reports
        
        info!("Generating reports for missions: {:?}", missions);
        
        Ok(())
    }
    
    /// Generate reports from scan results
    pub fn generate_from_results(&self, game_data: &GameDataClasses, mission_results: &[MissionResults]) -> Result<()> {
        // Create report directory if it doesn't exist
        if !self.report_dir.exists() {
            fs::create_dir_all(&self.report_dir)
                .map_err(|e| ToolError::ReportError(format!("Failed to create report directory: {}", e)))?;
        }
        
        // Generate a report for each mission
        for mission_result in mission_results {
            let analysis = self.analyze_dependencies(game_data, mission_result);
            self.write_report(&analysis)?;
        }
        
        // Generate a summary report
        self.write_summary_report(mission_results)?;
        
        Ok(())
    }
    
    /// Analyze dependencies for a mission
    pub fn analyze_dependencies(&self, game_data: &GameDataClasses, mission_result: &MissionResults) -> DependencyAnalysis {
        let mut analysis = DependencyAnalysis {
            mission_name: mission_result.mission_name.clone(),
            mission_path: mission_result.mission_dir.clone(),
            required_mods: Vec::new(),
            missing_dependencies: Vec::new(),
            class_usage: HashMap::new(),
        };
        
        // Create a map of class names to their definitions
        let mut class_map: HashMap<&str, &ClassDefinition> = HashMap::new();
        for class in &game_data.classes {
            class_map.insert(&class.name, class);
        }
        
        // Create a map of class names to their source PBOs
        let mut class_sources: HashMap<&str, &PboSource> = HashMap::new();
        for class in &game_data.classes {
            if let Some(source) = game_data.sources.get(class.source_pbo_index) {
                class_sources.insert(&class.name, source);
            }
        }
        
        // Track mod usage
        let mut mod_usage: HashMap<String, HashSet<String>> = HashMap::new();
        
        // Process each dependency
        for dep in &mission_result.class_dependencies {
            // Update class usage statistics
            *analysis.class_usage.entry(dep.class_name.clone()).or_insert(0) += 1;
            
            // Check if class exists in game data
            if let Some(class_def) = class_map.get(dep.class_name.as_str()) {
                // Class exists, track mod usage
                if let Some(source) = class_sources.get(dep.class_name.as_str()) {
                    let mod_name = source.mod_name.clone();
                    mod_usage.entry(mod_name)
                        .or_insert_with(HashSet::new)
                        .insert(dep.class_name.clone());
                }
            } else {
                // Class doesn't exist, add to missing dependencies
                analysis.missing_dependencies.push(MissingDependency {
                    class_name: dep.class_name.clone(),
                    context: dep.context.clone(),
                    reference_type: format!("{:?}", dep.reference_type),
                });
            }
        }
        
        // Convert mod usage to required mods
        for (mod_name, classes) in mod_usage {
            let class_vec: Vec<String> = classes.into_iter().collect();
            analysis.required_mods.push(ModRequirement {
                name: mod_name,
                class_count: class_vec.len(),
                classes: class_vec,
            });
        }
        
        // Sort required mods by class count (descending)
        analysis.required_mods.sort_by(|a, b| b.class_count.cmp(&a.class_count));
        
        analysis
    }
    
    /// Write a report for a mission
    fn write_report(&self, analysis: &DependencyAnalysis) -> Result<()> {
        let mission_name = analysis.mission_name.replace(" ", "_").replace("/", "_");
        let report_path = self.report_dir.join(format!("{}_report.json", mission_name));
        
        // Serialize to JSON
        let json = serde_json::to_string_pretty(analysis)
            .map_err(|e| ToolError::ReportError(format!("Failed to serialize report: {}", e)))?;
            
        // Write to file
        let mut file = File::create(&report_path)
            .map_err(|e| ToolError::ReportError(format!("Failed to create report file: {}", e)))?;
            
        file.write_all(json.as_bytes())
            .map_err(|e| ToolError::ReportError(format!("Failed to write report: {}", e)))?;
            
        info!("Wrote report to {}", report_path.display());
        
        Ok(())
    }
    
    /// Write a summary report for all missions
    fn write_summary_report(&self, mission_results: &[MissionResults]) -> Result<()> {
        let summary_path = self.report_dir.join("summary_report.json");
        
        // Create summary data
        let summary = MissionSummary {
            total_missions: mission_results.len(),
            missions: mission_results.iter().map(|m| MissionInfo {
                name: m.mission_name.clone(),
                path: m.mission_dir.clone(),
                sqf_files: m.sqf_files.len(),
                cpp_files: m.cpp_files.len(),
                dependencies: m.class_dependencies.len(),
            }).collect(),
        };
        
        // Serialize to JSON
        let json = serde_json::to_string_pretty(&summary)
            .map_err(|e| ToolError::ReportError(format!("Failed to serialize summary report: {}", e)))?;
            
        // Write to file
        let mut file = File::create(&summary_path)
            .map_err(|e| ToolError::ReportError(format!("Failed to create summary report file: {}", e)))?;
            
        file.write_all(json.as_bytes())
            .map_err(|e| ToolError::ReportError(format!("Failed to write summary report: {}", e)))?;
            
        info!("Wrote summary report to {}", summary_path.display());
        
        Ok(())
    }
}

/// Represents a summary of all missions
#[derive(Debug, Serialize, Deserialize)]
struct MissionSummary {
    /// Total number of missions
    total_missions: usize,
    
    /// Information about each mission
    missions: Vec<MissionInfo>,
}

/// Represents information about a mission
#[derive(Debug, Serialize, Deserialize)]
struct MissionInfo {
    /// Name of the mission
    name: String,
    
    /// Path to the mission
    path: PathBuf,
    
    /// Number of SQF files
    sqf_files: usize,
    
    /// Number of CPP/HPP files
    cpp_files: usize,
    
    /// Number of dependencies
    dependencies: usize,
} 