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

    /// Validate the configuration
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>> {
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
    fn test_config_loading_from_file() {
        // Test loading our actual config file
        let config = ItemFilterConfig::from_json_file("item_filter_config.json");
        
        // Should load successfully
        assert!(config.is_ok());
        
        let config = config.unwrap();
        assert_eq!(config.version, "1.0");
        assert!(config.item_types.contains_key("weapons"));
        assert!(config.item_types.contains_key("uniforms"));
        assert!(config.item_types.contains_key("vests"));
        assert!(config.item_types.contains_key("backpacks"));
        
        // Check weapons base classes
        let weapons = config.get_base_classes("weapons").unwrap();
        assert!(weapons.contains(&"Rifle_Base_F".to_string()));
        assert!(weapons.contains(&"Pistol_Base_F".to_string()));
        assert!(weapons.contains(&"Launcher_Base_F".to_string()));
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
} 