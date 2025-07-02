# HPP Parser

A high-performance parser for Arma 3 configuration files (config.cpp, .hpp files) built on the HEMTT parsing libraries.

## Overview

This crate provides parsing capabilities for Arma 3's HPP configuration format, which is used extensively for defining classes, weapons, vehicles, and other game objects. It handles preprocessor directives, class inheritance, and property extraction.

## Features

- Full HPP/config.cpp syntax support
- Preprocessor directive handling (#include, #define, etc.)
- Class hierarchy parsing
- Property extraction with type inference
- Query system for efficient data retrieval
- Built on battle-tested HEMTT libraries

## Usage

```rust
use parser_hpp::{parse_config, query_classes};
use std::path::Path;

// Parse a config file
let config_path = Path::new("addons/weapons/config.cpp");
let parsed_config = parse_config(config_path)?;

// Query specific classes
let weapons = query_classes(&parsed_config, "CfgWeapons")?;

// Access class properties
for weapon_class in weapons {
    if let Some(display_name) = weapon_class.get_property("displayName") {
        println!("Found weapon: {}", display_name);
    }
}
```

## Supported Syntax

The parser handles all standard HPP/config.cpp constructs:

### Class Definitions
```cpp
class CfgWeapons {
    class Rifle_Base_F;
    class arifle_MX_Base_F: Rifle_Base_F {
        displayName = "MX Base";
        magazines[] = {"30Rnd_65x39_caseless_mag"};
        reloadTime = 0.12;
    };
};
```

### Arrays and Properties
```cpp
class MyClass {
    // Simple properties
    author = "MyName";
    value = 123;
    
    // Arrays
    items[] = {"item1", "item2", "item3"};
    nested[] = {{"a", 1}, {"b", 2}};
    
    // Inheritance
    class SubClass: ParentClass {
        // ...
    };
};
```

### Preprocessor Directives
```cpp
#include "macros.hpp"
#define MY_MACRO(x) class x##_Base

MY_MACRO(Weapon) {
    // Expands to: class Weapon_Base
};
```

## Query System

The parser includes an efficient query system for finding and filtering classes:

```rust
use parser_hpp::{QueryBuilder, ClassFilter};

// Build a query
let query = QueryBuilder::new()
    .with_parent("Rifle_Base_F")
    .has_property("magazines")
    .build();

// Execute query
let assault_rifles = query.execute(&parsed_config)?;

// Filter results
let filtered = assault_rifles
    .into_iter()
    .filter(|class| {
        class.get_property_array("magazines")
            .map(|mags| mags.len() > 0)
            .unwrap_or(false)
    })
    .collect::<Vec<_>>();
```

## Performance

The parser is optimized for large configuration files:

- Streaming parser for memory efficiency
- Lazy evaluation of includes
- Indexed queries for fast lookups
- Benchmarked on real Arma 3 configs

## Integration with Arma 3 Tool

This parser is used by:
- `scanner_gamedata` for extracting class definitions
- `weapon_magazine_scanner` for analyzing weapon configs
- Database population for class hierarchies

## Error Handling

The parser provides detailed error messages with file locations:

```rust
match parse_config(path) {
    Ok(config) => { /* ... */ },
    Err(e) => {
        eprintln!("Parse error at {}:{}: {}", 
            e.file(), e.line(), e.message());
    }
}
```

## Dependencies

This crate relies on the HEMTT project's parsing libraries, which are battle-tested in the Arma 3 modding community.

## License

Part of the arma3_tool project. 