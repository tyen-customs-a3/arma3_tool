mod scanner;
mod extractor;
mod analyzer;
pub mod validator;

use std::fs;
use anyhow::Result;
use log::{info, warn};

use crate::commands::ScanMissionsArgs;
use crate::scanning::classes::processor::ProcessedClass;
pub use analyzer::MissionDependencyResult;
pub use validator::ClassExistenceReport;

/// Scan mission files without generating reports
pub async fn scan_missions_only(args: &ScanMissionsArgs) -> Result<Vec<MissionDependencyResult>> {
    info!("Scanning mission files in {}", args.input_dir.display());
    
    // Create necessary directories
    fs::create_dir_all(&args.cache_dir)?;
    fs::create_dir_all(&args.output_dir)?;
    
    // Create a mission scanner
    let mission_scanner = scanner::MissionScanner::new(
        &args.input_dir,
        &args.cache_dir,
        args.threads
    );
    
    // Scan and extract mission files
    info!("Scanning and extracting mission files");
    let extraction_results = mission_scanner.scan_and_extract().await?;
    
    if extraction_results.is_empty() {
        warn!("No mission files found or extracted");
        return Ok(Vec::new());
    }
    
    info!("Extracted {} mission files", extraction_results.len());
    
    // Analyze mission dependencies
    let dependency_analyzer = analyzer::DependencyAnalyzer::new(&args.cache_dir);
    let mission_results = dependency_analyzer.analyze_missions(&extraction_results)?;
    
    info!("Analyzed dependencies for {} missions", mission_results.len());
    
    Ok(mission_results)
}

/// Validate classes against mission dependencies
pub async fn validate_classes_only(
    mission_results: &[MissionDependencyResult],
    processed_classes: &[ProcessedClass]
) -> Result<ClassExistenceReport> {
    info!("Validating classes against mission dependencies");
    
    // Create a class validator
    let mut class_validator = validator::ClassExistenceValidator::new();
    
    // Load class database from memory
    class_validator.load_class_database_from_memory(processed_classes)?;
    
    // Validate mission classes
    let class_existence_report = class_validator.validate_mission_classes(mission_results)?;
    
    info!("Validated {} missions with {} unique classes", 
        class_existence_report.total_missions,
        class_existence_report.total_unique_classes
    );
    
    Ok(class_existence_report)
}

/// Legacy function for backward compatibility
#[deprecated(since = "0.2.0", note = "use scan_missions_only instead")]
pub async fn scan_missions(args: ScanMissionsArgs) -> Result<()> {
    scan_missions_only(&args).await?;
    Ok(())
}