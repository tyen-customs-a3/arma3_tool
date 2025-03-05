use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use log::{debug, trace};

/// Create a directory if it doesn't exist
pub fn ensure_dir_exists(dir: impl AsRef<Path>) -> Result<()> {
    let dir = dir.as_ref();
    if !dir.exists() {
        debug!("Creating directory: {}", dir.display());
        fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create directory {}", dir.display()))?;
    }
    Ok(())
}

/// Check if a file has a specific extension
pub fn has_extension(path: impl AsRef<Path>, extension: &str) -> bool {
    let path = path.as_ref();
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return ext_str.eq_ignore_ascii_case(extension);
        }
    }
    false
}

/// Check if a file has one of the specified extensions
pub fn has_any_extension(path: impl AsRef<Path>, extensions: &[&str]) -> bool {
    extensions.iter().any(|ext| has_extension(path.as_ref(), ext))
}

/// Get all files in a directory with specific extensions
pub fn get_files_with_extensions(dir: impl AsRef<Path>, extensions: &[&str]) -> Result<Vec<PathBuf>> {
    let dir = dir.as_ref();
    debug!("Collecting files from directory: {}", dir.display());
    
    let mut files = Vec::new();
    
    for entry in walkdir::WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if has_any_extension(path, extensions) {
            trace!("Found file: {}", path.display());
            files.push(path.to_owned());
        }
    }
    
    debug!("Collected {} files with extensions {:?}", files.len(), extensions);
    Ok(files)
}

/// Read a file to string with better error handling
pub fn read_file_to_string(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read file {}", path.display()))
}

/// Write a string to a file with better error handling
pub fn write_string_to_file(path: impl AsRef<Path>, content: &str) -> Result<()> {
    let path = path.as_ref();
    
    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        ensure_dir_exists(parent)?;
    }
    
    fs::write(path, content)
        .with_context(|| format!("Failed to write file {}", path.display()))
} 