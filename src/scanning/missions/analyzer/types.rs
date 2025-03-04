use std::path::PathBuf;
use std::collections::HashSet;
use serde::Serialize;

use crate::reporting::mission_report_writer::{
    MissionName, MissionEquipment, MissionDependencies, 
    MissionEquipmentReport, MissionDependenciesReport, EquipmentItemReport
};

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

// Implement Display for ReferenceType
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