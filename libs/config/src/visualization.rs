use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Configuration for visualization features
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub preserved_classes: Vec<String>,
    pub excluded_classes: Vec<String>,
    pub graph_settings: GraphSettings,
    pub export_settings: ExportSettings,
}

/// Settings for graph visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSettings {
    pub max_depth: usize,
    pub show_inheritance: bool,
    pub show_compositions: bool,
    pub layout_algorithm: LayoutAlgorithm,
    pub node_styling: NodeStyling,
}

/// Settings for exporting visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportSettings {
    pub format: ExportFormat,
    pub output_dir: String,
    pub filename_prefix: String,
    pub include_metadata: bool,
}

/// Available layout algorithms for graph visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutAlgorithm {
    Hierarchical,
    Force,
    Circular,
    Grid,
}

/// Node styling options for graph visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStyling {
    pub color_by_type: bool,
    pub show_labels: bool,
    pub node_size: NodeSize,
    pub font_size: u32,
}

/// Node size options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeSize {
    Small,
    Medium,
    Large,
    Auto,
}

/// Export format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Svg,
    Png,
    Dot,
    Json,
}

impl Default for GraphSettings {
    fn default() -> Self {
        Self {
            max_depth: 5,
            show_inheritance: true,
            show_compositions: false,
            layout_algorithm: LayoutAlgorithm::Hierarchical,
            node_styling: NodeStyling::default(),
        }
    }
}

impl Default for ExportSettings {
    fn default() -> Self {
        Self {
            format: ExportFormat::Svg,
            output_dir: "visualizations".to_string(),
            filename_prefix: "class_graph".to_string(),
            include_metadata: true,
        }
    }
}

impl Default for NodeStyling {
    fn default() -> Self {
        Self {
            color_by_type: true,
            show_labels: true,
            node_size: NodeSize::Medium,
            font_size: 12,
        }
    }
}

impl VisualizationConfig {
    /// Load configuration from a JSON file
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: VisualizationConfig = serde_json::from_str(&contents)?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration from a JSON string
    pub fn from_json_str(json_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: VisualizationConfig = serde_json::from_str(json_str)?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration from environment variables
    pub fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::default();
        
        // Allow overriding max depth from environment
        if let Ok(max_depth) = std::env::var("ARMA3_VIZ_MAX_DEPTH") {
            config.graph_settings.max_depth = max_depth.parse()?;
        }
        
        // Allow overriding output directory from environment
        if let Ok(output_dir) = std::env::var("ARMA3_VIZ_OUTPUT_DIR") {
            config.export_settings.output_dir = output_dir;
        }
        
        // Allow adding preserved classes from environment
        if let Ok(preserved_classes) = std::env::var("ARMA3_VIZ_PRESERVED_CLASSES") {
            config.preserved_classes = preserved_classes
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
        
        // Allow adding excluded classes from environment
        if let Ok(excluded_classes) = std::env::var("ARMA3_VIZ_EXCLUDED_CLASSES") {
            config.excluded_classes = excluded_classes
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
        
        config.validate()?;
        Ok(config)
    }

    /// Save configuration to a JSON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Check if a class should be preserved
    pub fn is_preserved_class(&self, class_name: &str) -> bool {
        self.preserved_classes.iter().any(|preserved| class_name.starts_with(preserved))
    }

    /// Check if a class should be excluded
    pub fn is_excluded_class(&self, class_name: &str) -> bool {
        self.excluded_classes.iter().any(|excluded| class_name.starts_with(excluded))
    }

    /// Add a class to be preserved
    pub fn add_preserved_class(&mut self, class_name: String) {
        if !self.preserved_classes.contains(&class_name) {
            self.preserved_classes.push(class_name);
        }
    }

    /// Remove a preserved class
    pub fn remove_preserved_class(&mut self, class_name: &str) -> bool {
        if let Some(pos) = self.preserved_classes.iter().position(|c| c == class_name) {
            self.preserved_classes.remove(pos);
            true
        } else {
            false
        }
    }

    /// Add a class to be excluded
    pub fn add_excluded_class(&mut self, class_name: String) {
        if !self.excluded_classes.contains(&class_name) {
            self.excluded_classes.push(class_name);
        }
    }

    /// Remove an excluded class
    pub fn remove_excluded_class(&mut self, class_name: &str) -> bool {
        if let Some(pos) = self.excluded_classes.iter().position(|c| c == class_name) {
            self.excluded_classes.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get the maximum depth for graph visualization
    pub fn get_max_depth(&self) -> usize {
        self.graph_settings.max_depth
    }

    /// Set the maximum depth for graph visualization
    pub fn set_max_depth(&mut self, depth: usize) {
        self.graph_settings.max_depth = depth;
    }

    /// Get the export format
    pub fn get_export_format(&self) -> &ExportFormat {
        &self.export_settings.format
    }

    /// Set the export format
    pub fn set_export_format(&mut self, format: ExportFormat) {
        self.export_settings.format = format;
    }

    /// Get the output directory
    pub fn get_output_dir(&self) -> &str {
        &self.export_settings.output_dir
    }

    /// Set the output directory
    pub fn set_output_dir(&mut self, dir: String) {
        self.export_settings.output_dir = dir;
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check that max depth is reasonable
        if self.graph_settings.max_depth == 0 {
            return Err("Max depth must be greater than 0".into());
        }
        
        if self.graph_settings.max_depth > 20 {
            return Err("Max depth should not exceed 20 to avoid performance issues".into());
        }
        
        // Check that output directory is not empty
        if self.export_settings.output_dir.is_empty() {
            return Err("Output directory cannot be empty".into());
        }
        
        // Check that filename prefix is not empty
        if self.export_settings.filename_prefix.is_empty() {
            return Err("Filename prefix cannot be empty".into());
        }
        
        // Check that font size is reasonable
        if self.graph_settings.node_styling.font_size == 0 {
            return Err("Font size must be greater than 0".into());
        }
        
        if self.graph_settings.node_styling.font_size > 72 {
            return Err("Font size should not exceed 72 for readability".into());
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = VisualizationConfig::default();
        
        assert_eq!(config.graph_settings.max_depth, 5);
        assert!(config.graph_settings.show_inheritance);
        assert!(!config.graph_settings.show_compositions);
        assert_eq!(config.export_settings.output_dir, "visualizations");
        assert_eq!(config.export_settings.filename_prefix, "class_graph");
        assert!(config.export_settings.include_metadata);
    }

    #[test]
    fn test_validation() {
        let mut config = VisualizationConfig::default();
        
        // Valid config should pass
        assert!(config.validate().is_ok());
        
        // Zero max depth should fail
        config.graph_settings.max_depth = 0;
        assert!(config.validate().is_err());
        
        // Reset and test excessive max depth
        config.graph_settings.max_depth = 25;
        assert!(config.validate().is_err());
        
        // Reset and test empty output dir
        config.graph_settings.max_depth = 5;
        config.export_settings.output_dir = String::new();
        assert!(config.validate().is_err());
        
        // Reset and test empty filename prefix
        config.export_settings.output_dir = "test".to_string();
        config.export_settings.filename_prefix = String::new();
        assert!(config.validate().is_err());
        
        // Reset and test zero font size
        config.export_settings.filename_prefix = "test".to_string();
        config.graph_settings.node_styling.font_size = 0;
        assert!(config.validate().is_err());
        
        // Reset and test excessive font size
        config.graph_settings.node_styling.font_size = 100;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_class_management() {
        let mut config = VisualizationConfig::default();
        
        // Add preserved class
        config.add_preserved_class("TestClass".to_string());
        assert!(config.is_preserved_class("TestClass"));
        assert!(config.is_preserved_class("TestClass_Extended"));
        
        // Add excluded class
        config.add_excluded_class("ExcludedClass".to_string());
        assert!(config.is_excluded_class("ExcludedClass"));
        assert!(config.is_excluded_class("ExcludedClass_Extended"));
        
        // Remove preserved class
        let removed = config.remove_preserved_class("TestClass");
        assert!(removed);
        assert!(!config.is_preserved_class("TestClass"));
        
        // Remove excluded class
        let removed = config.remove_excluded_class("ExcludedClass");
        assert!(removed);
        assert!(!config.is_excluded_class("ExcludedClass"));
    }

    #[test]
    fn test_getters_and_setters() {
        let mut config = VisualizationConfig::default();
        
        // Test max depth
        config.set_max_depth(10);
        assert_eq!(config.get_max_depth(), 10);
        
        // Test export format
        config.set_export_format(ExportFormat::Png);
        assert!(matches!(config.get_export_format(), ExportFormat::Png));
        
        // Test output directory
        config.set_output_dir("test_output".to_string());
        assert_eq!(config.get_output_dir(), "test_output");
    }

    #[test]
    fn test_save_and_load_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("viz_config.json");
        
        let mut original_config = VisualizationConfig::default();
        original_config.add_preserved_class("TestClass".to_string());
        original_config.set_max_depth(10);
        
        // Save config
        original_config.save_to_file(&config_path).unwrap();
        
        // Load config
        let loaded_config = VisualizationConfig::from_json_file(&config_path).unwrap();
        
        assert_eq!(loaded_config.get_max_depth(), 10);
        assert!(loaded_config.is_preserved_class("TestClass"));
    }

    #[test]
    fn test_load_from_env() {
        // Set environment variables
        env::set_var("ARMA3_VIZ_MAX_DEPTH", "8");
        env::set_var("ARMA3_VIZ_OUTPUT_DIR", "/tmp/visualizations");
        env::set_var("ARMA3_VIZ_PRESERVED_CLASSES", "Class1, Class2, Class3");
        env::set_var("ARMA3_VIZ_EXCLUDED_CLASSES", "BadClass1, BadClass2");
        
        let config = VisualizationConfig::load_from_env().unwrap();
        
        assert_eq!(config.get_max_depth(), 8);
        assert_eq!(config.get_output_dir(), "/tmp/visualizations");
        assert!(config.is_preserved_class("Class1"));
        assert!(config.is_preserved_class("Class2"));
        assert!(config.is_preserved_class("Class3"));
        assert!(config.is_excluded_class("BadClass1"));
        assert!(config.is_excluded_class("BadClass2"));
        
        // Clean up
        env::remove_var("ARMA3_VIZ_MAX_DEPTH");
        env::remove_var("ARMA3_VIZ_OUTPUT_DIR");
        env::remove_var("ARMA3_VIZ_PRESERVED_CLASSES");
        env::remove_var("ARMA3_VIZ_EXCLUDED_CLASSES");
    }

    #[test]
    fn test_json_serialization() {
        let config = VisualizationConfig::default();
        
        // Test serialization to JSON string
        let json_str = serde_json::to_string_pretty(&config).unwrap();
        
        // Test deserialization from JSON string
        let loaded_config = VisualizationConfig::from_json_str(&json_str).unwrap();
        
        assert_eq!(loaded_config.get_max_depth(), config.get_max_depth());
        assert_eq!(loaded_config.get_output_dir(), config.get_output_dir());
    }
}