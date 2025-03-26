use crate::utils;
use crate::error::Result;
use crate::utils::mission_dependency_builder::{ScanReport, Dependency};
use arma3_tool_shared_models::GameDataClasses;
use prettytable::{Row, Cell};
use std::collections::HashMap;
use std::path::Path;

pub struct ComparisonReport {
    pub missing_in_b: Vec<Dependency>,
    pub found_in_a: HashMap<String, String>, // Class name -> Original PBO/file path
    pub total_deps_a: usize,
    pub total_deps_b: usize,
    pub total_missing_a: usize,
    pub total_missing_b: usize,
}

pub struct ComparisonReportWriter<'a> {
    report_a: &'a ScanReport,
    report_b: &'a ScanReport,
    game_data_a: &'a GameDataClasses,
}

impl<'a> ComparisonReportWriter<'a> {
    pub fn new(report_a: &'a ScanReport, report_b: &'a ScanReport, game_data_a: &'a GameDataClasses) -> Self {
        Self {
            report_a,
            report_b,
            game_data_a,
        }
    }

    pub fn write_report(&self, output_path: &Path) -> Result<ComparisonReport> {
        // Create a map of all class definitions in set A
        let mut found_in_a: HashMap<String, String> = HashMap::new();
        for class in &self.game_data_a.classes {
            if let Some(source_idx) = class.source_file_index {
                if let Some(source_path) = self.game_data_a.get_file_source(source_idx) {
                    found_in_a.insert(class.name.to_lowercase(), source_path.display().to_string());
                }
            }
        }

        // Find dependencies missing in B that were defined in A
        let mut comparison = ComparisonReport {
            missing_in_b: Vec::new(),
            found_in_a: HashMap::new(),
            total_deps_a: self.report_a.total_dependencies_checked,
            total_deps_b: self.report_b.total_dependencies_checked,
            total_missing_a: self.report_a.missing.len(),
            total_missing_b: self.report_b.missing.len(),
        };

        // Check each missing dependency in B against what was found in A
        for dep in &self.report_b.missing {
            let lowercase_name = dep.class_name.to_lowercase();
            
            // Check if this class was found in A's game data
            if let Some(original_path) = found_in_a.get(&lowercase_name) {
                comparison.missing_in_b.push(dep.clone());
                comparison.found_in_a.insert(dep.class_name.clone(), original_path.clone());
            }
        }

        // Write the comparison report
        self.write_comparison_text_report(&comparison, output_path)?;

        Ok(comparison)
    }

    fn write_comparison_text_report(&self, comparison: &ComparisonReport, output_path: &Path) -> Result<()> {
        let mut report_text = String::new();
        
        // Create report header
        report_text.push_str("=== DEPENDENCY COMPARISON REPORT ===\n\n");
        
        // Create summary table
        let mut summary_table = utils::create_summary_table();
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Dependencies Checked (A)"),
            Cell::new(&comparison.total_deps_a.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Dependencies Checked (B)"),
            Cell::new(&comparison.total_deps_b.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Missing Dependencies (A)"),
            Cell::new(&comparison.total_missing_a.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Missing Dependencies (B)"),
            Cell::new(&comparison.total_missing_b.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Dependencies Missing in B but Found in A"),
            Cell::new(&comparison.missing_in_b.len().to_string()),
        ]));
        
        report_text.push_str(&utils::table_to_string(&summary_table));
        report_text.push_str("\n\n");

        // Create detailed comparison table
        if !comparison.missing_in_b.is_empty() {
            report_text.push_str("=== MISSING DEPENDENCIES COMPARISON ===\n");
            
            let mut comparison_table = utils::create_standard_table();
            comparison_table.set_titles(Row::new(vec![
                Cell::new("#"),
                Cell::new("Class Name"),
                Cell::new("Previously Defined In"),
                Cell::new("Current Usage In"),
            ]));

            // Create a sorted list of dependencies
            let mut sorted_deps = comparison.missing_in_b.clone();
            sorted_deps.sort_by(|a, b| a.class_name.cmp(&b.class_name));

            for (i, dep) in sorted_deps.iter().enumerate() {
                let empty_string = String::new();
                let original_path = comparison.found_in_a.get(&dep.class_name).unwrap_or(&empty_string);
                comparison_table.add_row(Row::new(vec![
                    Cell::new(&(i + 1).to_string()),
                    Cell::new(&dep.class_name),
                    Cell::new(original_path),
                    Cell::new(&dep.source_file.display().to_string()),
                ]));
            }
            
            report_text.push_str(&utils::table_to_string(&comparison_table));
            report_text.push_str("\n\n");
        }

        // Write to file
        utils::write_report(output_path, "dependency_comparison_report.txt", &report_text)?;

        Ok(())
    }
} 