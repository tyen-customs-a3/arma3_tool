use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use log::{debug, warn};

// Import directly from the crate
use ::sqm_parser::Class as SqmClass;
use ::sqm_parser::parse_sqm;

use super::types::{ClassDependency, ReferenceType};

/// Analyze SQM file to extract class dependencies using sqm_parser
pub fn analyze_sqm_file(sqm_file: &Path) -> Result<Vec<ClassDependency>> {
    debug!("Analyzing SQM file for class dependencies: {}", sqm_file.display());
    
    let mut dependencies = Vec::new();
    
    // Parse SQM file using sqm_parser
    let content = fs::read_to_string(sqm_file)
        .context(format!("Failed to read SQM file: {}", sqm_file.display()))?;
    
    match parse_sqm(&content) {
        Ok(classes) => {
            extract_dependencies_from_sqm_classes(&classes, sqm_file, &mut dependencies);
            debug!("Successfully parsed and extracted {} dependencies from SQM file", 
                  dependencies.len());
        },
        Err(e) => {
            warn!("Failed to parse SQM file: {}. Trying to read as text.", e);
        }
    }
    
    Ok(dependencies)
}

/// Extract dependencies from SQM classes
pub fn extract_dependencies_from_sqm_classes(
    classes: &[SqmClass],
    file_path: &Path,
    dependencies: &mut Vec<ClassDependency>
) {
    for class in classes {
        // Add the class itself as a dependency
        if !class.name.is_empty() {
            dependencies.push(ClassDependency {
                class_name: class.name.clone(),
                source_file: file_path.to_path_buf(),
                line_number: 0, // SqmClass doesn't have line numbers
                context: format!("Class definition: {}", class.name),
                reference_type: ReferenceType::Definition,
            });
        }
        
        // Process class properties for vehicle types
        if let Some(vehicle_type) = class.properties.get("vehicle") {
            if !vehicle_type.name.is_empty() {
                dependencies.push(ClassDependency {
                    class_name: vehicle_type.name.clone(),
                    source_file: file_path.to_path_buf(),
                    line_number: 0, // SqmClass doesn't have line numbers
                    context: format!("Vehicle property in class {}", class.name),
                    reference_type: ReferenceType::Direct,
                });
            }
        }
        
        // Process parent class if present
        if let Some(parent) = &class.parent {
            if !parent.is_empty() {
                dependencies.push(ClassDependency {
                    class_name: parent.clone(),
                    source_file: file_path.to_path_buf(),
                    line_number: 0, // SqmClass doesn't have line numbers
                    context: format!("Parent class of {}", class.name),
                    reference_type: ReferenceType::Parent,
                });
            }
        }
        
        // Recursively process nested classes
        if !class.nested_classes.is_empty() {
            extract_dependencies_from_sqm_classes(&class.nested_classes, file_path, dependencies);
        }
    }
} 