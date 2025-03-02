use std::path::Path;
use anyhow::{Result, Context};
use log::{info, warn, error};
use crate::commands::{ScanMissionsArgs, AnalyzeMissionDependenciesArgs};
use std::fs;
use std::io::Write;

mod scanner;
mod extractor;
mod analyzer;
mod reports;

use scanner::MissionScanner;
use extractor::MissionExtractor;
use analyzer::DependencyAnalyzer;
use reports::{MissionReportManager, DependencyReportManager};

/// Scan mission files, extract PBOs, and analyze class dependencies
pub async fn scan_missions(args: ScanMissionsArgs) -> Result<()> {
    info!("Starting mission scan in {} to extract class dependencies", args.input_dir.display());

    // Create the mission scanner
    let scanner = MissionScanner::new(&args.input_dir, &args.cache_dir, args.threads);
    
    // Scan and extract mission PBOs
    let extraction_results = scanner.scan_and_extract().await
        .context("Failed to scan and extract mission PBOs")?;
    
    if extraction_results.is_empty() {
        warn!("No mission files were found or extracted. Check input directory.");
        return Ok(());
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
    
    // Write summary report
    let summary_path = args.output_dir.join("mission_dependencies_summary.txt");
    let mut summary_file = fs::File::create(&summary_path)
        .context(format!("Failed to create summary file: {}", summary_path.display()))?;
    
    writeln!(summary_file, "MISSION CLASS DEPENDENCIES SUMMARY")?;
    writeln!(summary_file, "===============================\n")?;
    writeln!(summary_file, "Analyzed {} missions\n", dependency_results.len())?;
    
    // Track total unique classes across all missions
    let mut all_classes = std::collections::HashSet::new();
    
    for result in &dependency_results {
        writeln!(summary_file, "Mission: {}", result.mission_name)?;
        writeln!(summary_file, "  - Total unique classes: {}", result.unique_class_names.len())?;
        writeln!(summary_file, "  - Total class references: {}", result.class_dependencies.len())?;
        
        // Add to global set
        for class_name in &result.unique_class_names {
            all_classes.insert(class_name.clone());
        }
        
        // Create a mission-specific folder for reports
        let mission_folder = args.output_dir.join(sanitize_mission_name(&result.mission_name));
        fs::create_dir_all(&mission_folder)
            .context(format!("Failed to create mission folder: {}", mission_folder.display()))?;
        
        // Write detailed report for this mission in its own folder
        let mission_report_path = mission_folder.join("dependencies.json");
        let json_content = serde_json::to_string_pretty(&result.unique_class_names.iter().collect::<Vec<_>>())?;
        fs::write(&mission_report_path, json_content)
            .context(format!("Failed to write mission dependency report: {}", mission_report_path.display()))?;
    }
    
    writeln!(summary_file, "\nTotal unique class dependencies across all missions: {}", all_classes.len())?;
    
    // Write the consolidated list of all unique classes
    let all_classes_path = args.output_dir.join("all_unique_classes.json");
    let all_classes_vec: Vec<_> = all_classes.iter().collect();
    let json_content = serde_json::to_string_pretty(&all_classes_vec)?;
    fs::write(&all_classes_path, json_content)
        .context(format!("Failed to write all classes report: {}", all_classes_path.display()))?;
    
    info!("Mission class dependency analysis complete. Reports written to {}", args.output_dir.display());
    info!("Total unique class dependencies across all missions: {}", all_classes.len());
    
    Ok(())
}

// Helper function to sanitize mission names for folder creation
fn sanitize_mission_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
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
    
    // Extract mission PBOs and analyze class dependencies
    let mission_args = ScanMissionsArgs {
        input_dir: args.mission_dir.clone(),
        cache_dir: mission_cache_dir.clone(),
        output_dir: args.output_dir.clone(), // Use the output directory directly
        threads: args.threads,
    };
    
    match scan_missions(mission_args).await {
        Ok(_) => info!("Successfully extracted and analyzed mission class dependencies"),
        Err(e) => {
            error!("Failed to extract and analyze mission class dependencies: {}", e);
            return Err(anyhow::anyhow!("Failed to analyze mission dependencies"));
        }
    }
    
    info!("Mission dependency analysis complete. Reports written to: {}", args.output_dir.display());
    
    Ok(())
}