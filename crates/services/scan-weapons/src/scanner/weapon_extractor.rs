use std::path::Path;
use arma3_types::{GameClass};
use crate::models::WeaponInfo;

/// Handles extraction of weapon information from configuration classes
pub struct WeaponExtractor;

impl WeaponExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract weapon information from a game class
    pub fn extract_weapon_info(&self, class: &GameClass, file: &Path) -> Option<WeaponInfo> {
        Self::extract_weapon_info_static(class, file)
    }

    /// Static version for use in spawned threads
    pub fn extract_weapon_info_static(class: &GameClass, file: &Path) -> Option<WeaponInfo> {
        let name = class.name.clone();
        
        // Check if this class has magazineWell property (indicates it's a weapon)
        // Updated to iterate properties and check name, removed children access
        let has_magazine_well = class.properties.iter().any(|(prop_name, _)| prop_name == "magazineWell");
        
        if !has_magazine_well {
            // Also check if any nested class within properties might define magazineWell
            // This is a common pattern for LinkedItems etc.
            let nested_has_magazine_well = class.properties.iter().any(|(_, prop_value)| {
                if let arma3_types::PropertyValue::Object(ref nested_class_map) = prop_value {
                    nested_class_map.iter().any(|(nested_prop_name, _)| nested_prop_name == "magazineWell")
                } else {
                    false
                }
            });
            if !nested_has_magazine_well {
                return None;
            }
        }

        let parent = class.parent.clone();
        let magazine_wells = Self::extract_magazine_wells_from_class(class);
        
        // Only return weapons that have magazine wells
        if magazine_wells.is_empty() {
            return None;
        }

        Some(WeaponInfo {
            name,
            parent,
            file_path: file.to_path_buf(),
            magazine_wells,
            compatible_magazines: Vec::new(),
            mod_source: None, // This will be set by the caller
        })
    }

    /// Extract magazine wells from weapon class properties
    fn extract_magazine_wells_from_class(class: &GameClass) -> Vec<String> {
        let mut magazine_wells = Vec::new();
        
        // Iterate through properties to find "magazineWell"
        for (prop_name, prop_value) in &class.properties {
            if prop_name == "magazineWell" {
                if let arma3_types::PropertyValue::Array(wells_pv) = prop_value {
                    for well_name_str in wells_pv {
                        if let arma3_types::PropertyValue::String(s) = well_name_str {
                            magazine_wells.push(s.trim_matches('"').to_string());
                        }
                    }
                }
            }
            // Check for nested magazineWell in Class type properties (e.g. LinkedItems)
            else if let arma3_types::PropertyValue::Object(ref nested_class_map) = prop_value {
                for (nested_prop_name, nested_prop_value) in nested_class_map {
                    if nested_prop_name == "magazineWell" {
                        if let arma3_types::PropertyValue::Array(wells_pv) = nested_prop_value {
                            for well_name_str in wells_pv {
                                if let arma3_types::PropertyValue::String(s) = well_name_str {
                                    magazine_wells.push(s.trim_matches('"').to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        magazine_wells.sort();
        magazine_wells.dedup();
        magazine_wells
    }

    /// Extract declared weapons from CfgPatches
    pub fn extract_declared_weapons(&self, class: &GameClass) -> Option<Vec<String>> {
        Self::extract_declared_weapons_static(class)
    }

    /// Static version for extracting declared weapons
    pub fn extract_declared_weapons_static(class: &GameClass) -> Option<Vec<String>> {
        // Iterate through properties to find "weapons"
        for (prop_name, prop_value) in &class.properties {
            if prop_name == "weapons" {
                if let arma3_types::PropertyValue::Array(weapons_pv) = prop_value {
                    let weapon_names: Vec<String> = weapons_pv.iter()
                        .filter_map(|name_val| {
                            if let arma3_types::PropertyValue::String(s) = name_val {
                                Some(s.trim_matches('"').to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                    
                    if !weapon_names.is_empty() {
                        return Some(weapon_names);
                    }
                }
            }
        }
        None
    }
}
