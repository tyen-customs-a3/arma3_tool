use std::path::Path;
use std::fs;
use anyhow::{Result, Context};

/// Ensure a directory exists, creating it if necessary
pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .context(format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_ensure_dir_exists() {
        let temp_dir = tempdir().unwrap();
        let test_path = temp_dir.path().join("subdir").join("nested");
        
        assert!(!test_path.exists());
        ensure_dir_exists(&test_path).unwrap();
        assert!(test_path.exists());
    }
}