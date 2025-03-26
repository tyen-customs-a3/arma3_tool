use crate::utils;
use crate::error::Result;
use crate::utils::mission_dependency_builder::{ScanReport, Dependency};
use prettytable::{Row, Cell};
use std::collections::HashMap;
use std::path::Path;

pub struct DependencyReport {
    pub total_missions: usize,
    pub total_dependencies: usize,
    pub missing_dependencies: Vec<Dependency>,
    pub mission_details: HashMap<String, MissionDependencyInfo>,
}

pub struct MissionDependencyInfo {
    pub found_dependencies: Vec<String>,
    pub missing_dependencies: Vec<Dependency>,
}

pub struct DependencyReportWriter<'a> {
    report: &'a ScanReport,
}

impl<'a> DependencyReportWriter<'a> {
    pub fn new(report: &'a ScanReport) -> Self {
        Self { report }
    }

    pub fn write_report(&self, output_path: &Path) -> Result<DependencyReport> {
        let mut dependency_report = DependencyReport {
            total_missions: self.report.total_missions_scanned,
            total_dependencies: self.report.total_dependencies_checked,
            missing_dependencies: self.report.missing.clone(),
            mission_details: HashMap::new(),
        };

        // Group dependencies by mission
        let mut mission_deps: HashMap<String, MissionDependencyInfo> = HashMap::new();
        
        // Process found dependencies
        for dep in &self.report.found {
            let info = mission_deps
                .entry(dep.mission_name.clone())
                .or_insert_with(|| MissionDependencyInfo {
                    found_dependencies: Vec::new(),
                    missing_dependencies: Vec::new(),
                });
            info.found_dependencies.push(dep.class_name.clone());
        }

        // Process missing dependencies
        for dep in &self.report.missing {
            let info = mission_deps
                .entry(dep.mission_name.clone())
                .or_insert_with(|| MissionDependencyInfo {
                    found_dependencies: Vec::new(),
                    missing_dependencies: Vec::new(),
                });
            info.missing_dependencies.push(dep.clone());
        }

        dependency_report.mission_details = mission_deps;

        // Write the report
        self.write_dependency_text_report(&dependency_report, output_path)?;

        Ok(dependency_report)
    }

    fn write_dependency_text_report(&self, report: &DependencyReport, output_path: &Path) -> Result<()> {
        let mut report_text = String::new();
        
        // Create report header
        report_text.push_str("=== DEPENDENCY SCAN REPORT ===\n\n");
        
        // Create summary table
        let mut summary_table = utils::create_summary_table();
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Missions Scanned"),
            Cell::new(&report.total_missions.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Dependencies Checked"),
            Cell::new(&report.total_dependencies.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Missing Dependencies"),
            Cell::new(&report.missing_dependencies.len().to_string()),
        ]));
        
        report_text.push_str(&utils::table_to_string(&summary_table));
        report_text.push_str("\n\n");

        // Create consolidated missing dependencies section
        if !report.missing_dependencies.is_empty() {
            report_text.push_str("=== CONSOLIDATED MISSING DEPENDENCIES ===\n");
            
            let mut missing_table = utils::create_standard_table();
            missing_table.set_titles(Row::new(vec![
                Cell::new("#"),
                Cell::new("Class Name"),
                Cell::new("File Path"),
            ]));

            // Sort missing dependencies for consistent output
            let mut sorted_deps = report.missing_dependencies.clone();
            sorted_deps.sort_by(|a, b| a.class_name.cmp(&b.class_name));

            for (i, dep) in sorted_deps.iter().enumerate() {
                missing_table.add_row(Row::new(vec![
                    Cell::new(&(i + 1).to_string()),
                    Cell::new(&dep.class_name),
                    Cell::new(&dep.source_file.display().to_string()),
                ]));
            }
            
            report_text.push_str(&utils::table_to_string(&missing_table));
            report_text.push_str("\n\n");
        }

        // Create mission details section
        report_text.push_str("=== MISSION DEPENDENCY DETAILS ===\n");
        
        // Sort missions for consistent output
        let mut sorted_missions: Vec<_> = report.mission_details.iter().collect();
        sorted_missions.sort_by(|a, b| a.0.cmp(b.0));

        for (mission_name, info) in sorted_missions {
            report_text.push_str(&format!("\nMission: {}\n", mission_name));
            
            // Found dependencies
            if !info.found_dependencies.is_empty() {
                let mut found_table = utils::create_standard_table();
                found_table.set_titles(Row::new(vec![
                    Cell::new("#"),
                    Cell::new("Found Dependencies"),
                ]));

                let mut sorted_found = info.found_dependencies.clone();
                sorted_found.sort();

                for (i, class_name) in sorted_found.iter().enumerate() {
                    found_table.add_row(Row::new(vec![
                        Cell::new(&(i + 1).to_string()),
                        Cell::new(class_name),
                    ]));
                }

                report_text.push_str(&utils::table_to_string(&found_table));
                report_text.push_str("\n");
            }

            // Missing dependencies
            if !info.missing_dependencies.is_empty() {
                let mut missing_table = utils::create_standard_table();
                missing_table.set_titles(Row::new(vec![
                    Cell::new("#"),
                    Cell::new("Missing Dependencies"),
                    Cell::new("File Path"),
                ]));

                let mut sorted_missing = info.missing_dependencies.clone();
                sorted_missing.sort_by(|a, b| a.class_name.cmp(&b.class_name));

                for (i, dep) in sorted_missing.iter().enumerate() {
                    missing_table.add_row(Row::new(vec![
                        Cell::new(&(i + 1).to_string()),
                        Cell::new(&dep.class_name),
                        Cell::new(&dep.source_file.display().to_string()),
                    ]));
                }

                report_text.push_str(&utils::table_to_string(&missing_table));
                report_text.push_str("\n");
            }
        }

        // Write to file
        utils::write_report(output_path, "dependency_scan_report.txt", &report_text)?;

        Ok(())
    }
} 