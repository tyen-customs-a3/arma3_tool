use anyhow::Result;
use std::fmt::Write;
use crate::models::ScanResult;
use super::ReportFormatter;

/// Plain text report formatter
pub struct TextFormatter;

impl TextFormatter {
    pub fn new() -> Self {
        Self
    }

    fn format_header(&self, result: &ScanResult) -> String {
        let weapons_with_wells: Vec<_> = result.weapons.iter()
            .filter(|w| !w.magazine_wells.is_empty())
            .collect();

        format!(
            "Weapon Magazine Scan Report\n\
             ===========================\n\
             Scan Time: {}\n\
             Total Weapons with Magazine Wells: {}\n\
             Total Magazine Wells: {}\n\n",
            result.scan_timestamp,
            weapons_with_wells.len(),
            result.magazine_wells.len()
        )
    }

    fn format_weapons_section(&self, result: &ScanResult) -> String {
        let mut text = String::new();
        text.push_str("WEAPONS\n");
        text.push_str("=======\n\n");

        for weapon in &result.weapons {
            if weapon.magazine_wells.is_empty() {
                continue;
            }

            let _ = writeln!(text, "Name: {}", weapon.name);
            
            if let Some(ref parent) = weapon.parent {
                let _ = writeln!(text, "Parent: {}", parent);
            }
            
            let _ = writeln!(text, "File: {}", weapon.file_path.display());
            let _ = writeln!(text, "Magazine Wells ({}):", weapon.magazine_wells.len());
            for well in &weapon.magazine_wells {
                let _ = writeln!(text, "  {}", well);
            }
            let _ = writeln!(text, "Compatible Magazines ({}):", weapon.compatible_magazines.len());
            for magazine in &weapon.compatible_magazines {
                let _ = writeln!(text, "  {}", magazine);
            }
            text.push('\n');
        }

        text
    }

    fn format_magazine_wells_section(&self, result: &ScanResult) -> String {
        let mut text = String::new();
        text.push_str("MAGAZINE WELLS\n");
        text.push_str("==============\n\n");

        for (name, info) in &result.magazine_wells {
            let _ = writeln!(text, "Name: {}", name);
            let _ = writeln!(text, "File: {}", info.file_path.display());

            let total_magazines: usize = info.magazines.values()
                .map(|magazines| magazines.len())
                .sum();
            let _ = writeln!(text, "Total Magazines: {}", total_magazines);

            for (mag_type, magazines) in &info.magazines {
                let _ = writeln!(text, "  {}: {}", mag_type, magazines.join(", "));
            }
            text.push('\n');
        }

        text
    }

    fn format_statistics(&self, result: &ScanResult) -> String {
        let mut text = String::new();
        text.push_str("STATISTICS\n");
        text.push_str("==========\n\n");

        let weapons_with_wells: Vec<_> = result.weapons.iter()
            .filter(|w| !w.magazine_wells.is_empty())
            .collect();

        // Basic stats
        let _ = writeln!(text, "Weapons with magazine wells: {}", weapons_with_wells.len());
        let _ = writeln!(text, "Total magazine wells: {}", result.magazine_wells.len());

        // Magazine well usage frequency
        let mut well_usage = std::collections::HashMap::new();
        for weapon in &weapons_with_wells {
            for well_name in &weapon.magazine_wells {
                *well_usage.entry(well_name.clone()).or_insert(0) += 1;
            }
        }

        if !well_usage.is_empty() {
            text.push_str("\nMost used magazine wells:\n");
            let mut usage_pairs: Vec<_> = well_usage.iter().collect();
            usage_pairs.sort_by(|a, b| b.1.cmp(a.1));
            
            for (well_name, count) in usage_pairs.iter().take(10) {
                let _ = writeln!(text, "  {} (used by {} weapons)", well_name, count);
            }
        }

        // Magazine compatibility stats
        if !weapons_with_wells.is_empty() {
            let total_magazines: usize = weapons_with_wells.iter()
                .map(|w| w.compatible_magazines.len())
                .sum();
            let avg_magazines = total_magazines as f64 / weapons_with_wells.len() as f64;
            let _ = writeln!(text, "\nAverage magazines per weapon: {:.1}", avg_magazines);

            let max_magazines = weapons_with_wells.iter()
                .map(|w| w.compatible_magazines.len())
                .max()
                .unwrap_or(0);
            let _ = writeln!(text, "Maximum magazines for a single weapon: {}", max_magazines);
        }

        text
    }
}

impl ReportFormatter for TextFormatter {
    fn format(&self, result: &ScanResult) -> Result<String> {
        let mut output = String::new();
        
        output.push_str(&self.format_header(result));
        output.push_str(&self.format_weapons_section(result));
        output.push_str(&self.format_magazine_wells_section(result));
        output.push_str(&self.format_statistics(result));
        
        Ok(output)
    }

    fn format_name(&self) -> &'static str {
        "Plain Text"
    }

    fn file_extension(&self) -> &'static str {
        "txt"
    }
}
