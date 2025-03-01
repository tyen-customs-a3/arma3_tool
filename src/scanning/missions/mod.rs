use std::path::Path;
use anyhow::{Result, Context};
use log::{info, warn, error};
use crate::commands::{ScanMissionsArgs, AnalyzeMissionDependenciesArgs};

mod scanner;
mod extractor;
mod analyzer;
mod reports;

use scanner::MissionScanner;
use extractor::MissionExtractor;
use analyzer::EquipmentAnalyzer;
use reports::MissionReportWriter;

/// Scan mission files, extract PBOs, and analyze equipment dependencies
pub async fn scan_missions(args: ScanMissionsArgs) -> Result<()> {
    info!("Starting mission scan in {}", args.input_dir.display());

    // Create the mission scanner
    let scanner = MissionScanner::new(&args.input_dir, &args.cache_dir, args.threads);
    
    // Scan and extract mission PBOs
    let extraction_results = scanner.scan_and_extract().await
        .context("Failed to scan and extract mission PBOs")?;
    
    if extraction_results.is_empty() {
        warn!("No mission files were found or extracted. Check input directory.");
        return Ok(());
    }
    
    info!("Successfully extracted {} mission files", extraction_results.len());
    
    // Analyze equipment dependencies in the extracted missions
    let analyzer = EquipmentAnalyzer::new(&args.cache_dir);
    let analysis_results = analyzer.analyze_missions(&extraction_results)
        .context("Failed to analyze mission equipment dependencies")?;
    
    // Generate reports
    let report_writer = MissionReportWriter::new(&args.output_dir);
    report_writer.write_reports(&analysis_results)
        .context("Failed to write mission reports")?;
    
    info!("Mission analysis complete. Reports written to {}", args.output_dir.display());
    
    Ok(())
}

/// Run a complete analysis pipeline: extract PBOs, scan missions, and verify class dependencies
pub async fn analyze_mission_dependencies(args: AnalyzeMissionDependenciesArgs) -> Result<()> {
    info!("Starting complete mission dependency analysis");
    
    // Step 1: Extract addon PBOs to cache
    let addon_cache_dir = args.cache_dir.join("addons");
    let addon_args = crate::commands::ScanPboArgs {
        input_dir: args.addon_dir.clone(),
        cache_dir: addon_cache_dir.clone(),
        extensions: "hpp,cpp".to_string(),
        threads: args.threads,
    };
    
    info!("Step 1/4: Extracting addon PBOs");
    crate::scanning::pbo::scan_pbos(addon_args).await
        .context("Failed to extract addon PBOs")?;
    
    // Step 2: Extract mission PBOs to cache
    let mission_cache_dir = args.cache_dir.join("missions");
    let mission_args = ScanMissionsArgs {
        input_dir: args.mission_dir.clone(),
        cache_dir: mission_cache_dir.clone(),
        output_dir: args.output_dir.join("mission_reports"),
        threads: args.threads,
    };
    
    info!("Step 2/4: Extracting and analyzing mission PBOs");
    scan_missions(mission_args).await
        .context("Failed to extract and analyze mission PBOs")?;
    
    // Step 3: Scan addon classes
    let class_args = crate::commands::ScanClassesArgs {
        input_dir: addon_cache_dir,
        output_dir: args.output_dir.join("class_reports"),
    };
    
    info!("Step 3/4: Scanning addon classes");
    crate::scanning::classes::scan_classes(class_args).await
        .context("Failed to scan addon classes")?;
    
    // Step 4: Cross-reference mission equipment with available classes
    info!("Step 4/4: Cross-referencing mission equipment with available classes");
    let mission_reports_dir = args.output_dir.join("mission_reports");
    let class_reports_dir = args.output_dir.join("class_reports");
    let dependency_analyzer = analyzer::DependencyAnalyzer::new(
        &mission_cache_dir,
        &mission_reports_dir,
        &class_reports_dir,
    );
    
    let dependency_results = dependency_analyzer.analyze()
        .context("Failed to analyze dependencies")?;
    
    // Write final dependency report
    let report_writer = reports::DependencyReportWriter::new(&args.output_dir);
    report_writer.write_dependency_report(&dependency_results)
        .context("Failed to write dependency report")?;
    
    info!("Mission dependency analysis complete. Final report written to {}", 
          args.output_dir.join("dependency_report.json").display());
    
    Ok(())
} 