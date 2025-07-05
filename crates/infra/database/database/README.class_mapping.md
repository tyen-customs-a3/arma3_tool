# Arma 3 Class Mapping Analysis Tool

This tool analyzes missions for missing class dependencies and helps create mappings to compatible replacement classes.

## Features

- Identifies missing classes in missions
- Loads class mappings from a CSV file
- Generates detailed reports of missing classes
- Watches the mappings file for changes
- Shows both mapped and unmapped missing classes

## Building

```bash
# Build with the class_mapping feature enabled
cargo build --release --features "class_mapping"
```

## Usage

```bash
# Basic usage
cargo run --features "class_mapping" --bin class_mapping -- --database path/to/arma3.db --mappings class_mappings.csv

# Watch mode (automatically rerun when mappings file changes)
cargo run --features "class_mapping" --bin class_mapping -- --database path/to/arma3.db --mappings class_mappings.csv --watch
```
cargo run --features "class_mapping" --bin class_mapping -- --database "D:/pca/git/dep/rs/arma3_tool/cache/pca_next/arma3.db" --input reporting/input/mappings.csv --output reporting/mapping.txt --watch

## Input Format

The mappings file should be a CSV file with the following columns:
- `original_class`: The missing class name
- `replacement_class`: The compatible replacement class
- `notes`: Description or notes about the mapping

Example:
```csv
original_class,replacement_class,notes
aegis_30rnd_545x39_7n10_ak12,30Rnd_545x39_AK12_Mag_F,"30rnd 5.45x39mm 7N10 AK-12 mag, from Aegis"
aegis_30rnd_545x39_7n10_ak12_mag,30Rnd_545x39_AK12_Mag_F,"Alternative name for the same magazine"
```

## Output Format

The tool generates a report showing:

1. Summary statistics:
   - Total number of missions
   - Number of missions with missing classes
   - Total number of unique missing classes

2. Missing classes by mission:
   - Lists each mission with its missing classes
   - Shows replacement mappings where available
   - Indicates unmapped classes

3. Unmapped classes:
   - List of all missing classes that don't have a mapping

Example output:
```
# Mission Class Dependency Analysis Report

## Summary
- Total Missions: 42
- Missions with Missing Classes: 15
- Total Unique Missing Classes: 23

## Missing Classes by Mission

### Mission1
- aegis_30rnd_545x39_7n10_ak12 -> 30Rnd_545x39_AK12_Mag_F (30rnd 5.45x39mm 7N10 AK-12 mag, from Aegis)
- some_other_missing_class (No mapping found)

### Mission2
- aegis_30rnd_545x39_7n10_ak12_mag -> 30Rnd_545x39_AK12_Mag_F (Alternative name for the same magazine)

## Unmapped Classes
- some_other_missing_class
```

## Command Line Arguments

| Argument | Description |
|----------|-------------|
| `-d, --database` | Path to the SQLite database file (required) |
| `-m, --mappings` | Path to the CSV file with class mappings (required) |
| `-w, --watch` | Watch the mappings file for changes and rerun analysis |

## Integration with Other Tools

The output can be used to:
1. Generate class mapping files for mod compatibility
2. Identify missing dependencies in missions
3. Track progress in mapping missing classes
4. Automate class replacement in mission files 