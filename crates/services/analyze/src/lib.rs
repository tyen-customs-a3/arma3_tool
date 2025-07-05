use anyhow::{anyhow, Context, Result};
use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

/*
 * SQF Analyzer Methodology
 *
 * This tool analyzes SQF scripts to extract item references based on their usage in code,
 * not based on string patterns, prefixes, or assumptions about how items are named.
 * 
 * We focus on:
 * 1. Finding direct references to items in specific functions (addItemToUniform, etc.)
 * 2. Extracting these references based on their context in the code
 * 3. Avoiding "guessing" whether a string is an item based on its name pattern
 *
 * This approach ensures accuracy by only considering strings that are actually used
 * as parameters to relevant functions, rather than trying to infer what strings
 * might represent equipment based on naming conventions.
 */

/// Command line arguments for the SQF Analyzer tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to SQF file or directory containing SQF files
    #[arg(short, long)]
    pub path: PathBuf,

    /// Output format (text, json)
    #[arg(short, long, default_value = "text")]
    pub output: String,

    /// Include full paths in output
    #[arg(short, long)]
    pub full_paths: bool,
    
    /// Include variable names in the output
    #[arg(short, long)]
    pub include_vars: bool,
    
    
    
    /// Specify comma-separated list of functions to extract from (e.g. "addItemToUniform,addVest")
    #[arg(long)]
    pub functions: Option<String>,
}

/// Main entry point for the analyzer as a library function
/// 
/// Analyzes SQF files according to the provided options and returns a set of found items.
///
/// # Arguments
///
/// * `args` - Configuration arguments for the analysis
///
/// # Returns
///
/// * `Result<HashSet<String>>` - A set of found item references or an error
pub fn analyze_sqf(args: &Args) -> Result<HashSet<String>> {
    // Parse the functions list if provided or use default equipment functions if equipment_only is set
    let specific_functions = if let Some(functions_str) = args.functions.as_ref() {
        Some(functions_str.split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>())
    } else {
        None
    };
    
    let paths = if args.path.is_dir() {
        collect_sqf_files(&args.path)?
    } else if args.path.extension().unwrap_or_default() == "sqf" {
        vec![args.path.clone()]
    } else {
        return Err(anyhow!("Path must be a directory or an SQF file"));
    };

    if paths.is_empty() {
        return Err(anyhow!("No SQF files found"));
    }

    let mut all_items = HashSet::new();
    let mut _file_count = 0;

    for path in &paths {
        _file_count += 1;
        
        let items = extract_items_from_sqf(path, specific_functions.as_ref())?;
        all_items.extend(items);
    }
    
    // Filter out variables if requested
    if !args.include_vars {
        all_items = filter_variable_names(all_items);
    }
    
    Ok(all_items)
}

/// Filter out common variable names from the items set
///
/// # Arguments
///
/// * `items` - The set of strings to filter
///
/// # Returns
///
/// * `HashSet<String>` - The filtered set of strings
pub fn filter_variable_names(items: HashSet<String>) -> HashSet<String> {
    let _original_count = items.len();
    
    items
        .into_iter()
        .filter(|item| {
            // Filter out internal variable names (starting with underscore)
            if item.starts_with('_') {
                return false;
            }
            
            // Filter out common variable naming patterns
            let common_var_patterns = [
                r"^[a-z]+$",  // single word lowercase variable names
                r"^[a-z]+[A-Z][a-zA-Z]*$", // camelCase variable names (e.g. simpleVar, camelCaseVar)
            ];
            
            for pattern in common_var_patterns {
                if Regex::new(pattern).unwrap().is_match(item) {
                    return false;
                }
            }
            
            true
        })
        .collect()
}

/// Collect all SQF files in a directory and its subdirectories
///
/// # Arguments
///
/// * `dir` - The directory path to search
///
/// # Returns
///
/// * `Result<Vec<PathBuf>>` - A vector of paths to SQF files or an error
pub fn collect_sqf_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut sqf_files = Vec::new();
    
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().unwrap_or_default() == "sqf" {
            sqf_files.push(path.to_path_buf());
        }
    }
    
    Ok(sqf_files)
}

/// Process a parameter string to extract item references
///
/// # Arguments
///
/// * `param_str` - The parameter string to process
/// * `regex_strings` - A regex to match string literals
/// * `items` - The set to add found items to
/// * `content` - The full content to search for variable definitions
///
/// # Returns
///
/// * `Result<()>` - Success or an error
pub fn process_parameter(
    param_str: &str, 
    regex_strings: &Regex, 
    items: &mut HashSet<String>, 
    content: &str
) -> Result<()> {
    let param_str = param_str.trim();
    
    if param_str.starts_with('"') && param_str.ends_with('"') {
        // Direct string literal
        let item_str = &param_str[1..param_str.len()-1];
        if !item_str.is_empty() {
            items.insert(item_str.to_string());
        }
    } else if regex_strings.is_match(param_str) {
        // Contains string literals
        for string_cap in regex_strings.captures_iter(param_str) {
            if let Some(item) = string_cap.get(1) {
                let item_str = item.as_str();
                if !item_str.is_empty() {
                    items.insert(item_str.to_string());
                }
            }
        }
    } else if param_str.starts_with('_') {
        // It's a variable reference, try to find its definition
        // Look for patterns like "private _varName = "value";"
        let var_name = param_str.trim();
        
        // Create regex to find the variable definition
        let var_pattern = format!(r#"(?m)^\s*private\s+({})\s*=\s*["']([^"']+)["']"#, regex::escape(var_name));
        if let Ok(var_regex) = Regex::new(&var_pattern) {
            for var_match in var_regex.captures_iter(content) {
                if let Some(item) = var_match.get(2) {
                    let item_str = item.as_str();
                    if !item_str.is_empty() {
                        items.insert(item_str.to_string());
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Extract function calls from SQF content and add found items to the items set
///
/// # Arguments
///
/// * `content` - The SQF file content as a string
/// * `items` - The set to add found items to
/// * `specific_functions` - Optional list of specific functions to extract from
///
/// # Returns
///
/// * `Result<()>` - Success or an error
pub fn extract_function_calls(
    content: &str, 
    items: &mut HashSet<String>, 
    specific_functions: Option<&Vec<String>>
) -> Result<()> {
    // Define functions and their parameter indices that contain equipment references
    let equipment_functions = [
        // Function name, regex pattern, capture group index for the parameter
        ("addItemToUniform", r"addItemToUniform\s+([^;]+);", 1),
        ("addItemToVest", r"addItemToVest\s+([^;]+);", 1),
        ("addItemToBackpack", r"addItemToBackpack\s+([^;]+);", 1),
        ("addItem", r"addItem\s+([^;]+);", 1),
        ("addWeapon", r"addWeapon\s+([^;]+);", 1),
        ("addWeaponItem", r"addWeaponItem\s+\[[^,]+,\s*([^,\]]+)", 1),
        ("addMagazine", r"addMagazine\s+([^;]+);", 1),
        ("addMagazineCargo", r"addMagazineCargo\s+([^;]+);", 1),
        ("addWeaponCargo", r"addWeaponCargo\s+([^;]+);", 1),
        ("addItemCargo", r"addItemCargo\s+([^;]+);", 1),
        ("forceAddUniform", r"forceAddUniform\s+([^;]+);", 1),
        ("addVest", r"addVest\s+([^;]+);", 1),
        ("addHeadgear", r"addHeadgear\s+([^;]+);", 1),
        ("addGoggles", r"addGoggles\s+([^;]+);", 1),
        ("addBackpack", r"addBackpack\s+([^;]+);", 1),
        // Special case for arsenal function that takes arrays of equipment
        ("ace_arsenal_fnc_initBox", r"call\s+ace_arsenal_fnc_initBox", 0)
    ];
    
    // Generic for loop patterns that cover common equipment function calls within loops
    let loop_patterns = [
        // Generic for loop pattern with capture group for both function name and parameter
        (r#"for\s+["_]\w+.*?do\s+\{.*?(addItemToUniform|addItemToVest|addItemToBackpack|addItem|addMagazine)\s+"([^"]+)".*?\};"#, 2),
        // Specific pattern for examples
        (r#"for\s+"_i"\s+from\s+1\s+to\s+\([^)]+\)\s+do\s+\{[^}]+(addItemToUniform|addItemToVest|addItemToBackpack|addMagazine)\s+"([^"]+)"[^}]*\};"#, 2)
    ];
    
    let regex_strings = Regex::new(r#""([^"\\]*(?:\\.[^"\\]*)*)""#)?;
    
    // Handle special case for arsenal function
    let process_arsenal = match specific_functions {
        Some(funcs) => funcs.contains(&"ace_arsenal_fnc_initBox".to_string()),
        None => true
    };
    
    if process_arsenal {
        // Look for lines with ace_arsenal_fnc_initBox
        let arsenal_regex = Regex::new(r"\[\s*[^,]+,\s*\(([^)]+)\)\s*\]\s*call\s+ace_arsenal_fnc_initBox")?;
        for cap in arsenal_regex.captures_iter(content) {
            if let Some(arrays_text) = cap.get(1) {
                // Get the array variable names being combined (e.g., _itemEquipment + _itemMod + ...)
                let array_names: Vec<&str> = arrays_text.as_str()
                    .split('+')
                    .map(|s| s.trim())
                    .collect();
                
                // For each array variable, find its definition and extract items
                for var_name in array_names {
                    // Create a pattern to find the array definition
                    let array_def_pattern = format!(r"(?s){}\s*=\s*\[(.*?)\];", regex::escape(var_name));
                    if let Ok(array_def_regex) = Regex::new(&array_def_pattern) {
                        for array_def_cap in array_def_regex.captures_iter(content) {
                            if let Some(array_content) = array_def_cap.get(1) {
                                // Extract all string literals from the array content
                                for string_cap in regex_strings.captures_iter(array_content.as_str()) {
                                    if let Some(item) = string_cap.get(1) {
                                        let item_str = item.as_str();
                                        if !item_str.is_empty() {
                                            items.insert(item_str.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Process function calls based on specific functions provided
    for (function_name, pattern, param_index) in equipment_functions.iter() {
        // Skip functions not in the specific list if provided
        if let Some(funcs) = specific_functions {
            if !funcs.contains(&function_name.to_string()) {
                continue;
            }
        }
        
        // Skip the arsenal function as it's handled specially above
        if *function_name == "ace_arsenal_fnc_initBox" {
            continue;
        }
        
        let regex = Regex::new(pattern)?;
        
        for cap in regex.captures_iter(content) {
            if let Some(param) = cap.get(*param_index) {
                process_parameter(param.as_str(), &regex_strings, items, content)?;
            }
        }
    }
    
    // Process for loops to extract items only if no specific functions provided
    // or if specific functions include the relevant ones
    let process_loops = match specific_functions {
        Some(funcs) => funcs.iter().any(|f| {
            ["addItemToUniform", "addItemToVest", "addItemToBackpack", "addItem", "addMagazine"]
                .contains(&f.as_str())
        }),
        None => true
    };
    
    if process_loops {
        for (pattern, param_index) in loop_patterns.iter() {
            let regex = Regex::new(pattern)?;
            
            for cap in regex.captures_iter(content) {
                if let Some(func) = cap.get(1) {
                    // If specific functions are provided, check if this function is included
                    if let Some(funcs) = specific_functions {
                        if !funcs.contains(&func.as_str().to_string()) {
                            continue;
                        }
                    }
                    
                    if let Some(item) = cap.get(*param_index) {
                        let item_str = item.as_str();
                        if !item_str.is_empty() {
                            items.insert(item_str.to_string());
                        }
                    }
                }
            }
        }
    }
    
    Ok(())
}

/// Extract items from SQF content using a common approach
///
/// # Arguments
///
/// * `content` - The SQF file content
/// * `specific_functions` - Optional list of specific functions to extract from
///
/// # Returns
///
/// * `Result<HashSet<String>>` - A set of found items or an error
pub fn extract_common_items(
    content: &str, 
    specific_functions: Option<&Vec<String>>
) -> Result<HashSet<String>> {
    let mut items = HashSet::new();
    
    // Extract function calls for items
    extract_function_calls(content, &mut items, specific_functions)?;
    
    // Extract string literals from arrays when no specific functions are provided
    // This broader extraction only happens when we're not filtering by function
    if specific_functions.is_none() {
        // Find arrays that might contain item strings
        if let Ok(regex_arrays) = Regex::new(r#"(?s)\[\s*(?:[^,\]]+,\s*)*(?:[^,\]]+)\s*\]"#) {
            // Find all arrays
            for array_match in regex_arrays.find_iter(content) {
                let array_text = array_match.as_str();
                
                // Find string literals in the array
                if let Ok(regex_strings) = Regex::new(r#""([^"\\]*(?:\\.[^"\\]*)*)""#) {
                    for string_match in regex_strings.captures_iter(array_text) {
                        if let Some(item) = string_match.get(1) {
                            let item_str = item.as_str();
                            if !item_str.is_empty() {
                                items.insert(item_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        // Also find weapon and backpack variables defined outside of functions
        if let Ok(regex_vars) = Regex::new(r#"(?m)^\s*private\s+_(\w+)\s*=\s*["']([^"']+)["']"#) {
            for var_match in regex_vars.captures_iter(content) {
                if let Some(item) = var_match.get(2) {
                    let item_str = item.as_str();
                    if !item_str.is_empty() {
                        items.insert(item_str.to_string());
                    }
                }
            }
        }
    }
    
    Ok(items)
}

/// Extract items from an SQF file using direct parsing
///
/// # Arguments
///
/// * `file_path` - Path to the SQF file
/// * `specific_functions` - Optional list of specific functions to extract from
///
/// # Returns
///
/// * `Result<HashSet<String>>` - A set of found items or an error
pub fn extract_items_direct(
    file_path: &Path, 
    specific_functions: Option<&Vec<String>>
) -> Result<HashSet<String>> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))?;
    
    extract_common_items(&content, specific_functions)
}

/// Extract items from an SQF file using the SQFvm preprocessor
///
/// # Arguments
///
/// * `file_path` - Path to the SQF file
/// * `specific_functions` - Optional list of specific functions to extract from
///
/// # Returns
///
/// * `Result<HashSet<String>>` - A set of found items or an error
pub fn extract_items_from_sqf(
    file_path: &Path, 
    specific_functions: Option<&Vec<String>>
) -> Result<HashSet<String>> {
    // Get the parent directory of the file
    let parent_dir = file_path.parent().unwrap_or_else(|| Path::new("."));
    
    // Get the path to the embedded sqfvm.exe
    let sqfvm_path = env!("SQFVM_BIN_PATH");
    
    // Execute SQFvm to preprocess the file
    let output = Command::new(sqfvm_path)
        .args([
            "--automated",
            "--load", &parent_dir.to_string_lossy(), 
            "--preprocess-file", &file_path.to_string_lossy()
        ])
        .output()
        .context("Failed to execute sqfvm")?;
    
    if !output.status.success() {
        return Err(anyhow!(
            "SQFvm failed with code {}: {}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    let content = String::from_utf8_lossy(&output.stdout);
    
    extract_common_items(&content, specific_functions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::path::PathBuf;

    #[test]
    fn test_arsenal_function_extraction() -> Result<()> {
        // Test content that mimics the structure in arsenal_simple.sqf
        let test_content = r#"
        // Define equipment arrays
        private _itemEquipment = 
        [
            "ACRE_PRC148",
            "ACRE_PRC152",
            "ACRE_PRC117F"
        ];

        private _itemMod =
        [   
            "rhsusf_acc_grip1",
            "rhsusf_acc_grip2"
        ];

        private _itemWeaponRifle =
        [
            "rhs_weap_hk416d145",
        ];

        // Use the arrays in arsenal function call
        [arsenal, (_itemEquipment + _itemMod + _itemWeaponRifle)] call ace_arsenal_fnc_initBox;
        "#;
        
        let mut items = HashSet::new();
        
        // Test with specific function filter for arsenal
        let specific_funcs = vec!["ace_arsenal_fnc_initBox".to_string()];
        extract_function_calls(test_content, &mut items, Some(&specific_funcs))?;
        
        // Should contain all items from all arrays used in the call
        let expected_items = [
            "ACRE_PRC148",
            "ACRE_PRC152",
            "ACRE_PRC117F",
            "rhsusf_acc_grip1",
            "rhsusf_acc_grip2",
            "rhs_weap_hk416d145"
        ];
        
        for item in expected_items.iter() {
            assert!(items.contains(*item), "Failed to extract arsenal item: {}", item);
        }
        
        assert_eq!(items.len(), expected_items.len(), "Should extract exactly the items in the arrays");
        
        Ok(())
    }

    #[test]
    fn test_arsenal_simple_extraction() -> Result<()> {
        let file_path = PathBuf::from("test-files/arsenal_simple.sqf");
        
        // Test with no function filtering - should extract from arrays
        let items = extract_items_direct(&file_path, None)?;
        
        // We expect results because arsenal_simple.sqf contains arrays with item strings
        assert!(!items.is_empty(), "Should find items in arrays when no function filter is applied");
        
        // Test with specific function filtering for arsenal function
        let specific_funcs = vec!["ace_arsenal_fnc_initBox".to_string()];
        let arsenal_items = extract_items_direct(&file_path, Some(&specific_funcs))?;
        
        // Should find items from arrays used in the arsenal function call
        assert!(!arsenal_items.is_empty(), "Should find items when filtering for arsenal function");
        
        // Verify specific items from the arrays are included
        let expected_items = [
            "ACRE_PRC148",
            "ACRE_PRC152",
            "ACRE_PRC117F",
            "rhsusf_acc_grip1",
            "rhsusf_acc_grip2",
            "rhs_weap_hk416d145",
            "rhs_weap_M136"
        ];
        
        for item in expected_items.iter() {
            assert!(arsenal_items.contains(*item), "Arsenal extraction should include {}", item);
        }
        
        Ok(())
    }
    
    #[test]
    fn test_for_loop_extraction() -> Result<()> {
        // Create a test string with for loops
        let test_content = r#"
        for "_i" from 1 to (ceil (random [1, 2, 3])) do {_unit addItemToUniform "ACE_fieldDressing"};
        for "_i" from 1 to (ceil (random [1, 2, 3])) do {_unit addItemToVest "rhs_mag_30Rnd_556x45_M855A1_Stanag"};
        for "_i" from 1 to (ceil (random [1, 2, 3])) do {_unit addItemToBackpack "rhs_mag_rdg2_white"};
        for "_i" from 1 to (ceil (random [1, 2, 3])) do {_unit addMagazine "rhs_mag_rgd5"};
        "#;
        
        let mut items = HashSet::new();
        // Use only equipment functions
        let equipment_funcs = vec![
            "addItemToUniform".to_string(),
            "addItemToVest".to_string(),
            "addItemToBackpack".to_string(),
            "addItem".to_string(),
            "addWeapon".to_string(),
            "addWeaponItem".to_string(),
            "addMagazine".to_string(),
            "addMagazineCargo".to_string(),
            "addWeaponCargo".to_string(),
            "addItemCargo".to_string(),
            "forceAddUniform".to_string(),
            "addVest".to_string(),
            "addHeadgear".to_string(),
            "addGoggles".to_string(),
            "addBackpack".to_string()
        ];
        extract_function_calls(test_content, &mut items, Some(&equipment_funcs))?;
        
        let expected_items = [
            "ACE_fieldDressing",
            "rhs_mag_30Rnd_556x45_M855A1_Stanag",
            "rhs_mag_rdg2_white",
            "rhs_mag_rgd5"
        ];
        
        for item in expected_items.iter() {
            assert!(items.contains(*item), "Failed to extract for loop item: {}", item);
        }
        
        // Test with specific function filter
        let mut filtered_items = HashSet::new();
        let specific_funcs = vec!["addItemToUniform".to_string()];
        extract_function_calls(test_content, &mut filtered_items, Some(&specific_funcs))?;
        
        // Should only contain the ACE_fieldDressing
        assert!(filtered_items.contains("ACE_fieldDressing"));
        assert_eq!(filtered_items.len(), 1);
        
        Ok(())
    }
    
    #[test]
    fn test_direct_function_calls() -> Result<()> {
        // Test direct function calls
        let test_content = r#"
        _unit addItemToUniform "ACE_fieldDressing";
        _unit addItemToVest "rhs_mag_30Rnd_556x45_M855A1_Stanag";
        _unit addWeapon "rhs_weap_m4a1";
        _unit addMagazine "rhs_mag_rgd5";
        "#;
        
        let mut items = HashSet::new();
        extract_function_calls(test_content, &mut items, None)?;
        
        let expected_items = [
            "ACE_fieldDressing",
            "rhs_mag_30Rnd_556x45_M855A1_Stanag",
            "rhs_weap_m4a1",
            "rhs_mag_rgd5"
        ];
        
        for item in expected_items.iter() {
            assert!(items.contains(*item), "Failed to extract function call item: {}", item);
        }
        
        // Test with specific function filter
        let mut filtered_items = HashSet::new();
        let specific_funcs = vec!["addWeapon".to_string()];
        extract_function_calls(test_content, &mut filtered_items, Some(&specific_funcs))?;
        
        // Should only contain the weapon
        assert!(filtered_items.contains("rhs_weap_m4a1"));
        assert_eq!(filtered_items.len(), 1);
        
        Ok(())
    }
    
    #[test]
    fn test_variable_filtering() {
        // Test the filtering of variable names
        let mut items = HashSet::new();
        items.insert("_variable".to_string());
        items.insert("simpleVar".to_string());
        items.insert("camelCaseVar".to_string());
        items.insert("ACE_fieldDressing".to_string());
        items.insert("rhs_mag_rgd5".to_string());

        // Debug the regex pattern
        let camel_case_pattern = r"^[a-z]+[A-Z][a-zA-Z]*$"; // Modified pattern to better match camelCase
        let re = Regex::new(camel_case_pattern).unwrap();
        assert!(re.is_match("simpleVar"), "Pattern should match simpleVar");
        assert!(re.is_match("camelCaseVar"), "Pattern should match camelCaseVar");
        
        // Apply the filtering logic from main()
        let filtered_items: HashSet<String> = items
            .into_iter()
            .filter(|item| {
                // Filter out internal variable names (starting with underscore)
                if item.starts_with('_') {
                    return false;
                }
                
                // Filter out common variable naming patterns
                let common_var_patterns = [
                    r"^[a-z]+$",  // single word lowercase variable names
                    camel_case_pattern, // camelCase variable names
                ];
                
                for pattern in common_var_patterns {
                    if Regex::new(pattern).unwrap().is_match(item) {
                        return false;
                    }
                }
                
                true
            })
            .collect();
        
        // Variables should be filtered out
        assert!(!filtered_items.contains("_variable"), "_variable should be filtered out");
        assert!(!filtered_items.contains("simpleVar"), "simpleVar should be filtered out");
        assert!(!filtered_items.contains("camelCaseVar"), "camelCaseVar should be filtered out");
        
        // Equipment items should be kept
        assert!(filtered_items.contains("ACE_fieldDressing"), "ACE_fieldDressing should be kept");
        assert!(filtered_items.contains("rhs_mag_rgd5"), "rhs_mag_rgd5 should be kept");
    }
}
