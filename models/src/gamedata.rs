use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::types::PropertyValue;

/// Collection of game data classes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataClasses {
    /// Collection of classes
    pub classes: Vec<GameDataClass>,
    
    /// Collection of source files
    pub file_sources: Vec<PathBuf>,
}

impl GameDataClasses {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
            file_sources: Vec::new(),
        }
    }
    
    /// Add a file source and return its index
    pub fn add_file_source(&mut self, path: PathBuf) -> usize {
        let index = self.file_sources.len();
        self.file_sources.push(path);
        index
    }
    
    /// Get file source by index
    pub fn get_file_source(&self, index: usize) -> Option<&PathBuf> {
        self.file_sources.get(index)
    }
    
    /// Add a class to the collection
    pub fn add_class(&mut self, class: GameDataClass) {
        self.classes.push(class);
    }
    
    /// Get a class by name
    pub fn get_class(&self, name: &str) -> Option<&GameDataClass> {
        self.classes.iter().find(|c| c.name.eq_ignore_ascii_case(name))
    }
    
    /// Get all classes with a specific parent
    pub fn get_classes_by_parent(&self, parent: &str) -> Vec<&GameDataClass> {
        self.classes.iter()
            .filter(|c| c.parent.as_ref().map_or(false, |p| p.eq_ignore_ascii_case(parent)))
            .collect()
    }
    
    /// Get classes by property name and value (case-insensitive)
    pub fn get_classes_by_property(&self, property_name: &str, property_value: &str) -> Vec<&GameDataClass> {
        self.classes.iter()
            .filter(|c| {
                c.properties.iter().any(|(key, value)| {
                    key.eq_ignore_ascii_case(property_name) && 
                    match value {
                        PropertyValue::String(s) => s.eq_ignore_ascii_case(property_value),
                        PropertyValue::Number(n) => property_value == &n.to_string(),
                        _ => false,
                    }
                })
            })
            .collect()
    }
    
    /// Check if a class with the given name exists (case-insensitive)
    pub fn contains_class(&self, name: &str) -> bool {
        self.classes.iter().any(|c| c.name.eq_ignore_ascii_case(name))
    }
}

/// Game data class definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataClass {
    /// Class name
    pub name: String,
    
    /// Parent class (if any)
    pub parent: Option<String>,
    
    /// Properties
    pub properties: HashMap<String, PropertyValue>,
    
    /// Source file index in the file list
    pub source_file_index: Option<usize>,
}

impl GameDataClass {
    /// Create a new game data class
    pub fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name,
            parent,
            properties: HashMap::new(),
            source_file_index: None,
        }
    }
    
    /// Add a property
    pub fn add_property(&mut self, key: String, value: PropertyValue) {
        self.properties.insert(key, value);
    }
    
    /// Set the source file index
    pub fn set_source_file_index(&mut self, index: usize) {
        self.source_file_index = Some(index);
    }
    
    /// Get a property value by key (case-insensitive)
    pub fn get_property(&self, key: &str) -> Option<&PropertyValue> {
        self.properties.iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v)
    }
    
    /// Check if the class has a property (case-insensitive)
    pub fn has_property(&self, key: &str) -> bool {
        self.properties.keys()
            .any(|k| k.eq_ignore_ascii_case(key))
    }
}

/// Game data inheritance hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataHierarchy {
    /// Map of class name to child classes
    pub hierarchy: HashMap<String, Vec<String>>,
    
    /// Root classes (classes without parents)
    pub roots: Vec<String>,
}

impl GameDataHierarchy {
    /// Create a hierarchy from a collection of classes
    pub fn from_classes(classes: &GameDataClasses) -> Self {
        let mut hierarchy = HashMap::new();
        let mut roots = Vec::new();
        
        for class in &classes.classes {
            if let Some(parent) = &class.parent {
                hierarchy.entry(parent.clone())
                    .or_insert_with(Vec::new)
                    .push(class.name.clone());
            } else {
                roots.push(class.name.clone());
            }
        }
        
        Self {
            hierarchy,
            roots,
        }
    }
    
    /// Get the children of a class
    pub fn get_children(&self, class_name: &str) -> Option<&Vec<String>> {
        self.hierarchy.get(class_name)
    }
    
    /// Get all descendants of a class (recursive)
    pub fn get_descendants(&self, class_name: &str) -> Vec<String> {
        let mut result = Vec::new();
        
        if let Some(children) = self.get_children(class_name) {
            for child in children {
                result.push(child.clone());
                let descendants = self.get_descendants(child);
                result.extend(descendants);
            }
        }
        
        result
    }
} 