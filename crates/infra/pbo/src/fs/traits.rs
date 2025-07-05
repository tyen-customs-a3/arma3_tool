use std::path::Path;
use std::fs::{create_dir_all, remove_dir_all, remove_file};
use log::debug;
use crate::ops::{PboOperationResult, PboOperationError};

pub trait FileOperation {
    fn validate_path(&self) -> PboOperationResult<()>;
    fn ensure_parent_exists(&self) -> PboOperationResult<()>;
    fn remove_if_exists(&self) -> PboOperationResult<()>;
    fn is_safe_path(&self) -> bool;
    fn ensure_directory(&self) -> PboOperationResult<()>;
    fn validate_filename(&self) -> PboOperationResult<()>;
    fn validate_path_safety(&self) -> PboOperationResult<()>;
}

impl FileOperation for Path {
    fn validate_path(&self) -> PboOperationResult<()> {
        self.validate_path_safety()
    }

    fn ensure_parent_exists(&self) -> PboOperationResult<()> {
        self.validate_path_safety()?;
        
        if let Some(parent) = self.parent() {
            if !parent.exists() {
                create_dir_all(parent)
                    .map_err(|e| PboOperationError::io_error("creating parent directory", e))?;
            }
        }
        Ok(())
    }

    fn remove_if_exists(&self) -> PboOperationResult<()> {
        if self.exists() {
            if self.is_dir() {
                remove_dir_all(self).map_err(|e| 
                    PboOperationError::io_error("removing directory", e)
                )?;
            } else {
                remove_file(self).map_err(|e| 
                    PboOperationError::io_error("removing file", e)
                )?;
            }
        }
        Ok(())
    }

    fn is_safe_path(&self) -> bool {
        let path_str = self.to_str().unwrap_or("");
        debug!("Checking path safety for: {}", path_str);

        // Check for common directory traversal patterns
        if path_str.contains("..") || path_str.contains("//") {
            debug!("Rejecting path with directory traversal");
            return false;
        }

        // Check for suspicious characters in path
        let suspicious_chars = ['<', '>', '|', '*', '?', '"', '`', '$', '&', '{', '}', ';', '#', '='];
        if path_str.chars().any(|c| suspicious_chars.contains(&c)) {
            debug!("Rejecting path with suspicious characters");
            return false;
        }

        // Handle Windows paths
        #[cfg(windows)]
        {
            // Allow only drive letter colons (C:\)
            let colon_positions: Vec<_> = path_str.match_indices(':').map(|(i, _)| i).collect();
            match colon_positions.len() {
                0 => (),  // No colons is fine
                1 if colon_positions[0] == 1 => (), // Single colon at position 1 is fine (drive letter)
                _ => {
                    debug!("Rejecting path with invalid colon placement");
                    return false;
                }
            }

            // Reject UNC paths and other potentially unsafe Windows paths
            if path_str.starts_with("\\\\") || path_str.contains("://") {
                debug!("Rejecting UNC or URL-like path");
                return false;
            }
        }

        #[cfg(not(windows))]
        if path_str.contains(':') {
            debug!("Rejecting path with colon on non-Windows system");
            return false;
        }

        // Check for null bytes and control characters
        if path_str.chars().any(|c| c.is_control()) {
            debug!("Rejecting path with control characters");
            return false;
        }

        debug!("Path passed safety check");
        true
    }

    fn ensure_directory(&self) -> PboOperationResult<()> {
        if !self.exists() {
            create_dir_all(self)
                .map_err(|e| PboOperationError::io_error("creating directory", e))?;
        } else if !self.is_dir() {
            return Err(PboOperationError::invalid_format(
                "Path exists but is not a directory".to_string()
            ));
        }
        Ok(())
    }

    fn validate_filename(&self) -> PboOperationResult<()> {
        if let Some(filename) = self.file_name() {
            let filename_str = filename.to_string_lossy();
            // Check for reserved filenames on Windows
            #[cfg(windows)]
            {
                let reserved = [
                    "CON", "PRN", "AUX", "NUL",
                    "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
                    "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
                ];
                
                if reserved.iter().any(|&r| filename_str.eq_ignore_ascii_case(r)) {
                    return Err(PboOperationError::invalid_format(
                        format!("Invalid filename: {}", self.to_string_lossy())
                    ));
                }
            }

            // Check for dots and spaces
            if filename_str.starts_with('.') || filename_str.ends_with('.') || 
               filename_str.starts_with(' ') || filename_str.ends_with(' ') {
                return Err(PboOperationError::invalid_format(
                    format!("Invalid filename: {}", self.to_string_lossy())
                ));
            }
        }
        Ok(())
    }

    fn validate_path_safety(&self) -> PboOperationResult<()> {
        // Convert path to string for validation
        let path_str = self.to_string_lossy();

        // Check for invalid characters
        let invalid_chars = ['<', '>', '|', '"', '*', '?'];
        if path_str.contains(&invalid_chars[..]) {
            return Err(PboOperationError::invalid_path(
                format!("Path contains invalid characters: {}", path_str)
            ));
        }

        // Check for reasonable path length
        if path_str.len() > 260 {
            return Err(PboOperationError::invalid_path(
                format!("Path exceeds maximum length (260): {}", path_str)
            ));
        }

        // Check for parent directory traversal
        if path_str.contains("..") {
            return Err(PboOperationError::invalid_path(
                format!("Path contains parent directory traversal: {}", path_str)
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_safe_path_validation() {
        assert!(!Path::new("../test.txt").is_safe_path());
        assert!(!Path::new("test<>.txt").is_safe_path());
        assert!(Path::new("normal.txt").is_safe_path());
        assert!(Path::new("path/to/file.txt").is_safe_path());
        assert!(!Path::new("test:file.txt").is_safe_path());
        assert!(!Path::new("//test.txt").is_safe_path());
    }

    #[test]
    fn test_directory_operations() {
        let temp = tempdir().unwrap();
        let test_dir = temp.path().join("test_dir");
        
        // Test directory creation
        test_dir.ensure_directory().unwrap();
        assert!(test_dir.is_dir());
        
        // Test file conflict
        let test_file = temp.path().join("test_file");
        File::create(&test_file).unwrap();
        assert!(test_file.ensure_directory().is_err());
    }

    #[test]
    fn test_filename_validation() {
        let temp = tempdir().unwrap();
        
        // Test invalid filenames
        let invalid_names = [
            ".hidden",
            " space.txt",
            "test.",
            "test ",
        ];
        
        for name in invalid_names.iter() {
            let path = temp.path().join(name);
            assert!(path.validate_filename().is_err());
        }
        
        // Test valid filename
        let valid_path = temp.path().join("normal.txt");
        assert!(valid_path.validate_filename().is_ok());
    }

    #[test]
    fn test_remove_if_exists() {
        let temp = tempdir().unwrap();
        let file_path = temp.path().join("test.txt");
        let dir_path = temp.path().join("testdir");

        // Test file removal
        File::create(&file_path).unwrap();
        assert!(file_path.exists());
        file_path.remove_if_exists().unwrap();
        assert!(!file_path.exists());

        // Test directory removal
        create_dir_all(&dir_path).unwrap();
        File::create(dir_path.join("file.txt")).unwrap();
        assert!(dir_path.exists());
        dir_path.remove_if_exists().unwrap();
        assert!(!dir_path.exists());
    }

    #[test]
    fn test_ensure_parent_exists() {
        let temp = tempdir().unwrap();
        let deep_path = temp.path().join("a/b/c/file.txt");
        deep_path.ensure_parent_exists().unwrap();
        assert!(deep_path.parent().unwrap().exists());
    }
}