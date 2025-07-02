use std::collections::HashSet;
use hemtt_sqm::{Class, SqmFile, Value};
use crate::models::{ClassExt, DependencyCollector};

/// Represents a query pattern to search for and extract data from SQM classes
#[derive(Debug, Clone)]
pub struct QueryPattern {
    /// The path to search for (e.g. "Inventory/primaryWeapon")
    path: Vec<String>,
    /// Properties to extract from matching classes
    properties: Vec<String>,
}

impl QueryPattern {
    /// Create a new query pattern
    pub fn new(path: &str, properties: &[&str]) -> Self {
        Self {
            path: path.split('/').map(String::from).collect(),
            properties: properties.iter().map(|&s| s.to_string()).collect(),
        }
    }

    /// Check if a class matches this pattern's path
    fn matches_path(&self, class_path: &[String]) -> bool {
        if self.path.is_empty() {
            return false;
        }
        
        // Handle leading wildcard
        if self.path[0] == "*" {
            // Try to match the pattern starting from any position in the class_path
            for start_pos in 0..=class_path.len().saturating_sub(self.path.len() - 1) {
                if self.matches_from_position(class_path, start_pos, 1) {
                    return true;
                }
            }
            false
        } else {
            // No leading wildcard, match from the end as before
            if class_path.len() < self.path.len() {
                return false;
            }
            
            let start_idx = class_path.len() - self.path.len();
            self.matches_from_position(class_path, start_idx, 0)
        }
    }
    
    /// Helper function to match pattern from a specific position
    fn matches_from_position(&self, class_path: &[String], start_pos: usize, pattern_start: usize) -> bool {
        let mut class_idx = start_pos;
        
        for pattern_idx in pattern_start..self.path.len() {
            if class_idx >= class_path.len() {
                return false;
            }
            
            let pattern_part = &self.path[pattern_idx];
            
            if pattern_part == "*" {
                // Wildcard matches anything
                class_idx += 1;
                continue;
            } else if pattern_part.contains("*") {
                // Pattern contains wildcard - check if the actual path starts with the prefix
                let prefix = pattern_part.replace("*", "");
                if !prefix.is_empty() && !class_path[class_idx].starts_with(&prefix) {
                    return false;
                }
                class_idx += 1;
            } else {
                // Exact match required
                if &class_path[class_idx] != pattern_part {
                    return false;
                }
                class_idx += 1;
            }
        }
        
        true
    }
}

/// Extracts class dependencies from an SQM file using predefined patterns
pub struct DependencyExtractor<'a> {
    sqm_file: &'a SqmFile,
    patterns: Vec<QueryPattern>,
}

impl<'a> DependencyExtractor<'a> {
    /// Create a new dependency extractor with default patterns
    pub fn new(sqm_file: &'a SqmFile) -> Self {
        let patterns = vec![
            // Inventory direct properties (simple string properties) - match at any depth
            QueryPattern::new("*/Inventory", &[
                "uniform", "vest", "backpack", "headgear",
                "map", "compass", "watch", "radio", "gps", "goggles"
            ]),
            
            // Properties inside Item* classes within Inventory
            QueryPattern::new("*/Inventory/Item*", &[
                "uniform", "vest", "backpack", "headgear",
                "map", "compass", "watch", "radio", "gps", "goggles"
            ]),
            
            // Primary weapon and magazines - match at any depth
            QueryPattern::new("*/Inventory/primaryWeapon", &["name", "muzzle"]),
            QueryPattern::new("*/Inventory/primaryWeapon/primaryMuzzleMag", &["name"]),
            
            // Primary weapon inside Item* classes
            QueryPattern::new("*/Inventory/Item*/primaryWeapon", &["name", "muzzle"]),
            QueryPattern::new("*/Inventory/Item*/primaryWeapon/primaryMuzzleMag", &["name"]),
            
            // Secondary weapon and magazines - match at any depth
            QueryPattern::new("*/Inventory/secondaryWeapon", &["name", "muzzle"]),
            QueryPattern::new("*/Inventory/secondaryWeapon/primaryMuzzleMag", &["name"]),
            
            // Handgun weapon and magazines (note: "handgun" not "handgunWeapon") - match at any depth
            QueryPattern::new("*/Inventory/handgun", &["name", "muzzle"]),
            QueryPattern::new("*/Inventory/handgun/primaryMuzzleMag", &["name"]),
            QueryPattern::new("*/Inventory/handgunWeapon", &["name", "muzzle"]),
            QueryPattern::new("*/Inventory/handgunWeapon/primaryMuzzleMag", &["name"]),
            
            // Container items with typeName (uniform, vest, backpack as objects) - match at any depth
            QueryPattern::new("*/Inventory/uniform", &["typeName"]),
            QueryPattern::new("*/Inventory/vest", &["typeName"]),
            QueryPattern::new("*/Inventory/backpack", &["typeName"]),
            QueryPattern::new("*/Inventory/headgear", &["typeName"]),
            
            // Container contents (items inside containers) - match at any depth
            QueryPattern::new("*/Inventory/uniform/ItemCargo/Item*", &["name"]),
            QueryPattern::new("*/Inventory/vest/ItemCargo/Item*", &["name"]),
            QueryPattern::new("*/Inventory/backpack/ItemCargo/Item*", &["name"]),
            QueryPattern::new("*/Inventory/uniform/MagazineCargo/Item*", &["name"]),
            QueryPattern::new("*/Inventory/vest/MagazineCargo/Item*", &["name"]),
            QueryPattern::new("*/Inventory/backpack/MagazineCargo/Item*", &["name"]),
            
            // General container contents (wildcard pattern for any container) - match at any depth
            QueryPattern::new("*/Inventory/*/ItemCargo/Item*", &["name"]),
            QueryPattern::new("*/Inventory/*/MagazineCargo/Item*", &["name"]),
            
            // General object types
            QueryPattern::new("*", &["type"]),
        ];
        
        Self { sqm_file, patterns }
    }

    /// Extract all class dependencies from the SQM file
    pub fn extract_dependencies(&self) -> HashSet<String> {
        let mut collector = DependencyCollector::new();
        
        // Process all Mission classes
        for mission_class in self.get_mission_classes() {
            self.process_class(mission_class, &[], &mut collector);
        }
        
        collector.get_dependencies()
    }
    
    /// Process a class and its children recursively
    fn process_class(&self, class: &Class, current_path: &[String], collector: &mut DependencyCollector) {
        // Build the current class path
        let mut class_path = current_path.to_vec();
        class_path.push(class.name.clone());
        
        // Check each pattern against the current class
        for pattern in &self.patterns {
            if pattern.matches_path(&class_path) {
                // Extract properties defined in the pattern
                for prop_name in &pattern.properties {
                    if let Some(value) = class.get_property_string(prop_name) {
                        collector.add_dependency(value);
                    }
                }
            }
        }
        
        // Process child classes
        for (child_name, child_classes) in &class.classes {
            for child_class in child_classes {
                self.process_class(child_class, &class_path, collector);
            }
        }
    }
    
    /// Get all Mission classes from the SQM file
    fn get_mission_classes(&self) -> Vec<&Class> {
        self.sqm_file.classes.get("Mission")
            .map(|classes| classes.iter().collect())
            .unwrap_or_default()
    }
}