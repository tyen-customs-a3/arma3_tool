use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug, warn};
use serde::Serialize;

use super::analyzer::MissionDependencyResult;
use crate::scanning::classes::processor::ProcessedClass;
use crate::reporting::class_search::{search_classes_parallel, ClassSearchResult, MatchType};

/// Validates if classes used in missions exist in the scanned database
pub struct ClassExistenceValidator {
    /// Processed classes for searching
    processed_classes: Vec<ProcessedClass>,
    /// Flag indicating if the class database has been loaded
    db_loaded: bool,
}

impl ClassExistenceValidator {
    /// Create a new validator with the path to the class database
    pub fn new() -> Self {
        Self {
            processed_classes: Vec::new(),
            db_loaded: false,
        }
    }
    
    /// Load the class database from in-memory processed classes
    pub fn load_class_database_from_memory(&mut self, processed_classes: &[ProcessedClass]) -> Result<()> {
        if self.db_loaded {
            debug!("Class database already loaded");
            return Ok(());
        }
        
        info!("Loading class database from {} in-memory processed classes", processed_classes.len());
        
        // Store the processed classes directly
        self.processed_classes = processed_classes.to_vec();
        
        // Also add alternative classes based on 'name' properties
        let mut alternative_classes = Vec::new();
        
        for class in &self.processed_classes {
            // Check for a 'name' property and add it as an alternative class
            for (prop_name, prop_value) in &class.properties {
                if prop_name.to_lowercase() == "name" {
                    // Remove quotes if present
                    let clean_value = prop_value.trim_matches('"').trim_matches('\'');
                    if !clean_value.is_empty() {
                        debug!("Adding alternative class name from 'name' property: {} for class {}", 
                              clean_value, class.name);
                        
                        let alt_class = ProcessedClass {
                            name: clean_value.to_string(),
                            file_path: class.file_path.clone(),
                            parent: None,
                            properties: Vec::new(),
                        };
                        
                        alternative_classes.push(alt_class);
                    }
                    break; // Found the name property, no need to continue
                }
            }
        }
        
        // Add the alternative classes to the main collection
        self.processed_classes.extend(alternative_classes);
        
        info!("Loaded a total of {} unique classes from memory", self.processed_classes.len());
        
        self.db_loaded = true;
        
        Ok(())
    }
    
    /// Validate if classes used in missions exist in the database
    pub fn validate_mission_classes(&self, mission_results: &[MissionDependencyResult]) -> Result<ClassExistenceReport> {
        if !self.db_loaded {
            return Err(anyhow::anyhow!("Class database not loaded. Call load_class_database_from_memory() first."));
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
            
            // Use the class_search functionality to search for all classes at once
            let class_names: Vec<String> = mission.unique_class_names.iter().cloned().collect();
            let search_results = search_classes_parallel(&class_names, &self.processed_classes);
            
            // Process the search results
            for result in search_results {
                all_classes.insert(result.class_name.clone());
                
                if result.found {
                    mission_existing.insert(result.class_name.clone());
                    existing_classes.insert(result.class_name.clone());
                } else {
                    // Find references to this class in the dependencies
                    let references = mission.class_dependencies.iter()
                        .filter(|dep| dep.class_name.eq_ignore_ascii_case(&result.class_name))
                        .map(|dep| format!("{}:{}", 
                            dep.source_file.file_name().unwrap_or_default().to_string_lossy(),
                            dep.line_number))
                        .collect::<Vec<_>>();
                    
                    // Find potential alternatives
                    let alternatives = self.find_similar_classes(&result.class_name);
                    
                    mission_missing.push(MissingClassInfo {
                        class_name: result.class_name.clone(),
                        reference_count: references.len(),
                        reference_locations: references,
                        suggested_alternatives: alternatives,
                    });
                    
                    missing_classes.insert(result.class_name.clone());
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
        
        // Check if this might be a prefixed class name (like _xx_ClassName)
        // Common prefixes in Arma configs
        let prefixes = ["_xx_", "item_", "weapon_", "magazine_", "backpack_", "uniform_", "vest_", "headgear_"];
        
        // Try removing common prefixes to see if we find a match
        for prefix in &prefixes {
            if class_lower.starts_with(prefix) {
                let without_prefix = &class_lower[prefix.len()..];
                
                // Use the class_search functionality to check if this class exists
                let search_result = crate::reporting::class_search::find_class_parallel(without_prefix, &self.processed_classes);
                if search_result.found {
                    alternatives.push(search_result.actual_class_name.unwrap_or_else(|| without_prefix.to_string()));
                }
            }
        }
        
        // If we don't have enough alternatives yet, look for partial matches
        if alternatives.len() < 5 {
            // Get all class names
            let all_class_names: Vec<String> = self.processed_classes.iter()
                .map(|c| c.name.clone())
                .collect();
                
            // Simple similarity check - find classes that contain parts of the missing class name
            // or have similar prefixes/suffixes
            for known_class in all_class_names {
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
                    alternatives.push(known_class);
                    
                    // Limit the number of alternatives
                    if alternatives.len() >= 5 {
                        break;
                    }
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
    // Add properties field that might contain a 'name' property
    #[serde(default)]
    properties: HashMap<String, String>,
}

/// Information about a missing class
#[derive(Debug, Clone, Serialize)]
pub struct MissingClassInfo {
    pub class_name: String,
    pub reference_count: usize,
    pub reference_locations: Vec<String>,
    pub suggested_alternatives: Vec<String>,
}

/// Report on class existence for a single mission
#[derive(Debug, Clone, Serialize)]
pub struct MissionClassExistenceReport {
    pub mission_name: String,
    pub total_classes: usize,
    pub existing_classes: usize,
    pub missing_classes: usize,
    pub existence_percentage: f64,
    pub missing_class_list: Vec<MissingClassInfo>,
}

/// Report on class existence validation
#[derive(Debug, Clone, Serialize)]
pub struct ClassExistenceReport {
    pub total_missions: usize,
    pub total_unique_classes: usize,
    pub existing_classes: usize,
    pub missing_classes: usize,
    pub existence_percentage: f64,
    pub mission_reports: Vec<MissionClassExistenceReport>,
} 