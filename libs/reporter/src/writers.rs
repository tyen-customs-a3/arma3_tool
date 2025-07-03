use std::path::{Path, PathBuf};
use chrono::Utc;
use log::info;

use crate::error::{ReporterError, Result as ReporterResult};
use crate::models::{DependencyReport, FuzzyMissingClassReport};

/// Creates report files in different formats
pub struct ReportWriter {
    output_dir: PathBuf,
}

impl ReportWriter {
    /// Create a new report writer with the specified output directory
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_path_buf(),
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
            content.push('\n');
        }

        content
    }

    /// Write the fuzzy missing class report to a file
    pub fn write_fuzzy_missing_class_report(&self, report: &FuzzyMissingClassReport) -> ReporterResult<()> {
        std::fs::create_dir_all(&self.output_dir)
            .map_err(|e| ReporterError::CreateDirError {
                path: self.output_dir.clone(),
                source: e,
            })?;

        let report_content = self.generate_fuzzy_missing_class_report_content_csv(report); // Changed method name
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let report_path = self
            .output_dir
            .join(format!("fuzzy_missing_class_report_{}.csv", timestamp)); // Changed extension to .csv

        std::fs::write(&report_path, report_content).map_err(|e| {
            ReporterError::WriteError {
                path: report_path.clone(),
                source: e,
            }
        })?;

        info!(
            "Fuzzy missing class report written to: {}",
            report_path.display()
        );
        Ok(())
    }

    /// Generate the content for the fuzzy missing class report in CSV format
    fn generate_fuzzy_missing_class_report_content_csv(
        &self,
        report: &FuzzyMissingClassReport,
    ) -> String {
        let mut content = String::new();

        // New CSV Header (without similarity scores)
        content.push_str("missing_class_name,potential_match_1_name,potential_match_2_name,potential_match_3_name\n");

        if !report.missing_class_matches.is_empty() {
            for missing_match in &report.missing_class_matches {
                // Write missing class name (no quotes, assuming class names don't contain commas)
                let mut row_parts = vec![missing_match.missing_class_name.clone()];

                for i in 0..crate::analyzers::MAX_FUZZY_MATCHES {
                    if let Some(pot_match) = missing_match.potential_matches.get(i) {
                        // Add potential match name (no quotes)
                        row_parts.push(pot_match.class_name.clone());
                    } else {
                        // Fill with empty string if fewer than MAX_FUZZY_MATCHES matches
                        row_parts.push(String::new());
                    }
                }
                // Join parts with comma and add newline
                content.push_str(&row_parts.join(","));
                content.push('\n');
            }
        }
        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporter::models::{MissingClassMatch, PotentialMatch};
    use crate::reporter::analyzers::MAX_FUZZY_MATCHES; // Import for test
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
        // Corrected: Insert into missing_deps with mission_id as key
        missing_deps.insert("test_mission".to_string(), mission_deps);
        
        let report = DependencyReport::new(missing_deps, 1, 1, 100, 5);
        
        // Create writer and write report
        let writer = ReportWriter::new(output_dir.as_path());
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

    #[test]
    fn test_fuzzy_report_writer() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path().join("reports");

        let matches = vec![
            MissingClassMatch {
                missing_class_name: "MyClas".to_string(),
                potential_matches: vec![
                    PotentialMatch { class_name: "MyClass".to_string(), similarity: 0.93 },
                    PotentialMatch { class_name: "MyClassExtended".to_string(), similarity: 0.81 },
                ]
            },
            MissingClassMatch {
                missing_class_name: "AnotherMiss".to_string(),
                potential_matches: vec![]
            }
        ];
        let report = FuzzyMissingClassReport::new(matches);
        
        let writer = ReportWriter::new(output_dir.as_path());
        writer.write_fuzzy_missing_class_report(&report).unwrap();

        let report_files: Vec<_> = std::fs::read_dir(&output_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("fuzzy_missing_class_report_") && e.file_name().to_string_lossy().ends_with(".csv"))
            .collect();
        assert_eq!(report_files.len(), 1);

        let report_path = &report_files[0].path();
        let content = std::fs::read_to_string(report_path).unwrap();
        
        let expected_header = "missing_class_name,potential_match_1_name,potential_match_2_name,potential_match_3_name\n";
        assert!(content.starts_with(expected_header));
        
        let lines: Vec<&str> = content.trim().split('\n').collect();
        assert_eq!(lines.len(), 3); // Header + 2 data rows

        // Check MyClas row
        let myclas_line_parts: Vec<&str> = lines[1].split(',').collect();
        assert_eq!(myclas_line_parts[0], "MyClas");
        assert_eq!(myclas_line_parts[1], "MyClass");
        assert_eq!(myclas_line_parts[2], "MyClassExtended");
        // Check for empty cell for the 3rd potential match (since MAX_FUZZY_MATCHES is 3)
        if MAX_FUZZY_MATCHES > 2 { // This test data has 2 matches for MyClas
             assert_eq!(myclas_line_parts[3], ""); // 3rd match is empty
        }

        // Check AnotherMiss row
        let anothermiss_line_parts: Vec<&str> = lines[2].split(',').collect();
        assert_eq!(anothermiss_line_parts[0], "AnotherMiss");
        for i in 0..MAX_FUZZY_MATCHES {
            assert_eq!(anothermiss_line_parts[1 + i], ""); // name
        }
    }

    #[test]
    fn test_fuzzy_report_writer_no_missing_classes_csv() {
        let dir = tempdir().unwrap();
        let output_dir = dir.path().join("reports");
        let report = FuzzyMissingClassReport::new(vec![]);
        
        let writer = ReportWriter::new(output_dir.as_path());
        writer.write_fuzzy_missing_class_report(&report).unwrap();

        let report_files: Vec<_> = std::fs::read_dir(&output_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("fuzzy_missing_class_report_") && e.file_name().to_string_lossy().ends_with(".csv"))
            .collect();
        assert_eq!(report_files.len(), 1);
        
        let report_path = &report_files[0].path();
        let content = std::fs::read_to_string(report_path).unwrap();
        let expected_header = "missing_class_name,potential_match_1_name,potential_match_2_name,potential_match_3_name\n";
        assert_eq!(content, expected_header); // Should only contain the header
    }
}
