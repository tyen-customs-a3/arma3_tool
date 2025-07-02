use std::path::{Path, PathBuf};
use std::fs;
use log::{debug, error, trace, info};
use std::time::Instant;
use std::collections::HashSet;

/// Scan a directory for files with specified extensions
pub fn scan_directory(dir_path: &Path, extensions: &[String], max_files: Option<usize>) -> Vec<PathBuf> {
    let start_time = Instant::now();
    let mut files = Vec::new();
    let mut visited_dirs = HashSet::new();
    
    if !dir_path.exists() {
        error!("Directory does not exist: {}", dir_path.display());
        return files;
    }
    
    // Normalize extensions once
    let normalized_extensions: Vec<String> = extensions.iter()
        .map(|e| e.trim().to_lowercase())
        .collect();
    
    debug!("Starting scan in directory: {} for extensions: {:?}", dir_path.display(), normalized_extensions);
    
    scan_directory_recursive(dir_path, &mut files, max_files, &normalized_extensions, &mut visited_dirs);
    
    let duration = start_time.elapsed();
    info!("Scan completed in {:.2}s, found {} files", duration.as_secs_f32(), files.len());
    
    files
}

fn scan_directory_recursive(
    current_dir: &Path,
    files: &mut Vec<PathBuf>,
    max_files: Option<usize>,
    extensions: &[String],
    visited_dirs: &mut HashSet<PathBuf>
) {
    // Check max files limit
    if let Some(max) = max_files {
        if files.len() >= max {
            return;
        }
    }
    
    // Prevent infinite recursion from symlinks
    if !visited_dirs.insert(current_dir.to_path_buf()) {
        debug!("Skipping already visited directory: {}", current_dir.display());
        return;
    }

    match fs::read_dir(current_dir) {
        Ok(entries) => {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                
                if path.is_dir() {
                    scan_directory_recursive(&path, files, max_files, extensions, visited_dirs);
                } else if path.is_file() {
                    // Check extension before adding
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if extensions.iter().any(|e| e == &ext_str) {
                            trace!("Found matching file: {}", path.display());
                            files.push(path);
                            
                            // Check max files limit after each addition
                            if let Some(max) = max_files {
                                if files.len() >= max {
                                    info!("Reached maximum file limit of {}", max);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to read directory {}: {}", current_dir.display(), e);
        }
    }
}

/// Filter files by size
pub fn filter_by_size(files: &[PathBuf], min_size: u64, max_size: Option<u64>) -> Vec<PathBuf> {
    let start_time = Instant::now();
    
    let filtered: Vec<PathBuf> = files.iter()
        .filter(|path| {
            if let Ok(metadata) = fs::metadata(path) {
                let size = metadata.len();
                let matches = size >= min_size && max_size.map_or(true, |max| size <= max);
                if matches {
                    trace!("File {} matches size criteria (size: {} bytes)", path.display(), size);
                }
                matches
            } else {
                error!("Failed to get metadata for file: {}", path.display());
                false
            }
        })
        .cloned()
        .collect();
    
    let duration = start_time.elapsed();
    debug!("Size filtering completed in {:.2}s, {} files matched", 
           duration.as_secs_f32(), filtered.len());
    
    filtered
}

/// Filter files by pattern (simple substring matching)
pub fn filter_by_pattern(files: &[PathBuf], pattern: &str) -> Vec<PathBuf> {
    let start_time = Instant::now();
    
    let filtered: Vec<PathBuf> = files.iter()
        .filter(|path| {
            let path_str = path.to_string_lossy();
            let matches = path_str.contains(pattern);
            if matches {
                trace!("File {} matches pattern: {}", path.display(), pattern);
            }
            matches
        })
        .cloned()
        .collect();
    
    let duration = start_time.elapsed();
    debug!("Pattern filtering completed in {:.2}s, {} files matched",
           duration.as_secs_f32(), filtered.len());
    
    filtered
}