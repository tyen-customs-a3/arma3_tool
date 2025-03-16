use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::types::PropertyValue;

/// Collection of mission data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionData {
    /// Collection of missions
    pub missions: Vec<Mission>,
}

impl MissionData {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self {
            missions: Vec::new(),
        }
    }
    
    /// Add a mission to the collection
    pub fn add_mission(&mut self, mission: Mission) {
        self.missions.push(mission);
    }
    
    /// Get a mission by name
    pub fn get_mission(&self, name: &str) -> Option<&Mission> {
        self.missions.iter().find(|m| m.name == name)
    }
}

/// Mission definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    /// Mission name
    pub name: String,
    
    /// Path to the mission
    pub path: PathBuf,
    
    /// Mission components
    pub components: Vec<MissionComponent>,
    
    /// Dependencies
    pub dependencies: Vec<DependencyRef>,
    
    /// Mission properties
    pub properties: HashMap<String, PropertyValue>,
    
    /// Mission author
    pub author: Option<String>,
    
    /// Mission description
    pub description: Option<String>,
}

impl Mission {
    /// Create a new mission
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            components: Vec::new(),
            dependencies: Vec::new(),
            properties: HashMap::new(),
            author: None,
            description: None,
        }
    }
    
    /// Add a component to the mission
    pub fn add_component(&mut self, component: MissionComponent) {
        self.components.push(component);
    }
    
    /// Add a dependency to the mission
    pub fn add_dependency(&mut self, dependency: DependencyRef) {
        // Check if the dependency already exists
        if !self.dependencies.iter().any(|d| d.class_name == dependency.class_name) {
            self.dependencies.push(dependency);
        }
    }
    
    /// Add a property to the mission
    pub fn add_property(&mut self, key: String, value: PropertyValue) {
        self.properties.insert(key, value);
    }
    
    /// Set the author
    pub fn set_author(&mut self, author: String) {
        self.author = Some(author);
    }
    
    /// Set the description
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

/// Mission component type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MissionComponentType {
    Vehicle,
    Unit,
    Group,
    Marker,
    Trigger,
    Logic,
    Item,
    Other(String),
}

/// Mission component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionComponent {
    /// Component name
    pub name: String,
    
    /// Component type
    pub component_type: MissionComponentType,
    
    /// Component properties
    pub properties: HashMap<String, PropertyValue>,
    
    /// Component dependencies
    pub dependencies: Vec<DependencyRef>,
    
    /// Source file
    pub source_file: PathBuf,
}

impl MissionComponent {
    /// Create a new mission component
    pub fn new(name: String, component_type: MissionComponentType, source_file: PathBuf) -> Self {
        Self {
            name,
            component_type,
            properties: HashMap::new(),
            dependencies: Vec::new(),
            source_file,
        }
    }
    
    /// Add a property to the component
    pub fn add_property(&mut self, key: String, value: PropertyValue) {
        self.properties.insert(key, value);
    }
    
    /// Add a dependency to the component
    pub fn add_dependency(&mut self, dependency: DependencyRef) {
        // Check if the dependency already exists
        if !self.dependencies.iter().any(|d| d.class_name == dependency.class_name) {
            self.dependencies.push(dependency);
        }
    }
}

/// Reference type for dependencies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceType {
    DirectClass,
    InheritedClass,
    CreateVehicle,
    CreateUnit,
    SetObject,
    Other(String),
}

/// Dependency reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyRef {
    /// Class name
    pub class_name: String,
    
    /// Reference type
    pub reference_type: ReferenceType,
    
    /// Source file
    pub source_file: PathBuf,
    
    /// Line number (if available)
    pub line_number: Option<usize>,
}

impl DependencyRef {
    /// Create a new dependency reference
    pub fn new(class_name: String, reference_type: ReferenceType, source_file: PathBuf) -> Self {
        Self {
            class_name,
            reference_type,
            source_file,
            line_number: None,
        }
    }
    
    /// Set the line number
    pub fn set_line_number(&mut self, line_number: usize) {
        self.line_number = Some(line_number);
    }
} 