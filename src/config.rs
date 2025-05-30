use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use arma3_extractor::ExtractionConfig;

/// Configuration for visualization features
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub preserved_classes: Vec<String>,
    pub excluded_classes: Vec<String>,
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
    pub analysis_database_path: Option<PathBuf>, 
    pub extractor_database_path: Option<PathBuf>, 
    pub threads: usize,
    pub ignore_classes_file: Option<PathBuf>,
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
            analysis_database_path: None, // Defaults determined later based on cache_dir
            extractor_database_path: None, // Defaults determined later based on cache_dir
            threads: 4,
            ignore_classes_file: None,
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

    /// Creates an ExtractionConfig based on the ScanConfig settings.
    pub fn to_extractor_config(&self) -> ExtractionConfig {
        // Convert string paths to PathBuf
        let game_data_dirs = self.game_data_dirs.iter()
            .map(PathBuf::from)
            .collect();
            
        let mission_dirs = self.mission_dirs.iter()
            .map(PathBuf::from)
            .collect();

        // Initialize ExtractionConfig, which already sets up cache directories correctly
        let mut config = ExtractionConfig::new(self.cache_dir.clone());
        
        // Populate the rest of the ExtractionConfig fields (cache dirs are already set by new())
        config.game_data_dirs = game_data_dirs;
        config.game_data_extensions = self.game_data_extensions.clone();
        config.mission_dirs = mission_dirs;
        config.mission_extensions = self.mission_extensions.clone();
        config.threads = self.threads;
        config.timeout = 60; // Consider making this configurable in ScanConfig too
        config.verbose = true; // Consider making this configurable
        
        // Set the dedicated extractor database path
        config.db_path = self.extractor_database_path.clone()
            .unwrap_or_else(|| self.cache_dir.join("extractor.db")); // Default to extractor.db in cache_dir
        
        config
    }
    
    /// Gets the path for the analysis database, applying default if necessary.
    pub fn get_analysis_db_path(&self) -> PathBuf {
        self.analysis_database_path.clone()
            .unwrap_or_else(|| self.cache_dir.join("analysis.db")) // Default to analysis.db in cache_dir
    }
}
