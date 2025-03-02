use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use cpp_parser::{Class, Value};
use log::{debug, warn, info, error, trace};
use serde::Serialize;
use indicatif::{ProgressBar, ProgressStyle};
use super::scanner::parse_single_file;

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

pub fn process_classes(files: &[PathBuf], verbose_errors: bool) -> Result<(Vec<ProcessedClass>, ProcessingStats)> {
    let mut processed_classes = Vec::new();
    let mut stats = ProcessingStats::default();
    stats.total_files = files.len();
    
    // Create a progress bar for better user feedback
    let progress_bar = if files.len() > 10 {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
            .unwrap()
            .progress_chars("#>-"));
        Some(pb)
    } else {
        None
    };

    info!("Starting to process {} files", files.len());
    
    for (idx, file) in files.iter().enumerate() {
        if idx % 50 == 0 || idx == files.len() - 1 {
            info!("Processing file {}/{}: {}", idx + 1, files.len(), file.display());
        }
        
        if let Some(pb) = &progress_bar {
            pb.set_position(idx as u64);
            if idx % 10 == 0 {
                pb.set_message(format!("Processing: {}", file.file_name().unwrap_or_default().to_string_lossy()));
            }
        }
        
        let base_dir = file.parent().unwrap_or(Path::new(""));
        
        // Process the file and handle errors
        match parse_single_file(file, base_dir, verbose_errors) {
            Ok(classes) => {
                if classes.is_empty() {
                    stats.empty_files += 1;
                    trace!("No classes found in file: {}", file.display());
                } else {
                    stats.files_with_classes += 1;
                    debug!("Processing {} classes from file: {}", classes.len(), file.display());
                    process_top_level_classes(&classes, file, &mut processed_classes);
                }
            },
            Err(e) => {
                stats.error_files += 1;
                stats.error_file_paths.push(file.clone());
                error!("Error processing file {}/{}: {}", idx + 1, files.len(), file.display());
                error!("Error details: {}", e);
                // Continue processing other files
            }
        }
    }

    if let Some(pb) = progress_bar {
        pb.finish_with_message("Processing complete");
    }

    stats.total_classes = processed_classes.len();
    
    // Validate processing results
    if stats.total_classes == 0 {
        warn!("No classes were found in any of the files!");
    }
    
    info!("Processing stats:");
    info!("- Total files processed: {}", stats.total_files);
    info!("- Files containing classes: {}", stats.files_with_classes);
    info!("- Empty files: {}", stats.empty_files);
    info!("- Files with errors: {}", stats.error_files);
    info!("- Total classes found: {}", stats.total_classes);
    
    // Log error files if any
    if !stats.error_file_paths.is_empty() {
        warn!("Files with errors ({}):", stats.error_files);
        for (i, path) in stats.error_file_paths.iter().enumerate() {
            if i < 10 || stats.error_file_paths.len() <= 20 {
                warn!("  - {}", path.display());
            } else if i == 10 {
                warn!("  - ... and {} more", stats.error_file_paths.len() - 10);
                break;
            }
        }
    }

    Ok((processed_classes, stats))
}

fn process_top_level_classes(classes: &[Class], file_path: &Path, processed_classes: &mut Vec<ProcessedClass>) {
    for class in classes {
        trace!("Processing top-level class: {} from {}", class.name, file_path.display());
        process_class(class, file_path, processed_classes);
    }
}

fn process_class(class: &Class, file_path: &Path, classes: &mut Vec<ProcessedClass>) {
    let properties = class.properties.iter()
        .map(|(key, prop)| {
            let value_str = match &prop.value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Array(arr) => {
                    let values: Vec<String> = arr.iter().map(|v| match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        Value::Enum(e) => e.to_string(),
                        Value::Array(nested) => format!("{:?}", nested),
                    }).collect();
                    format!("[{}]", values.join(", "))
                },
                Value::Enum(e) => e.to_string(),
            };
            (key.clone(), value_str)
        })
        .collect();

    let processed = ProcessedClass {
        name: class.name.clone(),
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
    if !class.nested_classes.is_empty() {
        trace!("Processing {} nested classes for {}", class.nested_classes.len(), class.name);
        for nested in &class.nested_classes {
            process_class(nested, file_path, classes);
        }
    }
}