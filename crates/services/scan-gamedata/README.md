# Game Data Scanner

A comprehensive scanner for Arma 3 game data files, capable of parsing and extracting class definitions from config files across entire mod collections.

## Overview

The Game Data Scanner orchestrates the parsing of Arma 3 configuration files to build a complete picture of available classes, their properties, and inheritance relationships. It supports both simple and advanced parsing modes and can handle large-scale mod collections efficiently.

## Features

- **Multi-Parser Support**: Simple and advanced parsing modes
- **Batch Processing**: Scan entire directories of game data
- **Intelligent Caching**: Skip already-processed files
- **Error Resilience**: Continue on individual file failures
- **Flexible Configuration**: JSON/YAML config support
- **Progress Tracking**: Real-time progress with ETA

## Architecture

```
scanner_gamedata/
├── parser_simple/      # Fast, basic parser
├── parser_advanced/    # Full-featured parser with preprocessing
├── models/            # Shared data models
└── tool/              # Batch processing utility
```

## Usage

### As a Library

```rust
use gamedata_scanner::{Scanner, ScanConfig, ParserType};
use std::path::Path;

// Configure the scanner
let config = ScanConfig {
    directories: vec![Path::new("@CUP_Weapons").to_path_buf()],
    parser_type: ParserType::Advanced,
    cache_dir: Some(Path::new("./cache").to_path_buf()),
    threads: 8,
};

// Create and run scanner
let scanner = Scanner::new(config);
let results = scanner.scan()?;

println!("Found {} classes", results.classes.len());
println!("Processed {} files", results.files_processed);

// Access parsed classes
for class in &results.classes {
    println!("Class: {} extends {:?}", class.name, class.parent);
}
```

### Batch Processing Tool

```bash
# Scan a single mod
scanner_gamedata_tool --config scan_config.json --input "@ACE" --output ace_classes.json

# Scan multiple directories
scanner_gamedata_tool --config scan_config.json --batch

# Use simple parser for speed
scanner_gamedata_tool --parser simple --input "@CBA_A3" --output cba_classes.json
```

## Configuration

Example `scan_config.json`:

```json
{
  "parser_type": "advanced",
  "cache_dir": "./cache/gamedata",
  "output_dir": "./output",
  "threads": 8,
  "directories": [
    "C:/Games/Arma 3/@CUP_Weapons",
    "C:/Games/Arma 3/@RHS_AFRF"
  ],
  "extensions": ["cpp", "hpp", "h"],
  "ignore_patterns": ["test_*", "dev_*"]
}
```

## Parser Comparison

### Simple Parser
- Fast regex-based parsing
- Basic class and property extraction
- No preprocessing support
- Good for quick scans

### Advanced Parser
- Full config syntax support
- Preprocessor directive handling
- Accurate inheritance tracking
- Required for complex mods

Performance comparison:
| Parser | 1000 Files | Features | Accuracy |
|--------|------------|----------|----------|
| Simple | ~5 sec | Basic | 85% |
| Advanced | ~30 sec | Full | 99% |

## Output Format

The scanner produces structured JSON output:

```json
{
  "metadata": {
    "scan_date": "2024-01-15T10:30:00Z",
    "parser_type": "advanced",
    "total_files": 1523,
    "total_classes": 15234
  },
  "classes": [
    {
      "name": "arifle_AK12_F",
      "parent": "arifle_AK12_base_F",
      "source_file": "@CUP_Weapons/addons/weapons/config.cpp",
      "properties": {
        "displayName": "AK-12 7.62 mm",
        "magazines": ["30Rnd_762x39_Mag_F"],
        "modes": ["Single", "FullAuto"]
      }
    }
  ]
}
```

## Error Handling

The scanner provides detailed error reporting:

```rust
use gamedata_scanner::ScanError;

match scanner.scan() {
    Ok(results) => { /* ... */ },
    Err(ScanError::ParseError { file, line, message }) => {
        eprintln!("Parse error in {} at line {}: {}", file, line, message);
    },
    Err(ScanError::IoError(e)) => {
        eprintln!("IO error: {}", e);
    },
}
```

## Integration

Used by arma3_tool for:
- Building class databases
- Generating inheritance trees
- Validating mod compatibility
- Extracting game item catalogs

## Performance Tips

1. **Use Simple Parser** when you only need basic class info
2. **Enable Caching** to avoid re-parsing unchanged files
3. **Adjust Thread Count** based on your CPU cores
4. **Filter Extensions** to only scan relevant files
5. **Use Ignore Patterns** to skip test/development files

## License

Part of the arma3_tool project. 