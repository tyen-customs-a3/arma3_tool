# Arma 3 Models

Core data models and types used throughout the Arma 3 Tool ecosystem.

## Overview

This crate provides the fundamental data structures that represent Arma 3 game elements, including classes, missions, and various game data types. These models are used by parsers, scanners, and the database layer.

## Features

- Serializable data structures (using serde)
- Type-safe representations of Arma 3 concepts
- Shared across all arma3_tool components

## Main Types

### Game Data Models

```rust
use arma3_models::{ClassModel, PropertyModel};

// Represents an Arma 3 class definition
let class = ClassModel {
    name: "arifle_MX_F".to_string(),
    parent: Some("arifle_MX_Base_F".to_string()),
    source_path: "a3/weapons_f/rifles/mx/config.cpp".to_string(),
    properties: vec![
        PropertyModel {
            name: "displayName".to_string(),
            value: "MX 6.5 mm".to_string(),
        },
    ],
};
```

### Mission Models

```rust
use arma3_models::{MissionModel, MissionComponentModel, MissionDependencyModel};

// Represents a mission and its components
let mission = MissionModel::new(
    "co22_operation",
    "Operation Thunder",
    "missions/co22_operation.pbo",
    chrono::Utc::now(),
);

// Mission components (units, vehicles, etc.)
let component = MissionComponentModel::new(
    "co22_operation",
    "B_Soldier_F",
    "Unit",
    "mission.sqm",
);

// Dependencies found in missions
let dependency = MissionDependencyModel::new(
    "co22_operation",
    "CUP_arifle_AK74",
    "Weapon",
    "loadouts/opfor.sqf",
);
```

### Type Definitions

The crate also provides various type aliases and enums for common Arma 3 concepts:

```rust
use arma3_models::{ItemType, DependencyType};

// Item types for categorization
let item_type = ItemType::Weapon;

// Dependency types for analysis
let dep_type = DependencyType::DirectClass;
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
arma3_models = { path = "../models" }
```

Then use in your code:

```rust
use arma3_models::*;

// Models are designed to be created, serialized, and passed between components
let class = ClassModel::new("MyClass", Some("ParentClass"), "path/to/config.cpp");

// Serialize to JSON
let json = serde_json::to_string(&class)?;

// Or use with other arma3_tool components
database.store_class(&class)?;
```

## Integration

These models are used throughout the arma3_tool ecosystem:

- **Parsers** create model instances from game files
- **Database** stores and retrieves models
- **Scanners** populate models with discovered data
- **Reporters** use models to generate output

## License

Part of the arma3_tool project. 