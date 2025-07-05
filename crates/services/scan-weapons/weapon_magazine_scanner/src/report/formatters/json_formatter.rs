use anyhow::Result;
use serde_json;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use crate::models::ScanResult;
use super::ReportFormatter;

/// JSON report formatter
pub struct JsonFormatter;

impl JsonFormatter {
    pub fn new() -> Self {
        Self
    }



    /// Format with new magwell-centric structure
    pub fn format_with_project_root(&self, result: &ScanResult, _project_root: &Path) -> Result<String> {
        // Filter out weapons with no magazine wells
        let weapons_with_magazine_wells: Vec<_> = result.weapons.iter()
            .filter(|w| !w.magazine_wells.is_empty())
            .collect();

        // Build unique magazines map and magwell compatibility
        let mut magazines_map: HashMap<String, serde_json::Value> = HashMap::new();
        let mut magwell_compatibility: HashMap<String, Vec<String>> = HashMap::new();

        for (magwell_name, well_info) in &result.magazine_wells {
            let mut all_magazines_for_well: HashSet<String> = HashSet::new();
            
            // Collect all magazines for this magwell
            for magazine_list in well_info.magazines.values() {
                for magazine in magazine_list {
                    all_magazines_for_well.insert(magazine.clone());
                }
            }
            
            // Convert to sorted vector
            let mut magazine_list: Vec<String> = all_magazines_for_well.into_iter().collect();
            magazine_list.sort();
            
            magwell_compatibility.insert(magwell_name.clone(), magazine_list.clone());
            
            // Add magazines to global magazines map
            for mag_class_name in magazine_list {
                if !magazines_map.contains_key(&mag_class_name) {
                    magazines_map.insert(mag_class_name.clone(), serde_json::json!({
                        "class_name": mag_class_name,
                        "display_name": mag_class_name, // Fallback to class_name
                        "magwell": magwell_name,
                        "mod_source": well_info.mod_source
                    }));
                }
            }
        }

        // Build weapons object (as map, not array)
        let mut weapons_object = serde_json::Map::new();
        for weapon in &weapons_with_magazine_wells {
            weapons_object.insert(weapon.name.clone(), serde_json::json!({
                "class_name": weapon.name,
                "display_name": weapon.name, // Fallback to class_name for now
                "weapon_type": serde_json::Value::Null, // No longer derive weapon type from class names
                "magwells": weapon.magazine_wells,
                "mod_source": weapon.mod_source
            }));
        }

        // Build magazines object (as map, not array)
        let magazines_object: serde_json::Map<String, serde_json::Value> = magazines_map.into_iter().collect();

        // Create the new report structure
        let report = serde_json::json!({
            "metadata": {
                "version": "2.0.0",
                "timestamp": result.scan_timestamp,
                "source": "ARMA 3 Community Database",
                "weapon_count": weapons_with_magazine_wells.len(),
                "magazine_count": magazines_object.len(),
                "description": "Simplified magwell-centric compatibility database for Mission Patcher"
            },
            "weapons": weapons_object,
            "magazines": magazines_object,
            "magwell_compatibility": magwell_compatibility
        });

        Ok(serde_json::to_string_pretty(&report)?)
    }

    /// Format with filtering for relevant weapons only (legacy method)
    fn format_filtered(&self, result: &ScanResult) -> Result<String> {
        // For backwards compatibility, try to use current directory as project root
        let current_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        self.format_with_project_root(result, &current_dir)
    }
}

impl ReportFormatter for JsonFormatter {
    fn format(&self, result: &ScanResult) -> Result<String> {
        self.format_filtered(result)
    }

    fn format_name(&self) -> &'static str {
        "JSON"
    }

    fn file_extension(&self) -> &'static str {
        "json"
    }
}
