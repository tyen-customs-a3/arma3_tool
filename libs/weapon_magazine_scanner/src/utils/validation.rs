use anyhow::Result;
use std::path::Path;
use crate::models::{WeaponInfo, MagazineWellInfo, ScanResult};

/// Configuration validation utilities
pub struct ConfigValidator;

impl ConfigValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validate a scan result for consistency
    pub fn validate_scan_result(&self, result: &ScanResult) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();
        
        // Validate weapons
        for weapon in &result.weapons {
            self.validate_weapon(weapon, &mut report);
        }
        
        // Validate magazine wells
        for (name, well) in &result.magazine_wells {
            self.validate_magazine_well(name, well, &mut report);
        }
        
        // Cross-validate compatibility
        self.validate_compatibility(&result.weapons, &result.magazine_wells, &mut report);
        
        Ok(report)
    }

    fn validate_weapon(&self, weapon: &WeaponInfo, report: &mut ValidationReport) {
        // Check for empty names
        if weapon.name.is_empty() {
            report.add_error(format!("Weapon has empty name in file: {}", weapon.file_path.display()));
        }
        
        // Check for invalid file paths
        if !weapon.file_path.exists() {
            report.add_warning(format!("Weapon {} references non-existent file: {}", weapon.name, weapon.file_path.display()));
        }
        
        // Check for circular inheritance
        if let Some(ref parent) = weapon.parent {
            if parent == &weapon.name {
                report.add_error(format!("Weapon {} has circular inheritance (parent is self)", weapon.name));
            }
        }
        
        // Check magazine wells vs compatible magazines consistency
        if !weapon.magazine_wells.is_empty() && weapon.compatible_magazines.is_empty() {
            report.add_warning(format!("Weapon {} has magazine wells but no compatible magazines", weapon.name));
        }
    }

    fn validate_magazine_well(&self, name: &str, well: &MagazineWellInfo, report: &mut ValidationReport) {
        // Check for empty names
        if name.is_empty() || well.name.is_empty() {
            report.add_error(format!("Magazine well has empty name in file: {}", well.file_path.display()));
        }
        
        // Check name consistency
        if name != &well.name {
            report.add_warning(format!("Magazine well name mismatch: key='{}', name='{}'", name, well.name));
        }
        
        // Check for empty magazines
        if well.magazines.is_empty() {
            report.add_warning(format!("Magazine well {} has no magazines", name));
        }
    }

    fn validate_compatibility(&self, weapons: &[WeaponInfo], magazine_wells: &std::collections::HashMap<String, MagazineWellInfo>, report: &mut ValidationReport) {
        for weapon in weapons {
            for well_name in &weapon.magazine_wells {
                if !magazine_wells.contains_key(well_name) {
                    report.add_error(format!("Weapon {} references unknown magazine well: {}", weapon.name, well_name));
                }
            }
        }
    }

    /// Validate file extension
    pub fn is_valid_config_file(&self, path: &Path) -> bool {
        matches!(
            path.extension().and_then(|s| s.to_str()).unwrap_or(""),
            "cpp" | "hpp" | "h"
        )
    }

    /// Check if a weapon name is valid
    pub fn is_valid_weapon_name(&self, name: &str) -> bool {
        !name.is_empty() && 
        name.chars().all(|c| c.is_alphanumeric() || c == '_') &&
        !name.starts_with(char::is_numeric)
    }
}

/// Validation report containing errors and warnings
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    pub fn is_valid(&self) -> bool {
        !self.has_errors()
    }

    pub fn summary(&self) -> String {
        format!("Validation: {} errors, {} warnings", self.errors.len(), self.warnings.len())
    }
}
