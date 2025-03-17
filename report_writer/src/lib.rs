use std::path::PathBuf;
use std::fs;
use std::io;
use std::collections::{HashMap, HashSet};
use serde_json;
use thiserror::Error;
use arma3_tool_dependency_scanner::ScanReport;
use prettytable::{Table, Row, Cell, format};

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Cache error: {0}")]
    Cache(String),
}

pub type Result<T> = std::result::Result<T, ReportError>;

pub struct ReportWriter {
    output_dir: PathBuf,
}

impl ReportWriter {
    pub fn new(output_dir: impl Into<PathBuf>, _storage: impl Into<()>) -> Self {
        Self {
            output_dir: output_dir.into(),
        }
    }
    
    pub fn write_report(&self, report: &ScanReport) -> Result<()> {
        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;
        
        // Write the text report
        self.write_text_report(report)?;
        
        Ok(())
    }
    
    fn write_text_report(&self, report: &ScanReport) -> Result<()> {
        // Group dependencies by mission
        let mut mission_deps: HashMap<String, Vec<&arma3_tool_dependency_scanner::MissingDependency>> = HashMap::new();
        
        for dependency in &report.missing_dependencies {
            mission_deps
                .entry(dependency.mission_name.clone())
                .or_default()
                .push(dependency);
        }
        
        // Create report content
        let mut report_text = String::new();
        
        // Summary section
        report_text.push_str(&format!("=== DEPENDENCY REPORT ===\n"));
        
        let mut summary_table = Table::new();
        summary_table.set_format(*format::consts::FORMAT_BOX_CHARS);
        summary_table.set_titles(Row::new(vec![
            Cell::new("Metric"),
            Cell::new("Value"),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Total missions scanned"),
            Cell::new(&report.total_missions_scanned.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Total dependencies checked"),
            Cell::new(&report.total_dependencies_checked.to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Total missing dependencies"),
            Cell::new(&report.missing_dependencies.len().to_string()),
        ]));
        
        let mut summary_buffer = Vec::new();
        summary_table.print(&mut summary_buffer).unwrap();
        report_text.push_str(&String::from_utf8(summary_buffer).unwrap());
        report_text.push_str("\n\n");

        // Create consolidated list of all missing classes
        if !report.missing_dependencies.is_empty() {
            report_text.push_str("=== CONSOLIDATED MISSING CLASSES ===\n");
            
            // Use a HashSet to collect unique missing class names
            let mut missing_classes = HashSet::new();
            for dep in &report.missing_dependencies {
                missing_classes.insert(dep.class_name.clone());
            }
            
            // Convert to sorted vector
            let mut missing_classes_vec: Vec<_> = missing_classes.into_iter().collect();
            missing_classes_vec.sort();
            
            // Create a table for missing classes
            let mut consolidated_table = Table::new();
            // Use a custom format with the line separator after the header row
            let format = format::FormatBuilder::new()
                .column_separator('|')
                .borders('|')
                .separator(
                    format::LinePosition::Title,
                    format::LineSeparator::new('-', '+', '+', '+')
                )
                .padding(1, 1)
                .build();
            consolidated_table.set_format(format);
            
            consolidated_table.set_titles(Row::new(vec![
                Cell::new("#"),
                Cell::new("Class Name"),
            ]));
            
            for (i, class_name) in missing_classes_vec.iter().enumerate() {
                consolidated_table.add_row(Row::new(vec![
                    Cell::new(&(i + 1).to_string()),
                    Cell::new(class_name),
                ]));
            }
            
            let mut consolidated_buffer = Vec::new();
            consolidated_table.print(&mut consolidated_buffer).unwrap();
            report_text.push_str(&String::from_utf8(consolidated_buffer).unwrap());
            report_text.push_str("\n\n");
        }

        // Get all mission names (both with missing and found dependencies)
        let mut all_mission_names: Vec<String> = mission_deps.keys().cloned().collect();
        for mission_name in report.found_dependencies.keys() {
            if !all_mission_names.contains(mission_name) {
                all_mission_names.push(mission_name.clone());
            }
        }
        all_mission_names.sort();
        
        // Mission by mission breakdown
        report_text.push_str("=== MISSION DEPENDENCY DETAILS ===\n\n");
        
        if all_mission_names.is_empty() {
            report_text.push_str("No dependencies found!\n");
        } else {
            for mission_name in all_mission_names {
                report_text.push_str(&format!("MISSION: {}\n\n", mission_name));
                
                // Report found dependencies
                if let Some(found_deps) = report.found_dependencies.get(&mission_name) {
                    report_text.push_str(&format!("Found dependencies: {}\n", found_deps.len()));
                    
                    // Sort found dependencies for consistent output
                    let mut sorted_deps = found_deps.clone();
                    sorted_deps.sort();
                    
                    // Create table with the requested format
                    let mut found_table = Table::new();
                    // Use a custom format with the line separator after the header row
                    let format = format::FormatBuilder::new()
                        .column_separator('|')
                        .borders('|')
                        .separator(
                            format::LinePosition::Title, 
                            format::LineSeparator::new('-', '+', '+', '+')
                        )
                        .padding(1, 1)
                        .build();
                    found_table.set_format(format);
                    
                    found_table.set_titles(Row::new(vec![
                        Cell::new("#"),
                        Cell::new("Class Name"),
                    ]));
                    
                    for (i, class_name) in sorted_deps.iter().enumerate() {
                        found_table.add_row(Row::new(vec![
                            Cell::new(&(i + 1).to_string()),
                            Cell::new(class_name),
                        ]));
                    }
                    
                    let mut found_buffer = Vec::new();
                    found_table.print(&mut found_buffer).unwrap();
                    report_text.push_str(&String::from_utf8(found_buffer).unwrap());
                    report_text.push_str("\n\n");
                }
                
                // Report missing dependencies
                if let Some(deps) = mission_deps.get(&mission_name) {
                    report_text.push_str(&format!("Missing dependencies: {}\n", deps.len()));
                    
                    // Group dependencies by source file
                    let mut file_deps: HashMap<String, Vec<&arma3_tool_dependency_scanner::MissingDependency>> = HashMap::new();
                    
                    for dep in deps.iter() {
                        let file_path = dep.source_file.display().to_string();
                        file_deps
                            .entry(file_path)
                            .or_default()
                            .push(dep);
                    }
                    
                    // Sort files by path for consistent output
                    let mut file_paths: Vec<_> = file_deps.keys().collect();
                    file_paths.sort();
                    
                    for file_path in file_paths {
                        let file_deps_list = &file_deps[file_path];
                        report_text.push_str(&format!("\n  FILE: {}\n", file_path));
                        report_text.push_str(&format!("  Dependencies: {}\n", file_deps_list.len()));
                        
                        // Sort dependencies by class name for consistent output
                        let mut sorted_deps = file_deps_list.to_vec();
                        sorted_deps.sort_by(|a, b| a.class_name.cmp(&b.class_name));
                        
                        // Create table with the requested format
                        let mut missing_table = Table::new();
                        // Use a custom format with the line separator after the header row
                        let format = format::FormatBuilder::new()
                            .column_separator('|')
                            .borders('|')
                            .separator(
                                format::LinePosition::Title, 
                                format::LineSeparator::new('-', '+', '+', '+')
                            )
                            .padding(1, 1)
                            .build();
                        missing_table.set_format(format);
                        
                        missing_table.set_titles(Row::new(vec![
                            Cell::new("#"),
                            Cell::new("Class Name"),
                        ]));
                        
                        for (i, dep) in sorted_deps.iter().enumerate() {
                            missing_table.add_row(Row::new(vec![
                                Cell::new(&(i + 1).to_string()),
                                Cell::new(&dep.class_name),
                            ]));
                        }
                        
                        let mut missing_buffer = Vec::new();
                        missing_table.print(&mut missing_buffer).unwrap();
                        report_text.push_str(&String::from_utf8(missing_buffer).unwrap());
                    }
                }
                
                report_text.push_str("\n\n");
            }
        }
        
        // Write text report
        let report_path = self.output_dir.join("dependency_report.txt");
        fs::write(report_path, report_text)?;
        
        Ok(())
    }
} 