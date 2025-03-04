use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::Result;

/// Result of mission extraction process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionExtractionResult {
    /// Name of the mission (derived from PBO filename)
    pub mission_name: String,
    /// Path to the original PBO file
    pub pbo_path: PathBuf,
    /// Path to the extracted mission directory
    pub extracted_path: PathBuf,
    /// Path to the mission.sqm file if found
    pub sqm_file: Option<PathBuf>,
    /// Paths to all SQF script files
    pub sqf_files: Vec<PathBuf>,
    /// Paths to all CPP/HPP config files
    pub cpp_files: Vec<PathBuf>,
}

/// Information about an extracted mission
#[derive(Debug, Serialize, Deserialize)]
pub struct MissionInfo {
    pub hash: String,
    pub failed: bool,
    pub extraction_time: u64,
}

/// Database to track extracted missions
#[derive(Debug, Serialize, Deserialize)]
pub struct MissionDatabase {
    missions: HashMap<String, MissionInfo>,
}

/// Statistics about mission extraction
#[derive(Debug)]
pub struct MissionStats {
    pub total: usize,
    pub processed: usize,
    pub failed: usize,
    pub unchanged: usize,
}

impl MissionDatabase {
    pub fn new() -> Self {
        Self {
            missions: HashMap::new(),
        }
    }

    pub fn load_or_create(path: &Path) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_mission_info(&self, path: &Path) -> Option<&MissionInfo> {
        self.missions.get(&path.to_string_lossy().to_string())
    }

    pub fn update_mission(&mut self, path: &Path, hash: &str, failed: bool) {
        self.missions.insert(
            path.to_string_lossy().to_string(),
            MissionInfo {
                hash: hash.to_string(),
                failed,
                extraction_time: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            },
        );
    }

    pub fn get_stats(&self) -> MissionStats {
        let mut stats = MissionStats {
            total: self.missions.len(),
            processed: 0,
            failed: 0,
            unchanged: 0,
        };

        for info in self.missions.values() {
            if info.failed {
                stats.failed += 1;
            } else {
                stats.processed += 1;
            }
        }

        stats
    }
}