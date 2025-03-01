use std::path::Path;
use std::fs;
use anyhow::{Result, Context};
use sha2::{Sha256, Digest};
use walkdir::WalkDir;

pub fn calculate_file_hash(path: &Path) -> Result<String> {
    // Get file metadata
    let metadata = fs::metadata(path)
        .with_context(|| format!("Failed to get metadata for {}", path.display()))?;
    
    // Create a string combining path, size and mtime
    let file_size = metadata.len();
    let mtime = metadata.modified()
        .with_context(|| format!("Failed to get mtime for {}", path.display()))?
        .duration_since(std::time::UNIX_EPOCH)
        .with_context(|| "Failed to convert mtime")?
        .as_secs();

    // Combine path, size and mtime into a string
    let hash_input = format!("{}:{}:{}",
        path.to_string_lossy(),
        file_size,
        mtime
    );

    // Calculate hash of the combined string
    let mut hasher = Sha256::new();
    hasher.update(hash_input.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn count_matching_files(dir: &Path, extensions: &str) -> Result<usize> {
    let ext_list: Vec<&str> = extensions.split(',').collect();
    let mut count = 0;

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        
        if let Some(ext) = entry.path().extension() {
            if ext_list.iter().any(|&valid_ext| 
                ext.to_string_lossy().to_lowercase() == valid_ext.to_lowercase()
            ) {
                count += 1;
            }
        }
    }

    Ok(count)
}