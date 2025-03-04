mod scanner;
pub mod processor;

use std::path::Path;
use anyhow::{Result, Context};
use log::{info, debug, warn};
use crate::commands::ScanClassesArgs;
use processor::{ProcessedClass, process_classes};

/// Scan and process class files without generating reports
pub async fn scan_classes_only(args: &ScanClassesArgs) -> Result<Vec<ProcessedClass>> {
    info!("Scanning class files in {}", args.input_dir.display());
    
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&args.output_dir)
        .context("Failed to create output directory")?;
    
    // Collect all .cpp and .hpp files in the input directory
    let files = scanner::collect_files(&args.input_dir)?;
    
    // Limit the number of files if specified
    let files = if let Some(max_files) = args.max_files {
        debug!("Limiting to {} files", max_files);
        files.into_iter().take(max_files).collect()
    } else {
        files
    };
    
    info!("Found {} class files to process", files.len());
    
    // Process the classes using the two-phase approach
    let (processed_classes, stats) = process_classes(&files, args.verbose_errors, &args.output_dir)?;
    
    info!("Processed {} classes from {} files", stats.total_classes, stats.total_files);
    info!("  - Files with classes: {}", stats.files_with_classes);
    info!("  - Empty files: {}", stats.empty_files);
    info!("  - Files with errors: {}", stats.error_files);
    
    // Log some details about error files if there are any
    if stats.error_files > 0 {
        info!("Some files failed to parse but processing continued safely.");
        if args.verbose_errors && stats.error_files <= 10 {
            for path in &stats.error_file_paths {
                debug!("  - Error in file: {}", path.display());
            }
        }
    }
    
    // Check for timeout files
    let timeout_files = scanner::get_timeout_files();
    if !timeout_files.is_empty() {
        warn!("{} files timed out after the 10-second per-file timeout", timeout_files.len());
        if args.verbose_errors && timeout_files.len() <= 10 {
            for path in &timeout_files {
                debug!("  - Timeout: {}", path.display());
            }
        }
        info!("See timeout_files.log in the output directory for a complete list");
    }
    
    Ok(processed_classes)
}

/// Scan and process class files
pub async fn scan_classes(args: ScanClassesArgs) -> Result<Vec<ProcessedClass>> {
    let processed_classes = scan_classes_only(&args).await?;
    
    // Additional processing could be added here in the future
    
    Ok(processed_classes)
}