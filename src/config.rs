use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use arma3_tool_pbo_cache::ExtractionConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub game_data_dirs: Vec<String>,
    pub game_data_extensions: Vec<String>,
    pub mission_dirs: Vec<String>,
    pub mission_extensions: Vec<String>,
    pub cache_dir: PathBuf,
    pub report_dir: PathBuf,
    pub threads: usize,
}

impl ScanConfig {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&config_str)?;
        Ok(config)
    }

    pub fn to_pbo_cache_config(&self) -> ExtractionConfig {
        // Convert string paths to PathBuf
        let game_data_dirs = self.game_data_dirs.iter()
            .map(PathBuf::from)
            .collect();
            
        let mission_dirs = self.mission_dirs.iter()
            .map(PathBuf::from)
            .collect();

        ExtractionConfig::for_arma3_tool(
            self.cache_dir.clone(),
            game_data_dirs,
            self.game_data_extensions.clone(),
            mission_dirs,
            self.mission_extensions.clone(),
            self.threads,
            60,
        )
    }
} 