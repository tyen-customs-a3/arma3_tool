use std::path::PathBuf;
use dashmap::DashMap;
use rayon::prelude::*;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use arma3_tool_shared_models::{GameDataClasses, MissionData, Mission, DependencyRef};

/// Errors that can occur during dependency scanning
#[derive(Error, Debug)]
pub enum ScanError {
    /// No game data was provided for scanning
    #[error("No game data available")]
    NoGameData,
    
    /// No mission data was provided for scanning
    #[error("No mission data available")]
    NoMissionData,
}

/// Represents a single dependency reference found during scanning
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependency {
    /// The name of the class being referenced
    pub class_name: String,
    /// The name of the mission containing the reference
    pub mission_name: String,
    /// The type of reference (e.g., "Direct", "Inheritance")
    pub reference_type: String,
    /// The file containing the reference
    pub source_file: PathBuf,
    /// Optional line number where the reference appears
    pub line_number: Option<usize>,
}

/// Configuration options for dependency scanning
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// Whether to print summary statistics after scanning
    pub print_summary: bool,
    /// Whether to log detailed information during scanning
    pub verbose_logging: bool,
    /// Whether to use case-sensitive comparison (default: false)
    pub case_sensitive: bool,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            print_summary: true,
            verbose_logging: false,
            case_sensitive: false,
        }
    }
}

/// Report containing the results of a dependency scan
#[derive(Debug, Serialize, Deserialize)]
pub struct ScanReport {
    /// Dependencies that could not be found in the game data
    pub missing: Vec<Dependency>,
    /// Dependencies that were successfully found
    pub found: Vec<Dependency>,
    /// Total number of dependencies checked
    pub total_dependencies_checked: usize,
    /// Total number of missions scanned
    pub total_missions_scanned: usize,
}

/// Builder for scanning mission dependencies against game data
pub struct MissionDependencyBuilder {
    class_map: DashMap<String, bool>,
    config: ScanConfig,
}

impl MissionDependencyBuilder {
    /// Creates a new dependency scanner with default configuration
    pub fn new(game_data: &GameDataClasses) -> Self {
        Self::with_config(game_data, ScanConfig::default())
    }

    /// Creates a new dependency scanner with custom configuration
    pub fn with_config(game_data: &GameDataClasses, config: ScanConfig) -> Self {
        let class_map = DashMap::new();
        
        // Pre-populate the map with all known classes
        for class in &game_data.classes {
            let key = if config.case_sensitive {
                class.name.trim_matches('"').to_string()
            } else {
                class.name.trim_matches('"').to_lowercase()
            };
            class_map.insert(key, true);
        }
        
        if config.verbose_logging {
            log::info!("Found {} classes in the game data database", class_map.len());
        }
        
        Self { class_map, config }
    }
    
    /// Scans a set of missions for dependencies
    pub fn scan_missions(&self, mission_data: &MissionData) -> ScanReport {
        let missing_deps = DashMap::new();
        let found_deps = DashMap::new();
        let total_deps = std::sync::atomic::AtomicUsize::new(0);
        
        // Process missions in parallel
        mission_data.missions.par_iter().for_each(|mission| {
            self.scan_mission(mission, &missing_deps, &found_deps, &total_deps);
        });
        
        // Convert DashMap to Vec
        let missing_dependencies: Vec<Dependency> = missing_deps
            .into_iter()
            .map(|(_, v)| v)
            .collect();
            
        let found_dependencies: Vec<Dependency> = found_deps
            .into_iter()
            .flat_map(|(_, v)| v)
            .collect();
        
        let total_deps_checked = total_deps.load(std::sync::atomic::Ordering::Relaxed);
        
        // Generate statistics if enabled
        if self.config.print_summary {
            self.print_scan_summary(&missing_dependencies, total_deps_checked);
        }
        
        ScanReport {
            missing: missing_dependencies,
            found: found_dependencies,
            total_dependencies_checked: total_deps_checked,
            total_missions_scanned: mission_data.missions.len(),
        }
    }
    
    /// Scans a single mission for dependencies
    fn scan_mission(
        &self,
        mission: &Mission,
        missing_deps: &DashMap<String, Dependency>,
        found_deps: &DashMap<String, Vec<Dependency>>,
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
    
    /// Checks a single dependency reference
    fn check_dependency(
        &self,
        dep: &DependencyRef,
        mission: &Mission,
        missing_deps: &DashMap<String, Dependency>,
        found_deps: &mut Vec<Dependency>,
    ) {
        let class_name = if self.config.case_sensitive {
            dep.class_name.trim_matches('"').to_string()
        } else {
            dep.class_name.trim_matches('"').to_lowercase()
        };
        
        if !self.class_map.contains_key(&class_name) {
            missing_deps.entry(dep.class_name.clone()).or_insert_with(|| Dependency {
                class_name: dep.class_name.clone(),
                mission_name: mission.name.clone(),
                reference_type: format!("{:?}", dep.reference_type),
                source_file: dep.source_file.clone(),
                line_number: dep.line_number,
            });
        } else {
            found_deps.push(Dependency {
                class_name: dep.class_name.clone(),
                mission_name: mission.name.clone(),
                reference_type: format!("{:?}", dep.reference_type),
                source_file: dep.source_file.clone(),
                line_number: dep.line_number,
            });
        }
    }

    /// Prints a summary of the scan results
    fn print_scan_summary(&self, missing_dependencies: &[Dependency], total_deps_checked: usize) {
        // Count unique missing class names (case-insensitive)
        let mut unique_missing_classes = std::collections::HashSet::new();
        for dep in missing_dependencies {
            unique_missing_classes.insert(
                if self.config.case_sensitive {
                    dep.class_name.clone()
                } else {
                    dep.class_name.to_lowercase()
                }
            );
        }
        let unique_missing_count = unique_missing_classes.len();
        
        println!("==== Dependency Scan Summary ====");
        println!("Game Database Classes: {}", self.class_map.len());
        println!("Total Dependencies Checked: {}", total_deps_checked);
        println!("Missing Dependencies: {}", missing_dependencies.len());
        println!("Unique Missing Classes: {}", unique_missing_count);
        println!("Match Rate: {:.2}%", 100.0 * (total_deps_checked - missing_dependencies.len()) as f64 / total_deps_checked as f64);
        println!("True Match Rate (unique classes): {:.2}%", 100.0 * (total_deps_checked - unique_missing_count) as f64 / total_deps_checked as f64);
        println!("===============================");
    }
} 