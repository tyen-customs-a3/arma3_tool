# Arma 3 Database CSV Exporter

This tool exports Arma 3 class data from the database to a CSV file format suitable for visualization and analysis.

## Usage

```bash
# Basic usage with default output file
cargo run --bin export_csv -- --database /path/to/arma3_data.db

# Specify output file
cargo run --bin export_csv -- -d /path/to/arma3_data.db -o my_export.csv

# Filter by parent class prefixes
cargo run --bin export_csv -- -d /path/to/arma3_data.db -f "CfgVehicles,CfgWeapons"

# Limit the number of exported classes
cargo run --bin export_csv -- -d /path/to/arma3_data.db -l 1000
```

## Output Format

The CSV file contains the following columns:

1. `id` - Class identifier
2. `label` - Display name or class identifier if no display name exists
3. `type` - Always "class" for class entries
4. `parent_id` - Parent class identifier (empty for root classes)
5. `container_class` - Container class identifier (empty if none)
6. `source_path` - Source file path (empty if unknown)
7. `properties` - JSON object containing class properties

Example:

```csv
id,label,type,parent_id,container_class,source_path,properties
CfgVehicles,Vehicles,class,,,,"{}"
CfgWeapons,Weapons,class,,,,"{}"
Car_F,Car Base,class,CfgVehicles,CfgVehicles,"\A3\Soft_F\config.cpp","{""displayName"":""Car"",""simulation"":""car"",""scope"":0}"
```

## Filtering Options

### Parent Filters

Use the `-f` or `--filter-parents` option to specify comma-separated parent class prefixes. Only classes with a parent matching one of these prefixes will be included in the export.

```bash
# Export only classes with CfgVehicles parents
cargo run --bin export_csv -- -d db.sqlite -f CfgVehicles
```

### Export Limit

Use the `-l` or `--limit` option to specify the maximum number of classes to export. Default is 0 (unlimited).

```bash
# Export only the first 100 classes
cargo run --bin export_csv -- -d db.sqlite -l 100
```

## Integration with Other Tools

The exported CSV can be used with data visualization tools like:

1. Gephi for graph visualization
2. D3.js for web-based visualizations
3. Excel/Google Sheets for data analysis
4. R/Python for data processing and analysis 