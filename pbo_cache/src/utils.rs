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

/// Extract the base name of a PBO without extension
pub fn pbo_base_name(path: &Path) -> String {
    path.file_stem()
        .map_or_else(
            || "unknown".to_string(),
            |stem| stem.to_string_lossy().to_string()
        )
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
    
    #[test]
    fn test_pbo_base_name() {
        assert_eq!(pbo_base_name(Path::new("test.pbo")), "test");
        assert_eq!(pbo_base_name(Path::new("/path/to/addon.pbo")), "addon");
        assert_eq!(pbo_base_name(Path::new("C:\\path\\mission.pbo")), "mission");
    }
}