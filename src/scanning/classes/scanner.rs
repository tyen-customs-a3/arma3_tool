use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use cpp_parser::{Class, parse_cpp};
use log::{debug, warn};

pub fn collect_files(input_dir: &Path) -> Result<Vec<PathBuf>> {
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
                        files.push(entry.path().to_owned());
                    }
                }
            }
        }
    }
    Ok(files)
}

pub fn parse_single_file(
    file: &Path,
    _base_dir: &Path,
) -> Result<Vec<Class>> {
    debug!("Processing file: {}", file.display());
    
    // Read the file content
    let content = match fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to read file {}: {}", file.display(), e));
        }
    };
    
    if content.trim().is_empty() {
        warn!("Empty file found: {}", file.display());
        return Ok(Vec::new());
    }
    
    // Parse the content
    match parse_cpp(&content) {
        Ok(classes) => {
            if classes.is_empty() {
                warn!("No classes found in file: {}", file.display());
            } else {
                debug!("Found {} classes in {}", classes.len(), file.display());
            }
            Ok(classes)
        }
        Err(e) => {
            Err(anyhow::anyhow!("Failed to parse file {}: {}", file.display(), e))
        }
    }
}