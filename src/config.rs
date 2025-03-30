use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use arma3_tool_pbo_cache::ExtractionConfig;

/// Configuration for visualization features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub preserved_classes: Vec<String>,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            preserved_classes: Vec::new(),
        }
    }
}

/// Configuration for the application scan settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    pub game_data_dirs: Vec<String>,
    pub game_data_extensions: Vec<String>,
    pub mission_dirs: Vec<String>,
    pub mission_extensions: Vec<String>,
    pub cache_dir: PathBuf,
    pub report_dir: PathBuf,
    pub database_path: Option<PathBuf>,
    pub threads: usize,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            game_data_dirs: Vec::new(),
            game_data_extensions: vec!["pbo".to_string()],
            mission_dirs: Vec::new(),
            mission_extensions: vec!["pbo".to_string()],
            cache_dir: PathBuf::from("cache"),
            report_dir: PathBuf::from("reports"),
            database_path: None,
            threads: 4,
        }
    }
}

impl ScanConfig {
    pub fn load(config_path: &str) -> Result<Self, String> {
        match fs::read_to_string(config_path) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(config) => Ok(config),
                    Err(e) => Err(format!("Failed to parse config: {}", e)),
                }
            },
            Err(e) => Err(format!("Failed to read config file: {}", e)),
        }
    }
    
    pub fn save(&self, config_path: &str) -> Result<(), String> {
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                match fs::write(config_path, json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to write config file: {}", e)),
                }
            },
            Err(e) => Err(format!("Failed to serialize config: {}", e)),
        }
    }

    pub fn to_pbo_cache_config(&self) -> ExtractionConfig {
        // Convert string paths to PathBuf
        let game_data_dirs = self.game_data_dirs.iter()
            .map(PathBuf::from)
            .collect();
            
        let mission_dirs = self.mission_dirs.iter()
            .map(PathBuf::from)
            .collect();

        let mut config = ExtractionConfig::new(self.cache_dir.clone());
        config.cache_dir = self.cache_dir.clone();
        config.game_data_cache_dir = self.cache_dir.join("gamedata"); 
        config.mission_cache_dir = self.cache_dir.join("missions");
        config.game_data_dirs = game_data_dirs;
        config.game_data_extensions = self.game_data_extensions.clone();
        config.mission_dirs = mission_dirs;
        config.mission_extensions = self.mission_extensions.clone();
        config.threads = self.threads;
        config.timeout = 60;
        
        // Set database path if available
        if let Some(db_path) = &self.database_path {
            config.db_path = db_path.clone();
        } else {
            // Default to a database file in the cache directory
            config.db_path = self.cache_dir.join("arma3.db");
        }
        
        config
    }
}

/// Configuration for the application UI settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub default_cache_dir: Option<PathBuf>,
    pub dark_mode: bool,
    pub visualization_config_path: Option<PathBuf>,
    pub last_database_path: Option<PathBuf>,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            default_cache_dir: None,
            dark_mode: true,
            visualization_config_path: Some(PathBuf::from("visualization_config.json")),
            last_database_path: None,
        }
    }
}

impl UiSettings {
    pub fn load(settings_path: &str) -> Self {
        match fs::read_to_string(settings_path) {
            Ok(content) => {
                match serde_json::from_str(&content) {
                    Ok(settings) => settings,
                    Err(_) => Self::default(),
                }
            },
            Err(_) => Self::default(),
        }
    }
    
    pub fn save(&self, settings_path: &str) -> Result<(), String> {
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                match fs::write(settings_path, json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to write settings file: {}", e)),
                }
            },
            Err(e) => Err(format!("Failed to serialize settings: {}", e)),
        }
    }
}