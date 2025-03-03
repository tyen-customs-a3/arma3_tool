use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug, warn};
use serde::Serialize;

use super::analyzer::MissionDependencyResult;
use crate::scanning::classes::processor::ProcessedClass;

/// Validates if classes used in missions exist in the scanned database
pub struct ClassExistenceValidator {
    /// In-memory cache of known classes (stored in lowercase for case-insensitive comparison)
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
                    // Extract class names from the parsed data and also check for 'name' properties
                    let mut class_names = Vec::new();
                    
                    for class_data in data {
                        // Add the class name itself
                        class_names.push(class_data.name);
                        
                        // Check for a 'name' property
                        if let Some(name_prop) = class_data.properties.get("name") {
                            let clean_value = name_prop.trim_matches('"').trim_matches('\'');
                            if !clean_value.is_empty() {
                                debug!("Adding alternative class name from 'name' property in JSON: {}", clean_value);
                                class_names.push(clean_value.to_string());
                            }
                        }
                    }
                    
                    class_names
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
            // Convert all class names to lowercase for case-insensitive comparison
            self.known_classes.extend(classes.into_iter().map(|name| name.to_lowercase()));
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
        
        // Extract class names from the processed classes (convert to lowercase for case-insensitive comparison)
        for class in processed_classes {
            // Add the class name itself
            self.known_classes.insert(class.name.to_lowercase());
            
            // Also check for a 'name' property and add it as an alternative class name
            for (prop_name, prop_value) in &class.properties {
                if prop_name.to_lowercase() == "name" {
                    // Remove quotes if present
                    let clean_value = prop_value.trim_matches('"').trim_matches('\'');
                    if !clean_value.is_empty() {
                        debug!("Adding alternative class name from 'name' property: {} for class {}", 
                              clean_value, class.name);
                        self.known_classes.insert(clean_value.to_lowercase());
                    }
                    break; // Found the name property, no need to continue
                }
            }
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
                
                // Convert to lowercase for case-insensitive comparison
                if self.known_classes.contains(&class_name.to_lowercase()) {
                    mission_existing.insert(class_name.clone());
                    existing_classes.insert(class_name.clone());
                } else {
                    // Check if this might be a class with a prefixed name (like _xx_ClassName)
                    // where the actual class name is in the 'name' property
                    let mut found_in_name_property = false;
                    
                    // Extract potential name property from context in dependencies
                    for dep in &mission.class_dependencies {
                        if dep.class_name.to_lowercase() == class_name.to_lowercase() {
                            // Look for 'name = "SomeClassName";' pattern in the context
                            let context = dep.context.to_lowercase();
                            if let Some(name_pos) = context.find("name = ") {
                                let after_name = &context[name_pos + 7..];
                                if let Some(quote_pos) = after_name.find('"') {
                                    let after_quote = &after_name[quote_pos + 1..];
                                    if let Some(end_quote_pos) = after_quote.find('"') {
                                        let extracted_name = &after_quote[..end_quote_pos];
                                        if !extracted_name.is_empty() && self.known_classes.contains(&extracted_name.to_lowercase()) {
                                            debug!("Found class {} through 'name' property: {}", class_name, extracted_name);
                                            mission_existing.insert(class_name.clone());
                                            existing_classes.insert(class_name.clone());
                                            found_in_name_property = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    
                    if !found_in_name_property {
                        // Find references to this class in the dependencies
                        let references = mission.class_dependencies.iter()
                            .filter(|dep| dep.class_name.to_lowercase() == class_name.to_lowercase())
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
                if self.known_classes.contains(without_prefix) {
                    alternatives.push(without_prefix.to_string());
                }
            }
        }
        
        // Simple similarity check - find classes that contain parts of the missing class name
        // or have similar prefixes/suffixes
        for known_class in &self.known_classes {
            // known_class is already lowercase since we store all class names in lowercase
            
            // Skip exact matches (shouldn't happen) and very short class names
            if known_class == &class_lower || known_class.len() < 3 {
                continue;
            }
            
            // Check for common prefixes/suffixes or partial matches
            if (known_class.starts_with(&class_lower) || class_lower.starts_with(known_class)) ||
               (known_class.ends_with(&class_lower) || class_lower.ends_with(known_class)) ||
               (known_class.contains(&class_lower) || class_lower.contains(known_class))
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