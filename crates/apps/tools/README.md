# Quick Extract

A fast, command-line utility for extracting Arma 3 PBO files with intelligent filtering and template-based project setup.

## Overview

Quick Extract is designed for rapid PBO extraction workflows, particularly useful for mod developers who need to quickly extract specific files or set up new projects based on existing mod structures. It provides smart filtering, parallel extraction, and project templating capabilities.

## Features

- **Fast Parallel Extraction**: Multi-threaded PBO processing
- **Smart Filtering**: Extract only the files you need
- **Project Templates**: Create new mod projects from existing structures
- **Batch Operations**: Process multiple PBOs at once
- **Memory Efficient**: Streaming extraction for large files
- **Cross-Platform**: Works on Windows, Linux, and macOS

## Usage

### Basic Extraction

```bash
# Extract a single PBO
quick_extract input.pbo -o output_dir/

# Extract multiple PBOs
quick_extract *.pbo -o extracted/

# Extract with specific file filter
quick_extract mod.pbo -o output/ --filter "*.sqf,*.hpp"
```

### Project Templates

```bash
# Create a new project from template
quick_extract --template ace3_module input.pbo -o my_new_module/

# Use custom template directory
quick_extract --template-dir ./templates --template my_template mod.pbo -o project/
```

### Advanced Options

```bash
# Extract only config files
quick_extract mod.pbo -o configs/ --filter "config.cpp,*.hpp" --no-preserve-paths

# Parallel extraction with progress
quick_extract large_mod_collection/*.pbo -o extracted/ --threads 8 --progress

# Dry run to see what would be extracted
quick_extract mod.pbo --dry-run --filter "*.sqf"
```

## Configuration

Quick Extract can be configured via `quick_extract.toml`:

```toml
[extract]
default_output = "./extracted"
threads = 8
preserve_paths = true

[filters]
configs = ["config.cpp", "*.hpp", "*.h"]
scripts = ["*.sqf", "*.fsm"]
models = ["*.p3d", "*.rtm"]

[templates]
template_dir = "./templates"
default_template = "basic_addon"
```

## Templates

Templates define project structures for quick setup:

```
templates/
└── ace3_module/
    ├── template.toml
    ├── addons/
    │   └── @ADDON_NAME@/
    │       ├── config.cpp
    │       ├── functions/
    │       └── ui/
    └── tools/
        └── build.py
```

Template configuration (`template.toml`):
```toml
[template]
name = "ACE3 Module Template"
description = "Template for creating ACE3-compatible modules"

[variables]
ADDON_NAME = { prompt = "Addon name", default = "my_addon" }
AUTHOR = { prompt = "Author name", default = "Unknown" }

[post_process]
rename_files = true
process_content = true
```

## Performance

Optimized for speed:
- Parallel extraction scales with CPU cores
- Memory-mapped file reading
- Efficient file filtering before extraction
- Progress reporting with ETA

Benchmarks:
- Single large PBO (1GB): ~5 seconds
- 100 small PBOs (10MB each): ~3 seconds
- Full mod collection (50GB): ~2 minutes

## Integration

Quick Extract can be used as:
- Standalone CLI tool
- Library in other Rust projects
- Part of build pipelines

```rust
use quick_extract::{extract_pbo, ExtractOptions};

let options = ExtractOptions {
    output_dir: "extracted/",
    filter: Some(vec!["*.sqf".to_string()]),
    threads: 8,
    preserve_paths: true,
};

extract_pbo("input.pbo", options)?;
```

## Error Handling

Comprehensive error reporting:
- Invalid PBO format detection
- Permission errors
- Disk space checks
- Corrupted file handling

## Use Cases

### Mod Development
- Extract reference implementations
- Set up new projects quickly
- Update templates from existing mods

### Server Administration
- Extract specific mission files
- Audit PBO contents
- Batch configuration extraction

### Build Automation
- CI/CD pipeline integration
- Automated mod packaging
- Version control preparation

## License

Part of the arma3_tool project. 