# Type System Analysis for Unification

## Current Type System State

### PropertyValue Types (Two Variants)

#### 1. In `arma3-types` (crates/core/types/src/types.rs):
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Number(f64),        // Uses f64
    Array(Vec<PropertyValue>),  // Recursive PropertyValue
    Boolean(bool),
    Object(HashMap<String, PropertyValue>),
}
```

#### 2. In `arma3-gamedata-models` (crates/core/gamedata-models/src/lib.rs):
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyValue {
    String(String),
    Array(Vec<String>),     // Only string arrays
    Number(i64),           // Uses i64
    Class(Box<GameClass>), // Nested classes
}
```

### GameClass Types (Two Variants)

#### 1. In `arma3-types` - GameDataClass:
```rust
pub struct GameDataClass {
    pub name: String,
    pub parent: Option<String>,
    pub container_class: Option<String>,
    pub properties: HashMap<String, PropertyValue>, // HashMap storage
    pub is_forward_declaration: bool,
    pub source_file_index: Option<usize>,
}
```

#### 2. In `arma3-gamedata-models` - GameClass:
```rust
pub struct GameClass {
    pub name: String,
    pub parent: Option<String>,
    pub file_path: PathBuf,  // Direct path storage
    pub container_class: Option<String>,
    pub properties: Vec<ClassProperty>, // Vec of ClassProperty
    pub is_forward_declaration: bool,
}

pub struct ClassProperty {
    pub name: String,
    pub value: PropertyValue,
}
```

## Usage Patterns Analysis

### Critical Conversion Points

1. **Parser Layer**: HPP parser converts HEMTT values to gamedata-models PropertyValue
   - File: `crates/parsers/hpp/src/ast_transformer.rs`
   - Function: `convert_hemtt_value_to_property_value`
   - Converts HEMTT AST → gamedata-models PropertyValue
   - Numbers: Float32/Int32/Int64 → i64 (loses precision for floats)
   - Arrays: Recursive Items → Vec<String> (loses type information)

2. **Database Layer**: Converts between both type systems
   - Files: `crates/infra/database/src/repos/class_repository.rs`
   - Requires type conversions for storage/retrieval

3. **Export/Serialization**: Multiple export formats
   - Files: `src/cli/export_optimized.rs`, `benches/export_*.rs`
   - Needs consistent PropertyValue handling

### Type Flow Analysis

```
HEMTT AST → gamedata-models PropertyValue → Database Storage
    ↓                                           ↓
  Parser                                    Repository
    ↓                                           ↓
GameClass(Vec<ClassProperty>)              GameDataClass(HashMap)
```

### Incompatibility Issues

1. **Number Types**: f64 vs i64 causes precision loss and conversion errors
2. **Array Types**: Vec<PropertyValue> vs Vec<String> - recursive vs flat
3. **Property Storage**: HashMap vs Vec - different access patterns
4. **Class References**: PropertyValue::Class vs no equivalent in arma3-types
5. **File Handling**: PathBuf vs file index - different storage strategies

## Zero-Conversion Design Requirements

### Unified PropertyValue Design
```rust
pub enum Value {
    String(String),
    Integer(i64),           // For exact integers
    Number(f64),           // For floating point
    Boolean(bool),
    Array(Vec<Value>),     // Recursive for nested structures
    Object(HashMap<String, Value>), // For object-like structures
}
```

### Unified Class Design
```rust
pub struct Class {
    pub name: String,
    pub parent: Option<String>,
    pub properties: HashMap<String, Value>, // Fast lookup
    pub classes: HashMap<String, Class>,    // Nested classes
    pub arrays: HashMap<String, Vec<Value>>, // Array properties
    pub external_classes: Vec<String>,      // Forward declarations
    pub delete_properties: Vec<String>,     // Deleted properties
    pub delete_classes: Vec<String>,        // Deleted classes
    pub file_path: Option<PathBuf>,         // Source file
    pub container_class: Option<String>,    // Parent container
    pub is_forward_declaration: bool,
}
```

### Migration Strategy

1. **Phase 1**: Implement unified types in arma3-types
2. **Phase 2**: Update parsers to generate unified types directly
3. **Phase 3**: Update database layer to use unified types
4. **Phase 4**: Remove legacy types and conversion code

### Compatibility Matrix

| Component | Current Type Usage | Migration Required |
|-----------|-------------------|-------------------|
| HPP Parser | gamedata-models::PropertyValue | High - Direct usage |
| SQF Parser | ? | Medium - Check usage |
| SQM Parser | ? | Medium - Check usage |
| Database | Both types | High - Storage format |
| Reporter | gamedata-models::GameClass | Medium - Class processing |
| Scanner Tools | gamedata-models | Medium - Batch processing |
| Weapon Scanner | gamedata-models | Medium - Data extraction |
| Mission Scanner | gamedata-models | Medium - Analysis |

### Performance Considerations

- HashMap vs Vec for properties: HashMap provides O(1) lookup vs O(n) scan
- Recursive Value enum: Enables arbitrary nesting without type loss
- File path handling: Optional PathBuf vs required - more flexible
- Memory usage: Unified types may use more memory but eliminate conversions

## Recommendations

1. Implement unified types first with comprehensive tests
2. Create conversion utilities for migration period
3. Update parsers to target unified types directly
4. Migrate database schema to unified format
5. Add performance benchmarks to ensure no regressions