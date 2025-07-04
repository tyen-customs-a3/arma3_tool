use std::path::PathBuf;
use std::time::Instant;
use async_trait::async_trait;
use log::{info, debug};

use arma3_database::DatabaseManager;
use arma3_reporter::ReportCoordinator;
use arma3_workflow::report::ReporterInterface;
use arma3_workflow::types::summary::ReportingSummary;
use arma3_workflow::types::options::ReportingOptions;
use arma3_workflow::error::{WorkflowError, Result};

/// Adapter that bridges the legacy reporter system with the new workflow system
pub struct Arma3ReporterAdapter {
    db_path: PathBuf,
    ignore_classes_file: Option<PathBuf>,
}

/// Adapter for fuzzy report generation that implements the workflow interface
pub struct FuzzyReporterAdapter {
    db_path: PathBuf,
    ignore_classes_file: Option<PathBuf>,
}

impl Arma3ReporterAdapter {
    /// Create a new reporter adapter with the given database path and optional ignore classes file
    pub fn new(db_path: PathBuf, ignore_classes_file: Option<PathBuf>) -> Self {
        Self { 
            db_path,
            ignore_classes_file,
        }
    }
}

#[async_trait]
impl ReporterInterface for Arma3ReporterAdapter {
    async fn generate_reports(
        &self,
        _source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ReportingOptions,
    ) -> Result<ReportingSummary> {
        let start_time = Instant::now();
        
        debug!("Starting report generation to {:?}", output_dir);
        
        // Create report output directory if it doesn't exist
        std::fs::create_dir_all(output_dir)
            .map_err(|e| WorkflowError::reporting_error(format!("Failed to create report directory: {}", e)))?;

        // Create database manager
        let db_manager = DatabaseManager::new(&self.db_path)
            .map_err(|e| WorkflowError::reporting_error(format!("Failed to create database manager: {}", e)))?;

        // Create report coordinator with ignore classes file
        let coordinator = ReportCoordinator::new(&db_manager, self.ignore_classes_file.clone());

        let mut reports_generated = 0;
        let mut report_paths = Vec::new();
        let mut errors = Vec::new();

        // Generate dependency report
        if options.include_missing_dependencies {
            info!("Generating dependency report...");
            match coordinator.run_report(output_dir) {
                Ok(()) => {
                    reports_generated += 1;
                    report_paths.push(output_dir.join("dependency_report.html"));
                    info!("Dependency report generated successfully");
                }
                Err(e) => {
                    let error_msg = format!("Failed to generate dependency report: {}", e);
                    errors.push(error_msg.clone());
                    log::warn!("{}", error_msg);
                }
            }
        }

        // Generate class hierarchy graph report
        if options.include_game_data_classes {
            info!("Generating class hierarchy graph...");
            match coordinator.generate_class_graph(output_dir) {
                Ok(()) => {
                    reports_generated += 1;
                    report_paths.push(output_dir.join("class_hierarchy.html"));
                    info!("Class hierarchy graph generated successfully");
                }
                Err(e) => {
                    let error_msg = format!("Failed to generate class hierarchy graph: {}", e);
                    errors.push(error_msg.clone());
                    log::warn!("{}", error_msg);
                }
            }
        }

        // Generate mission class source report
        if options.include_mission_dependencies {
            info!("Generating mission class source report...");
            match coordinator.generate_mission_class_source_report(output_dir) {
                Ok(()) => {
                    reports_generated += 1;
                    report_paths.push(output_dir.join("mission_class_sources.html"));
                    info!("Mission class source report generated successfully");
                }
                Err(e) => {
                    let error_msg = format!("Failed to generate mission class source report: {}", e);
                    errors.push(error_msg.clone());
                    log::warn!("{}", error_msg);
                }
            }
        }

        let elapsed_time = start_time.elapsed();

        info!("Report generation completed in {:?}: {} reports generated", 
              elapsed_time, reports_generated);

        // If no reports were generated and we have errors, fail the operation
        if reports_generated == 0 && !errors.is_empty() {
            return Err(WorkflowError::reporting_error(
                format!("Report generation failed with no reports created. Errors: {}", errors.join("; "))
            ));
        }

        Ok(ReportingSummary {
            generated_reports: reports_generated,
            report_time: elapsed_time,
            output_files: report_paths,
        })
    }
    
    async fn validate_reporting_config(&self, options: &ReportingOptions) -> Result<()> {
        debug!("Validating reporting configuration");
        
        // Validate source directories if they are specified
        // Note: For workflow integration, source dirs might be empty if this follows processing
        for source_dir in &options.source_directories {
            if !source_dir.exists() {
                return Err(WorkflowError::validation_error(
                    format!("Source directory does not exist: {:?}", source_dir)
                ));
            }
            
            if !source_dir.is_dir() {
                return Err(WorkflowError::validation_error(
                    format!("Source path is not a directory: {:?}", source_dir)
                ));
            }
        }
        
        // Validate database path exists
        if !self.db_path.exists() {
            return Err(WorkflowError::validation_error(
                format!("Database file does not exist: {:?}", self.db_path)
            ));
        }
        
        // Validate ignore classes file if specified
        if let Some(ignore_file) = &self.ignore_classes_file {
            if !ignore_file.exists() {
                log::warn!("Ignore classes file does not exist: {:?}", ignore_file);
            }
        }
        
        // Validate that at least one report type is enabled
        if !options.include_missing_dependencies 
            && !options.include_game_data_classes 
            && !options.include_mission_dependencies {
            return Err(WorkflowError::validation_error(
                "At least one report type must be enabled"
            ));
        }
        
        debug!("Reporting configuration validation passed");
        Ok(())
    }
}

impl FuzzyReporterAdapter {
    /// Create a new fuzzy reporter adapter with the given database path and optional ignore classes file
    pub fn new(db_path: PathBuf, ignore_classes_file: Option<PathBuf>) -> Self {
        Self { 
            db_path,
            ignore_classes_file,
        }
    }
}

#[async_trait]
impl ReporterInterface for FuzzyReporterAdapter {
    async fn generate_reports(
        &self,
        _source_dir: &PathBuf,
        output_dir: &PathBuf,
        _options: &ReportingOptions,
    ) -> Result<ReportingSummary> {
        let start_time = Instant::now();
        
        debug!("Starting fuzzy report generation to {:?}", output_dir);
        
        // Create report output directory if it doesn't exist
        std::fs::create_dir_all(output_dir)
            .map_err(|e| WorkflowError::reporting_error(format!("Failed to create report directory: {}", e)))?;

        // Create database manager
        let db_manager = DatabaseManager::new(&self.db_path)
            .map_err(|e| WorkflowError::reporting_error(format!("Failed to create database manager: {}", e)))?;

        // Create report coordinator with ignore classes file
        let coordinator = ReportCoordinator::new(&db_manager, self.ignore_classes_file.clone());

        let mut reports_generated = 0;
        let mut report_paths = Vec::new();
        let mut errors = Vec::new();

        // Generate fuzzy missing class report
        info!("Generating fuzzy missing class report...");
        match coordinator.generate_fuzzy_missing_class_report(output_dir) {
            Ok(()) => {
                reports_generated += 1;
                report_paths.push(output_dir.join("fuzzy_missing_class_report.csv"));
                info!("Fuzzy missing class report generated successfully");
            }
            Err(e) => {
                let error_msg = format!("Failed to generate fuzzy missing class report: {}", e);
                errors.push(error_msg.clone());
                log::warn!("{}", error_msg);
            }
        }

        let elapsed_time = start_time.elapsed();

        info!("Fuzzy report generation completed in {:?}: {} reports generated", 
              elapsed_time, reports_generated);

        // If no reports were generated and we have errors, fail the operation
        if reports_generated == 0 && !errors.is_empty() {
            return Err(WorkflowError::reporting_error(
                format!("Fuzzy report generation failed with no reports created. Errors: {}", errors.join("; "))
            ));
        }

        Ok(ReportingSummary {
            generated_reports: reports_generated,
            report_time: elapsed_time,
            output_files: report_paths,
        })
    }
    
    async fn validate_reporting_config(&self, options: &ReportingOptions) -> Result<()> {
        debug!("Validating fuzzy reporting configuration");
        
        // Validate source directories if they are specified
        for source_dir in &options.source_directories {
            if !source_dir.exists() {
                return Err(WorkflowError::validation_error(
                    format!("Source directory does not exist: {:?}", source_dir)
                ));
            }
            
            if !source_dir.is_dir() {
                return Err(WorkflowError::validation_error(
                    format!("Source path is not a directory: {:?}", source_dir)
                ));
            }
        }
        
        // Validate database path exists
        if !self.db_path.exists() {
            return Err(WorkflowError::validation_error(
                format!("Database file does not exist: {:?}", self.db_path)
            ));
        }
        
        // Validate ignore classes file if specified
        if let Some(ignore_file) = &self.ignore_classes_file {
            if !ignore_file.exists() {
                log::warn!("Ignore classes file does not exist: {:?}", ignore_file);
            }
        }
        
        debug!("Fuzzy reporting configuration validation passed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    use arma3_workflow::types::options::ReportFormat;

    fn create_test_database(path: &PathBuf) -> Result<()> {
        // Create a simple test database file
        fs::write(path, "test database")?;
        Ok(())
    }

    fn create_test_reporting_options() -> ReportingOptions {
        ReportingOptions {
            format: ReportFormat::Markdown,
            output_path: None,
            source_directories: vec![],
            include_mission_dependencies: true,
            include_game_data_classes: true,
            include_missing_dependencies: true,
        }
    }

    #[tokio::test]
    async fn test_reporter_adapter_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, None);
        // Adapter should be created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_reporter_adapter_with_ignore_file() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let ignore_path = temp_dir.path().join("ignore.txt");
        create_test_database(&db_path).unwrap();
        fs::write(&ignore_path, "ignored_class").unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, Some(ignore_path));
        // Adapter should be created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_fuzzy_reporter_adapter_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = FuzzyReporterAdapter::new(db_path, None);
        // Adapter should be created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_validate_reporting_config_valid() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let source_dir = temp_dir.path().join("source");
        create_test_database(&db_path).unwrap();
        fs::create_dir_all(&source_dir).unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, None);
        let mut options = create_test_reporting_options();
        options.source_directories = vec![source_dir];
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_reporting_config_nonexistent_source() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, None);
        let mut options = create_test_reporting_options();
        options.source_directories = vec![PathBuf::from("/nonexistent/path")];
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Source directory does not exist"));
    }

    #[tokio::test]
    async fn test_validate_reporting_config_source_not_directory() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let source_file = temp_dir.path().join("source_file.txt");
        create_test_database(&db_path).unwrap();
        fs::write(&source_file, "test content").unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, None);
        let mut options = create_test_reporting_options();
        options.source_directories = vec![source_file];
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Source path is not a directory"));
    }

    #[tokio::test]
    async fn test_validate_reporting_config_nonexistent_database() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("nonexistent.db");
        
        let adapter = Arma3ReporterAdapter::new(db_path, None);
        let options = create_test_reporting_options();
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database file does not exist"));
    }

    #[tokio::test]
    async fn test_validate_reporting_config_no_report_types() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, None);
        let mut options = create_test_reporting_options();
        options.include_mission_dependencies = false;
        options.include_game_data_classes = false;
        options.include_missing_dependencies = false;
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("At least one report type must be enabled"));
    }

    #[tokio::test]
    async fn test_validate_reporting_config_with_ignore_file() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let ignore_path = temp_dir.path().join("ignore.txt");
        create_test_database(&db_path).unwrap();
        fs::write(&ignore_path, "ignored_class").unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, Some(ignore_path));
        let options = create_test_reporting_options();
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_reporting_config_nonexistent_ignore_file() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let ignore_path = temp_dir.path().join("nonexistent_ignore.txt");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, Some(ignore_path));
        let options = create_test_reporting_options();
        
        // Should succeed but log a warning about missing ignore file
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fuzzy_reporter_validate_config_valid() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = FuzzyReporterAdapter::new(db_path, None);
        let options = create_test_reporting_options();
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fuzzy_reporter_validate_config_nonexistent_database() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("nonexistent.db");
        
        let adapter = FuzzyReporterAdapter::new(db_path, None);
        let options = create_test_reporting_options();
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database file does not exist"));
    }

    #[tokio::test]
    async fn test_generate_reports_nonexistent_database() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("nonexistent.db");
        let output_dir = temp_dir.path().join("output");
        
        let adapter = Arma3ReporterAdapter::new(db_path, None);
        let options = create_test_reporting_options();
        
        // This should fail because database doesn't exist
        let result = adapter.generate_reports(&temp_dir.path().to_path_buf(), &output_dir, &options).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_fuzzy_generate_reports_nonexistent_database() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("nonexistent.db");
        let output_dir = temp_dir.path().join("output");
        
        let adapter = FuzzyReporterAdapter::new(db_path, None);
        let options = create_test_reporting_options();
        
        // This should fail because database doesn't exist
        let result = adapter.generate_reports(&temp_dir.path().to_path_buf(), &output_dir, &options).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reporting_options_mapping() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, None);
        
        // Test with selective report types
        let mut options = create_test_reporting_options();
        options.include_mission_dependencies = true;
        options.include_game_data_classes = false;
        options.include_missing_dependencies = false;
        
        let validation_result = adapter.validate_reporting_config(&options).await;
        assert!(validation_result.is_ok());
        
        // Test with different format
        options.format = ReportFormat::Json;
        let validation_result = adapter.validate_reporting_config(&options).await;
        assert!(validation_result.is_ok());
    }

    #[tokio::test]
    async fn test_empty_source_directories() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ReporterAdapter::new(db_path, None);
        let mut options = create_test_reporting_options();
        options.source_directories = vec![]; // Empty should be valid for workflow
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_ok());
    }
}