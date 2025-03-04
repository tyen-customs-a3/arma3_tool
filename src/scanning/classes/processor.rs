use std::path::{Path, PathBuf};
use anyhow::Result;
use cpp_parser::{Class, Value};
use log::{debug, warn, info, trace};
use serde::Serialize;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct ProcessedClass {
    pub name: String,
    pub parent: Option<String>,
    pub properties: Vec<(String, String)>,
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Default)]
pub struct ProcessingStats {
    pub total_files: usize,
    pub total_classes: usize,
    pub empty_files: usize,
    pub files_with_classes: usize,
    pub error_files: usize,
    pub error_file_paths: Vec<PathBuf>,
}

pub fn process_classes(
    files: &[PathBuf],
    verbose_errors: bool,
    output_dir: &Path,
) -> Result<(Vec<ProcessedClass>, ProcessingStats)> {
    use super::scanner::{scan_files_parallel, get_error_files};
    
    // First phase: Scan all files and collect classes
    info!("Phase 1: Scanning files for classes");
    let scanned_files = scan_files_parallel(files, verbose_errors, output_dir)?;
    
    // Get error files from the scanner
    let error_files = get_error_files();
    
    // Count empty files and error files
    let mut stats = ProcessingStats::default();
    stats.total_files = files.len();
    stats.files_with_classes = scanned_files.len();
    stats.error_files = error_files.len();
    stats.error_file_paths = error_files;
    stats.empty_files = files.len() - scanned_files.len() - stats.error_files;
    
    // Second phase: Process the collected classes
    info!("Phase 2: Processing {} files with classes", scanned_files.len());
    let (processed_classes, processing_stats) = process_scanned_classes(scanned_files, verbose_errors)?;
    
    // Combine stats
    stats.total_classes = processing_stats.total_classes;
    
    info!("Processing complete:");
    info!("- Total files processed: {}", stats.total_files);
    info!("- Files containing classes: {}", stats.files_with_classes);
    info!("- Empty files: {}", stats.empty_files);
    info!("- Files with errors: {}", stats.error_files);
    info!("- Total classes found: {}", stats.total_classes);
    
    Ok((processed_classes, stats))
}

/// Process pre-scanned classes from files
/// This function takes the results of scan_files_parallel and processes them
pub fn process_scanned_classes(
    scanned_files: Vec<(PathBuf, Vec<cpp_parser::Class>)>,
    _verbose_errors: bool, // Unused parameter, mark with underscore
) -> Result<(Vec<ProcessedClass>, ProcessingStats)> {
    let mut final_processed_classes = Vec::with_capacity(scanned_files.len() * 5); // Pre-allocate with estimated capacity
    let mut final_stats = ProcessingStats::default();
    final_stats.total_files = scanned_files.len();
    
    // Create a progress bar for better user feedback
    let multi_progress = indicatif::MultiProgress::new();
    let progress_bar = if scanned_files.len() > 10 {
        let pb = multi_progress.add(indicatif::ProgressBar::new(scanned_files.len() as u64));
        pb.set_style(indicatif::ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
            .unwrap()
            .progress_chars("#>-"));
        Some(Arc::new(pb))
    } else {
        None
    };
    
    // Create an atomic counter for tracking overall progress
    let processed_count = Arc::new(AtomicUsize::new(0));
    
    info!("Processing {} files with classes", scanned_files.len());
    
    // Process all scanned files in parallel
    let processed_results: Vec<Vec<ProcessedClass>> = scanned_files.par_iter()
        .map(|(file_path, classes)| {
            // Update progress bar with atomic counter
            let current_count = processed_count.fetch_add(1, Ordering::SeqCst) + 1;
            
            // Update progress bar
            let pb_clone = progress_bar.as_ref().map(Arc::clone);
            if let Some(pb) = &pb_clone {
                pb.set_position(current_count as u64);
                if current_count % 10 == 0 || current_count == 1 {
                    pb.set_message(format!("Processing: {}", file_path.file_name().unwrap_or_default().to_string_lossy()));
                }
            }
            
            // Process classes for this file
            let mut file_processed_classes = Vec::with_capacity(classes.len() * 2); // Pre-allocate with estimated capacity
            process_top_level_classes(classes, file_path, &mut file_processed_classes);
            file_processed_classes
        })
        .collect();
    
    // Finish the progress bar if it exists
    if let Some(pb) = &progress_bar {
        pb.finish_with_message("Processing complete");
    }
    
    // Combine all processed classes
    for processed in processed_results {
        final_processed_classes.extend(processed);
    }
    
    final_stats.files_with_classes = scanned_files.len();
    final_stats.total_classes = final_processed_classes.len();
    
    // Validate processing results
    if final_stats.total_classes == 0 {
        warn!("No classes were found in any of the files!");
    }
    
    info!("Processing stats:");
    info!("- Files containing classes: {}", final_stats.files_with_classes);
    info!("- Total classes found: {}", final_stats.total_classes);
    
    Ok((final_processed_classes, final_stats))
}

fn process_top_level_classes(classes: &[Class], file_path: &Path, processed_classes: &mut Vec<ProcessedClass>) {
    for class in classes {
        trace!("Processing top-level class: {:?} from {}", class.name, file_path.display());
        process_class(class, file_path, processed_classes);
    }
}

fn process_class(class: &Class, file_path: &Path, classes: &mut Vec<ProcessedClass>) {
    // Pre-allocate properties vector with estimated capacity
    let mut properties = Vec::with_capacity(class.properties.len());
    
    // Process properties more efficiently
    for (key, value) in &class.properties {
        let value_str = match value {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Array(arr) => {
                // Pre-allocate values vector
                let mut values = Vec::with_capacity(arr.len());
                for v in arr {
                    let val_str = match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Integer(i) => i.to_string(),
                        Value::Array(nested) => format!("{:?}", nested),
                        Value::Class(c) => format!("{:?}", c),
                        Value::Expression(expr) => format!("({})", expr),
                        Value::Reference(r) => format!("${{{}}}", r),
                    };
                    values.push(val_str);
                }
                format!("[{}]", values.join(", "))
            },
            Value::Class(c) => format!("{:?}", c),
            Value::Expression(expr) => format!("({})", expr),
            Value::Reference(r) => format!("${{{}}}", r),
        };
        properties.push((key.clone(), value_str));
    }

    // Ensure we have a valid name for the processed class
    let name = match &class.name {
        Some(name) => name.clone(),
        None => "UnnamedClass".to_string(), // Provide a default name for unnamed classes
    };

    let processed = ProcessedClass {
        name,
        parent: class.parent.clone(),
        properties,
        file_path: Some(file_path.to_owned()),
    };
    
    trace!("Added class: {} (parent: {:?}) with {} properties", 
           processed.name, 
           processed.parent, 
           processed.properties.len());
    
    classes.push(processed);

    // Process nested classes
    if !class.classes.is_empty() {
        trace!("Processing {} nested classes for {:?}", class.classes.len(), class.name);
        for (_, nested) in &class.classes {
            process_class(nested, file_path, classes);
        }
    }
}