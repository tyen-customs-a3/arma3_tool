use std::path::Path;
use anyhow::{Result, Context};
use log::{info, warn, error};
use crate::commands::ScanClassesArgs;

mod scanner;
mod processor;
mod reports;

use scanner::collect_files;
use processor::process_classes;
use reports::ReportWriter;

pub async fn scan_classes(args: ScanClassesArgs) -> Result<()> {
    info!("Starting class scan in {}", args.input_dir.display());

    // Collect all cpp/hpp files
    let files = collect_files(&args.input_dir)?;
    if files.is_empty() {
        error!("No files found to scan in {}", args.input_dir.display());
        return Ok(());
    }

    info!("Found {} files to scan", files.len());
    
    // Process all classes
    let (processed_results, stats) = process_classes(&files)
        .context("Failed to process classes")?;
    
    info!("Processing completed:");
    info!("- Total files processed: {}", stats.total_files);
    info!("- Files containing classes: {}", stats.files_with_classes);
    info!("- Empty files: {}", stats.empty_files);
    info!("- Total classes found: {}", stats.total_classes);

    if processed_results.is_empty() {
        warn!("No classes were processed. Check input directory and file patterns.");
        return Ok(());
    }
    
    // Generate reports
    let report_writer = ReportWriter::new(&args.output_dir);
    report_writer.write_all_reports(&processed_results)?;

    info!("Reports written to {}", args.output_dir.display());

    Ok(())
}