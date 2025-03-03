use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::fs;
use anyhow::{Result, Context};
use log::{debug, info, warn, error};
use rayon::prelude::*;

// Parser imports - use the specific parsers for each file type
use cpp_parser::{parse_cpp, parser::parse_cpp_file, Class as CppClass, Value as CppValue};
use sqm_parser::parser::{parse_sqm, Class as SqmClass, Value as SqmValue};
use sqf_parser::{SqfClassParser, Equipment, equipment::EquipmentType};

use serde::Serialize;

use super::scanner::MissionExtractionResult;
use crate::reporting::mission::{MissionName, MissionEquipment, MissionDependencies, MissionEquipmentReport, MissionDependenciesReport, EquipmentItemReport};

/// Represents a discovered class dependency in the mission files
#[derive(Debug, Clone, Serialize)]
pub struct ClassDependency {
    pub class_name: String,
    pub source_file: PathBuf,
    pub line_number: usize,
    pub context: String,
    pub reference_type: ReferenceType,
}

/// Enum representing how a class was referenced
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum ReferenceType {
    /// Class is directly referenced (e.g., in createVehicle)
    Direct,
    /// Class is defined in mission file
    Definition,
    /// Class is a parent class referenced in an inheritance relationship
    Parent,
    /// Class is referenced as a component/property
    Component,
}

/// Result of mission dependency analysis
#[derive(Debug, Clone, Serialize)]
pub struct MissionDependencyResult {
    pub mission_name: String,
    pub pbo_path: PathBuf,
    /// All class dependencies found in the mission
    pub class_dependencies: Vec<ClassDependency>,
    /// Consolidated list of unique class names the mission depends on
    pub unique_class_names: HashSet<String>,
}

// Implement the required traits for MissionDependencyResult

impl MissionName for MissionDependencyResult {
    fn mission_name(&self) -> String {
        self.mission_name.clone()
    }
}

impl MissionEquipment for MissionDependencyResult {
    fn get_equipment(&self) -> MissionEquipmentReport {
        let equipment_items: Vec<EquipmentItemReport> = self.class_dependencies.iter()
            .map(|dep| EquipmentItemReport {
                class_name: dep.class_name.clone(),
                source_file: dep.source_file.to_string_lossy().to_string(),
                line_number: dep.line_number,
                context: dep.context.clone(),
            })
            .collect();

        MissionEquipmentReport {
            total_items: equipment_items.len(),
            classes: equipment_items,
        }
    }
}

impl MissionDependencies for MissionDependencyResult {
    fn get_dependencies(&self) -> MissionDependenciesReport {
        // For this implementation, we'll consider all unique class names as "missing"
        // since we don't have information about which ones are actually missing
        // This can be refined later if needed
        let missing_classes = self.unique_class_names.iter()
            .map(|name| name.clone())
            .collect::<Vec<String>>();

        MissionDependenciesReport {
            total_missing_classes: missing_classes.len(),
            missing_classes,
        }
    }
}

pub struct DependencyAnalyzer<'a> {
    cache_dir: &'a Path,
    sqf_parser: SqfClassParser, // Add a reusable SQF parser instance
}

impl<'a> DependencyAnalyzer<'a> {
    pub fn new(cache_dir: &'a Path) -> Self {
        Self {
            cache_dir,
            sqf_parser: SqfClassParser::new(), // Initialize the SQF parser
        }
    }
    
    /// Main entry point for analyzing missions
    pub fn analyze_missions(
        &self,
        extraction_results: &[MissionExtractionResult],
    ) -> Result<Vec<MissionDependencyResult>> {
        info!("Analyzing class dependencies in {} missions", extraction_results.len());
        
        let results: Vec<MissionDependencyResult> = extraction_results.par_iter()
            .map(|result| self.analyze_single_mission(result))
            .filter_map(|r| match r {
                Ok(result) => Some(result),
                Err(e) => {
                    warn!("Failed to analyze mission: {}", e);
                    None
                }
            })
            .collect();
        
        info!("Completed class dependency analysis of {} missions", results.len());
        
        Ok(results)
    }
    
    /// Analyze a single mission's class dependencies
    fn analyze_single_mission(&self, extraction: &MissionExtractionResult) -> Result<MissionDependencyResult> {
        debug!("Analyzing class dependencies in mission: {}", extraction.mission_name);
        
        let mut all_dependencies = Vec::new();
        let mut defined_classes = HashSet::new();
        let mut referenced_classes = HashSet::new();
        let mut parent_classes = HashSet::new();

        // First pass: collect all defined classes and parent class references
        // Analyze mission.sqm file if available using sqm_parser
        if let Some(sqm_file) = &extraction.sqm_file {
            let sqm_dependencies = self.analyze_sqm_file(sqm_file)?;
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
            let sqf_dependencies = self.analyze_sqf_file(sqf_file)?;
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
        
        // Analyze CPP/HPP files using cpp_parser
        for cpp_file in &extraction.cpp_files {
            let cpp_dependencies = self.analyze_cpp_file(cpp_file)?;
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
    
    /// Analyze SQM file to extract class dependencies using sqm_parser
    fn analyze_sqm_file(&self, sqm_file: &Path) -> Result<Vec<ClassDependency>> {
        debug!("Analyzing SQM file for class dependencies: {}", sqm_file.display());
        
        let mut dependencies = Vec::new();
        
        // Parse SQM file using sqm_parser
        let content = fs::read_to_string(sqm_file)
            .context(format!("Failed to read SQM file: {}", sqm_file.display()))?;
        
        match parse_sqm(&content) {
            Ok(classes) => {
                self.extract_dependencies_from_sqm_classes(&classes, sqm_file, &mut dependencies);
                debug!("Successfully parsed and extracted {} dependencies from SQM file", 
                      dependencies.len());
            },
            Err(e) => {
                warn!("Failed to parse SQM file: {}. Trying to read as text.", e);
                
                // Fallback: try to parse the file as a CPP file
                match parse_cpp(&content) {
                    Ok(classes) => {
                        self.extract_dependencies_from_cpp_classes(&classes, sqm_file, &mut dependencies);
                        debug!("Successfully parsed SQM as CPP and extracted {} dependencies", 
                              dependencies.len());
                    },
                    Err(e) => {
                        warn!("Failed to parse SQM file as CPP: {}. No dependencies extracted.", e);
                    }
                }
            }
        }
        
        Ok(dependencies)
    }
    
    /// Analyze SQF file to extract class dependencies using sqf_parser
    fn analyze_sqf_file(&self, sqf_file: &Path) -> Result<Vec<ClassDependency>> {
        debug!("Analyzing SQF file for class dependencies: {}", sqf_file.display());
        
        let mut dependencies = Vec::new();
        
        // Parse SQF file using sqf_parser
        match self.sqf_parser.parse_file(sqf_file) {
            Ok(equipment) => {
                // Extract equipment items as class dependencies
                self.extract_dependencies_from_sqf_equipment(&equipment, sqf_file, &mut dependencies);
                debug!("Successfully parsed and extracted {} dependencies from SQF file", 
                      dependencies.len());
            },
            Err(e) => {
                warn!("Failed to parse SQF file: {}. No dependencies extracted.", e);
            }
        }
        
        Ok(dependencies)
    }
    
    /// Analyze CPP/HPP file to extract class dependencies using cpp_parser
    fn analyze_cpp_file(&self, cpp_file: &Path) -> Result<Vec<ClassDependency>> {
        debug!("Analyzing CPP/HPP file for class dependencies: {}", cpp_file.display());
        
        let mut dependencies = Vec::new();
        
        // Parse with cpp_parser
        match parse_cpp_file(cpp_file.to_str().unwrap_or_default()) {
            Ok(classes) => {
                // Successfully parsed, extract dependencies from the class structure
                self.extract_dependencies_from_cpp_classes(&classes, cpp_file, &mut dependencies);
                debug!("Successfully parsed and extracted {} dependencies from {}", 
                      dependencies.len(), cpp_file.display());
            },
            Err(e) => {
                // Log error but don't fall back to regex
                warn!("Failed to parse CPP file with parser: {}. No dependencies extracted.", e);
            }
        }
        
        Ok(dependencies)
    }
    
    /// Extract dependencies from sqm_parser's parsed class structure
    fn extract_dependencies_from_sqm_classes(
        &self,
        classes: &[SqmClass],
        file_path: &Path,
        dependencies: &mut Vec<ClassDependency>
    ) {
        for class in classes {
            // Add the class itself as a definition
            let mut context = format!("class definition: {}", class.name);
            
            // Check for a 'name' property in the class
            if let Some(name_prop) = class.properties.get("name") {
                if let Some(name_value) = name_prop.value.as_string() {
                    // Add the name property to the context
                    context = format!("{} with name = \"{}\"", context, name_value);
                    
                    // Also add the name property as a direct reference
                    dependencies.push(ClassDependency {
                        class_name: name_value.to_string(),
                        source_file: file_path.to_owned(),
                        line_number: 0, // sqm_parser doesn't provide line numbers
                        context: format!("name property in class {}: {}", class.name, name_value),
                        reference_type: ReferenceType::Direct,
                    });
                }
            }
            
            dependencies.push(ClassDependency {
                class_name: class.name.clone(),
                source_file: file_path.to_owned(),
                line_number: 0, // sqm_parser doesn't provide line numbers
                context,
                reference_type: ReferenceType::Definition,
            });
            
            // Add parent class as a dependency if it exists
            if let Some(parent) = &class.parent {
                dependencies.push(ClassDependency {
                    class_name: parent.clone(),
                    source_file: file_path.to_owned(),
                    line_number: 0,
                    context: format!("parent class of {}", class.name),
                    reference_type: ReferenceType::Parent,
                });
            }
            
            // Process properties for class references
            for (key, property) in &class.properties {
                match &property.value {
                    SqmValue::String(s) => {
                        // Check if the string looks like a class reference
                        if self.looks_like_classname(s) {
                            dependencies.push(ClassDependency {
                                class_name: s.clone(),
                                source_file: file_path.to_owned(),
                                line_number: 0,
                                context: format!("property '{}' in class '{}'", key, class.name),
                                reference_type: ReferenceType::Component,
                            });
                        }
                    },
                    SqmValue::Array(items) => {
                        for (idx, item) in items.iter().enumerate() {
                            if let SqmValue::String(s) = item {
                                if self.looks_like_classname(s) {
                                    dependencies.push(ClassDependency {
                                        class_name: s.clone(),
                                        source_file: file_path.to_owned(),
                                        line_number: 0,
                                        context: format!("property '{}[{}]' in class '{}'", key, idx, class.name),
                                        reference_type: ReferenceType::Component,
                                    });
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
            
            // Recursively process nested classes
            if !class.nested_classes.is_empty() {
                self.extract_dependencies_from_sqm_classes(&class.nested_classes, file_path, dependencies);
            }
        }
    }
    
    /// Extract dependencies from sqf_parser's equipment results
    fn extract_dependencies_from_sqf_equipment(
        &self,
        equipment: &Equipment,
        file_path: &Path,
        dependencies: &mut Vec<ClassDependency>
    ) {
        // Process all equipment items based on their type
        for item in &equipment.items {
            let context = match item.equipment_type {
                EquipmentType::Uniform => "uniform reference in SQF",
                EquipmentType::Vest => "vest reference in SQF",
                EquipmentType::Headgear => "headgear reference in SQF",
                EquipmentType::Weapon => "weapon reference in SQF",
                EquipmentType::Magazine => "magazine reference in SQF",
                EquipmentType::Item => "item reference in SQF",
                EquipmentType::Backpack => "backpack reference in SQF",
                EquipmentType::Goggles => "goggles reference in SQF",
                EquipmentType::Other(ref s) => s,
            };
            
            dependencies.push(ClassDependency {
                class_name: item.class_name.clone(),
                source_file: file_path.to_owned(),
                line_number: item.line_number,
                context: context.to_string(),
                reference_type: ReferenceType::Direct,
            });
        }
    }
    
    /// Extract dependencies from cpp_parser's parsed class structure
    fn extract_dependencies_from_cpp_classes(
        &self,
        classes: &[CppClass],
        file_path: &Path,
        dependencies: &mut Vec<ClassDependency>
    ) {
        // Common function-related class names that shouldn't be considered as external dependencies
        let function_related_classes = ["common", "functions", "CfgFunctions"];
        
        for class in classes {
            // Skip function-related classes
            if function_related_classes.contains(&class.name.as_str()) {
                debug!("Skipping function-related class: {}", class.name);
                
                // Still add it as a definition so we know it's defined in the mission
                dependencies.push(ClassDependency {
                    class_name: class.name.clone(),
                    source_file: file_path.to_owned(),
                    line_number: 0,
                    context: format!("function class definition: {}", class.name),
                    reference_type: ReferenceType::Definition,
                });
                
                // Process nested classes recursively
                if !class.nested_classes.is_empty() {
                    self.extract_dependencies_from_cpp_classes(&class.nested_classes, file_path, dependencies);
                }
                
                continue;
            }
            
            // Check if this is a function definition class
            let is_function_class = class.properties.iter()
                .any(|(key, _)| key == "file" || key == "tag" || key == "preInit" || key == "postInit");
            
            if is_function_class {
                debug!("Detected function class: {}", class.name);
                
                // Add as definition but don't process properties as dependencies
                dependencies.push(ClassDependency {
                    class_name: class.name.clone(),
                    source_file: file_path.to_owned(),
                    line_number: 0,
                    context: format!("function class definition: {}", class.name),
                    reference_type: ReferenceType::Definition,
                });
                
                // Process nested classes recursively
                if !class.nested_classes.is_empty() {
                    self.extract_dependencies_from_cpp_classes(&class.nested_classes, file_path, dependencies);
                }
                
                continue;
            }
            
            // Add the class itself as a definition
            let mut context = format!("class definition: {}", class.name);
            
            // Check for a 'name' property in the class
            if let Some(name_prop) = class.properties.get("name") {
                if let CppValue::String(name_value) = &name_prop.value {
                    // Add the name property to the context
                    context = format!("{} with name = \"{}\"", context, name_value);
                    
                    // Also add the name property as a direct reference
                    if self.looks_like_classname(name_value) {
                        dependencies.push(ClassDependency {
                            class_name: name_value.clone(),
                            source_file: file_path.to_owned(),
                            line_number: 0, // cpp_parser doesn't provide line numbers currently
                            context: format!("name property in class {}: {}", class.name, name_value),
                            reference_type: ReferenceType::Direct,
                        });
                    }
                }
            }
            
            dependencies.push(ClassDependency {
                class_name: class.name.clone(),
                source_file: file_path.to_owned(),
                line_number: 0, // cpp_parser doesn't provide line numbers currently
                context,
                reference_type: ReferenceType::Definition,
            });
            
            // Add parent class as a dependency if it exists
            if let Some(parent) = &class.parent {
                dependencies.push(ClassDependency {
                    class_name: parent.clone(),
                    source_file: file_path.to_owned(),
                    line_number: 0,
                    context: format!("parent class of {}", class.name),
                    reference_type: ReferenceType::Parent,
                });
            }
            
            // Process properties for class references
            for (key, prop) in &class.properties {
                match &prop.value {
                    CppValue::String(s) => {
                        // Check if the string looks like a class reference
                        if self.looks_like_classname(s) {
                            dependencies.push(ClassDependency {
                                class_name: s.clone(),
                                source_file: file_path.to_owned(),
                                line_number: 0,
                                context: format!("property '{}' in class '{}'", key, class.name),
                                reference_type: ReferenceType::Component,
                            });
                        }
                    },
                    CppValue::Array(items) => {
                        for (idx, item) in items.iter().enumerate() {
                            if let CppValue::String(s) = item {
                                if self.looks_like_classname(s) {
                                    dependencies.push(ClassDependency {
                                        class_name: s.clone(),
                                        source_file: file_path.to_owned(),
                                        line_number: 0,
                                        context: format!("property '{}[{}]' in class '{}'", key, idx, class.name),
                                        reference_type: ReferenceType::Component,
                                    });
                                }
                            }
                        }
                    },
                    _ => {}
                }
            }
            
            // Recursively process nested classes
            if !class.nested_classes.is_empty() {
                self.extract_dependencies_from_cpp_classes(&class.nested_classes, file_path, dependencies);
            }
        }
    }
    
    /// Check if a string looks like an Arma class name
    fn looks_like_classname(&self, name: &str) -> bool {
        // Common characteristics of Arma class names:
        // - Contains underscore or number
        // - Not empty
        // - No spaces
        // - Not a common variable name or special keyword
        
        if name.is_empty() || name.contains(' ') {
            return false;
        }
        
        // Skip function names (often have {} or ())
        if name.contains('{') || name.contains('}') || name.contains('(') || name.contains(')') {
            return false;
        }
        
        // Skip file paths
        if name.contains('/') || name.contains('\\') || name.contains(".sqf") || name.contains(".hpp") {
            return false;
        }
        
        // Common Arma class names often contain underscores or numbers
        let contains_identifier_chars = name.contains('_') || name.chars().any(|c| c.is_ascii_digit());
        
        // Skip common keywords
        let common_keywords = [
            "true", "false", "nil", "null", 
            "this", "player", "west", "east", "independent", "civilian",
            "blufor", "opfor", "indfor", "civ",
            "west", "east", "resistance",
            "false", "true", "yes", "no",
            // Common function names
            "drawObjectMapMarker", "findBuildings", "findOverwatch", 
            "nearbyBuildings", "removeEventHandlers", "file", "tag",
            // Common class properties that aren't class names
            "displayName", "scope", "author", "category", "side", "faction",
            "vehicleClass", "editorSubcategory", "cost", "threat", "type",
            "simulation", "model", "icon", "picture", "accuracy", "camouflage",
            "audible", "sensitivity", "hiddenSelections", "hiddenSelectionsTextures"
        ];
        
        let is_not_keyword = !common_keywords.contains(&name.to_lowercase().as_str());
        
        // For very short names, be more strict
        if name.len() <= 2 {
            return false;
        }
        
        // For names that look like function definitions, return false
        if name.ends_with("{}") || name.contains("()") {
            return false;
        }
        
        contains_identifier_chars && is_not_keyword
    }
}

// Implement Debug for ReferenceType
impl std::fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReferenceType::Direct => write!(f, "Direct"),
            ReferenceType::Definition => write!(f, "Definition"),
            ReferenceType::Parent => write!(f, "Parent"),
            ReferenceType::Component => write!(f, "Component"),
        }
    }
}