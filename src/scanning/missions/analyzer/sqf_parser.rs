use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use log::{debug, warn};

// Import directly from the crate
use ::sqf_parser::SqfClassParser;
use ::sqf_parser::Equipment;
use ::sqf_parser::equipment::{EquipmentItem, EquipmentType};

use super::types::{ClassDependency, ReferenceType};

/// Analyze SQF file to extract class dependencies using sqf_parser
pub fn analyze_sqf_file(sqf_parser: &SqfClassParser, sqf_file: &Path) -> Result<Vec<ClassDependency>> {
    debug!("Analyzing SQF file for class dependencies: {}", sqf_file.display());
    
    let mut dependencies = Vec::new();
    
    // Parse SQF file using sqf_parser
    match sqf_parser.parse_file(sqf_file) {
        Ok(equipment) => {
            // Extract equipment items as class dependencies
            extract_dependencies_from_sqf_equipment(&equipment, sqf_file, &mut dependencies);
            debug!("Successfully parsed and extracted {} dependencies from SQF file", 
                  dependencies.len());
        },
        Err(e) => {
            warn!("Failed to parse SQF file: {}. No dependencies extracted.", e);
        }
    }
    
    Ok(dependencies)
}

/// Extract dependencies from SQF equipment items
pub fn extract_dependencies_from_sqf_equipment(
    equipment: &Equipment,
    file_path: &Path,
    dependencies: &mut Vec<ClassDependency>
) {
    // Process each equipment item
    for item in &equipment.items {
        // Create a dependency based on the equipment type
        let context = match item.equipment_type {
            EquipmentType::Weapon => format!("Weapon: {}", item.class_name),
            EquipmentType::Magazine => format!("Magazine: {}", item.class_name),
            EquipmentType::Item => format!("Item: {}", item.class_name),
            EquipmentType::Uniform => format!("Uniform: {}", item.class_name),
            EquipmentType::Vest => format!("Vest: {}", item.class_name),
            EquipmentType::Headgear => format!("Headgear: {}", item.class_name),
            EquipmentType::Backpack => format!("Backpack: {}", item.class_name),
            EquipmentType::Goggles => format!("Goggles: {}", item.class_name),
            EquipmentType::Other(ref type_name) => format!("{}: {}", type_name, item.class_name),
        };
        
        dependencies.push(ClassDependency {
            class_name: item.class_name.clone(),
            source_file: file_path.to_path_buf(),
            line_number: item.line_number,
            context,
            reference_type: ReferenceType::Direct,
        });
    }
}

/// Check if a string looks like a valid class name
pub fn looks_like_classname(name: &str) -> bool {
    // Class names should be alphanumeric with underscores and no spaces
    !name.contains(' ') && 
    !name.is_empty() && 
    name.chars().all(|c| c.is_alphanumeric() || c == '_')
} 