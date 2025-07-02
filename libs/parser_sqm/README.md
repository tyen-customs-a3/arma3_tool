# SQM Parser

Parser for Arma 3 mission files (mission.sqm) with support for extracting unit inventories and class dependencies.

## Overview

This crate provides parsing and analysis capabilities for Arma 3's SQM (mission) file format. It can extract unit loadouts, vehicle configurations, and identify all class dependencies used in a mission.

## Features

- Complete SQM format parsing
- Unit inventory extraction
- Vehicle and object class detection
- Dependency analysis with wildcard path matching
- Efficient query system for large missions
- Support for both binarized and unbinarized formats

## Usage

```rust
use parser_sqm::{parse_sqm_file, extract_class_dependencies};
use std::path::Path;

// Parse a mission file
let mission_path = Path::new("missions/my_mission.sqm");
let sqm_file = parse_sqm_file(mission_path)?;

// Extract all class dependencies
let dependencies = extract_class_dependencies(&sqm_file)?;

println!("Found {} class dependencies:", dependencies.len());
for class_name in &dependencies {
    println!("  - {}", class_name);
}

// Access mission metadata
println!("Mission: {}", sqm_file.mission_name);
println!("Author: {}", sqm_file.author);
```

## SQM Structure

The parser handles the hierarchical structure of SQM files:

```sqm
class Mission {
    class Entities {
        class Item0 {
            dataType = "Group";
            class Entities {
                class Item0 {
                    dataType = "Object";
                    class Attributes {
                        class Inventory {
                            uniform = "U_B_CombatUniform_mcam";
                            vest = "V_PlateCarrier1_rgr";
                            backpack = "B_AssaultPack_mcamo";
                            primaryWeapon = "arifle_MX_F";
                        };
                    };
                };
            };
        };
    };
};
```

## Dependency Extraction

The parser uses pattern matching to find dependencies in various locations:

```rust
use parser_sqm::{DependencyExtractor, QueryPattern};

// Create a custom extractor with specific patterns
let patterns = vec![
    // Find all uniforms at any depth
    QueryPattern::new("*/Inventory", &["uniform"]),
    
    // Find weapons in specific paths
    QueryPattern::new("*/Inventory/primaryWeapon", &["name"]),
    
    // Find vehicle classes
    QueryPattern::new("*/Entities/Item*", &["type"]),
];

let extractor = DependencyExtractor::with_patterns(&sqm_file, patterns);
let custom_deps = extractor.extract()?;
```

## Query Patterns

The query system supports wildcards for flexible path matching:

- `*` - Matches any single path segment
- `Item*` - Matches any segment starting with "Item"
- `*/Inventory` - Matches "Inventory" at any depth

Example patterns:
```rust
// Match inventory items at any depth
"*/Inventory"

// Match specific weapon slots
"*/Inventory/primaryWeapon/primaryMuzzleMag"

// Match all entity items
"*/Entities/Item*"
```

## Performance

The parser is optimized for large mission files:

- Lazy parsing of nested structures
- Efficient string interning
- Minimal allocations during traversal
- Benchmarked on real-world missions

## Integration

Used by the mission scanner to:
- Identify required mods
- Validate loadouts
- Check vehicle availability
- Generate dependency reports

## Error Handling

```rust
use parser_sqm::ParseError;

match parse_sqm_file(path) {
    Ok(sqm) => { /* ... */ },
    Err(ParseError::InvalidFormat(msg)) => {
        eprintln!("Invalid SQM format: {}", msg);
    },
    Err(ParseError::IoError(e)) => {
        eprintln!("Failed to read file: {}", e);
    },
}
```

## Supported Formats

- Unbinarized SQM (text format)
- Binarized SQM (via HEMTT libraries)
- All Arma 3 mission versions

## License

Part of the arma3_tool project. 