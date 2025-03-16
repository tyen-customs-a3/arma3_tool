use std::path::PathBuf;
use dashmap::DashMap;
use rayon::prelude::*;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use arma3_tool_models::{GameDataClasses, MissionData, Mission, DependencyRef};

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
    pub total_dependencies_checked: usize,
    pub total_missions_scanned: usize,
}

pub struct DependencyScanner {
    class_map: DashMap<String, bool>,
}

impl DependencyScanner {
    pub fn new(game_data: &GameDataClasses) -> Self {
        let class_map = DashMap::new();
        
        // Pre-populate the map with all known classes
        for class in &game_data.classes {
            class_map.insert(class.name.clone(), true);
        }
        
        Self { class_map }
    }
    
    pub fn scan_missions(&self, mission_data: &MissionData) -> ScanReport {
        let missing_deps = DashMap::new();
        let total_deps = std::sync::atomic::AtomicUsize::new(0);
        
        // Process missions in parallel
        mission_data.missions.par_iter().for_each(|mission| {
            self.scan_mission(mission, &missing_deps, &total_deps);
        });
        
        // Convert DashMap to Vec
        let missing_dependencies: Vec<MissingDependency> = missing_deps
            .into_iter()
            .map(|(_, v)| v)
            .collect();
        
        ScanReport {
            missing_dependencies,
            total_dependencies_checked: total_deps.load(std::sync::atomic::Ordering::Relaxed),
            total_missions_scanned: mission_data.missions.len(),
        }
    }
    
    fn scan_mission(
        &self,
        mission: &Mission,
        missing_deps: &DashMap<String, MissingDependency>,
        total_deps: &std::sync::atomic::AtomicUsize,
    ) {
        // Process direct mission dependencies
        for dep in &mission.dependencies {
            total_deps.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.check_dependency(dep, mission, missing_deps);
        }
        
        // Process component dependencies
        for component in &mission.components {
            for dep in &component.dependencies {
                total_deps.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.check_dependency(dep, mission, missing_deps);
            }
        }
    }
    
    fn check_dependency(
        &self,
        dep: &DependencyRef,
        mission: &Mission,
        missing_deps: &DashMap<String, MissingDependency>,
    ) {
        if !self.class_map.contains_key(&dep.class_name) {
            missing_deps.entry(dep.class_name.clone()).or_insert_with(|| MissingDependency {
                class_name: dep.class_name.clone(),
                mission_name: mission.name.clone(),
                reference_type: format!("{:?}", dep.reference_type),
                source_file: dep.source_file.clone(),
                line_number: dep.line_number,
            });
        }
    }
} 