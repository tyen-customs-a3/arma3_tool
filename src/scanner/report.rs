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
    
    /// Validation issues found during analysis
    pub validation_issues: Vec<ValidationIssue>,
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

/// Represents a validation issue found during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Type of validation issue
    pub issue_type: String,
    
    /// Description of the issue
    pub description: String,
    
    /// Context where the issue was found
    pub context: String,
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
        
        // Log statistics about game data
        info!("Generating reports with {} game data classes for {} missions", 
            game_data.classes.len(), mission_results.len());
        
        // Track overall statistics
        let mut total_dependencies = 0;
        let mut total_matched = 0;
        let mut total_missing = 0;
        
        // Generate a report for each mission
        for mission_result in mission_results {
            let analysis = self.analyze_dependencies(game_data, mission_result);
            
            // Update statistics
            let dependencies = mission_result.class_dependencies.len();
            let missing = analysis.missing_dependencies.len();
            let matched = dependencies - missing;
            
            total_dependencies += dependencies;
            total_matched += matched;
            total_missing += missing;
            
            info!("Mission {}: {} dependencies, {} matched, {} missing", 
                mission_result.mission_name, dependencies, matched, missing);
            
            self.write_report(&analysis)?;
        }
        
        // Log overall statistics
        if total_dependencies > 0 {
            let match_percentage = (total_matched as f64 / total_dependencies as f64) * 100.0;
            info!("Overall: {} dependencies, {} matched ({:.1}%), {} missing", 
                total_dependencies, total_matched, match_percentage, total_missing);
        }
        
        // Generate a summary report
        self.write_summary_report(mission_results)?;
        
        Ok(())
    }
    
    /// Analyze dependencies for a mission
    pub fn analyze_dependencies(&self, game_data: &GameDataClasses, mission_result: &MissionResults) -> DependencyAnalysis {
        debug!("Analyzing dependencies for mission: {}", mission_result.mission_name);
        
        let mut analysis = DependencyAnalysis {
            mission_name: mission_result.mission_name.clone(),
            mission_path: mission_result.mission_dir.clone(),
            required_mods: Vec::new(),
            missing_dependencies: Vec::new(),
            class_usage: HashMap::new(),
            validation_issues: Vec::new(),
        };
        
        // Create a map of class names to their definitions (case-insensitive)
        // Use a capacity hint based on the number of classes for better performance
        let mut class_map: HashMap<String, &ClassDefinition> = HashMap::with_capacity(game_data.classes.len());
        for class in &game_data.classes {
            // Skip empty class names
            if !class.name.is_empty() {
                // Store with lowercase key for case-insensitive lookup
                class_map.insert(class.name.to_lowercase(), class);
            } else {
                warn!("Skipping empty class name in game data");
                analysis.validation_issues.push(ValidationIssue {
                    issue_type: "EmptyClassName".to_string(),
                    description: "Empty class name found in game data".to_string(),
                    context: "GameData".to_string(),
                });
            }
        }
        
        debug!("Created class map with {} entries", class_map.len());
        
        // Create a map of class names to their source PBOs (case-insensitive)
        // Use a capacity hint based on the number of classes for better performance
        let mut class_sources: HashMap<String, &PboSource> = HashMap::with_capacity(game_data.classes.len());
        for class in &game_data.classes {
            // Skip empty class names
            if !class.name.is_empty() {
                if let Some(source) = game_data.sources.get(class.source_pbo_index) {
                    // Store with lowercase key for case-insensitive lookup
                    class_sources.insert(class.name.to_lowercase(), source);
                }
            }
        }
        
        debug!("Created class sources map with {} entries", class_sources.len());
        
        // Track mod usage
        let mut mod_usage: HashMap<String, HashSet<String>> = HashMap::new();
        
        // Pre-allocate space for class usage based on the number of dependencies
        analysis.class_usage = HashMap::with_capacity(mission_result.class_dependencies.len());
        
        // Track match statistics
        let mut matched_count = 0;
        let mut missing_count = 0;
        
        // Process each dependency
        for dep in &mission_result.class_dependencies {
            // Skip empty class names
            if dep.class_name.is_empty() {
                warn!("Skipping empty class name in dependency: context={}", dep.context);
                analysis.validation_issues.push(ValidationIssue {
                    issue_type: "EmptyClassName".to_string(),
                    description: "Empty class name found in dependency".to_string(),
                    context: dep.context.clone(),
                });
                continue;
            }
            
            // Update class usage statistics (preserve original case for reporting)
            *analysis.class_usage.entry(dep.class_name.clone()).or_insert(0) += 1;
            
            // Convert to lowercase for case-insensitive lookup
            let lowercase_class_name = dep.class_name.to_lowercase();
            
            // Check if class exists in game data (case-insensitive)
            if let Some(class_def) = class_map.get(&lowercase_class_name) {
                // Class exists, track mod usage
                if let Some(source) = class_sources.get(&lowercase_class_name) {
                    let mod_name = source.mod_name.clone();
                    mod_usage.entry(mod_name.clone())
                        .or_insert_with(HashSet::new)
                        .insert(dep.class_name.clone()); // Preserve original case
                    
                    matched_count += 1;
                    debug!("Matched class '{}' to mod '{}'", dep.class_name, mod_name);
                } else {
                    // This shouldn't happen if our maps are consistent
                    warn!("Found class '{}' in class map but not in sources map", dep.class_name);
                    missing_count += 1;
                    analysis.missing_dependencies.push(MissingDependency {
                        class_name: dep.class_name.clone(),
                        context: dep.context.clone(),
                        reference_type: format!("{:?}", dep.reference_type),
                    });
                }
            } else {
                // Class doesn't exist, add to missing dependencies
                missing_count += 1;
                debug!("Missing class: '{}' (context: {})", dep.class_name, dep.context);
                analysis.missing_dependencies.push(MissingDependency {
                    class_name: dep.class_name.clone(),
                    context: dep.context.clone(),
                    reference_type: format!("{:?}", dep.reference_type),
                });
            }
        }
        
        debug!("Dependency analysis results: {} matched, {} missing", matched_count, missing_count);
        
        // Convert mod usage to required mods
        for (mod_name, classes) in mod_usage {
            // Convert HashSet to Vec, which ensures uniqueness
            let class_vec: Vec<String> = classes.into_iter().collect();
            let class_count = class_vec.len();
            analysis.required_mods.push(ModRequirement {
                name: mod_name.clone(),
                class_count,
                classes: class_vec,
            });
            debug!("Required mod: '{}' with {} classes", mod_name, class_count);
        }
        
        // Sort required mods by class count (descending)
        analysis.required_mods.sort_by(|a, b| b.class_count.cmp(&a.class_count));
        
        // Ensure missing dependencies have unique class names (case-insensitive)
        let mut unique_missing = HashMap::new();
        for missing in analysis.missing_dependencies.drain(..) {
            // Use lowercase key for case-insensitive uniqueness
            unique_missing
                .entry(missing.class_name.to_lowercase())
                .or_insert_with(Vec::new)
                .push(missing);
        }
        
        // Rebuild missing dependencies list with unique class names
        // Track duplicates for validation issues
        analysis.missing_dependencies = unique_missing
            .into_iter()
            .map(|(_, mut entries)| {
                // If we have more than one entry for this class name, it's a duplicate
                if entries.len() > 1 {
                    let class_name = entries[0].class_name.clone();
                    for duplicate in entries.iter().skip(1) {
                        analysis.validation_issues.push(ValidationIssue {
                            issue_type: "DuplicateClassName".to_string(),
                            description: format!("Duplicate class name: {} (case-insensitive)", class_name),
                            context: duplicate.context.clone(),
                        });
                    }
                }
                // Take the first entry for each unique class name
                entries.remove(0)
            })
            .collect();
        
        debug!("Final analysis: {} required mods, {} missing dependencies, {} validation issues",
            analysis.required_mods.len(), analysis.missing_dependencies.len(), analysis.validation_issues.len());
        
        analysis
    }
    
    /// Write a report for a mission
    fn write_report(&self, analysis: &DependencyAnalysis) -> Result<()> {
        // Validate the analysis before writing
        self.validate_analysis(analysis)?;
        
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
    
    /// Validate the analysis to ensure no empty or duplicate class names
    fn validate_analysis(&self, analysis: &DependencyAnalysis) -> Result<()> {
        // Check for empty class names in class usage
        for class_name in analysis.class_usage.keys() {
            if class_name.is_empty() {
                warn!("Empty class name found in class usage for mission: {}", analysis.mission_name);
            }
        }
        
        // Check for empty class names in required mods
        for mod_req in &analysis.required_mods {
            for class_name in &mod_req.classes {
                if class_name.is_empty() {
                    warn!("Empty class name found in mod requirement: {} for mission: {}", 
                          mod_req.name, analysis.mission_name);
                }
            }
            
            // Check for duplicate class names in required mods
            let mut unique_classes = HashSet::new();
            for class_name in &mod_req.classes {
                if !unique_classes.insert(class_name) {
                    warn!("Duplicate class name found in mod requirement: {} - class: {} for mission: {}", 
                          mod_req.name, class_name, analysis.mission_name);
                }
            }
        }
        
        // Check for empty class names in missing dependencies
        for missing in &analysis.missing_dependencies {
            if missing.class_name.is_empty() {
                warn!("Empty class name found in missing dependency for mission: {}", 
                      analysis.mission_name);
            }
        }
        
        // Log validation issues
        if !analysis.validation_issues.is_empty() {
            info!("Found {} validation issues for mission: {}", 
                  analysis.validation_issues.len(), analysis.mission_name);
            for issue in &analysis.validation_issues {
                warn!("Validation issue: {} - {} ({})", 
                      issue.issue_type, issue.description, issue.context);
            }
        }
        
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