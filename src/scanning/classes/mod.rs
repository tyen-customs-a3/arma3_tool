use std::path::Path;
use anyhow::{Result, Context, anyhow};
use log::{info, warn, error, debug};
use indicatif::{ProgressBar, ProgressStyle};
use crate::commands::ScanClassesArgs;
use crate::reporting::{ReportFormat, ReportConfig, ClassReportManager};

mod scanner;
pub mod processor;

use scanner::collect_files;
use processor::{process_classes, ProcessedClass, ProcessingStats};

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

/// Scan class definitions in extracted files
/// Returns the processed class data in addition to writing reports to disk
pub async fn scan_classes(args: ScanClassesArgs) -> Result<Vec<ProcessedClass>> {
    info!("Starting class scan in {}", args.input_dir.display());
    
    // Validate input directory
    if !args.input_dir.exists() {
        return Err(anyhow!("Input directory does not exist: {}", args.input_dir.display()));
    }
    
    if !args.input_dir.is_dir() {
        return Err(anyhow!("Input path is not a directory: {}", args.input_dir.display()));
    }
    
    debug!("Input directory validated: {}", args.input_dir.display());

    // Create output directory if it doesn't exist
    if !args.output_dir.exists() {
        debug!("Creating output directory: {}", args.output_dir.display());
        std::fs::create_dir_all(&args.output_dir)
            .context(format!("Failed to create output directory: {}", args.output_dir.display()))?;
    }

    // Collect all cpp/hpp files
    info!("Collecting files from {}", args.input_dir.display());
    let files = collect_files(&args.input_dir)
        .context(format!("Failed to collect files from {}", args.input_dir.display()))?;
        
    if files.is_empty() {
        error!("No files found to scan in {}", args.input_dir.display());
        return Ok(Vec::new());
    }

    info!("Found {} files to scan", files.len());
    
    // Apply max_files limit if specified
    let files_to_process = if let Some(max_files) = args.max_files {
        if max_files < files.len() {
            info!("Limiting to {} files as specified by --max-files", max_files);
            files[0..max_files].to_vec()
        } else {
            files
        }
    } else {
        files
    };
    
    // Process all classes
    info!("Starting to process classes from {} files", files_to_process.len());
    let process_result = process_classes(&files_to_process, args.verbose_errors);
    
    let (processed_results, stats) = match process_result {
        Ok(result) => result,
        Err(e) => {
            // Provide detailed error information
            error!("Failed to process classes: {}", e);
            error!("Error chain:");
            let mut source = e.source();
            let mut depth = 0;
            while let Some(err) = source {
                error!("  Caused by ({}): {}", depth, err);
                source = err.source();
                depth += 1;
            }
            return Err(anyhow!("Failed to process classes. See logs for details."));
        }
    };
    
    info!("Processing completed:");
    info!("- Total files processed: {}", stats.total_files);
    info!("- Files containing classes: {}", stats.files_with_classes);
    info!("- Empty files: {}", stats.empty_files);
    info!("- Files with errors: {}", stats.error_files);
    info!("- Total classes found: {}", stats.total_classes);
    
    // If there were errors but we continued processing, log a summary
    if stats.error_files > 0 {
        warn!("{} files had errors during processing but were skipped.", stats.error_files);
        if args.verbose_errors {
            warn!("Check the log for detailed error information.");
        } else {
            warn!("Run with --verbose-errors for more detailed error information.");
        }
    }

    if processed_results.is_empty() {
        warn!("No classes were processed. Check input directory and file patterns.");
        return Ok(Vec::new());
    }
    
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
    
    // Generate reports
    info!("Writing reports to {}", args.output_dir.display());
    let report_manager = ClassReportManager::with_config(&args.output_dir, report_config);
    
    // Write the main class reports
    report_manager.write_all_reports(&processed_results)
        .context(format!("Failed to write class reports to {}", args.output_dir.display()))?;
    
    // Write statistics report
    report_manager.write_stats_report(&stats)
        .context(format!("Failed to write statistics report to {}", args.output_dir.display()))?;

    info!("Reports written to {}", args.output_dir.display());
    info!("Class scan completed successfully");

    // Return the processed class data
    Ok(processed_results)
}