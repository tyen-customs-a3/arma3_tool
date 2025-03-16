use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::types::PropertyValue;

/// Collection of game data classes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataClasses {
    /// Collection of classes
    pub classes: Vec<GameDataClass>,
}

impl GameDataClasses {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }
    
    /// Add a class to the collection
    pub fn add_class(&mut self, class: GameDataClass) {
        self.classes.push(class);
    }
    
    /// Get a class by name
    pub fn get_class(&self, name: &str) -> Option<&GameDataClass> {
        self.classes.iter().find(|c| c.name == name)
    }
    
    /// Get all classes with a specific parent
    pub fn get_classes_by_parent(&self, parent: &str) -> Vec<&GameDataClass> {
        self.classes.iter()
            .filter(|c| c.parent.as_ref().map_or(false, |p| p == parent))
            .collect()
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
    
    /// Source file path
    pub source_file: Option<PathBuf>,
    
    /// PBO name
    pub pbo_name: Option<String>,
    
    /// Mod name
    pub mod_name: Option<String>,
}

impl GameDataClass {
    /// Create a new game data class
    pub fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name,
            parent,
            properties: HashMap::new(),
            source_file: None,
            pbo_name: None,
            mod_name: None,
        }
    }
    
    /// Add a property
    pub fn add_property(&mut self, key: String, value: PropertyValue) {
        self.properties.insert(key, value);
    }
    
    /// Set the source file
    pub fn set_source_file(&mut self, path: PathBuf) {
        self.source_file = Some(path);
    }
    
    /// Set the PBO name
    pub fn set_pbo_name(&mut self, name: String) {
        self.pbo_name = Some(name);
    }
    
    /// Set the mod name
    pub fn set_mod_name(&mut self, name: String) {
        self.mod_name = Some(name);
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