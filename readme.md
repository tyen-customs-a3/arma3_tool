# Arma 3 Dependency Analysis Flow

This document explains how to use the provided scripts and commands to analyze Arma 3 mods, missions, and their dependencies.

## Prerequisites

- Rust toolchain (for building the tool if not already built)
- Arma 3 installed (default path: `C:\Program Files (x86)\Steam\steamapps\common\Arma 3`)
- PCA mods in `E:\pca` (adjust path in scripts if different)
- PCA missions in `E:\pca_missions_quick` (adjust path in scripts if different)

## Available Commands

The `arma3_tool` provides several commands for analyzing Arma 3 content:

1. **scan-pbos**: Extract files from PBO archives
2. **scan-classes**: Analyze class definitions in CPP files
3. **scan-missions**: Extract and analyze mission files
4. **analyze-mission-dependencies**: Cross-reference mission equipment with available classes
5. **full-analysis**: Run the complete analysis pipeline in one command

## Running the Complete Analysis

### Using the Built-in Command

The simplest way to run the complete analysis is to use the built-in `full-analysis` command:

```bash
.\target\release\arma3_tool.exe --log-level error full-analysis --mods-dir "E:\pca" --missions-dir "E:\pca_missions_quick"
```

## Controlling Verbosity

You can control the verbosity of the output by setting the `--log-level` parameter:

```bash
.\target\release\arma3_tool.exe --log-level warn full-analysis ...
```

Available log levels:
- **error**: Only show errors
- **warn**: Show warnings and errors
- **info**: Show general progress information (default)
- **debug**: Show detailed debugging information
- **trace**: Show extremely verbose information

## Analysis Flow

The analysis performs the following steps:

1. **Scan Arma 3 Base Game Files**
   - Extracts all PBO files from the Arma 3 installation directory
   - Focuses on `.hpp`, `.cpp`, `.sqf`, and `.sqm` files
   - Stores extracted files in `.\cache\a3_base`

2. **Scan PCA Mod Files**
   - Extracts all PBO files from the PCA mods directory
   - Focuses on `.hpp`, `.cpp`, `.sqf`, and `.sqm` files
   - Stores extracted files in `.\cache\mods`

3. **Scan Mission Files**
   - Extracts all mission PBO files from the PCA missions directory
   - Extracts `.sqf` and `.sqm` files
   - Stores extracted files in `.\cache\missions`
   - Generates initial mission reports in `.\reports\mission_reports`

4. **Scan Class Definitions**
   - Analyzes class definitions in extracted Arma 3 base files
   - Analyzes class definitions in extracted PCA mod files
   - Generates class reports in `.\reports\a3_base_classes` and `.\reports\mods_classes`

5. **Analyze Dependencies**
   - Cross-references mission equipment with available classes
   - Generates comprehensive dependency reports in `.\reports\analysis_reports`

## Output Reports

The analysis generates several reports:

- **Mission Reports** (`.\reports\mission_reports`)
  - Individual JSON files for each mission
  - Lists equipment, vehicles, weapons, magazines, items, and backpacks used
  - Includes a summary report with statistics for all missions

- **Class Reports**
  - Arma 3 base classes (`.\reports\a3_base_classes`)
  - PCA mod classes (`.\reports\mods_classes`)
  - Lists all class definitions, their properties, and inheritance

- **Dependency Analysis** (`.\reports\analysis_reports`)
  - Cross-references mission equipment with available classes
  - Identifies missing dependencies
  - Provides statistics on dependency coverage
  - Includes a summary report with dependency metrics for all missions

## Running Individual Steps

If you prefer to run individual steps of the analysis, you can use the specific commands:

```bash
# Step 1: Scan Arma 3 base game files
.\target\release\arma3_tool.exe --log-level info scan-pbos \
    --input-dir "C:\Program Files (x86)\Steam\steamapps\common\Arma 3" \
    --cache-dir "./cache/a3_base" \
    --extensions "hpp,cpp,sqf,sqm" \
    --threads 8

# Step 2: Scan mod files
.\target\release\arma3_tool.exe --log-level info scan-pbos \
    --input-dir "E:\pca" \
    --cache-dir "./cache/mods" \
    --extensions "hpp,cpp,sqf,sqm" \
    --threads 8

# Step 3: Scan mission files
.\target\release\arma3_tool.exe --log-level info scan-missions \
    --input-dir "E:\pca_missions_quick" \
    --cache-dir "./cache/missions" \
    --output-dir "./reports/mission_reports" \
    --threads 8

# Step 4: Scan class definitions
.\target\release\arma3_tool.exe --log-level info scan-classes \
    --input-dir "./cache/a3_base" \
    --output-dir "./reports/a3_base_classes"

.\target\release\arma3_tool.exe --log-level info scan-classes \
    --input-dir "./cache/mods" \
    --output-dir "./reports/mods_classes"

# Step 5: Analyze dependencies
.\target\release\arma3_tool.exe --log-level info analyze-mission-dependencies \
    --mission-dir "E:\pca_missions_quick" \
    --addon-dir "./cache" \
    --cache-dir "./cache/analysis" \
    --output-dir "./reports/analysis_reports" \
    --threads 8
```

## Troubleshooting

- **Missing Executable**: Ensure the tool is built with `cargo build --release`
- **Access Denied**: Run the script with administrator privileges
- **Path Not Found**: Verify the paths in the script match your system
- **Out of Memory**: Reduce the number of threads or run steps individually

## Performance Considerations

- The analysis can be resource-intensive, especially for large mod collections
- Consider running during off-hours or when the system is not in use
- The cache directory can grow large; ensure sufficient disk space
- Subsequent runs will be faster as unchanged PBOs are skipped 