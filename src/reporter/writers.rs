use std::path::PathBuf;
use chrono::Utc;
use log::info;

use crate::reporter::error::{Result as ReporterResult, ReporterError};
use crate::reporter::models::DependencyReport;

/// Handles writing reports to files
pub struct ReportWriter {
    output_dir: PathBuf,
}

impl ReportWriter {
    /// Create a new report writer
    pub fn new(output_dir: &PathBuf) -> Self {
        Self {
            output_dir: output_dir.clone(),
        }
    }

    /// Write the dependency report to a file
    pub fn write_report(&self, report: &DependencyReport) -> ReporterResult<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .map_err(|e| ReporterError::CreateDirError {
                path: self.output_dir.clone(),
                source: e,
            })?;

        // Generate report content
        let report_content = self.generate_report_content(report);

        // Create report file path with timestamp
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let report_path = self.output_dir.join(format!("dependency_report_{}.txt", timestamp));

        // Write report to file
        std::fs::write(&report_path, report_content)
            .map_err(|e| ReporterError::WriteError {
                path: report_path.clone(),
                source: e,
            })?;

        info!("Dependency report written to: {}", report_path.display());
        Ok(())
    }

    /// Generate the report content
    fn generate_report_content(&self, report: &DependencyReport) -> String {
        let mut content = String::new();

        // Add summary
        content.push_str(&format!(
            "Dependency Analysis Report\n\
             =======================\n\n\
             Total Game Data Classes: {}\n\
             Total Missions Analyzed: {}\n\
             Total Dependencies: {}\n\
             Total Missing Dependencies: {}\n\
             Generated At: {}\n\n",
            report.total_classes,
            report.total_missions,
            report.total_dependencies,
            report.total_missing,
            report.generated_at
        ));

        // Add detailed breakdown by mission
        content.push_str("Missing Dependencies by Mission:\n");
        content.push_str("--------------------------------\n\n");

        for (mission_id, missing_classes) in &report.missing_dependencies {
            content.push_str(&format!(
                "Mission: {}\n\
                 Missing Classes: {}\n\n",
                mission_id,
                missing_classes.len()
            ));

            // List missing classes
            for class in missing_classes {
                content.push_str(&format!("  - {}\n", class));
            }
            content.push_str("\n");
        }

        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_report_writer() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path().join("reports");
        
        // Create test report
        let mut missing_deps = HashMap::new();
        let mut mission_deps = HashSet::new();
        mission_deps.insert("MissingClass".to_string());
        missing_deps.insert("test_mission".to_string(), mission_deps);
        
        let report = DependencyReport::new(missing_deps, 1, 1, 100, 5);
        
        // Create writer and write report
        let writer = ReportWriter::new(&output_dir);
        writer.write_report(&report).unwrap();
        
        // Verify report file was created
        let report_files: Vec<_> = std::fs::read_dir(&output_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("dependency_report_"))
            .collect();
        assert_eq!(report_files.len(), 1);
        
        // Verify report content
        let report_path = &report_files[0].path();
        let content = std::fs::read_to_string(report_path).unwrap();
        assert!(content.contains("Total Missions Analyzed: 1"));
        assert!(content.contains("Total Missing Dependencies: 1"));
        assert!(content.contains("Mission: test_mission"));
        assert!(content.contains("- MissingClass"));
    }
} 