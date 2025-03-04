use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use log::{debug, warn};
use rayon::prelude::*;

use super::types::{ClassDependency, ReferenceType};

/// Analyze CPP/HPP files using cpp_parser with parallel processing
pub fn analyze_cpp_files(cpp_files: &[PathBuf]) -> Result<Vec<ClassDependency>> {
    debug!("Analyzing {} CPP files for class dependencies", cpp_files.len());
    
    let dependencies = cpp_files.par_iter()
        .map(|cpp_file| analyze_single_cpp_file(cpp_file))
        .filter_map(|result| match result {
            Ok(deps) => Some(deps),
            Err(e) => {
                warn!("Failed to analyze CPP file: {}. Skipping.", e);
                None
            }
        })
        .flatten()
        .collect();
    
    Ok(dependencies)
}

/// Analyze a single CPP/HPP file using the cpp_parser crate
pub fn analyze_single_cpp_file(cpp_file: &Path) -> Result<Vec<ClassDependency>> {
    debug!("Analyzing CPP file for class dependencies: {}", cpp_file.display());
    
    // Use the cpp_parser crate to parse the file
    let parsed_classes = cpp_parser::parse_cpp_file(cpp_file.to_str().unwrap())
        .context(format!("Failed to parse CPP file: {}", cpp_file.display()))?;
    
    let mut dependencies = Vec::new();
    
    // Process the parsed classes to extract dependencies
    process_parsed_classes(&parsed_classes, cpp_file, &mut dependencies, None);
    
    if !dependencies.is_empty() {
        debug!("Successfully extracted {} dependencies", dependencies.len());
        Ok(dependencies)
    } else {
        debug!("No dependencies found in {}", cpp_file.display());
        Ok(Vec::new())
    }
}

/// Process parsed classes recursively to extract dependencies
fn process_parsed_classes(
    classes: &[cpp_parser::Class],
    file_path: &Path,
    dependencies: &mut Vec<ClassDependency>,
    parent_context: Option<&str>
) {
    for class in classes {
        // Get the class name
        if let Some(class_name) = &class.name {
            let context = parent_context.map_or_else(
                || "Class definition".to_string(),
                |parent| format!("Nested class in {}", parent)
            );
            
            // // Add the class itself as a definition
            // dependencies.push(ClassDependency {
            //     class_name: class_name.clone(),
            //     source_file: file_path.to_path_buf(),
            //     line_number: 0,
            //     context: context.clone(),
            //     reference_type: ReferenceType::Definition,
            // });
            
            // // Check for parent class (inheritance)
            // if let Some(parent_class) = &class.parent {
            //     dependencies.push(ClassDependency {
            //         class_name: parent_class.clone(),
            //         source_file: file_path.to_path_buf(),
            //         line_number: 0,
            //         context: format!("Parent class of {}", class_name),
            //         reference_type: ReferenceType::Parent,
            //     });
            // }
            
            // Process properties for equipment references
            process_properties(&class.properties, class_name, file_path, dependencies, 0);
            
            // Process nested classes
            for (nested_name, nested_class) in &class.classes {
                // Create a new Class with the correct name for recursive processing
                let mut nested_class_copy = nested_class.clone();
                nested_class_copy.name = Some(nested_name.clone());
                process_parsed_classes(&[nested_class_copy], file_path, dependencies, Some(class_name));
            }
        }
    }
}

/// Process properties to find equipment references
fn process_properties(
    properties: &std::collections::HashMap<String, cpp_parser::Value>,
    class_name: &str,
    file_path: &Path,
    dependencies: &mut Vec<ClassDependency>,
    line_number: usize
) {
    // Define equipment-related property names to look for
    let equipment_properties = [
        "vehicle", "weapon", "magazine", "item", "uniform", "vest", "backpack",
        "headgear", "goggles", "nvgoggles", "binoculars", "map", "gps", "radio",
        "compass", "watch", "primaryWeapon", "secondaryWeapon", "handgunWeapon"
    ];

    // Trace log the prop name
    println!("Processing properties for class: {}", class_name);
    
    for prop_name in &equipment_properties {
        if let Some(value) = properties.get(*prop_name) {
            println!("Processing property: {}", prop_name);
            match value {
                cpp_parser::Value::String(val) => {
                    if !val.is_empty() {
                        println!("Adding dependency for class: {}", val);
                        dependencies.push(ClassDependency {
                            class_name: val.clone(),
                            source_file: file_path.to_path_buf(),
                            line_number,
                            context: format!("Property: {} in {}", prop_name, class_name),
                            reference_type: ReferenceType::Component,
                        });
                    }
                },
                cpp_parser::Value::Array(items) => {
                    for (i, item) in items.iter().enumerate() {
                        if let cpp_parser::Value::String(val) = item {
                            if !val.is_empty() {
                                println!("Adding dependency for class: {}", val);
                                dependencies.push(ClassDependency {
                                    class_name: val.clone(),
                                    source_file: file_path.to_path_buf(),
                                    line_number,
                                    context: format!("Property: {}[{}] in {}", prop_name, i, class_name),
                                    reference_type: ReferenceType::Component,
                                });
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }
}

/// This function is kept for backward compatibility but now uses the cpp_parser crate internally
pub fn analyze_file_with_pattern_matching(file_path: &Path) -> Result<Vec<ClassDependency>> {
    analyze_single_cpp_file(file_path)
}
