use std::path::Path;
use gamedata_scanner_models::{GameClass, PropertyValue};
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
        let has_magazine_well = class.properties.iter().any(|p| p.name == "magazineWell");
        
        if !has_magazine_well {
            // Also check if any nested class within properties might define magazineWell
            // This is a common pattern for LinkedItems etc.
            let nested_has_magazine_well = class.properties.iter().any(|p| {
                if let PropertyValue::Class(ref nested_class) = p.value {
                    nested_class.properties.iter().any(|np| np.name == "magazineWell")
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
        for prop in &class.properties {
            if prop.name == "magazineWell" {
                if let PropertyValue::Array(wells_pv) = &prop.value {
                    for well_name_str in wells_pv {
                        magazine_wells.push(well_name_str.trim_matches('"').to_string());
                    }
                }
            }
            // Check for nested magazineWell in Class type properties (e.g. LinkedItems)
            else if let PropertyValue::Class(ref nested_class) = prop.value {
                for nested_prop in &nested_class.properties {
                    if nested_prop.name == "magazineWell" {
                        if let PropertyValue::Array(wells_pv) = &nested_prop.value {
                            for well_name_str in wells_pv {
                                magazine_wells.push(well_name_str.trim_matches('"').to_string());
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
        for prop in &class.properties {
            if prop.name == "weapons" {
                if let PropertyValue::Array(weapons_pv) = &prop.value {
                    let weapon_names: Vec<String> = weapons_pv.iter()
                        .map(|name_str| name_str.trim_matches('"').to_string())
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
