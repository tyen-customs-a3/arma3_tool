use std::path::PathBuf;
use serde::{Serialize, Deserialize};

// Game data classes model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameDataClasses {
    pub classes: Vec<GameDataClassDefinition>,
}

impl GameDataClasses {
    pub fn new() -> Self {
        Self {
            classes: Vec::new(),
        }
    }
}

// Game data class definition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameDataClassDefinition {
    pub name: String,
    pub parent: Option<String>,
}

impl GameDataClassDefinition {
    pub fn new(name: String, parent: Option<String>) -> Self {
        Self {
            name,
            parent,
        }
    }
}

// Mission data model
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionData {
    pub missions: Vec<MissionDefinition>,
}

impl MissionData {
    pub fn new() -> Self {
        Self {
            missions: Vec::new(),
        }
    }
}

// Mission definition
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MissionDefinition {
    pub name: String,
    pub path: PathBuf,
}

impl MissionDefinition {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
        }
    }
} 