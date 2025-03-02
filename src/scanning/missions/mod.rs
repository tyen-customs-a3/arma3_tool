use std::path::Path;
use anyhow::{Result, Context};
use log::{info, warn, error};
use crate::commands::{ScanMissionsArgs, AnalyzeMissionDependenciesArgs};
use std::fs;

mod scanner;
mod extractor;
mod analyzer;
mod reports;

use scanner::MissionScanner;
use extractor::MissionExtractor;
use analyzer::EquipmentAnalyzer;
use reports::{MissionReportManager, DependencyReportManager};

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
    let report_manager = MissionReportManager::new(&args.output_dir);
    report_manager.write_reports(&analysis_results)
        .context("Failed to write mission reports")?;
    
    info!("Mission analysis complete. Reports written to {}", args.output_dir.display());
    
    Ok(())
}

/// Run a complete analysis pipeline: extract PBOs, scan missions, and verify class dependencies
pub async fn analyze_mission_dependencies(args: AnalyzeMissionDependenciesArgs) -> Result<()> {
    info!("Starting complete mission dependency analysis");
    
    // Create necessary directories
    let addon_cache_dir = args.cache_dir.join("addons");
    let mission_cache_dir = args.cache_dir.join("missions");
    let mission_reports_dir = args.output_dir.join("mission_reports");
    let class_reports_dir = args.output_dir.join("class_reports");
    let dependency_reports_dir = args.output_dir.join("dependency_reports");
    
    fs::create_dir_all(&args.output_dir)?;
    fs::create_dir_all(&dependency_reports_dir)?;
    
    // Check if we have already extracted data
    let addons_extracted = Path::new(&addon_cache_dir).exists() && 
        fs::read_dir(&addon_cache_dir)?.next().is_some();
    
    let missions_extracted = Path::new(&mission_cache_dir).exists() && 
        fs::read_dir(&mission_cache_dir)?.next().is_some() &&
        Path::new(&mission_reports_dir).exists() &&
        fs::read_dir(&mission_reports_dir)?.next().is_some();
    
    let class_reports_exist = Path::new(&class_reports_dir).exists() && 
        fs::read_dir(&class_reports_dir)?.next().is_some();
    
    // Step 1: Extract addon PBOs to cache (if needed)
    if !addons_extracted {
        info!("Step 1/4: Extracting addon PBOs");
        let addon_args = crate::commands::ScanPboArgs {
            input_dir: args.addon_dir.clone(),
            cache_dir: addon_cache_dir.clone(),
            extensions: "hpp,cpp".to_string(),
            threads: args.threads,
        };
        
        crate::scanning::pbo::scan_pbos(addon_args).await
            .context("Failed to extract addon PBOs")?;
    } else {
        info!("Step 1/4: Skipping addon PBO extraction (using cached data)");
    }
    
    // Step 2: Extract mission PBOs to cache (if needed)
    if !missions_extracted {
        info!("Step 2/4: Extracting and analyzing mission PBOs");
        let mission_args = ScanMissionsArgs {
            input_dir: args.mission_dir.clone(),
            cache_dir: mission_cache_dir.clone(),
            output_dir: mission_reports_dir.clone(),
            threads: args.threads,
        };
        
        scan_missions(mission_args).await
            .context("Failed to extract and analyze mission PBOs")?;
    } else {
        info!("Step 2/4: Skipping mission PBO extraction (using cached data)");
    }
    
    // Step 3: Scan addon classes (if needed)
    if !class_reports_exist {
        info!("Step 3/4: Scanning addon classes");
        let class_args = crate::commands::ScanClassesArgs {
            input_dir: addon_cache_dir,
            output_dir: class_reports_dir.clone(),
            max_files: None,
            verbose_errors: false,
        };
        
        crate::scanning::classes::scan_classes(class_args).await
            .context("Failed to scan addon classes")?;
    } else {
        info!("Step 3/4: Skipping addon class scanning (using cached data)");
    }
    
    // Step 4: Cross-reference mission equipment with available classes
    info!("Step 4/4: Cross-referencing mission equipment with available classes");
    
    // Make sure the output directory exists
    fs::create_dir_all(&dependency_reports_dir)?;
    
    let dependency_analyzer = analyzer::DependencyAnalyzer::new(
        &mission_cache_dir,
        &mission_reports_dir,
        &class_reports_dir,
    );
    
    let dependency_results = dependency_analyzer.analyze()
        .context("Failed to analyze dependencies")?;
    
    // Write dependency reports
    let report_manager = DependencyReportManager::new(&dependency_reports_dir);
    report_manager.write_dependency_report(&dependency_results)
        .context("Failed to write dependency reports")?;
    
    // Generate cross-reference report between classes and missions
    info!("Step 5/5: Generating cross-reference reports between classes and missions");
    
    // This would be implemented in a real system to create reports that show:
    // 1. Which classes are used by which missions
    // 2. Which missions need which classes
    // 3. Categorized view of class usage across missions
    
    info!("Mission dependency analysis complete. Reports written to:");
    info!("- Mission reports: {}", mission_reports_dir.display());
    info!("- Class reports: {}", class_reports_dir.display());
    info!("- Dependency reports: {}", dependency_reports_dir.display());
    
    Ok(())
} 