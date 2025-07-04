use std::path::PathBuf;

use crate::error::Result;
use crate::scan::ScanConfig;
use crate::item_filter::{ItemFilterConfig, ItemTypeConfig, ExclusionRules};
use crate::visualization::VisualizationConfig;

/// Simple builder for ScanConfig with fluent API
impl ScanConfig {
    /// Create a new builder starting with default values
    pub fn builder() -> ScanConfig {
        Self::default()
    }

    /// Add a game data directory
    pub fn with_game_data_dir(mut self, dir: impl Into<String>) -> Self {
        self.game_data_dirs.push(dir.into());
        self
    }

    /// Set game data directories
    pub fn with_game_data_dirs(mut self, dirs: Vec<String>) -> Self {
        self.game_data_dirs = dirs;
        self
    }

    /// Add a mission directory
    pub fn with_mission_dir(mut self, dir: impl Into<String>) -> Self {
        self.mission_dirs.push(dir.into());
        self
    }

    /// Set mission directories
    pub fn with_mission_dirs(mut self, dirs: Vec<String>) -> Self {
        self.mission_dirs = dirs;
        self
    }

    /// Set cache directory
    pub fn with_cache_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.cache_dir = dir.into();
        self
    }

    /// Set report directory
    pub fn with_report_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.report_dir = dir.into();
        self
    }

    /// Set analysis database path
    pub fn with_analysis_db(mut self, path: impl Into<PathBuf>) -> Self {
        self.analysis_database_path = Some(path.into());
        self
    }

    /// Set extractor database path
    pub fn with_extractor_db(mut self, path: impl Into<PathBuf>) -> Self {
        self.extractor_database_path = Some(path.into());
        self
    }

    /// Set number of threads
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    /// Set ignore classes file
    pub fn with_ignore_classes_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.ignore_classes_file = Some(path.into());
        self
    }

    /// Validate and finalize the configuration
    pub fn build(self) -> Result<ScanConfig> {
        self.validate()?;
        Ok(self)
    }
}

/// Simple builder for ItemFilterConfig
impl ItemFilterConfig {
    /// Create a new builder with default values
    pub fn builder() -> Self {
        Self::default()
    }

    /// Add an item type configuration
    pub fn with_item_type(mut self, name: impl Into<String>, config: ItemTypeConfig) -> Self {
        self.item_types.insert(name.into(), config);
        self
    }

    /// Set exclusion rules
    pub fn with_exclusion_rules(mut self, rules: ExclusionRules) -> Self {
        self.exclusion_rules = rules;
        self
    }

    /// Add common game data item types
    pub fn with_common_game_data_types(mut self) -> Self {
        self.item_types.insert("weapons".to_string(), ItemTypeConfig::new("CfgWeapons".to_string()));
        self.item_types.insert("vehicles".to_string(), ItemTypeConfig::new("CfgVehicles".to_string()));
        self.item_types.insert("ammo".to_string(), ItemTypeConfig::new("CfgAmmo".to_string()));
        self.item_types.insert("magazines".to_string(), ItemTypeConfig::new("CfgMagazines".to_string()));
        self
    }

    /// Validate and finalize the configuration
    pub fn build(self) -> Result<ItemFilterConfig> {
        // ItemFilterConfig validation uses Box<dyn std::error::Error>, so we handle it manually
        if let Err(e) = self.validate() {
            return Err(crate::error::ConfigError::validation_error(&e.to_string()));
        }
        Ok(self)
    }
}

/// Simple builder for VisualizationConfig
impl VisualizationConfig {
    /// Create a new builder with default values
    pub fn builder() -> Self {
        Self::default()
    }

    /// Add a preserved class
    pub fn with_preserved_class(mut self, class: impl Into<String>) -> Self {
        self.preserved_classes.push(class.into());
        self
    }

    /// Set preserved classes
    pub fn with_preserved_classes(mut self, classes: Vec<String>) -> Self {
        self.preserved_classes = classes;
        self
    }

    /// Add an excluded class
    pub fn with_excluded_class(mut self, class: impl Into<String>) -> Self {
        self.excluded_classes.push(class.into());
        self
    }

    /// Set excluded classes
    pub fn with_excluded_classes(mut self, classes: Vec<String>) -> Self {
        self.excluded_classes = classes;
        self
    }

    /// Validate and finalize the configuration
    pub fn build(self) -> Result<VisualizationConfig> {
        // VisualizationConfig validation uses Box<dyn std::error::Error>, so we handle it manually
        if let Err(e) = self.validate() {
            return Err(crate::error::ConfigError::validation_error(&e.to_string()));
        }
        Ok(self)
    }
}

/// Builder for ExclusionRules
impl ExclusionRules {
    /// Create a new exclusion rules builder
    pub fn builder() -> Self {
        Self::default()
    }

    /// Set maximum scope
    pub fn with_max_scope(mut self, max_scope: i32) -> Self {
        self.max_scope = max_scope;
        self
    }

    /// Add an excluded prefix
    pub fn with_excluded_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.excluded_prefixes.push(prefix.into());
        self
    }

    /// Set excluded prefixes
    pub fn with_excluded_prefixes(mut self, prefixes: Vec<String>) -> Self {
        self.excluded_prefixes = prefixes;
        self
    }
}

impl Default for ExclusionRules {
    fn default() -> Self {
        Self {
            max_scope: 2,
            excluded_prefixes: vec!["_".to_string()],
        }
    }
}

/// Convenience functions for creating configurations
pub mod presets {
    use super::*;

    /// Create a basic scan configuration for game data analysis
    pub fn game_data_scan_config(arma_dir: impl Into<String>) -> Result<ScanConfig> {
        ScanConfig::builder()
            .with_game_data_dir(arma_dir)
            .with_cache_dir("cache")
            .with_report_dir("reports")
            .with_threads(num_cpus::get())
            .build()
    }

    /// Create a basic scan configuration for mission analysis
    pub fn mission_scan_config(mission_dirs: Vec<String>) -> Result<ScanConfig> {
        ScanConfig::builder()
            .with_mission_dirs(mission_dirs)
            .with_cache_dir("cache")
            .with_report_dir("reports")
            .with_threads(num_cpus::get())
            .build()
    }

    /// Create a basic item filter configuration
    pub fn basic_item_filter() -> Result<ItemFilterConfig> {
        ItemFilterConfig::builder()
            .with_common_game_data_types()
            .with_exclusion_rules(ExclusionRules::builder().with_max_scope(2))
            .build()
    }

    /// Create a basic visualization configuration
    pub fn basic_visualization() -> Result<VisualizationConfig> {
        VisualizationConfig::builder()
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_config_builder() {
        let config = ScanConfig::builder()
            .with_game_data_dir("/opt/arma3/addons")
            .with_mission_dir("/missions/test")
            .with_cache_dir("/tmp/cache")
            .with_threads(8)
            .build()
            .unwrap();

        assert_eq!(config.game_data_dirs.len(), 1);
        assert_eq!(config.mission_dirs.len(), 1);
        assert_eq!(config.cache_dir, PathBuf::from("/tmp/cache"));
        assert_eq!(config.threads, 8);
    }

    #[test]
    fn test_item_filter_builder() {
        let config = ItemFilterConfig::builder()
            .with_common_game_data_types()
            .with_exclusion_rules(
                ExclusionRules::builder()
                    .with_max_scope(3)
                    .with_excluded_prefix("test_")
            )
            .build()
            .unwrap();

        assert_eq!(config.item_types.len(), 4);
        assert!(config.item_types.contains_key("weapons"));
        assert_eq!(config.exclusion_rules.max_scope, 3);
        assert!(config.exclusion_rules.excluded_prefixes.contains(&"test_".to_string()));
    }

    #[test]
    fn test_visualization_builder() {
        let config = VisualizationConfig::builder()
            .with_preserved_class("MyClass")
            .with_excluded_class("TestClass")
            .build()
            .unwrap();

        assert!(config.preserved_classes.contains(&"MyClass".to_string()));
        assert!(config.excluded_classes.contains(&"TestClass".to_string()));
    }

    #[test]
    fn test_presets() {
        let scan_config = presets::game_data_scan_config("/opt/arma3").unwrap();
        assert_eq!(scan_config.game_data_dirs.len(), 1);

        let filter_config = presets::basic_item_filter().unwrap();
        assert!(filter_config.item_types.len() > 0);

        let viz_config = presets::basic_visualization().unwrap();
        assert!(viz_config.preserved_classes.is_empty());
    }

    #[test]
    fn test_builder_validation() {
        // Should fail validation with no directories
        let result = ScanConfig::builder().build();
        assert!(result.is_err());

        // Should pass with at least one directory
        let result = ScanConfig::builder()
            .with_game_data_dir("/test")
            .build();
        assert!(result.is_ok());
    }
}