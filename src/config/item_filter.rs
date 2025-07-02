use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Configuration for item filtering and categorization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemFilterConfig {
    pub version: String,
    pub description: String,
    pub item_types: HashMap<String, ItemTypeConfig>,
    pub exclusion_rules: ExclusionRules,
    pub validation_rules: ValidationRules,
}

/// Configuration for a specific item type (weapons, backpacks, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemTypeConfig {
    pub description: String,
    pub base_classes: Vec<String>,
    pub required_properties: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub optional_properties: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub required_nested_properties: HashMap<String, String>,
}

/// Rules for excluding classes from exports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExclusionRules {
    pub description: String,
    pub excluded_base_classes: Vec<String>,
    pub excluded_prefixes: Vec<String>,
    pub max_scope: i32,
}

/// Validation rules for the filtering system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRules {
    pub description: String,
    pub global_requirements: HashMap<String, PropertyRequirement>,
    pub inheritance_depth_limit: i32,
    pub cache_inheritance_results: bool,
}

/// Requirement for a specific property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyRequirement {
    pub operator: String,
    pub value: serde_json::Value,
    pub description: String,
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

    /// Check if a base class should be excluded
    pub fn is_excluded_base_class(&self, class_name: &str) -> bool {
        self.exclusion_rules.excluded_base_classes.contains(&class_name.to_string())
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

        // Validate inheritance depth limit is reasonable
        if self.validation_rules.inheritance_depth_limit <= 0 || self.validation_rules.inheritance_depth_limit > 1000 {
            return Err("Inheritance depth limit must be between 1 and 1000".into());
        }

        Ok(())
    }
}

impl Default for ItemFilterConfig {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            description: "Default item filtering configuration".to_string(),
            item_types: HashMap::new(),
            exclusion_rules: ExclusionRules {
                description: "Default exclusion rules".to_string(),
                excluded_base_classes: vec![
                    "RscConfActionButton".to_string(),
                    "RscActiveText".to_string(),
                    "RscButton".to_string(),
                    "B_Side".to_string(),
                    "BaseWest".to_string(),
                ],
                excluded_prefixes: vec![
                    "B_hub".to_string(),
                    "B_m0".to_string(),
                ],
                max_scope: 1,
            },
            validation_rules: ValidationRules {
                description: "Default validation rules".to_string(),
                global_requirements: HashMap::new(),
                inheritance_depth_limit: 50,
                cache_inheritance_results: true,
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
            "description": "Test configuration",
            "item_types": {
                "weapons": {
                    "description": "All weapon types",
                    "base_classes": ["Weapon_Base_F", "Rifle_Base_F"],
                    "required_properties": {
                        "scope": 2
                    }
                }
            },
            "exclusion_rules": {
                "description": "Test exclusion rules",
                "excluded_base_classes": ["RscActiveText"],
                "excluded_prefixes": ["B_hub"],
                "max_scope": 1
            },
            "validation_rules": {
                "description": "Test validation rules",
                "global_requirements": {},
                "inheritance_depth_limit": 50,
                "cache_inheritance_results": true
            }
        }
        "#;

        let config = ItemFilterConfig::from_json_str(json_config).unwrap();
        
        assert_eq!(config.version, "1.0");
        assert_eq!(config.item_types.len(), 1);
        
        let weapons_config = config.get_item_type_config("weapons").unwrap();
        assert_eq!(weapons_config.base_classes.len(), 2);
        assert!(weapons_config.base_classes.contains(&"Weapon_Base_F".to_string()));
        
        assert!(config.is_excluded_base_class("RscActiveText"));
        assert!(!config.is_excluded_base_class("Weapon_Base_F"));
        
        assert!(config.is_excluded_by_prefix("B_hub_test"));
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
            ..Default::default()
        };
        
        assert!(ItemFilterConfig::from_json_str(&serde_json::to_string(&invalid_config2).unwrap()).is_err());
    }

    #[test]
    fn test_default_config() {
        let config = ItemFilterConfig::default();
        
        assert_eq!(config.version, "1.0");
        assert!(config.exclusion_rules.excluded_base_classes.contains(&"RscActiveText".to_string()));
        assert_eq!(config.validation_rules.inheritance_depth_limit, 50);
    }
} 