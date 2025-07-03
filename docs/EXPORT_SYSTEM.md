# Inheritance-Based Export System

## Overview

The Arma 3 Tool now features a powerful inheritance-based export system that categorizes items based on their class inheritance hierarchy rather than simple prefix matching. This provides more accurate and reliable categorization while delivering significant performance improvements.

## Key Features

- **Accurate Categorization**: Uses recursive SQL CTEs to trace inheritance relationships
- **High Performance**: 1.98x speedup with inheritance caching and optimized queries
- **Configurable**: JSON-based configuration for item types and exclusion rules
- **Parallel Processing**: Leverages rayon for multi-core performance
- **Flexible CLI**: Support for exporting specific item types or all configured types

## Configuration Format

The export system uses a JSON configuration file (default: `item_filter_config.json`) that defines item types and their corresponding base classes.

### Configuration Structure

```json
{
  "version": "1.0",
  "item_types": {
    "weapons": {
      "base_classes": ["Rifle_Base_F", "Pistol_Base_F", "Launcher_Base_F"]
    },
    "uniforms": {
      "base_classes": ["Uniform_Base", "U_BasicBody"]
    },
    "vests": {
      "base_classes": ["Vest_Base", "Vest_Camo_Base"]
    },
    "backpacks": {
      "base_classes": ["Bag_Base"]
    }
  },
  "exclusion_rules": {
    "max_scope": 1,
    "excluded_prefixes": ["B_soldier_f"]
  }
}
```

### Configuration Fields

#### `version` (string, required)
Configuration format version. Currently supports "1.0".

#### `item_types` (object, required)
Defines the categories of items to export. Each key represents an item type name, with a value containing:

- **`base_classes`** (array of strings): List of base class names that define this item type. Any class that inherits from these base classes will be categorized under this type.

#### `exclusion_rules` (object, required)
Defines rules for excluding classes from export:

- **`max_scope`** (integer): Classes with scope values less than or equal to this value are excluded. Typically set to 1 to exclude prototype/base classes.
- **`excluded_prefixes`** (array of strings): Classes whose names start with any of these prefixes are excluded from export.

## CLI Usage

### Basic Export

Export all configured item types to CSV:

```bash
arma3tool_cli export --analysis-db-path analysis.db --output items.csv
```

### Export Specific Types

Export only weapons and vests:

```bash
arma3tool_cli export --analysis-db-path analysis.db --output weapons_vests.csv --types weapons,vests
```

### Custom Configuration

Use a custom configuration file:

```bash
arma3tool_cli export --analysis-db-path analysis.db --output custom_items.csv --item-config my_config.json
```

### Limit Results

Export only the first 1000 items:

```bash
arma3tool_cli export --analysis-db-path analysis.db --output limited.csv --limit 1000
```

### Complete CLI Options

```
Export items using inheritance-based categorization to CSV

Usage: arma3tool_cli export [OPTIONS]

Options:
      --cache-dir <CACHE_DIR>
          Override cache directory from config (used for default DB path if needed)
      --analysis-db-path <ANALYSIS_DB_PATH>
          Override analysis database file path (stores class/dependency data)
  -o, --output <OUTPUT>
          Output CSV file path [default: items_export.csv]
      --types <TYPES>
          Item types to export from config (comma-separated): weapons, uniforms, vests, backpacks
          If not specified, exports all types defined in configuration
      --item-config <ITEM_CONFIG>
          Path to custom item filter configuration file [default: item_filter_config.json]
  -l, --limit <LIMIT>
          Maximum number of items to export (0 for unlimited) [default: 0]
  -h, --help
          Print help
```

## Inheritance Detection Logic

The export system uses recursive SQL Common Table Expressions (CTEs) to efficiently trace inheritance relationships in the database. This approach provides several advantages:

### How Inheritance Detection Works

1. **Database-Optimized**: Inheritance tracing happens at the SQL level using recursive CTEs
2. **Case-Insensitive**: All class name comparisons use `UPPER()` for consistency
3. **Depth-Limited**: Configurable maximum depth prevents infinite loops
4. **Multi-Target**: Single query can check inheritance from multiple base classes

### SQL Query Structure

The core inheritance query follows this pattern:

```sql
WITH RECURSIVE inheritance_chain(id, parent_id, depth) AS (
    -- Base case: start with the given class
    SELECT id, parent_id, 0 as depth
    FROM classes
    WHERE UPPER(id) = UPPER(?)
    
    UNION ALL
    
    -- Recursive case: follow parent relationships
    SELECT c.id, c.parent_id, ic.depth + 1
    FROM classes c
    JOIN inheritance_chain ic ON UPPER(ic.parent_id) = UPPER(c.id)
    WHERE ic.depth < ? AND ic.parent_id IS NOT NULL
)
SELECT EXISTS(
    SELECT 1 FROM inheritance_chain 
    WHERE UPPER(id) IN (...)
) as inherits
```

### Performance Characteristics

| Dataset Size | Original Performance | Optimized Performance | Speedup |
|-------------|---------------------|----------------------|---------|
| 100 classes | 473 items/second    | 940 items/second     | 1.98x   |
| 1000 classes | 93 items/second     | ~185 items/second*   | ~2x*    |

*Estimated based on inheritance cache optimization

#### Key Performance Optimizations

1. **Inheritance Cache**: Pre-computes inheritance relationships once
2. **Parallel Processing**: Uses rayon for concurrent class processing
3. **Buffered I/O**: BufWriter for efficient CSV output
4. **Lazy Loading**: Source path cache only loaded when needed
5. **Memory Efficient**: Single-pass filtering and processing

## Migration Guide

### From Old `--filter-parents` System

The previous export system used simple prefix matching with the `--filter-parents` flag. The new system provides more accurate results through inheritance-based categorization.

#### Old Command Format
```bash
# OLD: Unreliable prefix-based filtering
arma3tool_cli export --analysis-db-path analysis.db --filter-parents --output items.csv
```

#### New Command Format
```bash
# NEW: Inheritance-based categorization
arma3tool_cli export --analysis-db-path analysis.db --output items.csv
```

#### Migration Steps

1. **Remove `--filter-parents` flag**: This flag is no longer supported
2. **Add configuration file**: Create `item_filter_config.json` with your desired item types
3. **Update scripts**: Replace old commands with new syntax
4. **Verify results**: Compare output to ensure expected items are included

#### Key Differences

| Aspect | Old System | New System |
|--------|------------|------------|
| **Accuracy** | Prefix matching (brittle) | Inheritance tracing (accurate) |
| **Performance** | O(n²) for large datasets | O(n) with caching |
| **Configuration** | Hard-coded prefixes | Flexible JSON configuration |
| **Maintenance** | Manual prefix updates | Automatic via inheritance |
| **CLI Interface** | `--filter-parents` flag | `--types` and `--item-config` flags |

#### Example Migration

**Before (Old System):**
```bash
arma3tool_cli export \
  --analysis-db-path game_data.db \
  --filter-parents \
  --output weapons.csv
```

**After (New System):**
```bash
# Create configuration file
cat > weapon_config.json << EOF
{
  "version": "1.0",
  "item_types": {
    "weapons": {
      "base_classes": ["Rifle_Base_F", "Pistol_Base_F", "Launcher_Base_F"]
    }
  },
  "exclusion_rules": {
    "max_scope": 1,
    "excluded_prefixes": []
  }
}
EOF

# Run new export command
arma3tool_cli export \
  --analysis-db-path game_data.db \
  --item-config weapon_config.json \
  --types weapons \
  --output weapons.csv
```

## CSV Output Format

The export generates CSV files with the following columns:

| Column | Description |
|--------|-------------|
| `id` | Class identifier (e.g., "arifle_MX_F") |
| `label` | Display name or class ID if no display name |
| `type` | Item type categories (semicolon-separated if multiple) |
| `parent_id` | Parent class identifier |
| `container_class` | Container class (e.g., "CfgWeapons") |
| `source_path` | Source PBO or file path |
| `properties` | JSON-encoded class properties |

### Example CSV Output

```csv
id,label,type,parent_id,container_class,source_path,properties
arifle_MX_F,MX 6.5 mm,weapons,Rifle_Base_F,CfgWeapons,A3/Weapons_F,"{""displayName"":{""String"":""MX 6.5 mm""},""scope"":{""Number"":2.0}}"
U_BasicBody,Basic Body,uniforms,Uniform_Base,CfgVehicles,A3/Characters_F,"{""displayName"":{""String"":""Basic Body""},""scope"":{""Number"":2.0}}"
```

## Troubleshooting

### Common Issues

#### Configuration File Not Found
```
Error: Failed to load configuration: No such file or directory (os error 2)
```

**Solution**: Ensure the configuration file exists at the specified path. The default is `item_filter_config.json` in the current directory.

```bash
# Check if file exists
ls -la item_filter_config.json

# Use absolute path if needed
arma3tool_cli export --item-config /full/path/to/config.json
```

#### Invalid JSON Configuration
```
Error: Failed to load configuration: expected `,` or `}` at line 5 column 3
```

**Solution**: Validate your JSON syntax. Common issues include:
- Missing commas between items
- Trailing commas after last items
- Unescaped quotes in strings

```bash
# Validate JSON syntax
python -m json.tool item_filter_config.json
```

#### No Items Exported
```
Export completed successfully. Exported 0 classes
```

**Solution**: Check your configuration and exclusion rules:

1. **Verify base classes exist**: Ensure base class names match actual classes in your database
2. **Check exclusion rules**: `max_scope` might be excluding all items
3. **Verify inheritance**: Classes might not inherit from specified base classes

```bash
# Check if base classes exist in database
sqlite3 analysis.db "SELECT id FROM classes WHERE id IN ('Rifle_Base_F', 'Pistol_Base_F');"

# Check scope values
sqlite3 analysis.db "SELECT c.id, p.property_value FROM classes c JOIN class_properties p ON c.id = p.class_id WHERE p.property_name = 'scope' LIMIT 10;"
```

#### Database Connection Issues
```
Error: Failed to create database manager: Database file not found
```

**Solution**: Verify the database path and ensure the analysis database exists:

```bash
# Check if database file exists
ls -la analysis.db

# Use absolute path
arma3tool_cli export --analysis-db-path /full/path/to/analysis.db
```

#### Performance Issues

**Symptoms**: Export takes a very long time for large datasets

**Solutions**:
1. **Use the optimized version**: Ensure you're using the latest version with inheritance caching
2. **Limit results**: Use `--limit` flag for testing
3. **Optimize database**: Run `VACUUM` and `ANALYZE` on your database
4. **Check disk space**: Ensure sufficient disk space for temporary files

```bash
# Optimize database
sqlite3 analysis.db "VACUUM; ANALYZE;"

# Test with limited results
arma3tool_cli export --limit 100 --output test.csv
```

### Debug Mode

Enable verbose logging to diagnose issues:

```bash
RUST_LOG=info arma3tool_cli export --analysis-db-path analysis.db --output debug.csv
```

Log levels (in order of verbosity):
- `error`: Only errors
- `warn`: Warnings and errors  
- `info`: General information (recommended for debugging)
- `debug`: Detailed debugging information
- `trace`: Very detailed tracing (may impact performance)

### Getting Help

If you encounter issues not covered in this guide:

1. **Check the logs**: Use `RUST_LOG=info` for detailed output
2. **Verify configuration**: Ensure your JSON configuration is valid
3. **Test with minimal dataset**: Use `--limit` to test with smaller datasets
4. **Check database integrity**: Verify your analysis database is not corrupted

## Advanced Usage

### Custom Item Types

You can define custom item types beyond the standard weapons, uniforms, vests, and backpacks:

```json
{
  "version": "1.0", 
  "item_types": {
    "vehicles": {
      "base_classes": ["Car_F", "Helicopter_Base_F", "Ship_F"]
    },
    "static_weapons": {
      "base_classes": ["StaticWeapon"]  
    },
    "medical_items": {
      "base_classes": ["FirstAidKit", "Medikit"]
    }
  },
  "exclusion_rules": {
    "max_scope": 1,
    "excluded_prefixes": ["B_soldier_f", "O_soldier_f"]
  }
}
```

### Batch Processing

Process multiple databases or configurations:

```bash
#!/bin/bash
# Batch export script

databases=("analysis_vanilla.db" "analysis_mods.db" "analysis_dlc.db")
configs=("vanilla_config.json" "mods_config.json" "dlc_config.json")

for i in ${!databases[@]}; do
    echo "Processing ${databases[$i]}..."
    arma3tool_cli export \
        --analysis-db-path "${databases[$i]}" \
        --item-config "${configs[$i]}" \
        --output "export_${i}.csv"
done
```

### Integration with Other Tools

The CSV output can be easily integrated with other tools:

```bash
# Convert to JSON
python -c "
import csv, json
with open('items.csv') as f:
    reader = csv.DictReader(f)
    print(json.dumps(list(reader), indent=2))
" > items.json

# Filter specific items with jq
cat items.json | jq '.[] | select(.type | contains(\"weapons\"))' > weapons.json

# Import into database
sqlite3 items.db <<EOF
CREATE TABLE items (id TEXT, label TEXT, type TEXT, parent_id TEXT, container_class TEXT, source_path TEXT, properties TEXT);
.mode csv
.import items.csv items
EOF
```

## Best Practices

1. **Version Control Configuration**: Keep your configuration files in version control
2. **Validate Before Production**: Test configuration changes with `--limit` first
3. **Monitor Performance**: Use `RUST_LOG=info` to monitor export times
4. **Backup Databases**: Always backup analysis databases before major changes
5. **Document Custom Types**: Document any custom item types for team members
6. **Regular Updates**: Update base class lists when game updates change inheritance

## Changelog

### Version 1.0 (Current)
- Initial release of inheritance-based export system
- JSON configuration format
- Recursive SQL CTE inheritance detection
- Performance optimizations with caching
- Comprehensive CLI interface
- Migration from old `--filter-parents` system