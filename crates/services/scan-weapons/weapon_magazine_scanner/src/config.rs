//! Configuration management for the weapon magazine scanner
//! 
//! Supports loading configuration from YAML, JSON, or TOML files,
//! with command-line argument overrides.

use crate::models::AppConfig;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::fs;

/// Configuration manager that handles loading and merging configurations
#[derive(Debug)]
pub struct ConfigManager {
    /// Default configuration search paths
    search_paths: Vec<PathBuf>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            search_paths: vec![
                PathBuf::from("config.yaml"),
                PathBuf::from("config.yml"),
                PathBuf::from("config.json"),
                PathBuf::from("config.toml"),
                PathBuf::from(".weapon_scanner.yaml"),
                PathBuf::from(".weapon_scanner.yml"),
                PathBuf::from(".weapon_scanner.json"),
                PathBuf::from(".weapon_scanner.toml"),
            ],
        }
    }

    /// Load configuration from file or use defaults
    pub fn load_config(&self, config_path: Option<&Path>) -> Result<AppConfig> {
        match config_path {
            Some(path) => self.load_from_file(path),
            None => self.auto_discover_config(),
        }
    }

    /// Load configuration from a specific file
    pub fn load_from_file(&self, path: &Path) -> Result<AppConfig> {
        if !path.exists() {
            anyhow::bail!("Configuration file not found: {}", path.display());
        }

        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read configuration file: {}", path.display()))?;

        let config = match path.extension().and_then(|ext| ext.to_str()) {
            Some("yaml") | Some("yml") => {
                serde_yaml::from_str::<AppConfig>(&content)
                    .with_context(|| format!("Failed to parse YAML configuration: {}", path.display()))?
            },
            Some("json") => {
                serde_json::from_str::<AppConfig>(&content)
                    .with_context(|| format!("Failed to parse JSON configuration: {}", path.display()))?
            },
            Some("toml") => {
                // Add toml dependency to Cargo.toml if you want TOML support
                anyhow::bail!("TOML configuration is not yet supported. Please use YAML or JSON.");
            },
            _ => {
                // Try to detect format from content
                if content.trim_start().starts_with('{') {
                    serde_json::from_str::<AppConfig>(&content)
                        .with_context(|| format!("Failed to parse JSON configuration: {}", path.display()))?
                } else {
                    serde_yaml::from_str::<AppConfig>(&content)
                        .with_context(|| format!("Failed to parse YAML configuration: {}", path.display()))?
                }
            }
        };

        log::info!("Loaded configuration from: {}", path.display());
        Ok(config)
    }

    /// Auto-discover configuration file in current directory
    pub fn auto_discover_config(&self) -> Result<AppConfig> {
        for config_path in &self.search_paths {
            if config_path.exists() {
                log::info!("Auto-discovered configuration file: {}", config_path.display());
                return self.load_from_file(config_path);
            }
        }

        log::info!("No configuration file found, using defaults");
        Ok(AppConfig::default())
    }

    /// Save current configuration to file
    pub fn save_config(&self, config: &AppConfig, path: &Path) -> Result<()> {
        let content = match path.extension().and_then(|ext| ext.to_str()) {
            Some("yaml") | Some("yml") => {
                serde_yaml::to_string(config)
                    .with_context(|| "Failed to serialize configuration to YAML")?
            },
            Some("json") => {
                serde_json::to_string_pretty(config)
                    .with_context(|| "Failed to serialize configuration to JSON")?
            },
            _ => {
                // Default to YAML
                serde_yaml::to_string(config)
                    .with_context(|| "Failed to serialize configuration to YAML")?
            }
        };

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        fs::write(path, content)
            .with_context(|| format!("Failed to write configuration file: {}", path.display()))?;

        log::info!("Configuration saved to: {}", path.display());
        Ok(())
    }

    /// Generate a sample configuration file
    pub fn generate_sample_config(&self, path: &Path) -> Result<()> {
        let sample_config = AppConfig {
            target: PathBuf::from("./gamedata"),
            output: PathBuf::from("./output/weapons_report.json"),
            database: PathBuf::from("./cache/weapon_cache.db"),
            force: false,
            threads: num_cpus::get(),
            format: "json".to_string(),
            verbose: false,
            timeout: 30,
            project_root: Some(PathBuf::from("./project_root")),
            weapons_by_mod: Some(PathBuf::from("./output/weapons_by_mod.txt")),
        };

        self.save_config(&sample_config, path)?;
        
        // Add comments if it's a YAML file
        if matches!(path.extension().and_then(|ext| ext.to_str()), Some("yaml") | Some("yml")) {
            self.add_yaml_comments(path)?;
        }

        Ok(())
    }

    /// Add helpful comments to YAML configuration file
    fn add_yaml_comments(&self, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path)?;
        
        let commented_content = format!(
r#"# Weapon Magazine Scanner Configuration
# 
# This file configures the weapon magazine scanner application.
# You can override any of these settings via command-line arguments.

# Target directory to scan for weapon and magazine data
# This should point to your gamedata folder or mod directory
{}

# Output settings

# Database settings
# The database file stores scan results for faster subsequent runs
"#, content);

        fs::write(path, commented_content)?;
        Ok(())
    }

    /// Merge configuration with command-line overrides
    pub fn merge_with_args<T>(&self, mut config: AppConfig, args: &T) -> AppConfig
    where
        T: CliArgsProvider
    {
        // Override with command-line arguments if provided
        if let Some(target) = args.target() {
            if !target.as_os_str().is_empty() {
                config.target = target.clone();
            }
        }
        
        if args.output() != &PathBuf::from("weapons_magazines_report.json") {
            config.output = args.output().clone();
        }
        
        if args.database() != &PathBuf::from("weapon_magazine_cache.db") {
            config.database = args.database().clone();
        }
        
        if args.force() {
            config.force = args.force();
        }
        
        if args.threads() != num_cpus::get() {
            config.threads = args.threads();
        }
        
        if args.format() != "json" {
            config.format = args.format().to_string();
        }
        
        if args.verbose() {
            config.verbose = args.verbose();
        }
        
        if args.timeout() != 30 {
            config.timeout = args.timeout();
        }
        
        if args.project_root().is_some() {
            config.project_root = args.project_root().clone();
        }

        if args.weapons_by_mod().is_some() {
            config.weapons_by_mod = args.weapons_by_mod().clone();
        }

        config
    }
}

/// Trait for providing command-line arguments to the config manager
pub trait CliArgsProvider {
    fn target(&self) -> Option<&PathBuf>;
    fn output(&self) -> &PathBuf;
    fn database(&self) -> &PathBuf;
    fn force(&self) -> bool;
    fn threads(&self) -> usize;
    fn format(&self) -> &String;
    fn verbose(&self) -> bool;
    fn timeout(&self) -> u64;
    fn project_root(&self) -> &Option<PathBuf>;
    fn weapons_by_mod(&self) -> &Option<PathBuf>;
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_default_config() {
        let config_manager = ConfigManager::new();
        let config = config_manager.auto_discover_config().unwrap();
        
        assert_eq!(config.format, "json");
        assert_eq!(config.timeout, 30);
        assert!(!config.force);
        assert!(!config.verbose);
    }

    #[test]
    fn test_save_and_load_yaml_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("test_config.yaml");
        
        let config_manager = ConfigManager::new();
        let original_config = AppConfig {
            target: PathBuf::from("/test/path"),
            verbose: true,
            threads: 8,
            ..Default::default()
        };

        // Save config
        config_manager.save_config(&original_config, &config_path).unwrap();
        
        // Load config
        let loaded_config = config_manager.load_from_file(&config_path).unwrap();
        
        assert_eq!(loaded_config.target, original_config.target);
        assert_eq!(loaded_config.verbose, original_config.verbose);
        assert_eq!(loaded_config.threads, original_config.threads);
    }

    #[test]
    fn test_generate_sample_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("sample_config.yaml");
        
        let config_manager = ConfigManager::new();
        config_manager.generate_sample_config(&config_path).unwrap();
        
        assert!(config_path.exists());
        
        // Verify we can load the generated config
        let loaded_config = config_manager.load_from_file(&config_path).unwrap();
        assert_eq!(loaded_config.format, "json");
    }
}