//! Unified type system for Arma3 configurations
//! 
//! This module provides the unified `Value` and `Class` types that eliminate
//! the need for conversions between different parts of the system.

use arma3_errors::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use thiserror::Error;

/// Unified value type supporting all Arma3 configuration value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    /// String value - most common type in configs
    String(String),
    
    /// Integer value - exact integer representation
    Integer(i64),
    
    /// Floating point number - for precise decimals
    Number(f64),
    
    /// Boolean value - true/false
    Boolean(bool),
    
    /// Array of values - supports nested structures
    Array(Vec<Value>),
    
    /// Object/map structure - for complex nested data
    Object(HashMap<String, Value>),
    
    /// Reference to another class - for inheritance/composition
    ClassRef(String),
    
    /// Expression - for macro expressions and calculations
    Expression(String),
}

impl Value {
    /// Convenience constructors
    pub fn string<S: Into<String>>(s: S) -> Self {
        Value::String(s.into())
    }

    pub fn integer(i: i64) -> Self {
        Value::Integer(i)
    }

    pub fn number(f: f64) -> Self {
        Value::Number(f)
    }

    pub fn boolean(b: bool) -> Self {
        Value::Boolean(b)
    }

    pub fn array(values: Vec<Value>) -> Self {
        Value::Array(values)
    }

    pub fn object(map: HashMap<String, Value>) -> Self {
        Value::Object(map)
    }

    pub fn class_ref<S: Into<String>>(name: S) -> Self {
        Value::ClassRef(name.into())
    }

    pub fn expression<S: Into<String>>(expr: S) -> Self {
        Value::Expression(expr.into())
    }

    /// Type checking methods
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Value::Integer(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(self, Value::Object(_))
    }

    pub fn is_class_ref(&self) -> bool {
        matches!(self, Value::ClassRef(_))
    }

    pub fn is_expression(&self) -> bool {
        matches!(self, Value::Expression(_))
    }

    /// Safe accessor methods
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Value::Number(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&[Value]> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }

    pub fn as_class_ref(&self) -> Option<&str> {
        match self {
            Value::ClassRef(name) => Some(name),
            _ => None,
        }
    }

    pub fn as_expression(&self) -> Option<&str> {
        match self {
            Value::Expression(expr) => Some(expr),
            _ => None,
        }
    }

    /// Conversion methods with intelligent coercion
    pub fn try_into_string(self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s),
            Value::Integer(i) => Ok(i.to_string()),
            Value::Number(f) => Ok(f.to_string()),
            Value::Boolean(b) => Ok(b.to_string()),
            Value::ClassRef(name) => Ok(name),
            Value::Expression(expr) => Ok(expr),
            Value::Array(_) | Value::Object(_) => {
                Err(Error::validation("Cannot convert complex type to string"))
            }
        }
    }

    pub fn try_into_integer(self) -> Result<i64> {
        match self {
            Value::Integer(i) => Ok(i),
            Value::Number(f) => {
                if f.fract() == 0.0 && f >= i64::MIN as f64 && f <= i64::MAX as f64 {
                    Ok(f as i64)
                } else {
                    Err(Error::validation("Number cannot be represented as integer"))
                }
            }
            Value::String(s) => s.parse().map_err(|_| Error::validation("String is not a valid integer")),
            _ => Err(Error::validation("Cannot convert to integer")),
        }
    }

    pub fn try_into_number(self) -> Result<f64> {
        match self {
            Value::Number(f) => Ok(f),
            Value::Integer(i) => Ok(i as f64),
            Value::String(s) => s.parse().map_err(|_| Error::validation("String is not a valid number")),
            _ => Err(Error::validation("Cannot convert to number")),
        }
    }

    pub fn try_into_boolean(self) -> Result<bool> {
        match self {
            Value::Boolean(b) => Ok(b),
            Value::Integer(i) => Ok(i != 0),
            Value::Number(f) => Ok(f != 0.0),
            Value::String(s) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => Ok(true),
                "false" | "0" | "no" | "off" => Ok(false),
                _ => Err(Error::validation("String is not a valid boolean")),
            },
            _ => Err(Error::validation("Cannot convert to boolean")),
        }
    }
}

/// Error type for value conversion operations
#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Cannot convert {from} to {to}")]
    IncompatibleTypes { from: String, to: String },
    
    #[error("Value out of range for target type: {value}")]
    OutOfRange { value: String },
    
    #[error("Invalid format for {target}: {value}")]
    InvalidFormat { target: String, value: String },
}

/// Metadata for classes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ClassMetadata {
    /// Whether this class was auto-generated
    pub is_generated: bool,
    
    /// Documentation comments
    pub documentation: Option<String>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Last modification timestamp
    #[serde(with = "system_time_option")]
    pub last_modified: Option<SystemTime>,
}

/// Custom serialization for SystemTime
mod system_time_option {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match time {
            Some(t) => {
                let duration = t.duration_since(UNIX_EPOCH).map_err(serde::ser::Error::custom)?;
                duration.as_secs().serialize(serializer)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<u64> = Option::deserialize(deserializer)?;
        Ok(opt.map(|secs| UNIX_EPOCH + std::time::Duration::from_secs(secs)))
    }
}

/// Unified class type representing Arma3 configuration classes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Class {
    /// Class name (e.g., "CfgVehicles", "B_Soldier_F")
    pub name: String,
    
    /// Parent class name for inheritance
    pub parent: Option<String>,
    
    /// Direct properties of this class
    pub properties: HashMap<String, Value>,
    
    /// Nested classes within this class
    pub classes: HashMap<String, Class>,
    
    /// Array properties (optimized storage for arrays)
    pub arrays: HashMap<String, Vec<Value>>,
    
    /// External class declarations (forward declarations)
    pub external_classes: Vec<String>,
    
    /// Deleted properties (delete keyword in configs)
    pub delete_properties: Vec<String>,
    
    /// Deleted classes (delete keyword for classes)
    pub delete_classes: Vec<String>,
    
    /// Source file path where this class was defined
    pub file_path: Option<PathBuf>,
    
    /// Container class if this is a nested class
    pub container_class: Option<String>,
    
    /// Whether this is a forward declaration only
    pub is_forward_declaration: bool,
    
    /// Line number in source file (for debugging/tools)
    pub line_number: Option<usize>,
    
    /// Additional metadata for tooling
    pub metadata: ClassMetadata,
}

impl Class {
    /// Create a new class with basic information
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            parent: None,
            properties: HashMap::new(),
            classes: HashMap::new(),
            arrays: HashMap::new(),
            external_classes: Vec::new(),
            delete_properties: Vec::new(),
            delete_classes: Vec::new(),
            file_path: None,
            container_class: None,
            is_forward_declaration: false,
            line_number: None,
            metadata: ClassMetadata::default(),
        }
    }

    /// Set parent class
    pub fn with_parent<S: Into<String>>(mut self, parent: S) -> Self {
        self.parent = Some(parent.into());
        self
    }

    /// Add a property
    pub fn with_property<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Add a nested class
    pub fn with_class<K: Into<String>>(mut self, key: K, class: Class) -> Self {
        self.classes.insert(key.into(), class);
        self
    }

    /// Add an array property
    pub fn with_array<K: Into<String>>(mut self, key: K, values: Vec<Value>) -> Self {
        self.arrays.insert(key.into(), values);
        self
    }

    /// Set file path
    pub fn with_file_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.file_path = Some(path.into());
        self
    }

    /// Set container class
    pub fn with_container<S: Into<String>>(mut self, container: S) -> Self {
        self.container_class = Some(container.into());
        self
    }

    /// Mark as forward declaration
    pub fn as_forward_declaration(mut self) -> Self {
        self.is_forward_declaration = true;
        self
    }

    /// Set line number
    pub fn with_line_number(mut self, line: usize) -> Self {
        self.line_number = Some(line);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: ClassMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// Add an external class reference
    pub fn with_external_class<S: Into<String>>(mut self, class_name: S) -> Self {
        self.external_classes.push(class_name.into());
        self
    }

    /// Mark a property as deleted
    pub fn with_deleted_property<S: Into<String>>(mut self, property: S) -> Self {
        self.delete_properties.push(property.into());
        self
    }

    /// Mark a class as deleted
    pub fn with_deleted_class<S: Into<String>>(mut self, class: S) -> Self {
        self.delete_classes.push(class.into());
        self
    }

    /// Get a property by name
    pub fn get_property(&self, name: &str) -> Option<&Value> {
        self.properties.get(name)
    }

    /// Get a nested class by name
    pub fn get_class(&self, name: &str) -> Option<&Class> {
        self.classes.get(name)
    }

    /// Get an array by name
    pub fn get_array(&self, name: &str) -> Option<&[Value]> {
        self.arrays.get(name).map(|v| v.as_slice())
    }

    /// Check if a property is deleted
    pub fn is_property_deleted(&self, name: &str) -> bool {
        self.delete_properties.contains(&name.to_string())
    }

    /// Check if a class is deleted
    pub fn is_class_deleted(&self, name: &str) -> bool {
        self.delete_classes.contains(&name.to_string())
    }

    /// Get all property names (excluding arrays and nested classes)
    pub fn property_names(&self) -> impl Iterator<Item = &String> {
        self.properties.keys()
    }

    /// Get all nested class names
    pub fn class_names(&self) -> impl Iterator<Item = &String> {
        self.classes.keys()
    }

    /// Get all array names
    pub fn array_names(&self) -> impl Iterator<Item = &String> {
        self.arrays.keys()
    }

    /// Add a property to an existing class
    pub fn add_property<K, V>(&mut self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<Value>,
    {
        self.properties.insert(key.into(), value.into());
    }

    /// Add a nested class to an existing class
    pub fn add_class<K: Into<String>>(&mut self, key: K, class: Class) {
        self.classes.insert(key.into(), class);
    }

    /// Add an array to an existing class
    pub fn add_array<K: Into<String>>(&mut self, key: K, values: Vec<Value>) {
        self.arrays.insert(key.into(), values);
    }

    /// Remove a property
    pub fn remove_property(&mut self, name: &str) -> Option<Value> {
        self.properties.remove(name)
    }

    /// Remove a nested class
    pub fn remove_class(&mut self, name: &str) -> Option<Class> {
        self.classes.remove(name)
    }

    /// Remove an array
    pub fn remove_array(&mut self, name: &str) -> Option<Vec<Value>> {
        self.arrays.remove(name)
    }

    /// Check if this class inherits from another class
    pub fn inherits_from(&self, class_name: &str) -> bool {
        if let Some(parent) = &self.parent {
            parent == class_name
        } else {
            false
        }
    }

    /// Get the full inheritance chain (requires class resolution context)
    pub fn inheritance_chain(&self) -> Vec<&str> {
        let mut chain = vec![self.name.as_str()];
        if let Some(parent) = &self.parent {
            chain.push(parent.as_str());
        }
        chain
    }
}

/// Convert from integer types
impl From<i32> for Value {
    fn from(i: i32) -> Self {
        Value::Integer(i as i64)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Integer(i)
    }
}

impl From<f32> for Value {
    fn from(f: f32) -> Self {
        Value::Number(f as f64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Number(f)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<Vec<Value>> for Value {
    fn from(arr: Vec<Value>) -> Self {
        Value::Array(arr)
    }
}

impl From<HashMap<String, Value>> for Value {
    fn from(obj: HashMap<String, Value>) -> Self {
        Value::Object(obj)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_constructors() {
        assert_eq!(Value::string("test"), Value::String("test".to_string()));
        assert_eq!(Value::integer(42), Value::Integer(42));
        assert_eq!(Value::number(3.14), Value::Number(3.14));
        assert_eq!(Value::boolean(true), Value::Boolean(true));
    }

    #[test]
    fn test_value_type_checking() {
        let val = Value::Integer(42);
        assert!(val.is_integer());
        assert!(!val.is_string());
        assert_eq!(val.as_integer(), Some(42));
        assert_eq!(val.as_string(), None);
    }

    #[test]
    fn test_value_conversions() {
        let val = Value::Integer(42);
        assert_eq!(val.clone().try_into_string().unwrap(), "42");
        assert_eq!(val.clone().try_into_number().unwrap(), 42.0);
        assert_eq!(val.try_into_boolean().unwrap(), true);

        let zero = Value::Integer(0);
        assert_eq!(zero.try_into_boolean().unwrap(), false);
    }

    #[test]
    fn test_class_builder() {
        let class = Class::new("TestClass")
            .with_parent("BaseClass")
            .with_property("displayName", Value::string("Test"))
            .with_property("armor", Value::integer(100))
            .with_array("weapons", vec![Value::string("rifle"), Value::string("pistol")])
            .with_file_path("/path/to/config.hpp")
            .with_line_number(42);

        assert_eq!(class.name, "TestClass");
        assert_eq!(class.parent, Some("BaseClass".to_string()));
        assert_eq!(class.properties.len(), 2);
        assert_eq!(class.arrays.len(), 1);
        assert_eq!(class.line_number, Some(42));
        
        assert_eq!(class.get_property("displayName").unwrap().as_string(), Some("Test"));
        assert_eq!(class.get_property("armor").unwrap().as_integer(), Some(100));
        assert_eq!(class.get_array("weapons").unwrap().len(), 2);
    }

    #[test]
    fn test_class_property_management() {
        let mut class = Class::new("TestClass");
        
        class.add_property("test", Value::string("value"));
        assert_eq!(class.get_property("test").unwrap().as_string(), Some("value"));
        
        let removed = class.remove_property("test");
        assert!(removed.is_some());
        assert!(class.get_property("test").is_none());
    }

    #[test]
    fn test_value_from_conversions() {
        assert_eq!(Value::from(42i32), Value::Integer(42));
        assert_eq!(Value::from(3.14f64), Value::Number(3.14));
        assert_eq!(Value::from(true), Value::Boolean(true));
        assert_eq!(Value::from("test"), Value::String("test".to_string()));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let class = Class::new("TestClass")
            .with_property("name", Value::string("test"))
            .with_property("value", Value::integer(42))
            .with_array("items", vec![Value::string("a"), Value::string("b")]);

        let serialized = serde_json::to_string(&class).unwrap();
        let deserialized: Class = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(class, deserialized);
    }

    #[test]
    fn test_inheritance_checking() {
        let class = Class::new("Child").with_parent("Parent");
        assert!(class.inherits_from("Parent"));
        assert!(!class.inherits_from("Other"));
        
        let chain = class.inheritance_chain();
        assert_eq!(chain, vec!["Child", "Parent"]);
    }

    #[test]
    fn test_deletion_tracking() {
        let class = Class::new("TestClass")
            .with_deleted_property("oldProp")
            .with_deleted_class("OldClass");
        
        assert!(class.is_property_deleted("oldProp"));
        assert!(class.is_class_deleted("OldClass"));
        assert!(!class.is_property_deleted("newProp"));
    }

    #[test]
    fn test_complex_nested_structure() {
        let nested_class = Class::new("NestedClass")
            .with_property("nestedProp", Value::integer(123));
        
        let mut obj = HashMap::new();
        obj.insert("key1".to_string(), Value::string("value1"));
        obj.insert("key2".to_string(), Value::integer(456));
        
        let class = Class::new("ComplexClass")
            .with_property("simple", Value::string("text"))
            .with_property("object", Value::Object(obj))
            .with_class("nested", nested_class)
            .with_array("mixed", vec![
                Value::string("text"),
                Value::integer(789),
                Value::boolean(true),
            ]);
        
        assert_eq!(class.properties.len(), 2);
        assert_eq!(class.classes.len(), 1);
        assert_eq!(class.arrays.len(), 1);
        
        let nested = class.get_class("nested").unwrap();
        assert_eq!(nested.get_property("nestedProp").unwrap().as_integer(), Some(123));
    }
}