use std::path::Path;
use anyhow::{Result, Context, anyhow};
use log::{info, warn, error, debug};
use indicatif::{ProgressBar, ProgressStyle};
use crate::commands::ScanClassesArgs;
use crate::reporting::ClassReportWriter;

mod scanner;
mod processor;
mod reports;

use scanner::collect_files;
use processor::process_classes;
use reports::ReportWriter;

pub async fn scan_classes(args: ScanClassesArgs) -> Result<()> {
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
        return Ok(());
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
        return Ok(());
    }
    
    // Generate reports
    info!("Writing reports to {}", args.output_dir.display());
    let report_writer = ReportWriter::new(&args.output_dir);
    
    // Write the main class reports
    report_writer.write_all_reports(&processed_results)
        .context(format!("Failed to write class reports to {}", args.output_dir.display()))?;
    
    // Write statistics report
    report_writer.write_stats_report(&stats)
        .context(format!("Failed to write statistics report to {}", args.output_dir.display()))?;

    info!("Reports written to {}", args.output_dir.display());
    info!("Class scan completed successfully");

    Ok(())
}