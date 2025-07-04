use std::path::PathBuf;
use std::time::Instant;
use async_trait::async_trait;
use log::{info, debug};

use arma3_database::DatabaseManager;
use arma3_reporter::ReportCoordinator;
use arma3_workflow::{ReporterInterface, ReportingOptions, ReportingSummary, WorkflowError, Result};

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
            reports_generated,
            report_paths,
            elapsed_time,
            errors,
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
            reports_generated,
            report_paths,
            elapsed_time,
            errors,
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