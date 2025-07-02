use anyhow::Result;
use std::fmt::Write;
use crate::models::ScanResult;
use super::ReportFormatter;

/// CSV report formatter
pub struct CsvFormatter;

impl CsvFormatter {
    pub fn new() -> Self {
        Self
    }

    fn format_weapons_csv(&self, result: &ScanResult) -> String {
        let mut csv = String::new();
        
        // Header
        csv.push_str("weapon_name,parent,file_path,magazine_wells,compatible_magazines_count,compatible_magazines\n");
        
        for weapon in &result.weapons {
            if weapon.magazine_wells.is_empty() {
                continue;
            }
            
            let magazine_wells = weapon.magazine_wells.join(";");
            let compatible_magazines = weapon.compatible_magazines.join(";");
            
            let _ = writeln!(
                csv,
                "\"{}\",\"{}\",\"{}\",\"{}\",{},\"{}\"",
                weapon.name,
                weapon.parent.as_deref().unwrap_or(""),
                weapon.file_path.display(),
                magazine_wells,
                weapon.compatible_magazines.len(),
                compatible_magazines
            );
        }
        
        csv
    }

    fn format_magazine_wells_csv(&self, result: &ScanResult) -> String {
        let mut csv = String::new();
        
        // Header
        csv.push_str("magazine_well_name,file_path,total_magazines,magazine_types,magazines\n");
        
        for (name, info) in &result.magazine_wells {
            let total_magazines: usize = info.magazines.values()
                .map(|magazines| magazines.len())
                .sum();
            
            let magazine_types: Vec<String> = info.magazines.keys().cloned().collect();
            let magazine_types_str = magazine_types.join(";");
            
            let all_magazines: Vec<String> = info.magazines.values()
                .flat_map(|magazines| magazines.iter().cloned())
                .collect();
            let all_magazines_str = all_magazines.join(";");
            
            let _ = writeln!(
                csv,
                "\"{}\",\"{}\",{},\"{}\",\"{}\"",
                name,
                info.file_path.display(),
                total_magazines,
                magazine_types_str,
                all_magazines_str
            );
        }
        
        csv
    }

    fn format_summary_csv(&self, result: &ScanResult) -> String {
        let mut csv = String::new();
        
        let weapons_with_wells: Vec<_> = result.weapons.iter()
            .filter(|w| !w.magazine_wells.is_empty())
            .collect();
        
        // Summary section
        csv.push_str("metric,value\n");
        let _ = writeln!(csv, "scan_timestamp,\"{}\"", result.scan_timestamp);
        let _ = writeln!(csv, "total_weapons,{}", weapons_with_wells.len());
        let _ = writeln!(csv, "total_magazine_wells,{}", result.magazine_wells.len());
        csv.push('\n');
        
        csv
    }
}

impl ReportFormatter for CsvFormatter {
    fn format(&self, result: &ScanResult) -> Result<String> {
        let mut output = String::new();
        
        // Summary
        output.push_str("# Summary\n");
        output.push_str(&self.format_summary_csv(result));
        
        // Weapons
        output.push_str("# Weapons\n");
        output.push_str(&self.format_weapons_csv(result));
        output.push('\n');
        
        // Magazine Wells
        output.push_str("# Magazine Wells\n");
        output.push_str(&self.format_magazine_wells_csv(result));
        
        Ok(output)
    }

    fn format_name(&self) -> &'static str {
        "CSV"
    }

    fn file_extension(&self) -> &'static str {
        "csv"
    }
}
