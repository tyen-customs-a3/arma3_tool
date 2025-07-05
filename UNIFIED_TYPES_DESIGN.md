# Unified Type System Design

## Overview

This document describes the design for a unified type system that eliminates conversions between the current `arma3-types` and `arma3-gamedata-models` crates, implementing a zero-conversion architecture.

## Core Design Principles

1. **Zero-Conversion**: All components use the same types directly, no conversion needed
2. **Complete Coverage**: Support all use cases from both existing type systems
3. **Performance**: O(1) lookups where possible, minimal memory overhead
4. **Extensibility**: Support future Arma3 config features
5. **Backward Compatibility**: Migration path for existing code

## Unified Value Type

```rust
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
```

### Value Type Rationale

- **String**: Primary config value type, UTF-8 support
- **Integer**: Preserves exact integer values without floating point errors
- **Number**: f64 for scientific notation and precise decimals
- **Boolean**: Native boolean support for true/false values
- **Array**: Recursive structure supports arbitrary nesting (arrays of arrays)
- **Object**: HashMap for fast property lookup and nested structures
- **ClassRef**: References to other classes for inheritance/composition
- **Expression**: Placeholder for macro expressions and calculated values

## Unified Class Type

```rust
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
    pub last_modified: Option<SystemTime>,
}
```

### Class Type Rationale

- **HashMap for properties**: O(1) lookup performance for property access
- **HashMap for classes**: O(1) lookup for nested class access
- **Separate arrays field**: Optimized storage for array properties, faster iteration
- **Delete tracking**: Native support for Arma3's delete keyword
- **External classes**: Forward declaration tracking for dependency analysis
- **Metadata**: Extensible metadata system for tooling and documentation

## Builder Pattern API

```rust
impl Class {
    /// Create a new class with basic information
    pub fn new<S: Into<String>>(name: S) -> Self;
    
    /// Set parent class
    pub fn with_parent<S: Into<String>>(mut self, parent: S) -> Self;
    
    /// Add a property
    pub fn with_property<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Value>;
    
    /// Add a nested class
    pub fn with_class<K: Into<String>>(mut self, key: K, class: Class) -> Self;
    
    /// Add an array property
    pub fn with_array<K: Into<String>>(mut self, key: K, values: Vec<Value>) -> Self;
    
    /// Set file path
    pub fn with_file_path<P: Into<PathBuf>>(mut self, path: P) -> Self;
    
    /// Set container class
    pub fn with_container<S: Into<String>>(mut self, container: S) -> Self;
    
    /// Mark as forward declaration
    pub fn as_forward_declaration(mut self) -> Self;
}

impl Value {
    /// Convenience constructors
    pub fn string<S: Into<String>>(s: S) -> Self;
    pub fn integer(i: i64) -> Self;
    pub fn number(f: f64) -> Self;
    pub fn boolean(b: bool) -> Self;
    pub fn array(values: Vec<Value>) -> Self;
    pub fn object(map: HashMap<String, Value>) -> Self;
    pub fn class_ref<S: Into<String>>(name: S) -> Self;
    pub fn expression<S: Into<String>>(expr: S) -> Self;
    
    /// Type checking methods
    pub fn is_string(&self) -> bool;
    pub fn is_integer(&self) -> bool;
    pub fn is_number(&self) -> bool;
    pub fn is_boolean(&self) -> bool;
    pub fn is_array(&self) -> bool;
    pub fn is_object(&self) -> bool;
    pub fn is_class_ref(&self) -> bool;
    pub fn is_expression(&self) -> bool;
    
    /// Safe accessor methods
    pub fn as_string(&self) -> Option<&str>;
    pub fn as_integer(&self) -> Option<i64>;
    pub fn as_number(&self) -> Option<f64>;
    pub fn as_boolean(&self) -> Option<bool>;
    pub fn as_array(&self) -> Option<&[Value]>;
    pub fn as_object(&self) -> Option<&HashMap<String, Value>>;
    pub fn as_class_ref(&self) -> Option<&str>;
    pub fn as_expression(&self) -> Option<&str>;
    
    /// Conversion methods (fallible)
    pub fn try_into_string(self) -> Result<String, ConversionError>;
    pub fn try_into_integer(self) -> Result<i64, ConversionError>;
    pub fn try_into_number(self) -> Result<f64, ConversionError>;
    pub fn try_into_boolean(self) -> Result<bool, ConversionError>;
}
```

## Migration Strategy

### Phase 1: Type Aliases for Compatibility

```rust
// Backward compatibility aliases
pub type PropertyValue = Value;
pub type GameClass = Class;
pub type GameDataClass = Class;
pub type ClassProperty = (String, Value);

// Conversion traits for migration period
impl From<legacy::PropertyValue> for Value;
impl From<legacy::GameClass> for Class;
impl From<Value> for legacy::PropertyValue;
impl From<Class> for legacy::GameClass;
```

### Phase 2: Migration Utilities

```rust
pub mod migration {
    /// Convert old gamedata-models types to unified types
    pub fn convert_gamedata_class(old: &legacy::GameClass) -> Class;
    pub fn convert_gamedata_property_value(old: &legacy::PropertyValue) -> Value;
    
    /// Convert old arma3-types to unified types
    pub fn convert_models_class(old: &legacy::GameDataClass) -> Class;
    pub fn convert_models_property_value(old: &legacy::PropertyValue) -> Value;
    
    /// Batch conversion utilities
    pub fn convert_scan_result(old: &legacy::ScanResult) -> ScanResult;
    pub fn convert_class_collection(old: &[legacy::GameClass]) -> Vec<Class>;
}
```

## Database Schema Migration

### New Unified Schema

```sql
-- Unified class storage
CREATE TABLE classes (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    parent TEXT,
    file_path TEXT,
    container_class TEXT,
    is_forward_declaration BOOLEAN DEFAULT FALSE,
    line_number INTEGER,
    metadata TEXT, -- JSON metadata
    properties TEXT, -- JSON serialized properties
    classes TEXT, -- JSON serialized nested classes
    arrays TEXT, -- JSON serialized arrays
    external_classes TEXT, -- JSON array of external class names
    delete_properties TEXT, -- JSON array of deleted property names
    delete_classes TEXT, -- JSON array of deleted class names
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_classes_name ON classes(name);
CREATE INDEX idx_classes_parent ON classes(parent);
CREATE INDEX idx_classes_container ON classes(container_class);
CREATE INDEX idx_classes_file_path ON classes(file_path);
```

### Migration Scripts

```rust
pub mod database_migration {
    /// Migrate from old separate property tables to unified schema
    pub async fn migrate_to_unified_schema(db: &Database) -> Result<(), MigrationError>;
    
    /// Rollback migration if needed
    pub async fn rollback_unified_schema(db: &Database) -> Result<(), MigrationError>;
    
    /// Verify migration integrity
    pub async fn verify_migration(db: &Database) -> Result<bool, MigrationError>;
}
```

## Parser Integration

### HEMTT Value Conversion

```rust
impl From<hemtt_config::Value> for Value {
    fn from(hemtt_value: hemtt_config::Value) -> Self {
        match hemtt_value {
            hemtt_config::Value::Str(s) => Value::String(s.value().to_string()),
            hemtt_config::Value::Number(n) => match n {
                HemttNumber::Int32 { value, .. } => Value::Integer(value as i64),
                HemttNumber::Int64 { value, .. } => Value::Integer(value),
                HemttNumber::Float32 { value, .. } => Value::Number(value as f64),
            },
            hemtt_config::Value::Array(arr) => {
                Value::Array(arr.items.into_iter().map(Item::into).collect())
            },
            hemtt_config::Value::Expression(e) => Value::Expression(format!("{:?}", e)),
            hemtt_config::Value::Macro(m) => Value::Expression(m.to_string()),
            // ... other variants
        }
    }
}
```

## Error Handling

```rust
#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Cannot convert {from} to {to}")]
    IncompatibleTypes { from: String, to: String },
    
    #[error("Value out of range for target type: {value}")]
    OutOfRange { value: String },
    
    #[error("Invalid format for {target}: {value}")]
    InvalidFormat { target: String, value: String },
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Missing required property: {property}")]
    MissingProperty { property: String },
    
    #[error("Invalid class reference: {class_name}")]
    InvalidClassRef { class_name: String },
    
    #[error("Circular inheritance detected: {path}")]
    CircularInheritance { path: String },
}
```

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_value_type_safety() {
        let val = Value::Integer(42);
        assert!(val.is_integer());
        assert_eq!(val.as_integer(), Some(42));
        assert!(val.as_string().is_none());
    }
    
    #[test]
    fn test_class_builder() {
        let class = Class::new("TestClass")
            .with_parent("BaseClass")
            .with_property("displayName", Value::string("Test"))
            .with_property("armor", Value::integer(100));
        
        assert_eq!(class.name, "TestClass");
        assert_eq!(class.parent, Some("BaseClass".to_string()));
        assert_eq!(class.properties.len(), 2);
    }
    
    #[test]
    fn test_migration_compatibility() {
        let old_class = create_legacy_gamedata_class();
        let new_class = migration::convert_gamedata_class(&old_class);
        verify_class_equivalence(&old_class, &new_class);
    }
}
```

### Property-Based Tests

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_value_serialization_roundtrip(value: Value) {
        let serialized = serde_json::to_string(&value).unwrap();
        let deserialized: Value = serde_json::from_str(&serialized).unwrap();
        prop_assert_eq!(value, deserialized);
    }
    
    #[test]
    fn test_class_property_access(
        name in any::<String>(),
        properties in prop::collection::hash_map(any::<String>(), any::<Value>(), 0..10)
    ) {
        let mut class = Class::new(name);
        for (key, value) in &properties {
            class = class.with_property(key.clone(), value.clone());
        }
        
        for (key, expected_value) in &properties {
            prop_assert_eq!(class.properties.get(key), Some(expected_value));
        }
    }
}
```

## Performance Considerations

### Memory Layout Optimization

```rust
// Use Box for large nested structures to reduce stack size
pub type NestedClasses = HashMap<String, Box<Class>>;

// Use Cow for strings that might be references
pub type StringRef<'a> = Cow<'a, str>;

// Consider using string interning for repeated class/property names
pub type InternedString = string_interner::StringIndex;
```

### Lazy Loading for Large Configs

```rust
#[derive(Debug, Clone)]
pub enum LazyClass {
    Loaded(Class),
    Deferred { 
        name: String, 
        file_path: PathBuf, 
        offset: u64 
    },
}

impl LazyClass {
    pub fn load(&mut self) -> Result<&Class, LoadError>;
    pub fn is_loaded(&self) -> bool;
}
```

## Implementation Phases

### Phase 1: Core Types (This Task)
- Implement Value and Class types
- Add builder patterns and accessors
- Comprehensive unit tests

### Phase 2: Migration Support
- Conversion traits and utilities
- Backward compatibility aliases
- Migration tools

### Phase 3: Parser Integration
- Update HPP parser to generate unified types
- Update other parsers
- Remove conversion code

### Phase 4: Database Migration
- New unified schema
- Migration scripts
- Performance testing

### Phase 5: Legacy Cleanup
- Remove old type definitions
- Remove conversion code
- Update documentation

This design provides a robust foundation for the unified type system that eliminates conversions while maintaining compatibility and performance.