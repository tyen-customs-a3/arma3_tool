use std::path::Path;
use thiserror::Error;

use crate::{ScanConfig, ItemFilterConfig, VisualizationConfig};

/// Errors that can occur during configuration validation
#[derive(Error, Debug, Clone)]
pub enum ValidationError {
    #[error("Invalid scan configuration: {0}")]
    ScanConfig(String),
    
    #[error("Invalid item filter configuration: {0}")]
    ItemFilterConfig(String),
    
    #[error("Invalid visualization configuration: {0}")]
    VisualizationConfig(String),
    
    #[error("File system error: {0}")]
    FileSystem(String),
    
    #[error("Invalid path: {path}")]
    InvalidPath { path: String },
    
    #[error("Configuration conflict: {0}")]
    Conflict(String),
}

/// Trait for configuration validation
pub trait ConfigValidation {
    /// Validate the configuration
    fn validate(&self) -> Result<(), ValidationError>;
    
    /// Validate the configuration with detailed error reporting
    fn validate_detailed(&self) -> ValidationResult;
}

/// Result of configuration validation with detailed error information
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

impl ValidationResult {
    /// Create a new successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    
    /// Create a new failed validation result with errors
    pub fn with_errors(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    
    /// Add a warning to the validation result
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    /// Add a suggestion to the validation result
    pub fn add_suggestion(&mut self, suggestion: String) {
        self.suggestions.push(suggestion);
    }
    
    /// Add an error to the validation result
    pub fn add_error(&mut self, error: ValidationError) {
        self.errors.push(error);
        self.is_valid = false;
    }
    
    /// Check if the validation result has any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// Check if the validation result has any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
    
    /// Check if the validation result has any suggestions
    pub fn has_suggestions(&self) -> bool {
        !self.suggestions.is_empty()
    }
}

impl ConfigValidation for ScanConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let result = self.validate_detailed();
        if result.is_valid {
            Ok(())
        } else {
            Err(result.errors.into_iter().next().unwrap_or(
                ValidationError::ScanConfig("Unknown validation error".to_string())
            ))
        }
    }
    
    fn validate_detailed(&self) -> ValidationResult {
        let mut result = ValidationResult::success();
        
        // Check that at least one directory is specified
        if self.game_data_dirs.is_empty() && self.mission_dirs.is_empty() {
            result.add_error(ValidationError::ScanConfig(
                "At least one game data directory or mission directory must be specified".to_string()
            ));
        }
        
        // Check that thread count is reasonable
        if self.threads == 0 {
            result.add_error(ValidationError::ScanConfig(
                "Number of threads must be greater than 0".to_string()
            ));
        } else if self.threads > 64 {
            result.add_warning(
                "Very high thread count may cause performance issues".to_string()
            );
            result.add_suggestion(
                "Consider using a thread count closer to the number of CPU cores".to_string()
            );
        }
        
        // Validate directory paths
        for dir in &self.game_data_dirs {
            if !Path::new(dir).exists() {
                result.add_warning(format!("Game data directory does not exist: {}", dir));
            }
        }
        
        for dir in &self.mission_dirs {
            if !Path::new(dir).exists() {
                result.add_warning(format!("Mission directory does not exist: {}", dir));
            }
        }
        
        // Check cache directory
        if let Some(parent) = self.cache_dir.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                result.add_error(ValidationError::InvalidPath {
                    path: format!("Cache directory parent does not exist: {}", parent.display())
                });
            }
        }
        
        // Check report directory
        if let Some(parent) = self.report_dir.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                result.add_error(ValidationError::InvalidPath {
                    path: format!("Report directory parent does not exist: {}", parent.display())
                });
            }
        }
        
        // Check file extensions
        if self.game_data_extensions.is_empty() {
            result.add_warning("No game data file extensions specified".to_string());
        }
        
        if self.mission_extensions.is_empty() {
            result.add_warning("No mission file extensions specified".to_string());
        }
        
        // Check for common extension patterns
        for ext in &self.game_data_extensions {
            if ext.starts_with('.') {
                result.add_suggestion(format!(
                    "Extension '{}' starts with '.', consider removing it", ext
                ));
            }
        }
        
        for ext in &self.mission_extensions {
            if ext.starts_with('.') {
                result.add_suggestion(format!(
                    "Extension '{}' starts with '.', consider removing it", ext
                ));
            }
        }
        
        result
    }
}

impl ConfigValidation for ItemFilterConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let result = self.validate_detailed();
        if result.is_valid {
            Ok(())
        } else {
            Err(result.errors.into_iter().next().unwrap_or(
                ValidationError::ItemFilterConfig("Unknown validation error".to_string())
            ))
        }
    }
    
    fn validate_detailed(&self) -> ValidationResult {
        let mut result = ValidationResult::success();
        
        // Check version
        if self.version.is_empty() {
            result.add_error(ValidationError::ItemFilterConfig(
                "Configuration version is required".to_string()
            ));
        }
        
        // Check that at least one item type is defined
        if self.item_types.is_empty() {
            result.add_error(ValidationError::ItemFilterConfig(
                "At least one item type must be defined".to_string()
            ));
        }
        
        // Validate each item type
        for (type_name, type_config) in &self.item_types {
            if type_config.base_classes.is_empty() {
                result.add_error(ValidationError::ItemFilterConfig(
                    format!("Item type '{}' must have at least one base class", type_name)
                ));
            }
            
            // Check for common base class patterns
            for base_class in &type_config.base_classes {
                if base_class.is_empty() {
                    result.add_error(ValidationError::ItemFilterConfig(
                        format!("Empty base class found in item type '{}'", type_name)
                    ));
                }
                
                if base_class.contains(' ') {
                    result.add_warning(format!(
                        "Base class '{}' contains spaces, this may cause issues", base_class
                    ));
                }
            }
        }
        
        // Validate exclusion rules
        if self.exclusion_rules.max_scope < 0 || self.exclusion_rules.max_scope > 10 {
            result.add_error(ValidationError::ItemFilterConfig(
                "max_scope must be between 0 and 10".to_string()
            ));
        }
        
        // Check for empty excluded prefixes
        for prefix in &self.exclusion_rules.excluded_prefixes {
            if prefix.is_empty() {
                result.add_warning("Empty excluded prefix found".to_string());
            }
        }
        
        // Check for common item types
        let common_types = ["weapons", "uniforms", "vests", "backpacks", "helmets"];
        for common_type in &common_types {
            if !self.item_types.contains_key(*common_type) {
                result.add_suggestion(format!(
                    "Consider adding '{}' item type configuration", common_type
                ));
            }
        }
        
        result
    }
}

impl ConfigValidation for VisualizationConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        let result = self.validate_detailed();
        if result.is_valid {
            Ok(())
        } else {
            Err(result.errors.into_iter().next().unwrap_or(
                ValidationError::VisualizationConfig("Unknown validation error".to_string())
            ))
        }
    }
    
    fn validate_detailed(&self) -> ValidationResult {
        let mut result = ValidationResult::success();
        
        // Check max depth
        if self.graph_settings.max_depth == 0 {
            result.add_error(ValidationError::VisualizationConfig(
                "Max depth must be greater than 0".to_string()
            ));
        } else if self.graph_settings.max_depth > 20 {
            result.add_warning(
                "Very high max depth may cause performance issues".to_string()
            );
            result.add_suggestion(
                "Consider reducing max depth to 10 or less for better performance".to_string()
            );
        }
        
        // Check output directory
        if self.export_settings.output_dir.is_empty() {
            result.add_error(ValidationError::VisualizationConfig(
                "Output directory cannot be empty".to_string()
            ));
        }
        
        // Check filename prefix
        if self.export_settings.filename_prefix.is_empty() {
            result.add_error(ValidationError::VisualizationConfig(
                "Filename prefix cannot be empty".to_string()
            ));
        }
        
        // Check font size
        if self.graph_settings.node_styling.font_size == 0 {
            result.add_error(ValidationError::VisualizationConfig(
                "Font size must be greater than 0".to_string()
            ));
        } else if self.graph_settings.node_styling.font_size > 72 {
            result.add_warning(
                "Very large font size may affect readability".to_string()
            );
        } else if self.graph_settings.node_styling.font_size < 8 {
            result.add_warning(
                "Very small font size may affect readability".to_string()
            );
        }
        
        // Check for conflicting class lists
        for preserved in &self.preserved_classes {
            if self.excluded_classes.iter().any(|excluded| preserved == excluded) {
                result.add_error(ValidationError::Conflict(
                    format!("Class '{}' is both preserved and excluded", preserved)
                ));
            }
        }
        
        // Check for empty class names
        for preserved in &self.preserved_classes {
            if preserved.is_empty() {
                result.add_warning("Empty preserved class name found".to_string());
            }
        }
        
        for excluded in &self.excluded_classes {
            if excluded.is_empty() {
                result.add_warning("Empty excluded class name found".to_string());
            }
        }
        
        result
    }
}

/// Validate multiple configurations together for conflicts
pub fn validate_configs(
    scan_config: &ScanConfig,
    item_filter_config: &ItemFilterConfig,
    viz_config: &VisualizationConfig,
) -> ValidationResult {
    let mut result = ValidationResult::success();
    
    // Validate individual configurations
    let scan_result = scan_config.validate_detailed();
    let filter_result = item_filter_config.validate_detailed();
    let viz_result = viz_config.validate_detailed();
    
    // Combine results
    result.errors.extend(scan_result.errors);
    result.errors.extend(filter_result.errors);
    result.errors.extend(viz_result.errors);
    
    result.warnings.extend(scan_result.warnings);
    result.warnings.extend(filter_result.warnings);
    result.warnings.extend(viz_result.warnings);
    
    result.suggestions.extend(scan_result.suggestions);
    result.suggestions.extend(filter_result.suggestions);
    result.suggestions.extend(viz_result.suggestions);
    
    if !result.errors.is_empty() {
        result.is_valid = false;
    }
    
    // Check for cross-configuration conflicts
    
    // Check if visualization output directory conflicts with scan cache/report directories
    let viz_output_path = Path::new(&viz_config.export_settings.output_dir);
    if viz_output_path == scan_config.cache_dir {
        result.add_error(ValidationError::Conflict(
            "Visualization output directory conflicts with scan cache directory".to_string()
        ));
    }
    
    if viz_output_path == scan_config.report_dir {
        result.add_error(ValidationError::Conflict(
            "Visualization output directory conflicts with scan report directory".to_string()
        ));
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ScanConfig, ItemFilterConfig, VisualizationConfig};
    
    #[test]
    fn test_scan_config_validation() {
        let mut config = ScanConfig::default();
        
        // Should fail with no directories
        let result = config.validate_detailed();
        assert!(!result.is_valid);
        assert!(result.has_errors());
        
        // Should pass with game data dir
        config.game_data_dirs.push("test".to_string());
        let result = config.validate_detailed();
        assert!(result.is_valid);
        
        // Should fail with zero threads
        config.threads = 0;
        let result = config.validate_detailed();
        assert!(!result.is_valid);
        
        // Should warn with high thread count
        config.threads = 128;
        let result = config.validate_detailed();
        assert!(result.has_warnings());
    }
    
    #[test]
    fn test_item_filter_config_validation() {
        let mut config = ItemFilterConfig::default();
        
        // Should pass with default config
        let result = config.validate_detailed();
        assert!(result.is_valid);
        
        // Should fail with empty version
        config.version = String::new();
        let result = config.validate_detailed();
        assert!(!result.is_valid);
        
        // Should fail with invalid max_scope
        config.version = "1.0".to_string();
        config.exclusion_rules.max_scope = -1;
        let result = config.validate_detailed();
        assert!(!result.is_valid);
    }
    
    #[test]
    fn test_visualization_config_validation() {
        let mut config = VisualizationConfig::default();
        
        // Should pass with default config
        let result = config.validate_detailed();
        assert!(result.is_valid);
        
        // Should fail with zero max depth
        config.graph_settings.max_depth = 0;
        let result = config.validate_detailed();
        assert!(!result.is_valid);
        
        // Should fail with empty output dir
        config.graph_settings.max_depth = 5;
        config.export_settings.output_dir = String::new();
        let result = config.validate_detailed();
        assert!(!result.is_valid);
    }
    
    #[test]
    fn test_config_conflicts() {
        let mut scan_config = ScanConfig::default();
        scan_config.game_data_dirs.push("test_dir".to_string()); // Make it valid
        let item_filter_config = ItemFilterConfig::default();
        let mut viz_config = VisualizationConfig::default();
        
        // Should pass with no conflicts
        let result = validate_configs(&scan_config, &item_filter_config, &viz_config);
        assert!(result.is_valid);
        
        // Should fail with conflicting preserved/excluded classes
        viz_config.preserved_classes.push("TestClass".to_string());
        viz_config.excluded_classes.push("TestClass".to_string());
        let result = validate_configs(&scan_config, &item_filter_config, &viz_config);
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }
    
    #[test]
    fn test_validation_result_methods() {
        let mut result = ValidationResult::success();
        
        assert!(result.is_valid);
        assert!(!result.has_errors());
        assert!(!result.has_warnings());
        assert!(!result.has_suggestions());
        
        result.add_warning("Test warning".to_string());
        assert!(result.has_warnings());
        
        result.add_suggestion("Test suggestion".to_string());
        assert!(result.has_suggestions());
        
        result.add_error(ValidationError::ScanConfig("Test error".to_string()));
        assert!(!result.is_valid);
        assert!(result.has_errors());
    }
}