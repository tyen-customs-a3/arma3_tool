use std::collections::{HashMap, HashSet};
use crate::{GameClass, PropertyValue};

#[cfg(test)]
use arma3_types::{Class, Value, ClassMetadata};

/// Represents a query pattern to search for and extract data from HPP classes
#[derive(Debug, Clone)]
pub struct QueryPattern {
    /// The path to search for (e.g. "baseMan/uniform")
    path: Vec<String>,
    /// Properties to extract from matching classes
    properties: HashSet<String>,
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
        if class_path.len() < self.path.len() {
            return false;
        }
        
        // Check if the end of the class_path matches our pattern path
        let start_idx = class_path.len() - self.path.len();
        class_path[start_idx..].iter().zip(&self.path)
            .all(|(a, b)| b == "*" || a == b)
    }
}

/// Extracts class dependencies from an HPP file using predefined patterns
pub struct DependencyExtractor {
    classes: Vec<GameClass>,
    patterns: Vec<QueryPattern>,
    /// Cache of property names to look for
    property_names: HashSet<String>,
}

impl DependencyExtractor {
    /// Create a new dependency extractor with default patterns for loadout files
    pub fn new(classes: Vec<GameClass>) -> Self {
        let patterns = vec![
            // Base equipment
            QueryPattern::new("*", &[
                "uniform", "vest", "backpack", "headgear",
                "goggles", "hmd", "faces", "insignias"
            ]),
            
            // Weapons and attachments
            QueryPattern::new("*", &[
                "primaryWeapon", "scope", "bipod", "attachment", "silencer",
                "secondaryWeapon", "secondaryAttachments",
                "sidearmWeapon", "sidearmAttachments"
            ]),
            
            // Items and magazines
            QueryPattern::new("*", &[
                "magazines", "items", "linkedItems", "backpackItems"
            ]),

            // Nested class properties
            QueryPattern::new("*/primaryWeapon", &["name"]),
            QueryPattern::new("*/secondaryWeapon", &["name"]),
            QueryPattern::new("*/sidearmWeapon", &["name"]),
        ];

        // Build property name cache
        let mut property_names = HashSet::new();
        for pattern in &patterns {
            property_names.extend(pattern.properties.iter().cloned());
        }

        Self { 
            classes,
            patterns,
            property_names,
        }
    }

    /// Extract all class dependencies from the HPP classes
    pub fn extract_dependencies(&self) -> HashSet<String> {
        let mut dependencies = HashSet::new();
        
        for class in &self.classes {
            // Build property index for fast lookup from HashMap properties
            let property_index: HashMap<_, _> = class.properties.iter()
                .filter(|(name, _)| self.property_names.contains(*name))
                .collect();
            
            self.process_class(class, &[], &property_index, &mut dependencies);
        }
        
        dependencies
    }
    
    /// Process a class and its properties recursively
    fn process_class(
        &self,
        class: &GameClass,
        current_path: &[String],
        property_index: &HashMap<&String, &PropertyValue>,
        dependencies: &mut HashSet<String>
    ) {
        // Build the current class path
        let mut class_path = current_path.to_vec();
        class_path.push(class.name.clone());
        
        // Check each pattern against the current class
        for pattern in &self.patterns {
            if pattern.matches_path(&class_path) {
                // Extract properties defined in the pattern
                for prop_name in &pattern.properties {
                    if let Some(value) = property_index.get(prop_name) {
                        match value {
                            PropertyValue::String(s) => {
                                dependencies.insert(s.to_string());
                            }
                            PropertyValue::Array(arr) => {
                                // Extract string values from the array
                                for value in arr {
                                    if let PropertyValue::String(s) = value {
                                        dependencies.insert(s.clone());
                                    }
                                }
                            }
                            PropertyValue::ClassRef(class_name) => {
                                dependencies.insert(class_name.clone());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        
        // Process nested classes
        for nested_class in class.classes.values() {
            // Build property index for nested class
            let nested_property_index: HashMap<_, _> = nested_class.properties.iter()
                .filter(|(name, _)| self.property_names.contains(*name))
                .collect();
            self.process_class(nested_class, &class_path, &nested_property_index, dependencies);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_simple_extraction() {
        let class = GameClass::new("baseMan")
            .with_property("uniform", PropertyValue::Array(vec![PropertyValue::String("test_uniform".to_string())]))
            .with_property("vest", PropertyValue::Array(vec![PropertyValue::String("test_vest".to_string())]))
            .with_file_path(PathBuf::from("test.hpp"));

        let extractor = DependencyExtractor::new(vec![class]);
        let dependencies = extractor.extract_dependencies();

        assert!(dependencies.contains("test_uniform"));
        assert!(dependencies.contains("test_vest"));
    }

    #[test]
    fn test_nested_extraction() {
        use std::collections::HashMap;
        
        let mut nested_properties = HashMap::new();
        nested_properties.insert("name".to_string(), Value::String("test_rifle".to_string()));
        
        let nested_class = Class {
            name: "primaryWeapon".to_string(),
            parent: None,
            properties: nested_properties,
            classes: HashMap::new(),
            arrays: HashMap::new(),
            external_classes: Vec::new(),
            delete_properties: Vec::new(),
            delete_classes: Vec::new(),
            container_class: Some("rifleman".to_string()),
            file_path: Some(PathBuf::from("test.hpp")),
            is_forward_declaration: false,
            line_number: None,
            metadata: ClassMetadata::default(),
        };

        let mut classes = HashMap::new();
        classes.insert("primaryWeapon".to_string(), nested_class);

        let class = Class {
            name: "rifleman".to_string(),
            parent: Some("baseMan".to_string()),
            properties: HashMap::new(),
            classes,
            arrays: HashMap::new(),
            external_classes: Vec::new(),
            delete_properties: Vec::new(),
            delete_classes: Vec::new(),
            container_class: None,
            file_path: Some(PathBuf::from("test.hpp")),
            is_forward_declaration: false,
            line_number: None,
            metadata: ClassMetadata::default(),
        };

        let extractor = DependencyExtractor::new(vec![class]);
        let dependencies = extractor.extract_dependencies();

        assert!(dependencies.contains("test_rifle"), "Dependencies: {:?}", dependencies);
    }

    #[test]
    fn test_path_matching() {
        let pattern = QueryPattern::new("*/primaryWeapon", &["name"]);
        assert!(pattern.matches_path(&["rifleman".to_string(), "primaryWeapon".to_string()]));
        assert!(pattern.matches_path(&["baseMan".to_string(), "inventory".to_string(), "primaryWeapon".to_string()]));
        assert!(!pattern.matches_path(&["rifleman".to_string()]));
        assert!(!pattern.matches_path(&["primaryWeapon".to_string(), "magazine".to_string()]));
    }
} 