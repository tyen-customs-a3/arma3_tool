use std::path::Path;
use anyhow::{Result, Context};
use log::{info, warn, error};
use crate::commands::{ScanMissionsArgs, AnalyzeMissionDependenciesArgs};
use std::fs;
use std::io::Write;
use crate::reporting::{ReportConfig, MissionReportManager, DependencyReportManager};

mod scanner;
mod extractor;
mod analyzer;
mod validator;

use scanner::MissionScanner;
use extractor::MissionExtractor;
use analyzer::DependencyAnalyzer;
use validator::ClassExistenceValidator;
use crate::scanning::classes::processor::ProcessedClass;

/// Create a report configuration from command-line arguments
fn create_report_config(disable_reports: Option<&str>, enable_reports: Option<&str>) -> ReportConfig {
    match (disable_reports, enable_reports) {
        // If enable_reports is specified, create a config with all reports disabled by default
        // and then enable only the specified reports
        (_, Some(enable_list)) => {
            let mut config = ReportConfig::all_disabled();
            for report_type in enable_list.split(',').map(|s| s.trim()) {
                if !report_type.is_empty() {
                    config.enable(report_type);
                }
            }
            config
        },
        // If only disable_reports is specified, create a config with all reports enabled by default
        // and then disable the specified reports
        (Some(disable_list), None) => {
            let mut config = ReportConfig::new();
            for report_type in disable_list.split(',').map(|s| s.trim()) {
                if !report_type.is_empty() {
                    config.disable(report_type);
                }
            }
            config
        },
        // If neither is specified, create a config with all reports enabled by default
        (None, None) => ReportConfig::new(),
    }
}

/// Scan mission files, extract PBOs, and analyze class dependencies
pub async fn scan_missions(args: ScanMissionsArgs) -> Result<()> {
    // Call the scan_missions_with_results function and discard the results
    scan_missions_with_results(args).await?;
    Ok(())
}

/// Run a complete analysis pipeline: extract PBOs, scan missions, and verify class dependencies
pub async fn analyze_mission_dependencies(args: AnalyzeMissionDependenciesArgs) -> Result<()> {
    info!("Starting mission class dependency analysis");
    
    // Create necessary directories - only create cache directories for analysis data, not duplicated files
    let mission_cache_dir = args.cache_dir.join("missions");
    
    // Use the output directory directly instead of creating a nested mission_reports directory
    fs::create_dir_all(&args.output_dir)?;
    
    // Check if we have already extracted data
    let missions_extracted = Path::new(&mission_cache_dir).exists() && 
        fs::read_dir(&mission_cache_dir)?.next().is_some();
    
    // Create report configuration from command-line arguments
    let report_config = create_report_config(
        args.disable_reports.as_deref(),
        args.enable_reports.as_deref()
    );
    
    // Log which reports are disabled
    if let Some(disable_reports) = &args.disable_reports {
        info!("Disabling reports: {}", disable_reports);
    }
    
    // Log which reports are enabled (if using enable_reports)
    if let Some(enable_reports) = &args.enable_reports {
        info!("Enabling only these reports: {}", enable_reports);
    }
    
    // Extract mission PBOs and analyze class dependencies
    let mission_args = ScanMissionsArgs {
        input_dir: args.mission_dir.clone(),
        cache_dir: mission_cache_dir.clone(),
        output_dir: args.output_dir.clone(), // Use the output directory directly
        threads: args.threads,
        disable_reports: args.disable_reports.clone(),
        enable_reports: args.enable_reports.clone(),
    };
    
    // Scan missions and analyze dependencies
    let dependency_results = match scan_missions_with_results(mission_args).await {
        Ok(results) => results,
        Err(e) => {
            error!("Failed to scan missions: {}", e);
            return Err(e);
        }
    };
    
    // If class_db_dir is provided, validate class existence
    if let Some(class_db_dir) = &args.class_db_dir {
        info!("Validating class existence against database in {}", class_db_dir.display());
        
        // Create a validator
        let mut validator = ClassExistenceValidator::new();
        
        // Load the class database from disk
        validator.load_class_database_from_disk(class_db_dir)
            .context(format!("Failed to load class database from {}", class_db_dir.display()))?;
        
        // Validate class existence
        let validation_result = validator.validate_mission_classes(&dependency_results)
            .context("Failed to validate class existence")?;
        
        // Write validation report
        let report_manager = DependencyReportManager::with_config(&args.output_dir, report_config);
        report_manager.write_class_existence_report(&validation_result)?;
        
        info!("Class existence validation complete");
        info!("- Total unique classes: {}", validation_result.total_unique_classes);
        info!("- Existing classes: {}", validation_result.existing_classes);
        info!("- Missing classes: {}", validation_result.missing_classes);
        info!("- Existence percentage: {:.2}%", validation_result.existence_percentage);
    }
    
    info!("Mission dependency analysis complete");
    Ok(())
}

/// Scan missions and return the dependency results for further processing
async fn scan_missions_with_results(args: ScanMissionsArgs) -> Result<Vec<analyzer::MissionDependencyResult>> {
    info!("Starting mission scan in {} to extract class dependencies", args.input_dir.display());

    // Create report configuration from command-line arguments
    let report_config = create_report_config(
        args.disable_reports.as_deref(),
        args.enable_reports.as_deref()
    );
    
    // Log which reports are disabled
    if let Some(disable_reports) = &args.disable_reports {
        info!("Disabling reports: {}", disable_reports);
    }
    
    // Log which reports are enabled (if using enable_reports)
    if let Some(enable_reports) = &args.enable_reports {
        info!("Enabling only these reports: {}", enable_reports);
    }

    // Create the mission scanner
    let scanner = MissionScanner::new(&args.input_dir, &args.cache_dir, args.threads);
    
    // Scan and extract mission PBOs
    let extraction_results = scanner.scan_and_extract().await
        .context("Failed to scan and extract mission PBOs")?;
    
    if extraction_results.is_empty() {
        warn!("No mission files were found or extracted. Check input directory.");
        return Ok(Vec::new());
    }
    
    // Verify extraction results
    let mut valid_extractions = Vec::new();
    let mut failed_extractions = Vec::new();
    
    for result in extraction_results {
        // Check if the extraction directory exists and has files
        if !result.extracted_path.exists() {
            warn!("Extraction directory doesn't exist for mission: {}", result.mission_name);
            failed_extractions.push(result.mission_name.clone());
            continue;
        }
        
        // Check if any files were extracted
        let has_files = walkdir::WalkDir::new(&result.extracted_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .any(|e| e.file_type().is_file());
        
        if !has_files {
            warn!("No files were extracted for mission: {}", result.mission_name);
            failed_extractions.push(result.mission_name.clone());
            continue;
        }
        
        // Check if mission.sqm was found
        if result.sqm_file.is_none() {
            warn!("No mission.sqm file found for mission: {}", result.mission_name);
            // We'll still process it, but log the warning
        }
        
        valid_extractions.push(result);
    }
    
    if valid_extractions.is_empty() {
        error!("All mission extractions failed. Check logs for details.");
        return Err(anyhow::anyhow!("All mission extractions failed"));
    }
    
    if !failed_extractions.is_empty() {
        warn!("Failed to extract {} missions: {}", 
              failed_extractions.len(), 
              failed_extractions.join(", "));
    }
    
    info!("Successfully extracted {} mission files", valid_extractions.len());
    
    // Analyze class dependencies in the extracted missions
    let analyzer = DependencyAnalyzer::new(&args.cache_dir);
    let dependency_results = analyzer.analyze_missions(&valid_extractions)
        .context("Failed to analyze mission class dependencies")?;
    
    // Create the main output directory
    fs::create_dir_all(&args.output_dir)
        .context(format!("Failed to create output directory: {}", args.output_dir.display()))?;
    
    // Write reports using the report managers with configuration
    let mission_report_manager = MissionReportManager::with_config(&args.output_dir, report_config.clone());
    mission_report_manager.write_reports(&dependency_results)?;
    
    let dependency_report_manager = DependencyReportManager::with_config(&args.output_dir, report_config);
    dependency_report_manager.write_dependency_report(&dependency_results)?;
    
    info!("Mission class dependency analysis complete. Reports written to {}", args.output_dir.display());
    
    // Return the dependency results for further processing
    Ok(dependency_results)
}

/// Run a complete analysis pipeline with in-memory class data
pub async fn analyze_mission_dependencies_with_classes(
    args: AnalyzeMissionDependenciesArgs, 
    class_data: Vec<ProcessedClass>
) -> Result<()> {
    info!("Starting mission class dependency analysis with in-memory class data");
    
    // Create necessary directories - only create cache directories for analysis data, not duplicated files
    let mission_cache_dir = args.cache_dir.join("missions");
    
    // Use the output directory directly instead of creating a nested mission_reports directory
    fs::create_dir_all(&args.output_dir)?;
    
    // Create report configuration from command-line arguments
    let report_config = create_report_config(
        args.disable_reports.as_deref(),
        args.enable_reports.as_deref()
    );
    
    // Log which reports are disabled
    if let Some(disable_reports) = &args.disable_reports {
        info!("Disabling reports: {}", disable_reports);
    }
    
    // Log which reports are enabled (if using enable_reports)
    if let Some(enable_reports) = &args.enable_reports {
        info!("Enabling only these reports: {}", enable_reports);
    }
    
    // Extract mission PBOs and analyze class dependencies
    let mission_args = ScanMissionsArgs {
        input_dir: args.mission_dir.clone(),
        cache_dir: mission_cache_dir.clone(),
        output_dir: args.output_dir.clone(), // Use the output directory directly
        threads: args.threads,
        disable_reports: args.disable_reports.clone(),
        enable_reports: args.enable_reports.clone(),
    };
    
    // Scan missions and analyze dependencies
    let dependency_results = match scan_missions_with_results(mission_args).await {
        Ok(results) => results,
        Err(e) => {
            error!("Failed to scan missions: {}", e);
            return Err(e);
        }
    };
    
    // Validate class existence using the provided class data
    info!("Validating class existence against in-memory class data ({} classes)", class_data.len());
    
    // Create a validator with in-memory class data
    let mut validator = ClassExistenceValidator::new();
    
    // Load the class data into the validator
    validator.load_class_database_from_memory(&class_data)
        .context("Failed to load class database from memory")?;
    
    // Validate class existence
    let validation_result = validator.validate_mission_classes(&dependency_results)
        .context("Failed to validate class existence")?;
    
    // Write validation report
    let report_manager = DependencyReportManager::with_config(&args.output_dir, report_config);
    report_manager.write_class_existence_report(&validation_result)?;
    
    info!("Class existence validation complete");
    info!("- Total unique classes: {}", validation_result.total_unique_classes);
    info!("- Existing classes: {}", validation_result.existing_classes);
    info!("- Missing classes: {}", validation_result.missing_classes);
    info!("- Existence percentage: {:.2}%", validation_result.existence_percentage);
    
    info!("Mission dependency analysis complete");
    Ok(())
}

// Helper function to sanitize mission names for folder creation
fn sanitize_mission_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}