# Arma 3 Tool CLI Usage Guide

This document describes how to use the Arma 3 Tool command-line interface to extract, process, and generate reports from Arma 3 PBO files and missions.

## Prerequisites

- Rust toolchain installed
- Compiled Arma 3 Tool project (`cargo build --release`)
- Configuration file (see Configuration section below)

## Basic Usage

The Arma 3 Tool CLI follows this pattern:
```bash
cargo run --release --bin arma3tool_cli -- [OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

Or if using the compiled binary:
```bash
arma3tool_cli [OPTIONS] <COMMAND> [COMMAND_OPTIONS]
```

## Environment Variables

Set the logging level for detailed output:
```bash
set RUST_LOG=info    # Windows
export RUST_LOG=info # Linux/macOS
```

## Commands

### 1. Extract Command

Extracts game data and missions from PBO files into cache directories.

```bash
# Basic extraction using default config
cargo run --release --bin arma3tool_cli -- --config scan_config.json extract

# With custom cache directory
cargo run --release --bin arma3tool_cli -- --config scan_config.json extract --cache-dir ./custom_cache

# With custom extractor database path
cargo run --release --bin arma3tool_cli -- --config scan_config.json extract --extractor-db-path ./custom_extractor.db
```

**Options:**
- `--cache-dir <PATH>`: Override cache directory from config
- `--extractor-db-path <PATH>`: Override extractor database file path (stores PBO extraction state)

**What it does:**
- Unpacks PBO files from configured game data directories
- Extracts mission files from configured mission directories
- Stores extraction state in extractor database
- Creates cache directory structure

### 2. Process Command

Processes extracted files to build the analysis database with class definitions and dependencies.

```bash
# Basic processing
cargo run --release --bin arma3tool_cli -- --config scan_config.json process

# With custom paths
cargo run --release --bin arma3tool_cli -- --config scan_config.json process --cache-dir ./custom_cache --analysis-db-path ./custom_analysis.db
```

**Options:**
- `--cache-dir <PATH>`: Override cache directory from config
- `--analysis-db-path <PATH>`: Override analysis database file path (stores class/dependency data)

**What it does:**
- Scans extracted game data files (.cpp, .hpp, .bin)
- Scans extracted mission files (.sqf, .cpp, .hpp, .sqm, .ext)
- Builds class hierarchy and dependency relationships
- Stores analysis results in analysis database

### 3. Report Command

Generates various reports from the analysis database.

```bash
# Basic report generation
cargo run --release --bin arma3tool_cli -- --config scan_config.json report

# With custom output directory
cargo run --release --bin arma3tool_cli -- --config scan_config.json report --output-dir ./custom_reports

# Using specific database
cargo run --release --bin arma3tool_cli -- --config scan_config.json report --analysis-db-path ./custom_analysis.db --output-dir ./reports
```

**Options:**
- `--cache-dir <PATH>`: Override cache directory from config (used for default DB path if needed)
- `--analysis-db-path <PATH>`: Override analysis database file path
- `--output-dir <PATH>`: Override output directory for reports

**Generated Reports:**
- **Dependency Report**: Lists missing class dependencies per mission (text format)
- **Class Hierarchy Graph**: Visual representation of class inheritance (CSV format)
- **Mission Class Source Report**: Maps mission classes to their source files (CSV format)

### 4. FuzzyReport Command

Generates a report of missing classes with fuzzy match suggestions.

```bash
# Generate fuzzy matching report
cargo run --release --bin arma3tool_cli -- --config scan_config.json fuzzy-report

# With custom paths
cargo run --release --bin arma3tool_cli -- --config scan_config.json fuzzy-report --analysis-db-path ./analysis.db --output-dir ./reports
```

**Options:**
- `--cache-dir <PATH>`: Override cache directory from config
- `--analysis-db-path <PATH>`: Override analysis database file path
- `--output-dir <PATH>`: Override output directory for reports

**What it does:**
- Analyzes missing class dependencies
- Provides fuzzy match suggestions for potentially misspelled class names
- Uses Normalized Levenshtein distance with 0.6 threshold for matching

### 5. All Command

Runs the complete workflow: extract, process, and report in sequence.

```bash
# Complete workflow with default config
cargo run --release --bin arma3tool_cli -- --config scan_config.json all

# With custom paths
cargo run --release --bin arma3tool_cli -- --config scan_config.json all \
  --cache-dir ./cache/pca_next \
  --extractor-db-path ./cache/pca_next/extractor.db \
  --analysis-db-path ./cache/pca_next/analysis.db \
  --output-dir ./reports
```

**Options:**
- `--cache-dir <PATH>`: Override cache directory from config
- `--extractor-db-path <PATH>`: Override extractor database file path
- `--analysis-db-path <PATH>`: Override analysis database file path
- `--output-dir <PATH>`: Override output directory for reports

**Workflow:**
1. Extract PBOs → cache directories
2. Process extracted files → analysis database
3. Generate reports → output directory

## Configuration

The tool uses JSON configuration files to specify paths and settings. See the provided example configurations:

- `scan_config.json` - Basic configuration
- `scan_config_pca.json` - PCA-specific configuration
- `scan_config_pcanext.json` - PCA Extended configuration

### Example Configuration Structure:
```json
{
  "game_data_dirs": [
    "E:/pcanext",
    "C:/Program Files (x86)/Steam/steamapps/common/Arma 3"
  ],
  "game_data_extensions": ["hpp", "cpp", "bin"],
  "mission_dirs": ["E:/pca_missions"],
  "mission_extensions": ["hpp", "cpp", "sqf", "xml", "sqm"],
  "cache_dir": "./cache/pca_next",
  "report_dir": "./reports_pca_next",
  "database_path": "./cache/pca_next/arma3.db",
  "threads": 31
}
```

## Common Workflows

### First-Time Setup
```bash
# 1. Extract and process everything from scratch
set RUST_LOG=info
cargo run --release --bin arma3tool_cli -- --config scan_config.json all

# 2. Generate additional fuzzy matching report
cargo run --release --bin arma3tool_cli -- --config scan_config.json fuzzy-report
```

### Update Analysis
```bash
# 1. Re-extract if PBOs changed
cargo run --release --bin arma3tool_cli -- --config scan_config.json extract

# 2. Re-process extracted files
cargo run --release --bin arma3tool_cli -- --config scan_config.json process

# 3. Generate fresh reports
cargo run --release --bin arma3tool_cli -- --config scan_config.json report
```

### Report Generation Only
```bash
# Generate reports from existing analysis database
cargo run --release --bin arma3tool_cli -- --config scan_config.json report --output-dir ./fresh_reports
```

### Compare Different Configurations
```bash
# Process PCA configuration
cargo run --release --bin arma3tool_cli -- --config scan_config_pca.json all

# Process PCA Extended configuration  
cargo run --release --bin arma3tool_cli -- --config scan_config_pcanext.json all

# Compare results by examining the generated reports in their respective output directories
```

## Database Architecture

The tool uses a dual database approach:

1. **Extractor Database** (`extractor.db`): Tracks PBO extraction state and timestamps
2. **Analysis Database** (`analysis.db`): Stores parsed class definitions, dependencies, and relationships

This separation allows efficient re-analysis without re-extracting unchanged PBOs.

## Output Structure

```
cache/
├── gamedata/          # Extracted game data files
├── missions/          # Extracted mission files
├── extractor.db       # PBO extraction state
└── analysis.db        # Class and dependency data

reports/
├── dependency_report.txt           # Missing dependencies per mission
├── class_hierarchy.csv            # Class inheritance relationships
├── mission_class_sources.csv      # Class-to-source-file mappings
└── fuzzy_missing_classes.txt      # Missing classes with suggestions
```

## Troubleshooting

### Common Issues

1. **CONFIG_LINTS duplicate registration**: This has been fixed in the current version by using sequential mission processing.

2. **Database locked errors**: Ensure no other instances are running and accessing the same database files.

3. **Missing PBO files**: Verify paths in configuration file and ensure PBO files exist and are readable.

4. **Out of memory**: Reduce thread count in configuration file or process smaller batches.

### Performance Tips

- Use SSD storage for cache directories
- Adjust thread count based on CPU cores and available memory
- Use separate cache directories for different configurations to avoid conflicts