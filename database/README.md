# Arma 3 Database

SQLite database integration for Arma 3 Tool.

This crate provides a database-backed system for storing and querying Arma 3 game data, including:
- Class hierarchies
- PBO files and extraction information
- Mission scan data

## Features

- **Class Storage**: Store and query Arma 3 class hierarchies
- **PBO Management**: Track PBO files and extraction status
- **Mission Data**: Store mission information, components, and class dependencies
- **Graph Queries**: Generate class hierarchy and dependency graphs

## Database Schema

The database schema (version 2) includes tables for:

- `classes`: Store Arma 3 class information
- `pbo_files`: Track PBO file metadata
- `extracted_files`: Track extracted file paths
- `failed_extractions`: Record extraction failures
- `missions`: Store mission scan data
- `mission_components`: Store components from missions
- `mission_dependencies`: Store class dependencies from missions

## Usage

```rust
use arma3_database::{DatabaseManager, ClassRepository, PboRepository, MissionRepository};
use std::path::Path;

// Create or open a database
let db = DatabaseManager::new(Path::new("arma3_data.db")).unwrap();

// Access repositories
let class_repo = ClassRepository::new(&db);
let pbo_repo = PboRepository::new(&db);
let mission_repo = MissionRepository::new(&db);

// Work with mission data
let mission = MissionModel::new(
    "test_mission",
    "Test Mission",
    "missions/test.pbo",
    chrono::Utc::now(),
);

mission_repo.create(&mission).unwrap();

// Add mission components and dependencies
let component = MissionComponentModel::new(
    "test_mission",
    "TestComponent",
    "Vehicle",
    "mission/component.sqf",
);

let component_id = mission_repo.add_component(&component).unwrap();

// Add a dependency
let dependency = MissionDependencyModel::new(
    "test_mission",
    "Vehicle_F",
    "DirectClass",
    "mission/dependency.sqf",
);

mission_repo.add_dependency(&dependency).unwrap();

// Find missions that depend on a class
let missions = mission_repo.find_by_class_dependency("Vehicle_F").unwrap();
```

## Schema Versioning

The database tracks schema version in the `metadata` table. If a database with a different schema version is encountered, the application will return an error. Simply delete the database file to recreate it with the current schema.

## License

This crate is part of the Arma 3 Tool project. 

cargo run --features "trim_analysis" --bin trim_analysis -- --database "D:/pca/git/dep/rs/arma3_tool/cache/pca_next/arma3.db" --input D:/pca/git/dep/rs/arma3_tool/database/example_classes_to_trim.txt --output analysis.json --watch

cargo run --bin find_similar_classes -- -d "D:/pca/git/dep/rs/arma3_tool/cache/pca_next/arma3.db" -o output.csv