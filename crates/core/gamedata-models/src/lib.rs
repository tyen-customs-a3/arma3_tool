use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// Include conversion modules
pub mod conversions;

/// Interface for file parsers
pub trait FileParser: Send + Sync {
    /// Parse a single file and return all classes found
    fn parse_file(&self, file_path: &Path) -> Vec<GameClass>;

    /// Get the name of the parser
    fn name(&self) -> &str;
}

/// Represents a class in the game data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameClass {
    /// Class name
    pub name: String,

    /// Parent class name (if any)
    pub parent: Option<String>,

    /// Path to the file where this class was found
    pub file_path: PathBuf,

    /// Container class (if this is a nested class)
    pub container_class: Option<String>,

    /// Properties of the class
    pub properties: Vec<ClassProperty>,

    /// Whether this is a forward declaration
    pub is_forward_declaration: bool,
}

impl GameClass {
    /// Create a new GameClass with empty properties
    pub fn new(name: String, parent: Option<String>, file_path: PathBuf) -> Self {
        Self {
            name,
            parent,
            file_path,
            container_class: None,
            properties: Vec::new(),
            is_forward_declaration: false,
        }
    }

    /// Create a new GameClass with a container class
    pub fn with_container(
        name: String,
        parent: Option<String>,
        file_path: PathBuf,
        container_class: Option<String>,
    ) -> Self {
        Self {
            name,
            parent,
            file_path,
            container_class,
            properties: Vec::new(),
            is_forward_declaration: false,
        }
    }

    /// Add a property to the class
    pub fn add_property(&mut self, name: String, value: PropertyValue) {
        self.properties.push(ClassProperty { name, value });
    }

    /// Set the container class
    pub fn set_container(&mut self, container: String) {
        self.container_class = Some(container);
    }
}

/// Represents a property of a class
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassProperty {
    /// Property name
    pub name: String,

    /// Property value
    pub value: PropertyValue,
}

/// Represents a property value
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyValue {
    /// String value
    String(String),

    /// Array of strings
    Array(Vec<String>),

    /// Numeric value
    Number(i64),

    /// Nested class
    Object(Box<GameClass>),

    /// Reference to another class
    ClassRef(String),
}

impl PropertyValue {
    /// Get the value as a string, if it's a string
    pub fn as_string(&self) -> Option<&str> {
        match self {
            PropertyValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get the value as an array, if it's an array
    pub fn as_array(&self) -> Option<&[String]> {
        match self {
            PropertyValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Get the value as a number, if it's a number
    pub fn as_number(&self) -> Option<i64> {
        match self {
            PropertyValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get the value as a class reference, if it's a class reference
    pub fn as_class_ref(&self) -> Option<&str> {
        match self {
            PropertyValue::ClassRef(class_name) => Some(class_name),
            _ => None,
        }
    }
}

/// Result of scanning a directory or file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Total number of files scanned
    pub files_scanned: usize,

    /// Total number of classes found
    pub classes_found: usize,

    /// Number of files that had errors during parsing
    pub files_with_errors: usize,

    /// Map of class names to class definitions
    pub class_map: HashMap<String, Vec<GameClass>>,

    /// Time taken to complete the scan (in milliseconds)
    pub scan_time_ms: Option<u64>,
}

impl Default for ScanResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ScanResult {
    /// Create a new empty scan result
    pub fn new() -> Self {
        Self {
            files_scanned: 0,
            classes_found: 0,
            files_with_errors: 0,
            class_map: HashMap::new(),
            scan_time_ms: None,
        }
    }

    /// Add classes to the scan result
    pub fn add_classes(&mut self, classes: Vec<GameClass>) {
        for class in classes {
            self.add_class(class);
        }
    }

    /// Add a single class to the scan result
    pub fn add_class(&mut self, class: GameClass) {
        self.class_map
            .entry(class.name.clone())
            .or_default()
            .push(class);
        self.classes_found += 1;
    }

    /// Merge another scan result into this one
    pub fn merge(&mut self, other: ScanResult) {
        self.files_scanned += other.files_scanned;
        self.classes_found += other.classes_found;
        self.files_with_errors += other.files_with_errors;

        for (class_name, classes) in other.class_map {
            for class in classes {
                self.class_map
                    .entry(class_name.clone())
                    .or_default()
                    .push(class);
            }
        }
    }

    /// Find a class by its name
    pub fn find_class(&self, name: &str) -> Option<&Vec<GameClass>> {
        self.class_map.get(name)
    }
}

/// Common trait for all scanner implementations
pub trait Scanner: Send + Sync {
    /// Scan a directory for class definitions
    fn scan_directory(&self, dir_path: &Path) -> Result<ScanResult, String>;

    /// Parse a single file for class definitions
    fn parse_file(&self, file_path: &Path) -> Result<Vec<GameClass>, String>;
}

/// Utility module with conversion functions
pub mod convert {
    use super::*;

    /// Convert any class to a game class
    ///
    /// This function converts a class with minimal properties to the standard format
    pub fn to_game_class(
        name: String,
        parent: Option<String>,
        file_path: PathBuf,
        container_class: Option<String>,
        is_forward_declaration: bool,
    ) -> GameClass {
        GameClass {
            name,
            parent,
            file_path,
            container_class,
            properties: Vec::new(),
            is_forward_declaration,
        }
    }

    /// Convert a property array to class properties
    ///
    /// This function helps standardize property conversion from different formats
    pub fn properties_to_class_properties<T, F>(
        properties: &[T],
        name_fn: F,
        value_fn: impl Fn(&T) -> PropertyValue,
    ) -> Vec<ClassProperty>
    where
        F: Fn(&T) -> String,
    {
        properties
            .iter()
            .map(|prop| ClassProperty {
                name: name_fn(prop),
                value: value_fn(prop),
            })
            .collect()
    }
}
