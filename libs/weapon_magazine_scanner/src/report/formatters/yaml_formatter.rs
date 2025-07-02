use anyhow::Result;
use crate::models::ScanResult;
use super::ReportFormatter;

/// YAML report formatter
pub struct YamlFormatter;

impl YamlFormatter {
    pub fn new() -> Self {
        Self
    }

    fn format_weapons_section(&self, result: &ScanResult) -> String {
        let mut yaml = String::new();
        yaml.push_str("weapons:\n");
        
        for weapon in &result.weapons {
            if weapon.magazine_wells.is_empty() {
                continue;
            }
            
            yaml.push_str(&format!("  {}:\n", weapon.name));
            yaml.push_str(&format!("    file: \"{}\"\n", weapon.file_path.display()));
            
            if let Some(ref parent) = weapon.parent {
                yaml.push_str(&format!("    parent: \"{}\"\n", parent));
            }
            
            yaml.push_str("    magazine_wells:\n");
            for well in &weapon.magazine_wells {
                yaml.push_str(&format!("      - \"{}\"\n", well));
            }
            
            yaml.push_str("    compatible_magazines:\n");
            for mag in &weapon.compatible_magazines {
                yaml.push_str(&format!("      - \"{}\"\n", mag));
            }
            
            yaml.push_str(&format!("    magazine_count: {}\n", weapon.compatible_magazines.len()));
            yaml.push('\n');
        }
        
        yaml
    }

    fn format_magazine_wells_section(&self, result: &ScanResult) -> String {
        let mut yaml = String::new();
        yaml.push_str("magazine_wells:\n");
        
        for (name, info) in &result.magazine_wells {
            yaml.push_str(&format!("  {}:\n", name));
            yaml.push_str(&format!("    file: \"{}\"\n", info.file_path.display()));
            
            let total_magazines: usize = info.magazines.values()
                .map(|magazines| magazines.len())
                .sum();
            yaml.push_str(&format!("    total_magazines: {}\n", total_magazines));
            
            yaml.push_str("    magazine_types:\n");
            for mag_type in info.magazines.keys() {
                yaml.push_str(&format!("      - \"{}\"\n", mag_type));
            }
            
            yaml.push_str("    magazines:\n");
            for (mag_type, magazines) in &info.magazines {
                yaml.push_str(&format!("      {}:\n", mag_type));
                for magazine in magazines {
                    yaml.push_str(&format!("        - \"{}\"\n", magazine));
                }
            }
            yaml.push('\n');
        }
        
        yaml
    }
}

impl ReportFormatter for YamlFormatter {
    fn format(&self, result: &ScanResult) -> Result<String> {
        let mut yaml = String::new();
        
        // Header
        yaml.push_str(&format!("scan_timestamp: \"{}\"\n", result.scan_timestamp));
        
        let weapons_with_wells: Vec<_> = result.weapons.iter()
            .filter(|w| !w.magazine_wells.is_empty())
            .collect();
        
        yaml.push_str(&format!("total_weapons: {}\n", weapons_with_wells.len()));
        yaml.push_str(&format!("total_magazine_wells: {}\n", result.magazine_wells.len()));
        yaml.push('\n');
        
        // Weapons section
        yaml.push_str(&self.format_weapons_section(result));
        
        // Magazine wells section
        yaml.push_str(&self.format_magazine_wells_section(result));
        
        Ok(yaml)
    }

    fn format_name(&self) -> &'static str {
        "YAML"
    }

    fn file_extension(&self) -> &'static str {
        "yaml"
    }
}
