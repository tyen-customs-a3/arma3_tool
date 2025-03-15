use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataClasses {
    pub classes: Vec<GameDataClassDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataClassDefinition {
    pub name: String,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionData {
    pub missions: Vec<MissionDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDefinition {
    pub name: String,
    pub path: std::path::PathBuf,
    pub dependencies: Vec<MissionDependency>,
    pub sqf_files: usize,
    pub cpp_files: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDependency {
    pub class_name: String,
    pub context: String,
    pub reference_type: String,
}

