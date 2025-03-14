use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use crate::error::{Result, ToolError};

/// Configuration for the Arma 3 Tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolConfig {
    /// Directories containing game data (PBOs with class definitions)
    pub game_data_dirs: Vec<PathBuf>,
    
    /// File extensions to process for game data
    pub game_data_extensions: Vec<String>,
    
    /// Directories containing missions (PBOs)
    pub mission_dirs: Vec<PathBuf>,
    
    /// File extensions to process for missions
    pub mission_extensions: Vec<String>,
    
    /// Directory for caching extracted files
    pub cache_dir: PathBuf,
    
    /// Directory for output reports
    pub report_dir: PathBuf,
    
    /// Number of threads to use for parallel processing
    #[serde(default = "default_threads")]
    pub threads: usize,
    
    /// Timeout in seconds for operations
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    
    /// Whether to process base game PBOs
    #[serde(default)]
    pub process_base_game: bool,
}

fn default_threads() -> usize {
    num_cpus::get()
}

fn default_timeout() -> u64 {
    60
}

impl ToolConfig {
    /// Load configuration from a JSON file
    pub fn from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(ToolError::InvalidPath(path.to_path_buf()));
        }
        
        let config_str = fs::read_to_string(path)
            .map_err(|e| ToolError::ConfigError(format!("Failed to read config file: {}", e)))?;
            
        let config: ToolConfig = serde_json::from_str(&config_str)
            .map_err(|e| ToolError::ConfigError(format!("Failed to parse config file: {}", e)))?;
            
        config.validate()?;
        
        Ok(config)
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // Validate game data directories
        for dir in &self.game_data_dirs {
            if !dir.exists() {
                return Err(ToolError::ConfigError(
                    format!("Game data directory does not exist: {}", dir.display())
                ));
            }
            if !dir.is_dir() {
                return Err(ToolError::ConfigError(
                    format!("Game data path is not a directory: {}", dir.display())
                ));
            }
        }
        
        // Validate mission directories
        for dir in &self.mission_dirs {
            if !dir.exists() {
                return Err(ToolError::ConfigError(
                    format!("Mission directory does not exist: {}", dir.display())
                ));
            }
            if !dir.is_dir() {
                return Err(ToolError::ConfigError(
                    format!("Mission path is not a directory: {}", dir.display())
                ));
            }
        }
        
        // Validate cache directory
        if !self.cache_dir.exists() {
            // Create cache directory if it doesn't exist
            fs::create_dir_all(&self.cache_dir)
                .map_err(|e| ToolError::ConfigError(
                    format!("Failed to create cache directory: {}", e)
                ))?;
        }
        
        // Validate report directory
        if !self.report_dir.exists() {
            // Create report directory if it doesn't exist
            fs::create_dir_all(&self.report_dir)
                .map_err(|e| ToolError::ConfigError(
                    format!("Failed to create report directory: {}", e)
                ))?;
        }
        
        // Validate threads
        if self.threads == 0 {
            return Err(ToolError::ConfigError(
                "Number of threads must be greater than 0".to_string()
            ));
        }
        
        // Validate timeout
        if self.timeout == 0 {
            return Err(ToolError::ConfigError(
                "Timeout must be greater than 0".to_string()
            ));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_load_valid_config() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"{{
            "game_data_dirs": ["."],
            "game_data_extensions": ["hpp", "cpp"],
            "mission_dirs": ["."],
            "mission_extensions": ["hpp", "cpp", "sqf", "sqm"],
            "cache_dir": ".",
            "report_dir": ".",
            "threads": 4,
            "timeout": 30
        }}"#).unwrap();
        
        let config = ToolConfig::from_file(file.path()).unwrap();
        assert_eq!(config.game_data_dirs.len(), 1);
        assert_eq!(config.mission_dirs.len(), 1);
        assert_eq!(config.threads, 4);
        assert_eq!(config.timeout, 30);
    }
    
    #[test]
    fn test_default_values() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, r#"{{
            "game_data_dirs": ["."],
            "game_data_extensions": ["hpp", "cpp"],
            "mission_dirs": ["."],
            "mission_extensions": ["hpp", "cpp", "sqf", "sqm"],
            "cache_dir": ".",
            "report_dir": "."
        }}"#).unwrap();
        
        let config = ToolConfig::from_file(file.path()).unwrap();
        assert_eq!(config.threads, num_cpus::get());
        assert_eq!(config.timeout, 60);
        assert_eq!(config.process_base_game, false);
    }
} 