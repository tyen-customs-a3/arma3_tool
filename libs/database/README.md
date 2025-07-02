# Arma 3 Database

SQLite database layer for persistent storage and efficient querying of Arma 3 game data.

## Overview

This crate provides a robust database backend for the Arma 3 Tool ecosystem, managing everything from class hierarchies to mission dependencies. It uses SQLite with connection pooling for high-performance concurrent access.

## Features

- **Connection Pooling**: Efficient database access using r2d2
- **Repository Pattern**: Clean, type-safe API for data access
- **Schema Versioning**: Automatic migration detection
- **Transaction Support**: ACID compliance for data integrity
- **Query Optimization**: Indexed queries for performance
- **Graph Queries**: Build class hierarchy and dependency graphs

## Database Schema

The current schema (version 2) includes:

### Core Tables

- **`classes`**: Arma 3 class definitions
  - `id`, `name`, `parent_id`, `source_path`, `properties`
  - Indexed on name for fast lookups
  
- **`pbo_files`**: PBO file tracking
  - `id`, `file_path`, `hash`, `size`, `modified_time`
  - Tracks extraction status and metadata
  
- **`extracted_files`**: Extracted file locations
  - Links PBO files to their extracted contents
  
- **`missions`**: Mission metadata
  - `id`, `name`, `display_name`, `source_path`, `scan_date`
  
- **`mission_dependencies`**: Class usage in missions
  - Tracks which classes are used where

### Analysis Tables

- **`class_hierarchy`**: Materialized view of inheritance
- **`dependency_graph`**: Cross-references between classes
- **`cache_metadata`**: Query result caching

## Usage

### Basic Operations

```rust
use arma3_database::{DatabaseManager, ClassRepository};
use std::path::Path;

// Initialize database
let db = DatabaseManager::new(Path::new("arma3.db"))?;

// Store a class
let class_repo = ClassRepository::new(&db);
class_repo.create_class(
    "arifle_MX_F",
    Some("arifle_MX_Base_F"),
    "a3/weapons_f/config.cpp",
    r#"{"displayName": "MX 6.5 mm", "mass": 80}"#
)?;

// Query classes
let rifles = class_repo.find_by_parent("arifle_MX_Base_F")?;
let mx_rifle = class_repo.find_by_name("arifle_MX_F")?;
```

### Advanced Queries

```rust
use arma3_database::{GraphQueryEngine, ClassHierarchy};

// Build class hierarchy
let graph_engine = GraphQueryEngine::new(&db);
let hierarchy = graph_engine.build_hierarchy("Rifle_Base_F")?;

// Find all descendants
let all_rifles = hierarchy.get_all_descendants("Rifle_Base_F");

// Get inheritance path
let path = hierarchy.get_inheritance_path("arifle_MX_GL_F")?;
// Returns: ["arifle_MX_GL_F", "arifle_MX_Base_F", "Rifle_Base_F", ...]

// Find classes with specific properties
let scoped_rifles = class_repo.find_with_property("opticType", "sniper")?;
```

### Mission Analysis

```rust
use arma3_database::MissionRepository;

let mission_repo = MissionRepository::new(&db);

// Find missions using specific class
let missions = mission_repo.find_by_class_dependency("CUP_arifle_AK74")?;

// Get mission statistics
let stats = mission_repo.get_dependency_stats("my_mission")?;
println!("Total dependencies: {}", stats.total_dependencies);
println!("Missing classes: {}", stats.missing_count);
```

## Binary Tools

### Class Mapping Analyzer

Analyzes class usage patterns and generates mapping reports:

```bash
cargo run --bin class_mapping -- \
    --database cache/arma3.db \
    --output reports/class_mappings.json
```

### Similarity Finder

Finds similar class names for fuzzy matching:

```bash
cargo run --bin find_similar_classes -- \
    -d cache/arma3.db \
    -o similar_classes.csv \
    --threshold 0.8
```

### Trim Analysis

Analyzes which classes can be safely removed:

```bash
cargo run --features trim_analysis --bin trim_analysis -- \
    --database cache/arma3.db \
    --input classes_to_check.txt \
    --output trim_analysis.json \
    --watch
```

## Performance Considerations

- **Connection Pool**: Default size is CPU count * 2
- **Write Batching**: Use transactions for bulk inserts
- **Index Usage**: Name and parent_id columns are indexed
- **Query Caching**: Frequently accessed data is cached

Example bulk insert:
```rust
use arma3_database::ClassRepository;

let class_repo = ClassRepository::new(&db);
class_repo.bulk_insert_classes(vec![
    // ... hundreds of classes
])?; // Automatically uses transaction
```

## Error Handling

```rust
use arma3_database::DatabaseError;

match db.operation() {
    Ok(result) => { /* ... */ },
    Err(DatabaseError::SchemaVersion(v)) => {
        eprintln!("Database schema v{} incompatible", v);
    },
    Err(DatabaseError::Connection(e)) => {
        eprintln!("Connection failed: {}", e);
    },
    Err(DatabaseError::Query(e)) => {
        eprintln!("Query error: {}", e);
    },
}
```

## Integration

The database is used throughout arma3_tool:
- PBO Extractor stores extraction results
- Scanners populate class data
- Reporters query for analysis
- Mission scanner stores dependencies

## License

Part of the arma3_tool project.