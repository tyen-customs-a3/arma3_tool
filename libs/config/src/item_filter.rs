use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Configuration for item filtering and categorization
/// Matches the JSON structure specified in the PRD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFilterConfig {
    pub version: String,
    pub item_types: HashMap<String, ItemTypeConfig>,
    pub exclusion_rules: ExclusionRules,
}

/// Configuration for a specific item type (weapons, backpacks, etc.)
/// Simplified structure matching PRD specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeConfig {
    pub base_classes: Vec<String>,
}

impl ItemTypeConfig {
    /// Create a new item type config with a single base class
    pub fn new(base_class: String) -> Self {
        Self {
            base_classes: vec![base_class],
        }
    }

    /// Create a new item type config with multiple base classes
    pub fn with_base_classes(base_classes: Vec<String>) -> Self {
        Self { base_classes }
    }
}

/// Rules for excluding classes from exports
/// Simplified structure matching PRD specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExclusionRules {
    pub max_scope: i32,
    pub excluded_prefixes: Vec<String>,
}

impl ItemFilterConfig {
    /// Load configuration from a JSON file
    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: ItemFilterConfig = serde_json::from_str(&contents)?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration from a JSON string
    pub fn from_json_str(json_str: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: ItemFilterConfig = serde_json::from_str(json_str)?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration from environment variables
    pub fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Self::default();
        
        // Allow overriding max_scope from environment
        if let Ok(max_scope) = std::env::var("ARMA3_MAX_SCOPE") {
            config.exclusion_rules.max_scope = max_scope.parse()?;
        }
        
        // Allow adding excluded prefixes from environment
        if let Ok(excluded_prefixes) = std::env::var("ARMA3_EXCLUDED_PREFIXES") {
            let additional_prefixes: Vec<String> = excluded_prefixes
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            config.exclusion_rules.excluded_prefixes.extend(additional_prefixes);
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

    /// Get base classes for a specific item type
    pub fn get_base_classes(&self, item_type: &str) -> Option<&Vec<String>> {
        self.item_types.get(item_type).map(|config| &config.base_classes)
    }

    /// Check if a class name matches excluded prefixes
    pub fn is_excluded_by_prefix(&self, class_name: &str) -> bool {
        self.exclusion_rules.excluded_prefixes.iter()
            .any(|prefix| class_name.starts_with(prefix))
    }

    /// Get all defined item types
    pub fn get_item_types(&self) -> Vec<&String> {
        self.item_types.keys().collect()
    }

    /// Get configuration for a specific item type
    pub fn get_item_type_config(&self, item_type: &str) -> Option<&ItemTypeConfig> {
        self.item_types.get(item_type)
    }

    /// Get the maximum scope value for exclusion
    pub fn get_max_scope(&self) -> i32 {
        self.exclusion_rules.max_scope
    }

    /// Add a new item type with base classes
    pub fn add_item_type(&mut self, item_type: String, base_classes: Vec<String>) {
        self.item_types.insert(item_type, ItemTypeConfig { base_classes });
    }

    /// Remove an item type
    pub fn remove_item_type(&mut self, item_type: &str) -> Option<ItemTypeConfig> {
        self.item_types.remove(item_type)
    }

    /// Add an excluded prefix
    pub fn add_excluded_prefix(&mut self, prefix: String) {
        if !self.exclusion_rules.excluded_prefixes.contains(&prefix) {
            self.exclusion_rules.excluded_prefixes.push(prefix);
        }
    }

    /// Remove an excluded prefix
    pub fn remove_excluded_prefix(&mut self, prefix: &str) -> bool {
        if let Some(pos) = self.exclusion_rules.excluded_prefixes.iter().position(|p| p == prefix) {
            self.exclusion_rules.excluded_prefixes.remove(pos);
            true
        } else {
            false
        }
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Check that version is specified
        if self.version.is_empty() {
            return Err("Configuration version is required".into());
        }

        // Check that at least one item type is defined
        if self.item_types.is_empty() {
            return Err("At least one item type must be defined".into());
        }

        // Validate each item type has at least one base class
        for (type_name, type_config) in &self.item_types {
            if type_config.base_classes.is_empty() {
                return Err(format!("Item type '{}' must have at least one base class", type_name).into());
            }
        }

        // Validate max_scope is reasonable
        if self.exclusion_rules.max_scope < 0 || self.exclusion_rules.max_scope > 10 {
            return Err("max_scope must be between 0 and 10".into());
        }

        Ok(())
    }
}

impl Default for ItemFilterConfig {
    fn default() -> Self {
        let mut item_types = HashMap::new();
        
        // Add default item types as specified in PRD
        item_types.insert("weapons".to_string(), ItemTypeConfig {
            base_classes: vec![
                "Rifle_Base_F".to_string(),
                "Pistol_Base_F".to_string(),
                "Launcher_Base_F".to_string(),
            ],
        });
        
        item_types.insert("uniforms".to_string(), ItemTypeConfig {
            base_classes: vec![
                "Uniform_Base".to_string(),
                "U_BasicBody".to_string(),
            ],
        });
        
        item_types.insert("vests".to_string(), ItemTypeConfig {
            base_classes: vec![
                "Vest_Base".to_string(),
                "Vest_Camo_Base".to_string(),
            ],
        });
        
        item_types.insert("backpacks".to_string(), ItemTypeConfig {
            base_classes: vec![
                "Bag_Base".to_string(),
            ],
        });

        Self {
            version: "1.0".to_string(),
            item_types,
            exclusion_rules: ExclusionRules {
                max_scope: 1,
                excluded_prefixes: vec![
                    "B_soldier_f".to_string(),
                ],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::tempdir;

    #[test]
    fn test_config_loading_from_json_string() {
        let json_config = r#"
        {
            "version": "1.0",
            "item_types": {
                "weapons": {
                    "base_classes": ["Rifle_Base_F", "Pistol_Base_F"]
                },
                "uniforms": {
                    "base_classes": ["Uniform_Base", "U_BasicBody"]
                }
            },
            "exclusion_rules": {
                "max_scope": 1,
                "excluded_prefixes": ["B_soldier_f"]
            }
        }
        "#;

        let config = ItemFilterConfig::from_json_str(json_config).unwrap();
        
        assert_eq!(config.version, "1.0");
        assert_eq!(config.item_types.len(), 2);
        
        let weapons_config = config.get_item_type_config("weapons").unwrap();
        assert_eq!(weapons_config.base_classes.len(), 2);
        assert!(weapons_config.base_classes.contains(&"Rifle_Base_F".to_string()));
        
        assert_eq!(config.get_max_scope(), 1);
        
        assert!(config.is_excluded_by_prefix("B_soldier_f_test"));
        assert!(!config.is_excluded_by_prefix("arifle_MX_F"));
    }

    #[test]
    fn test_config_validation() {
        // Test empty version
        let invalid_config = ItemFilterConfig {
            version: "".to_string(),
            ..Default::default()
        };
        
        assert!(ItemFilterConfig::from_json_str(&serde_json::to_string(&invalid_config).unwrap()).is_err());
        
        // Test empty item types
        let invalid_config2 = ItemFilterConfig {
            version: "1.0".to_string(),
            item_types: HashMap::new(),
            exclusion_rules: ExclusionRules {
                max_scope: 1,
                excluded_prefixes: vec![],
            },
        };
        
        assert!(ItemFilterConfig::from_json_str(&serde_json::to_string(&invalid_config2).unwrap()).is_err());
    }

    #[test]
    fn test_default_config() {
        let config = ItemFilterConfig::default();
        
        assert_eq!(config.version, "1.0");
        assert_eq!(config.item_types.len(), 4);
        assert!(config.item_types.contains_key("weapons"));
        assert!(config.item_types.contains_key("uniforms"));
        assert!(config.item_types.contains_key("vests"));
        assert!(config.item_types.contains_key("backpacks"));
        
        assert_eq!(config.exclusion_rules.max_scope, 1);
        assert!(config.exclusion_rules.excluded_prefixes.contains(&"B_soldier_f".to_string()));
    }

    #[test]
    fn test_exclusion_methods() {
        let config = ItemFilterConfig::default();
        
        // Test prefix exclusion
        assert!(config.is_excluded_by_prefix("B_soldier_f_test"));
        assert!(!config.is_excluded_by_prefix("arifle_MX_F"));
        
        // Test max scope
        assert_eq!(config.get_max_scope(), 1);
    }

    #[test]
    fn test_item_type_access() {
        let config = ItemFilterConfig::default();
        
        // Test getting item types
        let item_types = config.get_item_types();
        assert_eq!(item_types.len(), 4);
        
        // Test getting base classes
        let weapon_bases = config.get_base_classes("weapons").unwrap();
        assert!(weapon_bases.contains(&"Rifle_Base_F".to_string()));
        
        // Test non-existent item type
        assert!(config.get_base_classes("non_existent").is_none());
    }

    #[test]
    fn test_modification_methods() {
        let mut config = ItemFilterConfig::default();
        
        // Add new item type
        config.add_item_type("helmets".to_string(), vec!["Helmet_Base".to_string()]);
        assert!(config.item_types.contains_key("helmets"));
        
        // Remove item type
        let removed = config.remove_item_type("helmets");
        assert!(removed.is_some());
        assert!(!config.item_types.contains_key("helmets"));
        
        // Add excluded prefix
        config.add_excluded_prefix("test_prefix".to_string());
        assert!(config.exclusion_rules.excluded_prefixes.contains(&"test_prefix".to_string()));
        
        // Remove excluded prefix
        let removed = config.remove_excluded_prefix("test_prefix");
        assert!(removed);
        assert!(!config.exclusion_rules.excluded_prefixes.contains(&"test_prefix".to_string()));
    }

    #[test]
    fn test_save_and_load_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("test_config.json");
        
        let original_config = ItemFilterConfig::default();
        
        // Save config
        original_config.save_to_file(&config_path).unwrap();
        
        // Load config
        let loaded_config = ItemFilterConfig::from_json_file(&config_path).unwrap();
        
        assert_eq!(loaded_config.version, original_config.version);
        assert_eq!(loaded_config.item_types.len(), original_config.item_types.len());
    }

    #[test]
    fn test_load_from_env() {
        // Set environment variables
        env::set_var("ARMA3_MAX_SCOPE", "2");
        env::set_var("ARMA3_EXCLUDED_PREFIXES", "prefix1, prefix2, prefix3");
        
        let config = ItemFilterConfig::load_from_env().unwrap();
        
        assert_eq!(config.exclusion_rules.max_scope, 2);
        assert!(config.exclusion_rules.excluded_prefixes.contains(&"prefix1".to_string()));
        assert!(config.exclusion_rules.excluded_prefixes.contains(&"prefix2".to_string()));
        assert!(config.exclusion_rules.excluded_prefixes.contains(&"prefix3".to_string()));
        
        // Clean up
        env::remove_var("ARMA3_MAX_SCOPE");
        env::remove_var("ARMA3_EXCLUDED_PREFIXES");
    }
}