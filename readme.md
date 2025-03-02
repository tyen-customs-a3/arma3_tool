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

## Command Examples and Output

Below are detailed examples of each command with explanations of their parameters and expected output.

### 1. Scan PBOs Command

The `scan-pbos` command extracts files from PBO archives based on specified file extensions.

**Example:**
```bash
.\target\release\arma3_tool.exe scan-pbos --input-dir "C:\Program Files (x86)\Steam\steamapps\common\Arma 3\Addons" --cache-dir "./cache/a3_base" --extensions "hpp,cpp,sqf,sqm" --threads 4
```

**Parameters:**
- `--input-dir`: Directory containing PBO files to scan (recursive search)
- `--cache-dir`: Directory where extracted files will be stored
- `--extensions`: Comma-separated list of file extensions to extract
- `--threads`: Number of parallel extraction threads


### 2. Scan Classes Command

The `scan-classes` command analyzes class definitions in CPP/HPP files.

**Example:**
```bash
.\target\release\arma3_tool.exe scan-classes --input-dir "./cache/a3_base" --output-dir "./reports/a3_base_classes" --verbose-errors
```

**Parameters:**
- `--input-dir`: Directory containing extracted CPP/HPP files to analyze
- `--output-dir`: Directory where class reports will be stored
- `--max-files`: (Optional) Maximum number of files to process (useful for debugging)
- `--verbose-errors`: (Optional) Enable detailed error reporting for parse errors


### 3. Scan Missions Command

The `scan-missions` command extracts and analyzes mission files.

**Example:**
```bash
.\target\release\arma3_tool.exe scan-missions --input-dir "E:\pca_missions_quick" --cache-dir "./cache/missions" --output-dir "./reports/mission_reports" --threads 4
```

**Parameters:**
- `--input-dir`: Directory containing mission PBO files
- `--cache-dir`: Directory where extracted mission files will be stored
- `--output-dir`: Directory where mission reports will be stored
- `--threads`: Number of parallel extraction threads


### 4. Analyze Mission Dependencies Command

The `analyze-mission-dependencies` command cross-references mission equipment with available classes.

**Example:**
```bash
.\target\release\arma3_tool.exe analyze-mission-dependencies --mission-dir "E:\pca_missions_quick" --addon-dir "./cache" --cache-dir "./cache/analysis" --output-dir "./reports/analysis_reports" --threads 4
```

**Parameters:**
- `--mission-dir`: Directory containing mission PBO files
- `--addon-dir`: Directory containing extracted addon files (base game and mods)
- `--cache-dir`: Directory for temporary analysis files
- `--output-dir`: Directory where dependency reports will be stored
- `--threads`: Number of parallel analysis threads


### 5. Full Analysis Command

The `full-analysis` command runs the complete analysis pipeline in one go.

**Example:**
```bash
.\target\release\arma3_tool.exe full-analysis --arma3-dir "C:\Program Files (x86)\Steam\steamapps\common\Arma 3" --mods-dir "E:\pca" --missions-dir "E:\pca_missions_quick" --cache-dir "./cache" --output-dir "./reports" --threads 4
```

**Parameters:**
- `--arma3-dir`: Arma 3 base game directory
- `--mods-dir`: Directory containing mod PBO files
- `--missions-dir`: Directory containing mission PBO files
- `--cache-dir`: Directory for extracted files and temporary data
- `--output-dir`: Directory where all reports will be stored
- `--threads`: Number of parallel processing threads

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

## Report Format Examples

Below are examples of the actual report formats generated by the tool.

### Mission Report Example

Each mission report is stored as a JSON file with the following structure:

```json
{
  "mission_name": "pca_training_mission",
  "file_path": "E:\\pca_missions_quick\\pca_training_mission.pbo",
  "hash": "a1b2c3d4e5f6...",
  "timestamp": "2023-08-15T15:25:23.789Z",
  "equipment": {
    "weapons": [
      {
        "class_name": "arifle_MX_F",
        "count": 12,
        "locations": ["playableUnit", "unit1", "unit2"]
      },
      {
        "class_name": "hgun_P07_F",
        "count": 8,
        "locations": ["playableUnit", "unit3"]
      }
    ],
    "vehicles": [
      {
        "class_name": "B_MRAP_01_F",
        "count": 2,
        "locations": ["vehicle1", "vehicle2"]
      }
    ],
    "magazines": [
      {
        "class_name": "30Rnd_65x39_caseless_mag",
        "count": 36,
        "locations": ["playableUnit", "unit1", "unit2"]
      }
    ],
    "items": [
      {
        "class_name": "FirstAidKit",
        "count": 15,
        "locations": ["playableUnit", "unit1", "unit2", "unit3"]
      }
    ],
    "backpacks": [
      {
        "class_name": "B_AssaultPack_mcamo",
        "count": 8,
        "locations": ["playableUnit", "unit1", "unit2", "unit3"]
      }
    ]
  },
  "mission_info": {
    "author": "PCA Mission Team",
    "briefing_name": "PCA Training Mission",
    "min_players": 1,
    "max_players": 12,
    "game_type": "Coop"
  }
}
```

### Class Report Example

Class reports are stored as JSON files with the following structure:

```json
{
  "class_name": "arifle_MX_F",
  "file_path": "cache\\mods\\pca_weapons\\config.cpp",
  "parent_class": "Rifle_Base_F",
  "properties": {
    "scope": 2,
    "displayName": "MX 6.5 mm",
    "model": "\\A3\\Weapons_F\\Rifles\\MX\\MX_F.p3d",
    "picture": "\\A3\\Weapons_F\\Rifles\\MX\\Data\\UI\\gear_mx_rifle_X_CA.paa",
    "magazines": [
      "30Rnd_65x39_caseless_mag",
      "30Rnd_65x39_caseless_mag_Tracer"
    ],
    "modes": [
      "Single",
      "FullAuto"
    ]
  },
  "inheritance_chain": [
    "arifle_MX_F",
    "Rifle_Base_F",
    "Rifle",
    "WeaponSlotsInfo",
    "CfgWeapons"
  ],
  "referenced_classes": [
    "30Rnd_65x39_caseless_mag",
    "30Rnd_65x39_caseless_mag_Tracer"
  ]
}
```

### Dependency Analysis Report Example

Dependency analysis reports are stored as JSON files with the following structure:

```json
{
  "mission_name": "pca_training_mission",
  "file_path": "E:\\pca_missions_quick\\pca_training_mission.pbo",
  "timestamp": "2023-08-15T15:50:30.890Z",
  "dependency_status": {
    "total_classes_referenced": 45,
    "classes_found": 43,
    "classes_missing": 2,
    "dependency_coverage": 95.6
  },
  "missing_dependencies": [
    {
      "class_name": "pca_custom_weapon_F",
      "referenced_in": ["playableUnit", "unit1"],
      "possible_alternatives": [
        {
          "class_name": "arifle_MX_F",
          "similarity": 0.85,
          "source": "Arma 3 Base"
        }
      ]
    },
    {
      "class_name": "pca_custom_vest_F",
      "referenced_in": ["unit2"],
      "possible_alternatives": [
        {
          "class_name": "V_PlateCarrier1_rgr",
          "similarity": 0.78,
          "source": "Arma 3 Base"
        }
      ]
    }
  ],
  "dependency_sources": {
    "arma3_base": 28,
    "pca_weapons": 10,
    "pca_units": 5,
    "pca_vehicles": 2
  },
  "recommendations": [
    "Add dependency on pca_custom_equipment mod",
    "Consider replacing pca_custom_weapon_F with arifle_MX_F"
  ]
}
```

### Summary Report Example

The tool also generates summary reports that aggregate data across all missions:

```json
{
  "timestamp": "2023-08-15T17:30:20.901Z",
  "total_missions_analyzed": 45,
  "missions_with_complete_dependencies": 38,
  "missions_with_missing_dependencies": 7,
  "overall_dependency_coverage": 98.2,
  "most_common_missing_classes": [
    {
      "class_name": "pca_custom_weapon_F",
      "missing_count": 5,
      "affected_missions": ["mission1", "mission2", "mission3", "mission4", "mission5"]
    },
    {
      "class_name": "pca_custom_vest_F",
      "missing_count": 3,
      "affected_missions": ["mission2", "mission6", "mission7"]
    }
  ],
  "most_used_equipment": {
    "weapons": [
      {
        "class_name": "arifle_MX_F",
        "total_count": 156,
        "mission_count": 32
      }
    ],
    "vehicles": [
      {
        "class_name": "B_MRAP_01_F",
        "total_count": 87,
        "mission_count": 28
      }
    ]
  },
  "mod_dependency_distribution": {
    "arma3_base": 62.5,
    "pca_weapons": 18.3,
    "pca_units": 10.2,
    "pca_vehicles": 9.0
  }
}
```

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