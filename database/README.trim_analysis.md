# Arma 3 TrimAnalysis Tool

This tool analyzes the impact of removing classes from the Arma 3 game hierarchy.

## Features

- Identifies orphaned classes (classes whose parent would be removed)
- Finds PBOs that would be empty after trimming
- Supports file watching for interactive analysis
- Handles comments and empty lines in input files
- Outputs detailed JSON analysis results

## Key Features

- **Regex Pattern Support**: Use regular expressions to match multiple classes at once
- **Hierarchical Analysis**: Identify classes that would become orphaned when their parent classes are removed
- **Protected Classes**: Mark specific classes with `+` to prevent them from being removed
- **PBO Impact Analysis**: Identify PBOs that would become empty after class removal
- **Orphaned Class Filtering**: Classes directly marked for removal are not reported as orphaned, even if their parent would also be removed
- **Protected Class Warnings**: Get alerts if protected classes would become orphaned

## Building

```bash
# Build with the trim_analysis feature enabled
cargo build --release --features "trim_analysis"
```

## Usage

```bash
# Basic usage
cargo run --features "trim_analysis" --bin trim_analysis -- --database path/to/arma3.db --input classes_to_trim.txt --output analysis.json

# Watch mode (automatically rerun when input file changes)
cargo run --features "trim_analysis" --bin trim_analysis -- --database path/to/arma3.db --input classes_to_trim.txt --output analysis.json --watch
```
cargo run --features "trim_analysis" --bin trim_analysis -- --database "D:/pca/git/dep/rs/arma3_tool/cache/pca_next/arma3.db" --input D:/pca/git/dep/rs/arma3_tool/database/example_classes_to_trim.txt --output analysis.json --watch
## Input Format

The input file should contain one class name per line. You can use:
- Direct class names
- Regular expression patterns to match multiple classes
  - Supports advanced patterns including look-ahead and look-behind assertions
- Protected classes (prefixed with '+') that you want to keep
- Comments (lines starting with #)
- Empty lines (ignored)

Example:
```
# Direct class names to remove
CfgVehicles_Car_F
CfgWeapons_Rifle_Base_F

# Classes to protect (keep) - will warn if they become orphans
+USP_G3C_CU_MX
+USP_G3C_MX_VQ_MCB
+USP_G3C_IST_MX_MCB

# Regex patterns for classes to remove
USP_G3C_CU_.*  # Matches all USP_G3C_CU_ variants
USP_G3C.*_VQ_.*  # Matches all USP_G3C with VQ in the name
USP_G3C_IST_.*(?<!NAV)$  # Matches all USP_G3C_IST_ except those ending in NAV

# Protected regex patterns (classes to keep)
+cigs_black_devil_cig0  # Keep this specific cigarette
+cigs_crayons_crayon_(black|green)  # Keep black and green crayons
+helmet_(?!team_leader).*  # Keep all helmets except team leader variants

# Cigarette variants to remove using regex
cigs_baja_blast_cig[0-4]  # Matches cigs_baja_blast_cig0 through cigs_baja_blast_cig4
cigs_black_devil_cig[1-4]  # Matches cigs_black_devil_cig1 through cigs_black_devil_cig4
cigs_crayons_crayon_(orange|pink|white|yellow)  # Matches specified crayon colors
```

The tool will:
1. Process all direct class names
2. Expand regex patterns by matching against all classes in the database
3. Process protected classes (prefixed with '+')
4. Remove any duplicate class names
5. Use the resulting unique set of classes for analysis
6. Warn if any protected classes would become orphans due to removals

## Output Format

The output is a Markdown file containing:

```markdown
# Trim Analysis Results

## Summary

- **Classes to Remove**: 4
- **Orphaned Classes**: 12
- **Empty PBOs**: 2
- **Protected Classes**: 5
- **At-Risk Protected Classes**: 1

## Classes to Remove

CfgVehicles_Car_F
CfgWeapons_Rifle_Base_F
...

## Orphaned Class Chains

Classes that would become orphaned and their children:

CfgVehicles_Car_Base_F (orphan)
    CfgVehicles_Car_Advanced_F
        CfgVehicles_SUV_F
        CfgVehicles_Hatchback_F
    CfgVehicles_Car_Basic_F
        CfgVehicles_Offroad_F

## Empty PBOs

PBOs that would be empty after removing these classes:

A3_Weapons_F
A3_Soft_F

## Protected Classes

USP_G3C_CU_MX
USP_G3C_MX_VQ_MCB
...

## At-Risk Protected Classes

These protected classes would become orphans and may need their parent classes protected:

USP_G3C_MX_VQ_MCB
```

### Key Features of the Output

1. **Hierarchical Orphan Display**: Orphaned classes are shown with their child classes in an indented tree structure, making it easy to see the impact on class hierarchies.

2. **Protected Class Warnings**: Clearly identifies protected classes that would become orphans due to their parent classes being removed.

3. **No Graph Data**: The output focuses on readable, relevant information without unnecessary graph data.

4. **No Affected Classes Section**: Simplifies the output by omitting the affected classes section.

5. **Plain Text Format**: Class names are output as plain text without backticks or list markers for easy copying and further processing.

## Command Line Arguments

| Argument | Description |
|----------|-------------|
| `-d, --database` | Path to the SQLite database file (required) |
| `-i, --input` | Path to the input file with classes to trim (required) |
| `-o, --output` | Path to the output JSON file (default: trim_analysis.json) |
| `-w, --watch` | Watch the input file for changes and rerun the analysis |

## Examples

### Analyze specific vehicle classes

1. Create an input file `vehicles_to_trim.txt`:
   ```
   CfgVehicles_Car_F
   CfgVehicles_Air_Base_F
   ```

2. Run the analysis:
   ```bash
   cargo run --features "trim_analysis" --bin trim_analysis -- -d arma3.db -i vehicles_to_trim.txt -o vehicles_analysis.json
   ```

### Interactive analysis with file watching

1. Run in watch mode:
   ```bash
   cargo run --features "trim_analysis" --bin trim_analysis -- -d arma3.db -i classes_to_trim.txt -w
   ```

2. Edit the `classes_to_trim.txt` file to add or remove classes.
   The analysis will automatically rerun and update the output file.

## Integration with Other Tools

The JSON output can be used with:

1. Graph visualization tools like Gephi
2. Web visualization using D3.js
3. Custom analysis scripts
4. CI/CD pipelines for automated impact analysis

## Orphaned Class Detection

The tool intelligently handles orphaned classes:

1. A class is considered "orphaned" only if:
   - Its parent class would be removed
   - The class itself is NOT directly marked for removal
   
2. Classes that are both orphaned AND directly marked for removal:
   - Are treated as regular removed classes
   - Are not listed in the orphaned classes output
   - Do not trigger warnings when protected

This ensures that the analysis focuses on the true impact of your removals, without cluttering the output with classes that would be directly removed anyway. 