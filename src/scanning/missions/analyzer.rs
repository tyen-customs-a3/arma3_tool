use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::fs;
use anyhow::{Result, Context};
use log::{debug, info, warn};
use serde::{Serialize, Deserialize};
use rayon::prelude::*;

use super::scanner::MissionExtractionResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EquipmentItem {
    pub class_name: String,
    pub source_file: PathBuf,
    pub line_number: usize,
    pub context: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MissionAnalysisResult {
    pub mission_name: String,
    pub pbo_path: PathBuf,
    pub equipment: Vec<EquipmentItem>,
    pub vehicles: Vec<EquipmentItem>,
    pub weapons: Vec<EquipmentItem>,
    pub magazines: Vec<EquipmentItem>,
    pub items: Vec<EquipmentItem>,
    pub backpacks: Vec<EquipmentItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyAnalysisResult {
    pub mission_name: String,
    pub missing_classes: Vec<String>,
    pub available_classes: Vec<String>,
    pub total_equipment_count: usize,
}

pub struct EquipmentAnalyzer<'a> {
    cache_dir: &'a Path,
}

impl<'a> EquipmentAnalyzer<'a> {
    pub fn new(cache_dir: &'a Path) -> Self {
        Self {
            cache_dir,
        }
    }
    
    pub fn analyze_missions(
        &self,
        extraction_results: &[MissionExtractionResult],
    ) -> Result<Vec<MissionAnalysisResult>> {
        info!("Analyzing equipment in {} missions", extraction_results.len());
        
        let results: Vec<MissionAnalysisResult> = extraction_results.par_iter()
            .map(|result| self.analyze_single_mission(result))
            .filter_map(|r| match r {
                Ok(result) => Some(result),
                Err(e) => {
                    warn!("Failed to analyze mission: {}", e);
                    None
                }
            })
            .collect();
        
        info!("Completed analysis of {} missions", results.len());
        
        Ok(results)
    }
    
    fn analyze_single_mission(&self, extraction: &MissionExtractionResult) -> Result<MissionAnalysisResult> {
        debug!("Analyzing mission: {}", extraction.mission_name);
        
        let mut equipment = Vec::new();
        let mut vehicles = Vec::new();
        let mut weapons = Vec::new();
        let mut magazines = Vec::new();
        let mut items = Vec::new();
        let mut backpacks = Vec::new();
        
        // Analyze mission.sqm file if available
        if let Some(sqm_file) = &extraction.sqm_file {
            self.analyze_sqm_file(
                sqm_file,
                &mut equipment,
                &mut vehicles,
                &mut weapons,
                &mut magazines,
                &mut items,
                &mut backpacks,
            )?;
        }
        
        // Analyze SQF files
        for sqf_file in &extraction.sqf_files {
            self.analyze_sqf_file(
                sqf_file,
                &mut equipment,
                &mut vehicles,
                &mut weapons,
                &mut magazines,
                &mut items,
                &mut backpacks,
            )?;
        }
        
        info!("Mission '{}' analysis complete: {} equipment items found", 
              extraction.mission_name, 
              equipment.len() + vehicles.len() + weapons.len() + magazines.len() + items.len() + backpacks.len());
        
        Ok(MissionAnalysisResult {
            mission_name: extraction.mission_name.clone(),
            pbo_path: extraction.pbo_path.clone(),
            equipment,
            vehicles,
            weapons,
            magazines,
            items,
            backpacks,
        })
    }
    
    fn analyze_sqm_file(
        &self,
        sqm_file: &Path,
        equipment: &mut Vec<EquipmentItem>,
        vehicles: &mut Vec<EquipmentItem>,
        weapons: &mut Vec<EquipmentItem>,
        magazines: &mut Vec<EquipmentItem>,
        items: &mut Vec<EquipmentItem>,
        backpacks: &mut Vec<EquipmentItem>,
    ) -> Result<()> {
        debug!("Analyzing SQM file: {}", sqm_file.display());
        
        let content = fs::read_to_string(sqm_file)
            .context(format!("Failed to read SQM file: {}", sqm_file.display()))?;
        
        // Parse SQM content for equipment classes
        // This is a simplified implementation - in a real parser we'd use a proper SQM parser
        
        // Look for vehicle classes
        self.extract_classes_from_content(
            &content,
            sqm_file,
            vehicles,
            &["vehicle", "Car", "Tank", "Air", "Ship"],
            "vehicle",
        );
        
        // Look for weapon classes
        self.extract_classes_from_content(
            &content,
            sqm_file,
            weapons,
            &["weapon", "Rifle", "Pistol", "Launcher"],
            "weapon",
        );
        
        // Look for magazine classes
        self.extract_classes_from_content(
            &content,
            sqm_file,
            magazines,
            &["magazine"],
            "magazine",
        );
        
        // Look for item classes
        self.extract_classes_from_content(
            &content,
            sqm_file,
            items,
            &["item", "Binocular", "NVGoggles", "ItemMap", "ItemCompass", "ItemWatch", "ItemRadio"],
            "item",
        );
        
        // Look for backpack classes
        self.extract_classes_from_content(
            &content,
            sqm_file,
            backpacks,
            &["backpack", "Bag"],
            "backpack",
        );
        
        // Combine all equipment
        equipment.extend(vehicles.iter().cloned());
        equipment.extend(weapons.iter().cloned());
        equipment.extend(magazines.iter().cloned());
        equipment.extend(items.iter().cloned());
        equipment.extend(backpacks.iter().cloned());
        
        Ok(())
    }
    
    fn analyze_sqf_file(
        &self,
        sqf_file: &Path,
        equipment: &mut Vec<EquipmentItem>,
        vehicles: &mut Vec<EquipmentItem>,
        weapons: &mut Vec<EquipmentItem>,
        magazines: &mut Vec<EquipmentItem>,
        items: &mut Vec<EquipmentItem>,
        backpacks: &mut Vec<EquipmentItem>,
    ) -> Result<()> {
        debug!("Analyzing SQF file: {}", sqf_file.display());
        
        let content = fs::read_to_string(sqf_file)
            .context(format!("Failed to read SQF file: {}", sqf_file.display()))?;
        
        // Look for common SQF commands that reference equipment
        
        // Vehicle creation: createVehicle, createVehicleLocal, etc.
        self.extract_sqf_commands(
            &content,
            sqf_file,
            vehicles,
            &["createVehicle", "createVehicleLocal", "spawnVehicle"],
            "vehicle",
        );
        
        // Weapon handling: addWeapon, removeWeapon, etc.
        self.extract_sqf_commands(
            &content,
            sqf_file,
            weapons,
            &["addWeapon", "removeWeapon", "primaryWeapon", "secondaryWeapon", "handgunWeapon"],
            "weapon",
        );
        
        // Magazine handling: addMagazine, removeMagazine, etc.
        self.extract_sqf_commands(
            &content,
            sqf_file,
            magazines,
            &["addMagazine", "removeMagazine", "magazines"],
            "magazine",
        );
        
        // Item handling: addItem, removeItem, etc.
        self.extract_sqf_commands(
            &content,
            sqf_file,
            items,
            &["addItem", "removeItem", "linkedItems", "assignedItems"],
            "item",
        );
        
        // Backpack handling: addBackpack, removeBackpack, etc.
        self.extract_sqf_commands(
            &content,
            sqf_file,
            backpacks,
            &["addBackpack", "removeBackpack", "backpack"],
            "backpack",
        );
        
        // Combine all equipment
        equipment.extend(vehicles.iter().cloned());
        equipment.extend(weapons.iter().cloned());
        equipment.extend(magazines.iter().cloned());
        equipment.extend(items.iter().cloned());
        equipment.extend(backpacks.iter().cloned());
        
        Ok(())
    }
    
    fn extract_classes_from_content(
        &self,
        content: &str,
        file_path: &Path,
        target: &mut Vec<EquipmentItem>,
        class_indicators: &[&str],
        category: &str,
    ) {
        // Simple regex-based extraction
        // In a real implementation, we'd use a proper parser for SQM files
        
        for (line_num, line) in content.lines().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check if line contains class indicators
            if class_indicators.iter().any(|&indicator| line_lower.contains(indicator)) {
                // Extract class name - this is a simplified approach
                if let Some(class_name) = self.extract_class_name(line) {
                    target.push(EquipmentItem {
                        class_name: class_name.to_string(),
                        source_file: file_path.to_owned(),
                        line_number: line_num + 1,
                        context: format!("{}: {}", category, line.trim()),
                    });
                }
            }
        }
    }
    
    fn extract_sqf_commands(
        &self,
        content: &str,
        file_path: &Path,
        target: &mut Vec<EquipmentItem>,
        commands: &[&str],
        category: &str,
    ) {
        for (line_num, line) in content.lines().enumerate() {
            let line_lower = line.to_lowercase();
            
            // Check if line contains any of the commands
            if commands.iter().any(|&cmd| line_lower.contains(cmd)) {
                // Extract class name from command parameters
                if let Some(class_name) = self.extract_sqf_parameter(line, commands) {
                    target.push(EquipmentItem {
                        class_name: class_name.to_string(),
                        source_file: file_path.to_owned(),
                        line_number: line_num + 1,
                        context: format!("{}: {}", category, line.trim()),
                    });
                }
            }
        }
    }
    
    fn extract_class_name<'b>(&self, line: &'b str) -> Option<&'b str> {
        // This is a simplified extraction - in reality we'd use a proper parser
        
        // Look for patterns like: class = "ClassName";
        if let Some(idx) = line.find("class") {
            let after_class = &line[idx + 5..];
            if let Some(eq_idx) = after_class.find('=') {
                let after_eq = &after_class[eq_idx + 1..];
                if let Some(quote_idx) = after_eq.find('"') {
                    let after_quote = &after_eq[quote_idx + 1..];
                    if let Some(end_quote_idx) = after_quote.find('"') {
                        return Some(&after_quote[..end_quote_idx]);
                    }
                }
            }
        }
        
        // Look for patterns like: vehicle="ClassName";
        for indicator in &["vehicle", "weapon", "item", "magazine", "backpack"] {
            if let Some(idx) = line.find(indicator) {
                let after_indicator = &line[idx + indicator.len()..];
                if let Some(eq_idx) = after_indicator.find('=') {
                    let after_eq = &after_indicator[eq_idx + 1..];
                    if let Some(quote_idx) = after_eq.find('"') {
                        let after_quote = &after_eq[quote_idx + 1..];
                        if let Some(end_quote_idx) = after_quote.find('"') {
                            return Some(&after_quote[..end_quote_idx]);
                        }
                    } else {
                        // Handle case without quotes: vehicle=ClassName;
                        let after_eq_trimmed = after_eq.trim();
                        if let Some(semi_idx) = after_eq_trimmed.find(';') {
                            let class_name = &after_eq_trimmed[..semi_idx].trim();
                            if !class_name.is_empty() {
                                return Some(class_name);
                            }
                        } else if let Some(comma_idx) = after_eq_trimmed.find(',') {
                            // Handle case with comma: vehicle=ClassName,
                            let class_name = &after_eq_trimmed[..comma_idx].trim();
                            if !class_name.is_empty() {
                                return Some(class_name);
                            }
                        }
                    }
                }
            }
        }
        
        // Look for patterns like: "ClassName"
        if line.contains('"') {
            let mut in_quotes = false;
            let mut start_idx = 0;
            
            for (i, c) in line.char_indices() {
                if c == '"' {
                    if in_quotes {
                        // End of quoted string
                        let potential_class = &line[start_idx..i];
                        // Check if it looks like a class name (alphanumeric with underscores)
                        if !potential_class.is_empty() && 
                           potential_class.chars().all(|c| c.is_alphanumeric() || c == '_') {
                            return Some(potential_class);
                        }
                        in_quotes = false;
                    } else {
                        // Start of quoted string
                        start_idx = i + 1;
                        in_quotes = true;
                    }
                }
            }
        }
        
        None
    }
    
    fn extract_sqf_parameter<'b>(&self, line: &'b str, commands: &[&str]) -> Option<&'b str> {
        // Find which command is in the line
        let command = commands.iter().find(|&&cmd| line.contains(cmd))?;
        
        // Find the command in the line
        let cmd_idx = line.find(command)?;
        let after_cmd = &line[cmd_idx + command.len()..];
        
        // Look for the first parameter in quotes or without quotes
        if let Some(quote_idx) = after_cmd.find('"') {
            let after_quote = &after_cmd[quote_idx + 1..];
            if let Some(end_quote_idx) = after_quote.find('"') {
                return Some(&after_quote[..end_quote_idx]);
            }
        } else if let Some(open_idx) = after_cmd.find('[') {
            let after_open = &after_cmd[open_idx + 1..];
            if let Some(close_idx) = after_open.find(']') {
                // Extract the first item in the array
                let array_content = &after_open[..close_idx];
                if let Some(comma_idx) = array_content.find(',') {
                    let first_item = &array_content[..comma_idx];
                    return Some(first_item.trim());
                } else {
                    return Some(array_content.trim());
                }
            }
        }
        
        None
    }
}

pub struct DependencyAnalyzer<'a> {
    mission_cache_dir: &'a Path,
    mission_reports_dir: &'a Path,
    class_reports_dir: &'a Path,
}

impl<'a> DependencyAnalyzer<'a> {
    pub fn new(
        mission_cache_dir: &'a Path,
        mission_reports_dir: &'a Path,
        class_reports_dir: &'a Path,
    ) -> Self {
        Self {
            mission_cache_dir,
            mission_reports_dir,
            class_reports_dir,
        }
    }
    
    pub fn analyze(&self) -> Result<Vec<DependencyAnalysisResult>> {
        info!("Analyzing mission dependencies against available classes");
        
        // Load mission equipment reports
        let mission_reports = self.load_mission_reports()?;
        
        // Load available classes
        let available_classes = self.load_available_classes()?;
        
        // Cross-reference mission equipment with available classes
        let results = mission_reports.into_iter()
            .map(|report| self.analyze_mission_dependencies(report, &available_classes))
            .collect();
        
        Ok(results)
    }
    
    fn load_mission_reports(&self) -> Result<Vec<MissionAnalysisResult>> {
        let mut reports = Vec::new();
        
        for entry in walkdir::WalkDir::new(self.mission_reports_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "json" {
                    let content = fs::read_to_string(path)
                        .context(format!("Failed to read mission report: {}", path.display()))?;
                    
                    let report: MissionAnalysisResult = serde_json::from_str(&content)
                        .context(format!("Failed to parse mission report: {}", path.display()))?;
                    
                    reports.push(report);
                }
            }
        }
        
        Ok(reports)
    }
    
    fn load_available_classes(&self) -> Result<HashSet<String>> {
        let mut classes = HashSet::new();
        
        for entry in walkdir::WalkDir::new(self.class_reports_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "json" {
                    let content = fs::read_to_string(path)
                        .context(format!("Failed to read class report: {}", path.display()))?;
                    
                    // Parse the class report - format depends on the class scanner output
                    // This is a simplified approach
                    let json: serde_json::Value = serde_json::from_str(&content)
                        .context(format!("Failed to parse class report: {}", path.display()))?;
                    
                    if let Some(classes_array) = json.get("classes").and_then(|c| c.as_array()) {
                        for class in classes_array {
                            if let Some(class_name) = class.get("name").and_then(|n| n.as_str()) {
                                classes.insert(class_name.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        Ok(classes)
    }
    
    fn analyze_mission_dependencies(
        &self,
        mission_report: MissionAnalysisResult,
        available_classes: &HashSet<String>,
    ) -> DependencyAnalysisResult {
        let mut missing_classes = Vec::new();
        let mut available_mission_classes = Vec::new();
        
        // Collect all equipment classes from the mission
        let all_equipment: Vec<&EquipmentItem> = mission_report.equipment.iter()
            .chain(mission_report.vehicles.iter())
            .chain(mission_report.weapons.iter())
            .chain(mission_report.magazines.iter())
            .chain(mission_report.items.iter())
            .chain(mission_report.backpacks.iter())
            .collect();
        
        // Check each class against available classes
        for item in &all_equipment {
            if available_classes.contains(&item.class_name) {
                available_mission_classes.push(item.class_name.clone());
            } else {
                missing_classes.push(item.class_name.clone());
            }
        }
        
        // Remove duplicates
        missing_classes.sort();
        missing_classes.dedup();
        
        available_mission_classes.sort();
        available_mission_classes.dedup();
        
        DependencyAnalysisResult {
            mission_name: mission_report.mission_name,
            missing_classes,
            available_classes: available_mission_classes,
            total_equipment_count: all_equipment.len(),
        }
    }
} 