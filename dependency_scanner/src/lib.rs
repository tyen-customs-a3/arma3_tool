use std::path::PathBuf;
use dashmap::DashMap;
use rayon::prelude::*;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use arma3_tool_models::{GameDataClasses, MissionData, Mission, DependencyRef};
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum ScanError {
    #[error("No game data available")]
    NoGameData,
    
    #[error("No mission data available")]
    NoMissionData,
}

pub type Result<T> = std::result::Result<T, ScanError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MissingDependency {
    pub class_name: String,
    pub mission_name: String,
    pub reference_type: String,
    pub source_file: PathBuf,
    pub line_number: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanReport {
    pub missing_dependencies: Vec<MissingDependency>,
    pub found_dependencies: HashMap<String, Vec<String>>,
    pub total_dependencies_checked: usize,
    pub total_missions_scanned: usize,
}

pub struct DependencyScanner {
    class_map: DashMap<String, bool>,
}

impl DependencyScanner {
    pub fn new(game_data: &GameDataClasses) -> Self {
        let class_map = DashMap::new();
        
        // Pre-populate the map with all known classes - using lowercase keys for case-insensitive comparison
        for class in &game_data.classes {
            class_map.insert(class.name.trim_matches('"').to_lowercase(), true);
        }
        
        // Log the number of classes found in the database
        println!("Found {} classes in the game data database", class_map.len());
        log::info!("Found {} classes in the game data database", class_map.len());
        
        Self { class_map }
    }
    
    pub fn scan_missions(&self, mission_data: &MissionData) -> ScanReport {
        let missing_deps = DashMap::new();
        let found_deps = DashMap::new();
        let total_deps = std::sync::atomic::AtomicUsize::new(0);
        
        // Process missions in parallel
        mission_data.missions.par_iter().for_each(|mission| {
            self.scan_mission(mission, &missing_deps, &found_deps, &total_deps);
        });
        
        // Convert DashMap to Vec and HashMap
        let missing_dependencies: Vec<MissingDependency> = missing_deps
            .into_iter()
            .map(|(_, v)| v)
            .collect();
            
        let found_dependencies: HashMap<String, Vec<String>> = found_deps
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();
        
        let total_deps_checked = total_deps.load(std::sync::atomic::Ordering::Relaxed);
        let missing_count = missing_dependencies.len();
        
        // Print summary
        println!("==== Dependency Scan Summary ====");
        println!("Game Database Classes: {}", self.class_map.len());
        println!("Total Dependencies Checked: {}", total_deps_checked);
        println!("Missing Dependencies: {}", missing_count);
        println!("Match Rate: {:.2}%", 100.0 * (total_deps_checked - missing_count) as f64 / total_deps_checked as f64);
        println!("===============================");
        
        ScanReport {
            missing_dependencies,
            found_dependencies,
            total_dependencies_checked: total_deps_checked,
            total_missions_scanned: mission_data.missions.len(),
        }
    }
    
    fn scan_mission(
        &self,
        mission: &Mission,
        missing_deps: &DashMap<String, MissingDependency>,
        found_deps: &DashMap<String, Vec<String>>,
        total_deps: &std::sync::atomic::AtomicUsize,
    ) {
        let mut mission_found_deps = Vec::new();
        
        // Process direct mission dependencies
        for dep in &mission.dependencies {
            total_deps.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.check_dependency(dep, mission, missing_deps, &mut mission_found_deps);
        }
        
        // Process component dependencies
        for component in &mission.components {
            for dep in &component.dependencies {
                total_deps.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.check_dependency(dep, mission, missing_deps, &mut mission_found_deps);
            }
        }
        
        // Store found dependencies for this mission
        if !mission_found_deps.is_empty() {
            found_deps.insert(mission.name.clone(), mission_found_deps);
        }
    }
    
    fn check_dependency(
        &self,
        dep: &DependencyRef,
        mission: &Mission,
        missing_deps: &DashMap<String, MissingDependency>,
        found_deps: &mut Vec<String>,
    ) {
        // Strip quotes and convert class name to lowercase for case-insensitive comparison
        let lowercase_class_name = dep.class_name.trim_matches('"').to_lowercase();
        
        if !self.class_map.contains_key(&lowercase_class_name) {
            missing_deps.entry(dep.class_name.clone()).or_insert_with(|| MissingDependency {
                class_name: dep.class_name.clone(),
                mission_name: mission.name.clone(),
                reference_type: format!("{:?}", dep.reference_type),
                source_file: dep.source_file.clone(),
                line_number: dep.line_number,
            });
        } else {
            found_deps.push(dep.class_name.clone());
        }
    }
} 