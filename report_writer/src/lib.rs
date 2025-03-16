use std::path::PathBuf;
use std::fs;
use std::io;
use std::collections::HashMap;
use serde_json;
use thiserror::Error;
use arma3_tool_dependency_scanner::ScanReport;

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
        
        // Create text report
        let mut report_text = String::new();
        report_text.push_str(&format!("=== DEPENDENCY REPORT ===\n"));
        report_text.push_str(&format!("Total missions scanned: {}\n", report.total_missions_scanned));
        report_text.push_str(&format!("Total dependencies checked: {}\n", report.total_dependencies_checked));
        report_text.push_str(&format!("Total missing dependencies: {}\n\n", report.missing_dependencies.len()));
        
        if mission_deps.is_empty() {
            report_text.push_str("No missing dependencies found!\n");
        } else {
            // Sort missions by name for consistent output
            let mut mission_names: Vec<_> = mission_deps.keys().collect();
            mission_names.sort();
            
            for mission_name in mission_names {
                let deps = &mission_deps[mission_name];
                report_text.push_str(&format!("MISSION: {}\n", mission_name));
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
                    
                    for (i, dep) in sorted_deps.iter().enumerate() {
                        report_text.push_str(&format!("    {}. Class: {} ({})\n", 
                            i + 1, 
                            dep.class_name, 
                            dep.reference_type
                        ));
                        
                        if let Some(line) = dep.line_number {
                            report_text.push_str(&format!("       Location: line {}\n", line));
                        } else {
                            report_text.push_str("       Location: unknown line\n");
                        }
                    }
                }
                
                report_text.push_str("\n");
            }
        }
        
        // Write text report
        let report_path = self.output_dir.join("dependency_report.txt");
        fs::write(report_path, report_text)?;
        
        Ok(())
    }
} 