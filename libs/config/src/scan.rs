use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use arma3_extractor::ExtractionConfig;

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
    /// Load configuration from a JSON file
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
    
    /// Save configuration to a JSON file
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

    /// Load configuration from environment variables
    pub fn load_from_env() -> Result<Self, String> {
        let mut config = Self::default();
        
        if let Ok(game_data_dirs) = std::env::var("ARMA3_GAME_DATA_DIRS") {
            config.game_data_dirs = game_data_dirs.split(',').map(|s| s.trim().to_string()).collect();
        }
        
        if let Ok(mission_dirs) = std::env::var("ARMA3_MISSION_DIRS") {
            config.mission_dirs = mission_dirs.split(',').map(|s| s.trim().to_string()).collect();
        }
        
        if let Ok(cache_dir) = std::env::var("ARMA3_CACHE_DIR") {
            config.cache_dir = PathBuf::from(cache_dir);
        }
        
        if let Ok(report_dir) = std::env::var("ARMA3_REPORT_DIR") {
            config.report_dir = PathBuf::from(report_dir);
        }
        
        if let Ok(threads) = std::env::var("ARMA3_THREADS") {
            config.threads = threads.parse().map_err(|e| format!("Invalid threads value: {}", e))?;
        }
        
        Ok(config)
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
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.game_data_dirs.is_empty() && self.mission_dirs.is_empty() {
            return Err("At least one game data directory or mission directory must be specified".to_string());
        }
        
        if self.threads == 0 {
            return Err("Number of threads must be greater than 0".to_string());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_default_config() {
        let config = ScanConfig::default();
        
        assert_eq!(config.game_data_extensions, vec!["pbo"]);
        assert_eq!(config.mission_extensions, vec!["pbo"]);
        assert_eq!(config.cache_dir, PathBuf::from("cache"));
        assert_eq!(config.report_dir, PathBuf::from("reports"));
        assert_eq!(config.threads, 4);
    }
    
    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("test_config.json");
        
        let original_config = ScanConfig {
            game_data_dirs: vec!["test_dir".to_string()],
            threads: 8,
            ..Default::default()
        };
        
        // Save config
        original_config.save(config_path.to_str().unwrap()).unwrap();
        
        // Load config
        let loaded_config = ScanConfig::load(config_path.to_str().unwrap()).unwrap();
        
        assert_eq!(loaded_config.game_data_dirs, original_config.game_data_dirs);
        assert_eq!(loaded_config.threads, original_config.threads);
    }
    
    #[test]
    fn test_validation() {
        let mut config = ScanConfig::default();
        
        // Should fail validation with no directories
        assert!(config.validate().is_err());
        
        // Should pass with game data dir
        config.game_data_dirs.push("test".to_string());
        assert!(config.validate().is_ok());
        
        // Should fail with zero threads
        config.threads = 0;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_load_from_env() {
        // Set environment variables
        std::env::set_var("ARMA3_GAME_DATA_DIRS", "dir1, dir2, dir3");
        std::env::set_var("ARMA3_MISSION_DIRS", "mission1, mission2");
        std::env::set_var("ARMA3_CACHE_DIR", "/tmp/cache");
        std::env::set_var("ARMA3_THREADS", "8");
        
        let config = ScanConfig::load_from_env().unwrap();
        
        assert_eq!(config.game_data_dirs, vec!["dir1", "dir2", "dir3"]);
        assert_eq!(config.mission_dirs, vec!["mission1", "mission2"]);
        assert_eq!(config.cache_dir, PathBuf::from("/tmp/cache"));
        assert_eq!(config.threads, 8);
        
        // Clean up
        std::env::remove_var("ARMA3_GAME_DATA_DIRS");
        std::env::remove_var("ARMA3_MISSION_DIRS");
        std::env::remove_var("ARMA3_CACHE_DIR");
        std::env::remove_var("ARMA3_THREADS");
    }
}