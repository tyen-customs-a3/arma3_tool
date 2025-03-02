use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::fs;
use anyhow::{Result, Context};
use log::{debug, info, warn, error};
use rayon::prelude::*;
use cpp_parser::{parse_cpp, Class, Value};
use serde::Serialize;

use super::scanner::MissionExtractionResult;

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

/// Result of mission class dependency analysis
#[derive(Debug, Serialize)]
pub struct MissionDependencyResult {
    pub mission_name: String,
    pub pbo_path: PathBuf,
    /// All class dependencies found in the mission
    pub class_dependencies: Vec<ClassDependency>,
    /// Consolidated list of unique class names the mission depends on
    pub unique_class_names: HashSet<String>,
}

pub struct DependencyAnalyzer<'a> {
    cache_dir: &'a Path,
}

impl<'a> DependencyAnalyzer<'a> {
    pub fn new(cache_dir: &'a Path) -> Self {
        Self {
            cache_dir,
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
        
        // First, incorporate any class references found during extraction
        for class_name in &extraction.direct_class_references {
            referenced_classes.insert(class_name.clone());
        }
        
        // First pass: collect all defined classes and parent class references
        // Analyze mission.sqm file if available
        if let Some(sqm_file) = &extraction.sqm_file {
            let sqm_dependencies = self.analyze_sqm_file(sqm_file)?;
            for dep in &sqm_dependencies {
                if dep.reference_type == ReferenceType::Definition {
                    defined_classes.insert(dep.class_name.clone());
                } else if dep.reference_type == ReferenceType::Parent {
                    parent_classes.insert(dep.class_name.clone());
                    referenced_classes.insert(dep.class_name.clone());
                } else {
                    referenced_classes.insert(dep.class_name.clone());
                }
            }
            all_dependencies.extend(sqm_dependencies);
        }
        
        // Analyze SQF files
        for sqf_file in &extraction.sqf_files {
            let sqf_dependencies = self.analyze_sqf_file(sqf_file)?;
            for dep in &sqf_dependencies {
                if dep.reference_type == ReferenceType::Definition {
                    defined_classes.insert(dep.class_name.clone());
                } else if dep.reference_type == ReferenceType::Parent {
                    parent_classes.insert(dep.class_name.clone());
                    referenced_classes.insert(dep.class_name.clone());
                } else {
                    referenced_classes.insert(dep.class_name.clone());
                }
            }
            all_dependencies.extend(sqf_dependencies);
        }
        
        // Analyze CPP/HPP files
        for cpp_file in &extraction.cpp_files {
            let cpp_dependencies = self.analyze_cpp_file(cpp_file)?;
            for dep in &cpp_dependencies {
                if dep.reference_type == ReferenceType::Definition {
                    defined_classes.insert(dep.class_name.clone());
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
            if defined_classes.contains(*class_name) {
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
    
    /// Analyze SQM file to extract class dependencies
    fn analyze_sqm_file(&self, sqm_file: &Path) -> Result<Vec<ClassDependency>> {
        debug!("Analyzing SQM file for class dependencies: {}", sqm_file.display());
        
        let content = fs::read_to_string(sqm_file)
            .context(format!("Failed to read SQM file: {}", sqm_file.display()))?;
        
        let mut dependencies = Vec::new();
        
        // Try to parse as cpp file (some SQM files use CPP syntax)
        match parse_cpp(&content) {
            Ok(classes) => {
                self.extract_dependencies_from_cpp_classes(&classes, sqm_file, &mut dependencies);
            },
            Err(_) => {
                // Fall back to regex-based extraction
                self.extract_dependencies_with_regex(&content, sqm_file, &mut dependencies);
            }
        }
        
        Ok(dependencies)
    }
    
    /// Analyze SQF file to extract class dependencies
    fn analyze_sqf_file(&self, sqf_file: &Path) -> Result<Vec<ClassDependency>> {
        debug!("Analyzing SQF file for class dependencies: {}", sqf_file.display());
        
        let content = fs::read_to_string(sqf_file)
            .context(format!("Failed to read SQF file: {}", sqf_file.display()))?;
        
        let mut dependencies = Vec::new();
        self.extract_sqf_dependencies(&content, sqf_file, &mut dependencies);
        
        Ok(dependencies)
    }
    
    /// Analyze CPP/HPP file to extract class dependencies
    fn analyze_cpp_file(&self, cpp_file: &Path) -> Result<Vec<ClassDependency>> {
        debug!("Analyzing CPP/HPP file for class dependencies: {}", cpp_file.display());
        
        let mut dependencies = Vec::new();
        
        // Try parsing with cpp_parser
        match cpp_parser::parser::parse_cpp_file(cpp_file.to_str().unwrap_or_default()) {
            Ok(classes) => {
                // Successfully parsed, extract dependencies from the class structure
                self.extract_dependencies_from_cpp_classes(&classes, cpp_file, &mut dependencies);
                debug!("Successfully parsed and extracted {} dependencies from {}", 
                      dependencies.len(), cpp_file.display());
            },
            Err(e) => {
                // Fall back to regex extraction if parsing fails
                warn!("Failed to parse CPP file with parser: {}. Falling back to regex extraction.", e);
                let content = fs::read_to_string(cpp_file)
                    .context(format!("Failed to read CPP file: {}", cpp_file.display()))?;
                
                self.extract_dependencies_with_regex(&content, cpp_file, &mut dependencies);
            }
        }
        
        Ok(dependencies)
    }
    
    /// Extract dependencies from cpp_parser's parsed class structure
    fn extract_dependencies_from_cpp_classes(
        &self,
        classes: &[Class],
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
            dependencies.push(ClassDependency {
                class_name: class.name.clone(),
                source_file: file_path.to_owned(),
                line_number: 0, // cpp_parser doesn't provide line numbers currently
                context: format!("class definition: {}", class.name),
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
                    Value::String(s) => {
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
                    Value::Array(items) => {
                        for (idx, item) in items.iter().enumerate() {
                            if let Value::String(s) = item {
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
    
    /// Extract dependencies from SQF file content using regex patterns
    fn extract_sqf_dependencies(
        &self,
        content: &str,
        file_path: &Path,
        dependencies: &mut Vec<ClassDependency>
    ) {
        // Define regex patterns for common SQF functions that reference class names
        let command_patterns = [
            (r#"createVehicle\s*\[\s*"([^"]+)""#, "createVehicle"),
            (r#"createVehicle\s*"([^"]+)""#, "createVehicle direct"),
            (r#"addWeapon\s*"([^"]+)""#, "addWeapon"),
            (r#"addMagazine\s*"([^"]+)""#, "addMagazine"),
            (r#"addItem\s*"([^"]+)""#, "addItem"),
            (r#"addBackpack\s*"([^"]+)""#, "addBackpack"),
            (r#"vestContainer\s*"([^"]+)""#, "vestContainer"),
            (r#"uniformContainer\s*"([^"]+)""#, "uniformContainer"),
            (r#"typeOf\s*"([^"]+)""#, "typeOf"),
            (r#"isKindOf\s*"([^"]+)""#, "isKindOf"),
        ];
        
        // Process each line to extract class references
        for (line_num, line) in content.lines().enumerate() {
            let line_num = line_num + 1; // 1-indexed line numbers
            let line = line.trim();
            
            // Skip comments
            if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
                continue;
            }
            
            // Apply command patterns
            for (pattern, context) in &command_patterns {
                if let Ok(regex) = regex::Regex::new(pattern) {
                    for cap in regex.captures_iter(line) {
                        if let Some(class_name) = cap.get(1) {
                            let name = class_name.as_str().trim();
                            // Only add if it looks like a class name
                            if self.looks_like_classname(name) {
                                dependencies.push(ClassDependency {
                                    class_name: name.to_string(),
                                    source_file: file_path.to_owned(),
                                    line_number: line_num,
                                    context: format!("{}: {}", context, line),
                                    reference_type: ReferenceType::Direct,
                                });
                            }
                        }
                    }
                }
            }
            
            // Look for array literals with potential class names
            if line.contains('[') && line.contains(']') && line.contains('"') {
                // Process array literals
                let mut in_quotes = false;
                let mut current_item = String::new();
                let mut quote_start_pos = 0;
                
                for (pos, c) in line.chars().enumerate() {
                    match c {
                        '"' => {
                            if !in_quotes {
                                in_quotes = true;
                                current_item.clear();
                            } else {
                                in_quotes = false;
                                if self.looks_like_classname(&current_item) {
                                    dependencies.push(ClassDependency {
                                        class_name: current_item.clone(),
                                        source_file: file_path.to_owned(),
                                        line_number: line_num,
                                        context: format!("array item: {}", line),
                                        reference_type: ReferenceType::Direct,
                                    });
                                }
                            }
                        },
                        _ if in_quotes => {
                            current_item.push(c);
                        },
                        _ => {}
                    }
                }
            }
            
            // Look for variable assignments with class names
            if line.contains('=') && line.contains('"') {
                if let Ok(regex) = regex::Regex::new(r#"=\s*"([^"]+)""#) {
                    for cap in regex.captures_iter(line) {
                        if let Some(class_name) = cap.get(1) {
                            let name = class_name.as_str().trim();
                            if self.looks_like_classname(name) {
                                dependencies.push(ClassDependency {
                                    class_name: name.to_string(),
                                    source_file: file_path.to_owned(),
                                    line_number: line_num,
                                    context: format!("variable assignment: {}", line),
                                    reference_type: ReferenceType::Direct,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Extract dependencies using regex patterns for files that couldn't be parsed
    fn extract_dependencies_with_regex(
        &self,
        content: &str,
        file_path: &Path,
        dependencies: &mut Vec<ClassDependency>
    ) {
        // Common patterns for class references in various file formats
        let patterns = [
            (r#"vehicle\s*=\s*"([^"]+)""#, "vehicle property"),
            (r#"type\s*=\s*"([^"]+)""#, "type property"),
            (r#"weapon\s*=\s*"([^"]+)""#, "weapon property"),
            (r#"magazine\s*=\s*"([^"]+)""#, "magazine property"),
            (r#"backpack\s*=\s*"([^"]+)""#, "backpack property"),
            (r#"uniform\s*=\s*"([^"]+)""#, "uniform property"),
            (r#"vest\s*=\s*"([^"]+)""#, "vest property"),
            (r#"headgear\s*=\s*"([^"]+)""#, "headgear property"),
            (r#"goggles\s*=\s*"([^"]+)""#, "goggles property"),
            (r#"class\s+(\w+)\s*:\s*(\w+)"#, "class inheritance"),
        ];
        
        // Process each line to extract class references
        for (line_num, line) in content.lines().enumerate() {
            let line_num = line_num + 1; // 1-indexed line numbers
            let line = line.trim();
            
            // Skip comments
            if line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
                continue;
            }
            
            // Apply each pattern
            for (pattern, context) in &patterns {
                if let Ok(regex) = regex::Regex::new(pattern) {
                    for cap in regex.captures_iter(line) {
                        // First capture group is usually the class name
                        if let Some(class_name) = cap.get(1) {
                            let name = class_name.as_str().trim();
                            if self.looks_like_classname(name) {
                                dependencies.push(ClassDependency {
                                    class_name: name.to_string(),
                                    source_file: file_path.to_owned(),
                                    line_number: line_num,
                                    context: format!("{}: {}", context, line),
                                    reference_type: ReferenceType::Direct,
                                });
                            }
                        }
                        
                        // Second capture group might be a parent class in inheritance patterns
                        if let Some(parent_name) = cap.get(2) {
                            let name = parent_name.as_str().trim();
                            if self.looks_like_classname(name) {
                                dependencies.push(ClassDependency {
                                    class_name: name.to_string(),
                                    source_file: file_path.to_owned(),
                                    line_number: line_num,
                                    context: format!("parent class: {}", line),
                                    reference_type: ReferenceType::Parent,
                                });
                            }
                        }
                    }
                }
            }
            
            // Special patterns for class definitions
            if let Ok(regex) = regex::Regex::new(r"class\s+(\w+)") {
                for cap in regex.captures_iter(line) {
                    if let Some(class_name) = cap.get(1) {
                        let name = class_name.as_str().trim();
                        // Don't include very common base class names that might be false positives
                        if !["Items", "Weapons", "Vehicles", "Men", "All"].contains(&name) {
                            dependencies.push(ClassDependency {
                                class_name: name.to_string(),
                                source_file: file_path.to_owned(),
                                line_number: line_num,
                                context: format!("class definition: {}", line),
                                reference_type: ReferenceType::Definition,
                            });
                        }
                    }
                }
            }
            
            // Look for array literals with potential class names
            if line.contains('[') && line.contains(']') && line.contains('"') {
                let mut in_quotes = false;
                let mut current_item = String::new();
                
                for c in line.chars() {
                    match c {
                        '"' => {
                            in_quotes = !in_quotes;
                            if !in_quotes && !current_item.is_empty() {
                                if self.looks_like_classname(&current_item) {
                                    dependencies.push(ClassDependency {
                                        class_name: current_item.clone(),
                                        source_file: file_path.to_owned(),
                                        line_number: line_num,
                                        context: format!("array item: {}", line),
                                        reference_type: ReferenceType::Component,
                                    });
                                }
                                current_item.clear();
                            }
                        },
                        _ if in_quotes => {
                            current_item.push(c);
                        },
                        _ => {}
                    }
                }
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