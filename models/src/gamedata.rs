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
}

/// Game data class definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataClass {
    /// Class name
    pub name: String,
    
    /// Parent class (if any)
    pub parent: Option<String>,
    
    /// Container class (if this is a nested class)
    pub container_class: Option<String>,
    
    /// Properties
    pub properties: HashMap<String, PropertyValue>,
    
    /// Whether this class represents a forward declaration
    pub is_forward_declaration: bool,

    /// Source file index in the file list
    pub source_file_index: Option<usize>,
}

impl GameDataClass {
    /// Create a new game data class
    pub fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name,
            parent,
            container_class: None,
            properties: HashMap::new(),
            is_forward_declaration: false,
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
    
    /// Set the container class
    pub fn set_container_class(&mut self, container: String) {
        self.container_class = Some(container);
    }
} 