use std::path::Path;
use std::collections::HashMap;
use gamedata_scanner_models::{GameClass, PropertyValue};
use crate::models::MagazineWellInfo;

/// Handles extraction of magazine well information from configuration classes
pub struct MagazineExtractor;

impl MagazineExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract magazine wells from CfgMagazineWells class
    pub fn extract_magazine_wells(
        &self,
        cfg_magazine_wells: &GameClass,
        file: &Path,
        mod_source: &Option<String>,
        magazine_wells: &mut HashMap<String, MagazineWellInfo>
    ) {
        Self::extract_magazine_wells_static(cfg_magazine_wells, file, mod_source, magazine_wells)
    }

    /// Static version for use in spawned threads
    pub fn extract_magazine_wells_static(
        cfg_magazine_wells: &GameClass,
        file: &Path,
        mod_source: &Option<String>,
        magazine_wells: &mut HashMap<String, MagazineWellInfo>
    ) {
        // CfgMagazineWells is a class, its properties are the individual magazine wells.
        // Each property's name is the well_name, and its value is a PropertyValue::Class(well_class_definition)
        for prop in &cfg_magazine_wells.properties {
            let well_name = &prop.name;
            if let PropertyValue::Class(ref well_class_definition) = prop.value {
                if let Some(well_info) = Self::process_magazine_well_class_static(well_name, well_class_definition, file, mod_source) {
                    magazine_wells.insert(well_name.to_string(), well_info);
                }
            }
        }
    }

    /// Process a single magazine well class
    fn process_magazine_well_class_static(
        well_name: &str, // This is prop.name from the parent CfgMagazineWells
        well_class_definition: &GameClass, // This is the GameClass for the specific well
        file: &Path,
        mod_source: &Option<String>
    ) -> Option<MagazineWellInfo> {
        let mut magazines = HashMap::new();
        
        // Extract magazine entries from the properties of the well_class_definition
        // Each property here (e.g., "CBA_30Rnd_556x45_Stanag") is an array of magazine class names
        for prop in &well_class_definition.properties {
            let magazine_group_name = &prop.name; // e.g. "ACE_Magazines", "BI_Magazines"
            if let PropertyValue::Array(magazine_list_pv) = &prop.value {
                let magazine_names: Vec<String> = magazine_list_pv.iter()
                    .map(|name_str| name_str.trim_matches('"').to_string())
                    .collect();
                if !magazine_names.is_empty() {
                    magazines.insert(magazine_group_name.to_string(), magazine_names);
                }
            }
        }

        // Only create MagazineWellInfo if we found magazines
        if !magazines.is_empty() {
            Some(MagazineWellInfo {
                name: well_name.to_string(),
                file_path: file.to_path_buf(),
                magazines,
                mod_source: mod_source.clone(),
            })
        } else {
            None
        }
    }

    /// Get all magazine names from a magazine well
    pub fn get_all_magazines_from_well(well_info: &MagazineWellInfo) -> Vec<String> {
        let mut all_magazines = Vec::new();
        
        for magazine_list in well_info.magazines.values() {
            all_magazines.extend(magazine_list.clone());
        }
        
        all_magazines.sort();
        all_magazines.dedup();
        all_magazines
    }
}
