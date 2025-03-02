use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use cpp_parser::{Class, parse_cpp};
use log::{debug, warn, error, trace};

pub fn collect_files(input_dir: &Path) -> Result<Vec<PathBuf>> {
    debug!("Collecting files from directory: {}", input_dir.display());
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(input_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if let Some(ext_str) = ext.to_str() {
                    if ext_str.eq_ignore_ascii_case("cpp") {
                        trace!("Found file: {}", entry.path().display());
                        files.push(entry.path().to_owned());
                    }
                }
            }
        }
    }
    debug!("Collected {} files for processing", files.len());
    Ok(files)
}

pub fn parse_single_file(
    file: &Path,
    _base_dir: &Path,
    verbose_errors: bool,
) -> Result<Vec<Class>> {
    debug!("Processing file: {}", file.display());
    
    // Read the file content
    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to read file {}: {}", file.display(), e);
            return Err(anyhow::anyhow!("Failed to read file {}: {}", file.display(), e));
        }
    };
    
    if content.trim().is_empty() {
        warn!("Empty file found: {}", file.display());
        return Ok(Vec::new());
    }
    
    trace!("File size: {} bytes, starting parse", content.len());
    
    // Parse the content
    match parse_cpp(&content) {
        Ok(classes) => {
            if classes.is_empty() {
                debug!("No classes found in file: {}", file.display());
            } else {
                debug!("Found {} classes in {}", classes.len(), file.display());
                trace!("Class names: {}", classes.iter()
                    .map(|c| c.name.clone())
                    .collect::<Vec<_>>()
                    .join(", "));
            }
            Ok(classes)
        }
        Err(e) => {
            // Log detailed error information
            error!("Parse error in file {}: {}", file.display(), e);
            
            // Try to extract a snippet of the problematic content
            if verbose_errors {
                if let Some(error_location) = e.to_string().find("line") {
                    let error_info = &e.to_string()[error_location..];
                    if let Some(line_num) = parse_line_number(error_info) {
                        let context = extract_context(&content, line_num, 2);
                        error!("Error context:\n{}", context);
                    }
                }
            }
            
            Err(anyhow::anyhow!("Failed to parse file {}: {}", file.display(), e))
        }
    }
}

// Helper function to extract line number from error message
fn parse_line_number(error_info: &str) -> Option<usize> {
    let parts: Vec<&str> = error_info.split(',').collect();
    if parts.is_empty() {
        return None;
    }
    
    let line_part = parts[0].trim();
    if line_part.starts_with("line") {
        line_part[4..].trim().parse::<usize>().ok()
    } else {
        None
    }
}

// Helper function to extract context around the error
fn extract_context(content: &str, line_num: usize, context_lines: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let start = line_num.saturating_sub(context_lines);
    let end = std::cmp::min(line_num + context_lines, lines.len());
    
    let mut result = String::new();
    for i in start..end {
        let line_marker = if i + 1 == line_num { ">" } else { " " };
        if i < lines.len() {
            result.push_str(&format!("{} {:4}: {}\n", line_marker, i + 1, lines[i]));
        }
    }
    result
}