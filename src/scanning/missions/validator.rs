use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug, warn};

use crate::reporting::dependency::{
    ClassExistenceReport, MissionClassExistenceReport, MissingClassInfo
};
use super::analyzer::MissionDependencyResult;
use crate::scanning::classes::processor::ProcessedClass;

/// Validates if classes used in missions exist in the scanned database
pub struct ClassExistenceValidator {
    /// In-memory cache of known classes
    known_classes: HashSet<String>,
    /// Flag indicating if the class database has been loaded
    db_loaded: bool,
}

impl ClassExistenceValidator {
    /// Create a new validator with the path to the class database
    pub fn new() -> Self {
        Self {
            known_classes: HashSet::new(),
            db_loaded: false,
        }
    }
    
    /// Load the class database from the specified directory
    pub fn load_class_database_from_disk(&mut self, class_db_dir: &Path) -> Result<()> {
        if self.db_loaded {
            debug!("Class database already loaded");
            return Ok(());
        }
        
        info!("Loading class database from {}", class_db_dir.display());
        
        // Check if the directory exists
        if !class_db_dir.exists() {
            return Err(anyhow::anyhow!("Class database directory does not exist: {}", class_db_dir.display()));
        }
        
        // Find all JSON files in the directory that might contain class data
        let class_files = walkdir::WalkDir::new(class_db_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("json"))
                    .unwrap_or(false)
            })
            .map(|e| e.path().to_owned())
            .collect::<Vec<_>>();
        
        if class_files.is_empty() {
            return Err(anyhow::anyhow!("No class data files found in {}", class_db_dir.display()));
        }
        
        info!("Found {} potential class data files", class_files.len());
        
        // Load classes from each file
        let mut total_classes = 0;
        
        for file_path in &class_files {
            // Skip files that don't look like they contain class data
            let file_name = file_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
                
            if !file_name.contains("class") && !file_name.contains("classes") {
                debug!("Skipping non-class file: {}", file_path.display());
                continue;
            }
            
            debug!("Loading classes from {}", file_path.display());
            
            // Read and parse the file
            let content = std::fs::read_to_string(&file_path)
                .context(format!("Failed to read class data file: {}", file_path.display()))?;
                
            // Try to parse as different formats
            let classes = match serde_json::from_str::<Vec<ClassData>>(&content) {
                Ok(data) => {
                    // Extract class names from the parsed data
                    data.into_iter()
                        .map(|c| c.name)
                        .collect::<Vec<_>>()
                },
                Err(_) => {
                    // Try parsing as a simple array of strings
                    match serde_json::from_str::<Vec<String>>(&content) {
                        Ok(data) => data,
                        Err(_) => {
                            // Try parsing as a map with class names as keys
                            match serde_json::from_str::<HashMap<String, serde_json::Value>>(&content) {
                                Ok(data) => data.keys().cloned().collect(),
                                Err(e) => {
                                    warn!("Failed to parse class data from {}: {}", file_path.display(), e);
                                    continue;
                                }
                            }
                        }
                    }
                }
            };
            
            // Add classes to the known set
            let file_class_count = classes.len();
            self.known_classes.extend(classes);
            total_classes += file_class_count;
            
            debug!("Loaded {} classes from {}", file_class_count, file_path.display());
        }
        
        info!("Loaded a total of {} unique classes from {} files", 
              self.known_classes.len(), class_files.len());
        
        self.db_loaded = true;
        
        Ok(())
    }
    
    /// Load the class database from in-memory processed classes
    pub fn load_class_database_from_memory(&mut self, processed_classes: &[ProcessedClass]) -> Result<()> {
        if self.db_loaded {
            debug!("Class database already loaded");
            return Ok(());
        }
        
        info!("Loading class database from {} in-memory processed classes", processed_classes.len());
        
        // Extract class names from the processed classes
        for class in processed_classes {
            self.known_classes.insert(class.name.clone());
        }
        
        info!("Loaded a total of {} unique classes from memory", self.known_classes.len());
        
        self.db_loaded = true;
        
        Ok(())
    }
    
    /// Validate if classes used in missions exist in the database
    pub fn validate_mission_classes(&self, mission_results: &[MissionDependencyResult]) -> Result<ClassExistenceReport> {
        if !self.db_loaded {
            return Err(anyhow::anyhow!("Class database not loaded. Call load_class_database_from_disk() or load_class_database_from_memory() first."));
        }
        
        info!("Validating classes for {} missions", mission_results.len());
        
        let mut mission_reports = Vec::new();
        let mut all_classes = HashSet::new();
        let mut existing_classes = HashSet::new();
        let mut missing_classes = HashSet::new();
        
        // Process each mission
        for mission in mission_results {
            let mission_name = &mission.mission_name;
            debug!("Validating classes for mission: {}", mission_name);
            
            let total_classes = mission.unique_class_names.len();
            let mut mission_existing = HashSet::new();
            let mut mission_missing = Vec::new();
            
            // Check each class
            for class_name in &mission.unique_class_names {
                all_classes.insert(class_name.clone());
                
                if self.known_classes.contains(class_name) {
                    mission_existing.insert(class_name.clone());
                    existing_classes.insert(class_name.clone());
                } else {
                    // Find references to this class in the dependencies
                    let references = mission.class_dependencies.iter()
                        .filter(|dep| dep.class_name == *class_name)
                        .map(|dep| format!("{}:{}", 
                            dep.source_file.file_name().unwrap_or_default().to_string_lossy(),
                            dep.line_number))
                        .collect::<Vec<_>>();
                    
                    // Find potential alternatives
                    let alternatives = self.find_similar_classes(class_name);
                    
                    mission_missing.push(MissingClassInfo {
                        class_name: class_name.clone(),
                        reference_count: references.len(),
                        reference_locations: references,
                        suggested_alternatives: alternatives,
                    });
                    
                    missing_classes.insert(class_name.clone());
                }
            }
            
            // Calculate existence percentage
            let existing_count = mission_existing.len();
            let missing_count = mission_missing.len();
            let existence_percentage = if total_classes > 0 {
                (existing_count as f64 / total_classes as f64) * 100.0
            } else {
                100.0 // If no classes, consider 100% existing
            };
            
            // Create mission report
            mission_reports.push(MissionClassExistenceReport {
                mission_name: mission_name.clone(),
                total_classes,
                existing_classes: existing_count,
                missing_classes: missing_count,
                existence_percentage,
                missing_class_list: mission_missing,
            });
            
            debug!("Mission '{}' class validation: {}/{} classes exist ({}%)",
                  mission_name, existing_count, total_classes, existence_percentage);
        }
        
        // Calculate overall statistics
        let total_unique_classes = all_classes.len();
        let total_existing = existing_classes.len();
        let total_missing = missing_classes.len();
        let overall_percentage = if total_unique_classes > 0 {
            (total_existing as f64 / total_unique_classes as f64) * 100.0
        } else {
            100.0
        };
        
        info!("Overall class validation: {}/{} unique classes exist ({}%)",
             total_existing, total_unique_classes, overall_percentage);
        
        // Create the final report
        let report = ClassExistenceReport {
            total_missions: mission_results.len(),
            total_unique_classes,
            existing_classes: total_existing,
            missing_classes: total_missing,
            existence_percentage: overall_percentage,
            mission_reports,
        };
        
        Ok(report)
    }
    
    /// Find similar classes to suggest as alternatives
    fn find_similar_classes(&self, class_name: &str) -> Vec<String> {
        let mut alternatives = Vec::new();
        let class_lower = class_name.to_lowercase();
        
        // Simple similarity check - find classes that contain parts of the missing class name
        // or have similar prefixes/suffixes
        for known_class in &self.known_classes {
            let known_lower = known_class.to_lowercase();
            
            // Skip exact matches (shouldn't happen) and very short class names
            if known_lower == class_lower || known_lower.len() < 3 {
                continue;
            }
            
            // Check for common prefixes/suffixes or partial matches
            if (known_lower.starts_with(&class_lower) || class_lower.starts_with(&known_lower)) ||
               (known_lower.ends_with(&class_lower) || class_lower.ends_with(&known_lower)) ||
               (known_lower.contains(&class_lower) || class_lower.contains(&known_lower))
            {
                alternatives.push(known_class.clone());
                
                // Limit the number of alternatives
                if alternatives.len() >= 5 {
                    break;
                }
            }
        }
        
        alternatives
    }
}

/// Helper struct for parsing class data from JSON files
#[derive(serde::Deserialize)]
struct ClassData {
    name: String,
    // Other fields are ignored
} 