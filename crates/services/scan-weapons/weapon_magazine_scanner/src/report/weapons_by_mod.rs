use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::fs;
use crate::models::{ScanResult, WeaponInfo};

/// Handles exporting weapons grouped by their source mod
pub struct WeaponsByModExporter;

impl WeaponsByModExporter {
    pub fn new() -> Self {
        Self
    }

    /// Export weapons grouped by mod to a simple text file
    pub fn export_weapons_by_mod(&self, result: &ScanResult, output: &Path) -> Result<()> {
        // Create parent directory if needed
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }

        // Group weapons by mod_source and remove duplicates
        let weapons_by_mod = self.group_weapons_by_mod(&result.weapons);

        // Generate the simple text content
        let content = self.format_weapons_by_mod_text(&weapons_by_mod)?;

        // Write to file
        fs::write(output, content)?;
        log::info!("Weapons by mod report generated: {}", output.display());
        
        Ok(())
    }

    /// Group weapons by their mod_source, removing duplicates
    fn group_weapons_by_mod<'a>(&self, weapons: &'a [WeaponInfo]) -> HashMap<String, HashSet<&'a str>> {
        let mut grouped: HashMap<String, HashSet<&str>> = HashMap::new();

        for weapon in weapons {
            // Filter out weapons with no magazine wells as they're not useful
            if weapon.magazine_wells.is_empty() {
                continue;
            }

            let mod_name = weapon.mod_source.as_deref().unwrap_or("Unknown").to_string();
            grouped.entry(mod_name).or_insert_with(HashSet::new).insert(&weapon.name);
        }

        grouped
    }

    /// Format the grouped weapons data as simple indented text
    fn format_weapons_by_mod_text(&self, weapons_by_mod: &HashMap<String, HashSet<&str>>) -> Result<String> {
        let mut content = String::new();

        // Add header
        content.push_str("Weapons by Mod\n");
        content.push_str("==============\n\n");

        // Sort mods by name for consistent output
        let mut mod_names: Vec<&String> = weapons_by_mod.keys().collect();
        mod_names.sort();

        for mod_name in mod_names {
            let weapons = &weapons_by_mod[mod_name];
            
            // Add mod header
            content.push_str(&format!("{}:\n", mod_name));
            
            // Sort weapons within each mod
            let mut weapon_names: Vec<&str> = weapons.iter().copied().collect();
            weapon_names.sort();
            
            // Add weapons with indentation
            for weapon_name in weapon_names {
                content.push_str(&format!("    {}\n", weapon_name));
            }
            
            content.push('\n'); // Empty line between mods
        }

        Ok(content)
    }

    /// Get summary statistics for weapons by mod
    pub fn get_mod_statistics(&self, weapons: &[WeaponInfo]) -> ModStatistics {
        let weapons_by_mod = self.group_weapons_by_mod(weapons);
        
        let total_mods = weapons_by_mod.len();
        let total_weapons: usize = weapons_by_mod.values().map(|weapons| weapons.len()).sum();
        
        let mod_with_most_weapons = weapons_by_mod.iter()
            .max_by_key(|(_, weapons)| weapons.len())
            .map(|(mod_name, weapons)| (mod_name.clone(), weapons.len()));

        let avg_weapons_per_mod = if total_mods > 0 {
            total_weapons as f64 / total_mods as f64
        } else {
            0.0
        };

        ModStatistics {
            total_mods,
            total_weapons,
            avg_weapons_per_mod,
            mod_with_most_weapons,
        }
    }
}

/// Statistics about weapon distribution across mods
#[derive(Debug, Clone)]
pub struct ModStatistics {
    pub total_mods: usize,
    pub total_weapons: usize,
    pub avg_weapons_per_mod: f64,
    pub mod_with_most_weapons: Option<(String, usize)>,
}

impl Default for WeaponsByModExporter {
    fn default() -> Self {
        Self::new()
    }
} 