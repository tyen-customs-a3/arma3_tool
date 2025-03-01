use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use cpp_parser::{Class, Value};
use log::{debug, warn, info};
use serde::Serialize;
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
}

pub fn process_classes(files: &[PathBuf]) -> Result<(Vec<ProcessedClass>, ProcessingStats)> {
    let mut processed_classes = Vec::new();
    let mut stats = ProcessingStats::default();
    stats.total_files = files.len();

    for file in files {
        let base_dir = file.parent().unwrap_or(Path::new(""));
        let classes = parse_single_file(file, base_dir)
            .with_context(|| format!("Failed to process file: {}", file.display()))?;
        
        if classes.is_empty() {
            stats.empty_files += 1;
        } else {
            stats.files_with_classes += 1;
            process_top_level_classes(&classes, file, &mut processed_classes);
        }
    }

    stats.total_classes = processed_classes.len();
    
    // Validate processing results
    if stats.total_classes == 0 {
        warn!("No classes were found in any of the files!");
    }
    info!("Processing stats: {:?}", stats);

    Ok((processed_classes, stats))
}

fn process_top_level_classes(classes: &[Class], file_path: &Path, processed_classes: &mut Vec<ProcessedClass>) {
    for class in classes {
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
    
    classes.push(processed);

    // Process nested classes
    for nested in &class.nested_classes {
        process_class(nested, file_path, classes);
    }
}