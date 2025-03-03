mod scanner;
pub mod processor;

use std::path::Path;
use anyhow::{Result, Context};
use log::{info, debug};
use crate::commands::ScanClassesArgs;
use processor::{ProcessedClass, process_classes};

/// Scan and process class files without generating reports
pub async fn scan_classes_only(args: &ScanClassesArgs) -> Result<Vec<ProcessedClass>> {
    info!("Scanning class files in {}", args.input_dir.display());
    
    // Collect all .cpp files in the input directory
    let files = scanner::collect_files(&args.input_dir)?;
    
    // Limit the number of files if specified
    let files = if let Some(max_files) = args.max_files {
        debug!("Limiting to {} files", max_files);
        files.into_iter().take(max_files).collect()
    } else {
        files
    };
    
    info!("Found {} class files to process", files.len());
    
    // Process the classes
    let (processed_classes, stats) = process_classes(&files, args.verbose_errors)?;
    
    info!("Processed {} classes from {} files", stats.total_classes, stats.total_files);
    info!("  - Files with classes: {}", stats.files_with_classes);
    info!("  - Empty files: {}", stats.empty_files);
    info!("  - Files with errors: {}", stats.error_files);
    
    Ok(processed_classes)
}

/// Legacy function for backward compatibility
#[deprecated(since = "0.2.0", note = "use scan_classes_only instead")]
pub async fn scan_classes(args: ScanClassesArgs) -> Result<Vec<ProcessedClass>> {
    scan_classes_only(&args).await
}