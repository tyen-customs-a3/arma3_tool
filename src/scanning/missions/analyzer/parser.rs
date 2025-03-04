use std::path::{Path, PathBuf};
use std::collections::HashSet;
use anyhow::Result;
use log::{debug, info};

// Import directly from the crate
use ::sqf_parser::SqfClassParser;

use super::types::{MissionDependencyResult, ClassDependency, ReferenceType};
use crate::scanning::missions::extractor::types::MissionExtractionResult;

// Import the specialized parsers
use super::sqf_parser;
use super::sqm_parser;
use super::cpp_parser;

/// Analyze a single mission's dependencies
pub fn analyze_mission(
    cache_dir: &Path,
    sqf_parser: &SqfClassParser,
    extraction: &MissionExtractionResult
) -> Result<MissionDependencyResult> {
    debug!("Analyzing class dependencies in mission: {}", extraction.mission_name);
    
    let mut all_dependencies = Vec::new();
    let mut defined_classes = HashSet::new();
    let mut referenced_classes = HashSet::new();
    let mut parent_classes = HashSet::new();

    // First pass: collect all defined classes and parent class references
    // Analyze mission.sqm file if available using sqm_parser
    if let Some(sqm_file) = &extraction.sqm_file {
        let sqm_dependencies = sqm_parser::analyze_sqm_file(sqm_file)?;
        for dep in &sqm_dependencies {
            if dep.reference_type == ReferenceType::Definition {
                defined_classes.insert(dep.class_name.to_lowercase());
            } else if dep.reference_type == ReferenceType::Parent {
                parent_classes.insert(dep.class_name.clone());
                referenced_classes.insert(dep.class_name.clone());
            } else {
                referenced_classes.insert(dep.class_name.clone());
            }
        }
        all_dependencies.extend(sqm_dependencies);
    }
    
    // Analyze SQF files using sqf_parser
    for sqf_file in &extraction.sqf_files {
        let sqf_dependencies = sqf_parser::analyze_sqf_file(sqf_parser, sqf_file)?;
        for dep in &sqf_dependencies {
            if dep.reference_type == ReferenceType::Definition {
                defined_classes.insert(dep.class_name.to_lowercase());
            } else if dep.reference_type == ReferenceType::Parent {
                parent_classes.insert(dep.class_name.clone());
                referenced_classes.insert(dep.class_name.clone());
            } else {
                referenced_classes.insert(dep.class_name.clone());
            }
        }
        all_dependencies.extend(sqf_dependencies);
    }
    
    // Analyze CPP/HPP files using cpp_parser in parallel
    if !extraction.cpp_files.is_empty() {
        let cpp_dependencies = cpp_parser::analyze_cpp_files(&extraction.cpp_files)?;
        for dep in &cpp_dependencies {
            if dep.reference_type == ReferenceType::Definition {
                defined_classes.insert(dep.class_name.to_lowercase());
            } else if dep.reference_type == ReferenceType::Parent {
                parent_classes.insert(dep.class_name.clone());
                referenced_classes.insert(dep.class_name.clone());
            } else {
                referenced_classes.insert(dep.class_name.clone());
            }
        }
        all_dependencies.extend(cpp_dependencies);
    }
    
    // Special handling for function definitions in classes like "class common"
    // These often contain function definitions that shouldn't be considered dependencies
    let function_classes = ["common", "functions", "CfgFunctions"];
    for class_name in &function_classes {
        if defined_classes.contains(&class_name.to_lowercase()) {
            debug!("Ignoring function class: {}", class_name);
        }
    }
    
    // Common base classes that are often defined in missions and shouldn't be considered external dependencies
    let common_base_classes = [
        "baseMan", "Man", "CAManBase", "Civilian", "Soldier", 
        "SoldierWB", "SoldierEB", "SoldierGB", "CfgVehicles", "CfgWeapons", 
        "CfgMagazines", "CfgAmmo", "CfgPatches", "CfgWorlds"
    ];
    
    // Remove common base classes from referenced classes
    for class_name in &common_base_classes {
        referenced_classes.remove(*class_name);
    }
    
    // Remove classes that are defined in the mission from the referenced classes
    let external_dependencies: HashSet<String> = referenced_classes
        .difference(&defined_classes)
        .cloned()
        .collect();
    
    // Log some statistics about the analysis
    debug!("Mission '{}' analysis stats:", extraction.mission_name);
    debug!("  - Defined classes: {}", defined_classes.len());
    debug!("  - Referenced classes: {}", referenced_classes.len());
    debug!("  - Parent classes: {}", parent_classes.len());
    debug!("  - External dependencies: {}", external_dependencies.len());
    
    info!("Mission '{}' dependency analysis complete: {} total references, {} defined classes, {} external dependencies", 
          extraction.mission_name, all_dependencies.len(), defined_classes.len(), external_dependencies.len());
    
    Ok(MissionDependencyResult {
        mission_name: extraction.mission_name.clone(),
        pbo_path: extraction.pbo_path.clone(),
        class_dependencies: all_dependencies,
        unique_class_names: external_dependencies,
    })
} 