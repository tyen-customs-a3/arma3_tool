use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug, warn};

use crate::scanning::classes::processor::ProcessedClass;
use crate::scanning::missions::analyzer::types::MissionDependencyResult;
use crate::searching::class_search::search_classes_parallel;

use super::ClassExistenceValidator;
use super::types::{ClassExistenceReport, MissionClassExistenceReport, MissingClassInfo};

/// Load the class database from in-memory processed classes
pub fn load_class_database_from_memory(
    validator: &mut ClassExistenceValidator,
    processed_classes: &[ProcessedClass]
) -> Result<()> {
    if validator.db_loaded {
        debug!("Class database already loaded");
        return Ok(());
    }
    
    info!("Loading class database from {} in-memory processed classes", processed_classes.len());
    
    // Store the processed classes directly
    validator.processed_classes = processed_classes.to_vec();
    
    // Also add alternative classes based on 'name' properties
    let mut alternative_classes = Vec::new();
    
    for class in &validator.processed_classes {
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
    validator.processed_classes.extend(alternative_classes);
    
    info!("Loaded a total of {} unique classes from memory", validator.processed_classes.len());
    
    validator.db_loaded = true;
    
    Ok(())
}

/// Validate if classes used in missions exist in the database
pub fn validate_mission_classes(
    validator: &ClassExistenceValidator,
    mission_results: &[MissionDependencyResult]
) -> Result<ClassExistenceReport> {
    if !validator.db_loaded {
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
        
        // Track class references for better reporting
        let mut class_references: HashMap<String, (usize, Vec<String>)> = HashMap::new();
        
        // Count references for each class
        for dep in &mission.class_dependencies {
            let entry = class_references.entry(dep.class_name.clone()).or_insert((0, Vec::new()));
            entry.0 += 1;
            
            let location = format!("{}:{} ({})", 
                dep.source_file.display(), 
                dep.line_number,
                dep.reference_type);
            
            entry.1.push(location);
        }
        
        // Search for classes in the database
        let search_results = search_classes_parallel(
            &class_names,
            &validator.processed_classes
        );
        
        // Process search results
        for result in search_results {
            all_classes.insert(result.class_name.clone());
            
            if result.found {
                // Class exists
                mission_existing.insert(result.class_name.clone());
                existing_classes.insert(result.class_name.clone());
            } else {
                // Class doesn't exist
                let (ref_count, ref_locations) = class_references
                    .get(&result.class_name)
                    .cloned()
                    .unwrap_or((0, Vec::new()));
                
                // Get suggested alternatives - for simplicity, we'll just use similar classes
                let alternatives = find_similar_classes(validator, &result.class_name);
                
                mission_missing.push(MissingClassInfo {
                    class_name: result.class_name.clone(),
                    reference_count: ref_count,
                    reference_locations: ref_locations,
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
            100.0 // If no classes, consider 100% existence
        };
        
        // Create mission report
        let mission_report = MissionClassExistenceReport {
            mission_name: mission_name.clone(),
            total_classes,
            existing_classes: existing_count,
            missing_classes: missing_count,
            existence_percentage,
            missing_class_list: mission_missing,
        };
        
        mission_reports.push(mission_report);
    }
    
    // Calculate overall statistics
    let total_unique_classes = all_classes.len();
    let total_existing = existing_classes.len();
    let total_missing = missing_classes.len();
    let overall_existence_percentage = if total_unique_classes > 0 {
        (total_existing as f64 / total_unique_classes as f64) * 100.0
    } else {
        100.0 // If no classes, consider 100% existence
    };
    
    // Create overall report
    let report = ClassExistenceReport {
        total_missions: mission_results.len(),
        total_unique_classes,
        existing_classes: total_existing,
        missing_classes: total_missing,
        existence_percentage: overall_existence_percentage,
        mission_reports,
    };
    
    info!("Validation complete: {} missions, {} unique classes, {} existing, {} missing, {:.2}% existence",
          report.total_missions,
          report.total_unique_classes,
          report.existing_classes,
          report.missing_classes,
          report.existence_percentage);
    
    Ok(report)
}

/// Find similar classes to a given class name
pub fn find_similar_classes(
    validator: &ClassExistenceValidator,
    class_name: &str
) -> Vec<String> {
    if !validator.db_loaded {
        return Vec::new();
    }
    
    // Use fuzzy matching to find similar classes
    let mut similar_classes = Vec::new();
    let lower_class = class_name.to_lowercase();
    
    for class in &validator.processed_classes {
        let lower_name = class.name.to_lowercase();
        
        // Check for substring match
        if lower_name.contains(&lower_class) || lower_class.contains(&lower_name) {
            similar_classes.push(class.name.clone());
            continue;
        }
        
        // Check for Levenshtein distance (simple implementation)
        let distance = levenshtein_distance(&lower_class, &lower_name);
        let max_len = std::cmp::max(lower_class.len(), lower_name.len());
        
        // If distance is less than 30% of the max length, consider it similar
        if max_len > 0 && (distance as f64 / max_len as f64) < 0.3 {
            similar_classes.push(class.name.clone());
        }
    }
    
    // Limit to top 5 similar classes
    similar_classes.truncate(5);
    
    similar_classes
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    
    let m = s1_chars.len();
    let n = s2_chars.len();
    
    // Handle edge cases
    if m == 0 { return n; }
    if n == 0 { return m; }
    
    // Create distance matrix
    let mut matrix = vec![vec![0; n + 1]; m + 1];
    
    // Initialize first row and column
    for i in 0..=m {
        matrix[i][0] = i;
    }
    
    for j in 0..=n {
        matrix[0][j] = j;
    }
    
    // Fill the matrix
    for i in 1..=m {
        for j in 1..=n {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,     // deletion
                    matrix[i][j - 1] + 1      // insertion
                ),
                matrix[i - 1][j - 1] + cost   // substitution
            );
        }
    }
    
    matrix[m][n]
} 