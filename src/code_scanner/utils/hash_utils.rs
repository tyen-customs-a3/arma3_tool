use std::path::Path;
use anyhow::Result;
use sha2::{Sha256, Digest};
use log::trace;

use super::file_utils;

/// Calculate SHA-256 hash of a string
pub fn hash_string(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Calculate SHA-256 hash of a file
pub fn hash_file(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    trace!("Calculating hash for file: {}", path.display());
    
    let content = file_utils::read_file_to_string(path)?;
    Ok(hash_string(&content))
}

/// Calculate SHA-256 hash of multiple files
pub fn hash_files(paths: &[impl AsRef<Path>]) -> Result<String> {
    let mut combined_content = String::new();
    
    for path in paths {
        let content = file_utils::read_file_to_string(path)?;
        combined_content.push_str(&content);
    }
    
    Ok(hash_string(&combined_content))
} 