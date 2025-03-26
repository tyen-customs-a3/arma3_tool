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
    
    /// Properties
    pub properties: HashMap<String, PropertyValue>,
    
    /// Source file index in the file list
    pub source_file_index: Option<usize>,
    
    /// PBO identifier (source PBO index)
    pub pbo_id: Option<usize>,
    
    /// Line number in source file
    pub line_number: Option<usize>,
}

impl GameDataClass {
    /// Create a new game data class
    pub fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name,
            parent,
            properties: HashMap::new(),
            source_file_index: None,
            pbo_id: None,
            line_number: None,
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
    
    /// Set the PBO identifier
    pub fn set_pbo_id(&mut self, pbo_id: usize) {
        self.pbo_id = Some(pbo_id);
    }
    
    /// Set the line number in source file
    pub fn set_line_number(&mut self, line: usize) {
        self.line_number = Some(line);
    }
} 