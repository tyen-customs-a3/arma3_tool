use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use log::{debug, info};
use walkdir::WalkDir;

/// Collect all mission PBO files from the input directory
pub fn collect_mission_files(input_dir: &Path) -> Result<Vec<PathBuf>> {
    debug!("Collecting mission PBO files from {}", input_dir.display());
    
    // Check if input directory exists
    if !input_dir.exists() {
        return Err(anyhow::anyhow!("Input directory does not exist: {}", input_dir.display()));
    }
    
    // Collect all PBO files
    let mut mission_files = Vec::new();
    
    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        // Skip directories
        if path.is_dir() {
            continue;
        }
        
        // Check if file has .pbo extension
        if let Some(ext) = path.extension() {
            if ext.to_string_lossy().to_lowercase() == "pbo" {
                // Check if file name contains "mission" or other mission-related keywords
                let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                
                debug!("Found mission PBO: {}", path.display());
                mission_files.push(path.to_path_buf());
            }
        }
    }
    
    info!("Found {} mission PBO files", mission_files.len());
    
    Ok(mission_files)
}
