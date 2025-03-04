use std::collections::HashMap;
use serde::Serialize;

/// Information about a missing class
#[derive(Debug, Clone, Serialize)]
pub struct MissingClassInfo {
    pub class_name: String,
    pub reference_count: usize,
    pub reference_locations: Vec<String>,
    pub suggested_alternatives: Vec<String>,
}

/// Report on class existence for a single mission
#[derive(Debug, Clone, Serialize)]
pub struct MissionClassExistenceReport {
    pub mission_name: String,
    pub total_classes: usize,
    pub existing_classes: usize,
    pub missing_classes: usize,
    pub existence_percentage: f64,
    pub missing_class_list: Vec<MissingClassInfo>,
}

/// Report on class existence for all missions
#[derive(Debug, Clone, Serialize)]
pub struct ClassExistenceReport {
    pub total_missions: usize,
    pub total_unique_classes: usize,
    pub existing_classes: usize,
    pub missing_classes: usize,
    pub existence_percentage: f64,
    pub mission_reports: Vec<MissionClassExistenceReport>,
} 