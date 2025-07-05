//! Trait definitions for PBO operations

use std::path::Path;
use std::io::Read;
use async_trait::async_trait;

use super::types::{PboFileInfo, PboProperties, PboValidation};
use super::error::PboOperationResult;

/// Main trait for PBO operations
/// 
/// This trait provides a unified interface for different PBO backend implementations,
/// allowing for easy switching between different PBO processing libraries while
/// maintaining API compatibility.
/// 
/// # Example
/// 
/// ```rust
/// use pbo_tools::ops::{PboOperations, HemttPboOperations};
/// use std::path::Path;
/// 
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let pbo_ops = HemttPboOperations::new();
/// let pbo_path = Path::new("example.pbo");
/// 
/// // List contents
/// let files = pbo_ops.list_contents(pbo_path).await?;
/// println!("Found {} files", files.len());
/// 
/// // Get properties
/// let properties = pbo_ops.get_properties(pbo_path).await?;
/// println!("PBO prefix: {:?}", properties.prefix);
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait PboOperations: Send + Sync {
    /// List all files contained in a PBO
    /// 
    /// Returns a vector of file information for all files in the PBO,
    /// including metadata such as size, timestamp, and compression info.
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// 
    /// # Returns
    /// 
    /// A vector of `PboFileInfo` structs containing file metadata
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - The PBO file is corrupted or invalid
    /// - I/O errors occur during reading
    async fn list_contents(&self, pbo_path: &Path) -> PboOperationResult<Vec<PboFileInfo>>;

    /// Extract a specific file from a PBO to the specified output path
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// * `file_path` - Path of the file within the PBO (using forward slashes)
    /// * `output_path` - Destination path for the extracted file
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - The specified file does not exist in the PBO
    /// - The output path cannot be written to
    /// - I/O errors occur during extraction
    async fn extract_file(
        &self,
        pbo_path: &Path,
        file_path: &str,
        output_path: &Path,
    ) -> PboOperationResult<()>;

    /// Extract all files from a PBO to the specified output directory
    /// 
    /// Creates the directory structure as needed to match the PBO's internal
    /// file organization.
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// * `output_dir` - Directory where files should be extracted
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - The output directory cannot be created or written to
    /// - I/O errors occur during extraction
    async fn extract_all(&self, pbo_path: &Path, output_dir: &Path) -> PboOperationResult<()>;

    /// Extract files matching a filter pattern from a PBO
    /// 
    /// The filter supports glob-style patterns:
    /// - `*` matches any sequence of characters
    /// - `?` matches any single character
    /// - `**` matches any sequence of directories
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// * `filter` - Glob pattern to match files
    /// * `output_dir` - Directory where matching files should be extracted
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// # use pbo_tools::ops::PboOperations;
    /// # use std::path::Path;
    /// # async fn example(pbo_ops: impl PboOperations) -> Result<(), Box<dyn std::error::Error>> {
    /// // Extract all .cpp files
    /// pbo_ops.extract_filtered(Path::new("mod.pbo"), "**/*.cpp", Path::new("output/")).await?;
    /// 
    /// // Extract all files in config directory
    /// pbo_ops.extract_filtered(Path::new("mod.pbo"), "config/**", Path::new("output/")).await?;
    /// # Ok(())
    /// # }
    /// ```
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - The filter pattern is invalid
    /// - The output directory cannot be created or written to
    /// - I/O errors occur during extraction
    async fn extract_filtered(
        &self,
        pbo_path: &Path,
        filter: &str,
        output_dir: &Path,
    ) -> PboOperationResult<()>;

    /// Get properties and metadata from a PBO
    /// 
    /// Returns comprehensive information about the PBO including version,
    /// author, prefix, custom properties, and file statistics.
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// 
    /// # Returns
    /// 
    /// A `PboProperties` struct containing PBO metadata
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - The PBO file is corrupted or invalid
    /// - I/O errors occur during reading
    async fn get_properties(&self, pbo_path: &Path) -> PboOperationResult<PboProperties>;

    /// Validate a PBO file for integrity and correctness
    /// 
    /// Performs comprehensive validation including:
    /// - File structure validation
    /// - Checksum verification
    /// - File sorting validation
    /// - Header integrity checks
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// 
    /// # Returns
    /// 
    /// A `PboValidation` struct containing validation results
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - Critical I/O errors occur during validation
    async fn validate_pbo(&self, pbo_path: &Path) -> PboOperationResult<PboValidation>;

    /// Read a file from a PBO into memory
    /// 
    /// Returns the file contents as a byte vector. Use this method when you
    /// need to process the file contents directly without writing to disk.
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// * `file_path` - Path of the file within the PBO
    /// 
    /// # Returns
    /// 
    /// A vector containing the file's bytes
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - The specified file does not exist in the PBO
    /// - I/O errors occur during reading
    async fn read_file(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<Vec<u8>>;

    /// Get a reader for a file within a PBO
    /// 
    /// Returns a reader that can be used to stream the file contents.
    /// This is more memory-efficient than `read_file` for large files.
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// * `file_path` - Path of the file within the PBO
    /// 
    /// # Returns
    /// 
    /// A boxed reader for the file contents
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - The specified file does not exist in the PBO
    /// - I/O errors occur during reading
    async fn get_file_reader(
        &self,
        pbo_path: &Path,
        file_path: &str,
    ) -> PboOperationResult<Box<dyn Read + Send>>;

    /// Check if a file exists in a PBO
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// * `file_path` - Path of the file within the PBO
    /// 
    /// # Returns
    /// 
    /// `true` if the file exists, `false` otherwise
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - I/O errors occur during reading
    async fn file_exists(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<bool>;

    /// Get information about a specific file in a PBO
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// * `file_path` - Path of the file within the PBO
    /// 
    /// # Returns
    /// 
    /// File information if the file exists, `None` otherwise
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - The PBO file cannot be found or opened
    /// - I/O errors occur during reading
    async fn get_file_info(
        &self,
        pbo_path: &Path,
        file_path: &str,
    ) -> PboOperationResult<Option<PboFileInfo>>;
}

/// Extension trait providing convenience methods for PBO operations
/// 
/// This trait provides default implementations for common operations
/// that can be built on top of the core `PboOperations` trait.
#[async_trait]
pub trait PboOperationsExt: PboOperations {
    /// Extract multiple specific files from a PBO
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// * `file_paths` - Slice of file paths within the PBO to extract
    /// * `output_dir` - Directory where files should be extracted
    /// 
    /// # Returns
    /// 
    /// Number of files successfully extracted
    /// 
    /// # Errors
    /// 
    /// Returns an error if critical issues occur, but continues processing
    /// other files if possible. Use the return value to check how many
    /// files were successfully extracted.
    async fn extract_files(
        &self,
        pbo_path: &Path,
        file_paths: &[&str],
        output_dir: &Path,
    ) -> PboOperationResult<usize> {
        let mut extracted = 0;
        
        for file_path in file_paths {
            let output_path = output_dir.join(file_path);
            
            // Create parent directories if needed
            if let Some(parent) = output_path.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    log::warn!("Failed to create directory {}: {}", parent.display(), e);
                    continue;
                }
            }
            
            match self.extract_file(pbo_path, file_path, &output_path).await {
                Ok(()) => extracted += 1,
                Err(e) => log::warn!("Failed to extract '{}': {}", file_path, e),
            }
        }
        
        Ok(extracted)
    }

    /// Validate multiple PBO files
    /// 
    /// # Arguments
    /// 
    /// * `pbo_paths` - Slice of paths to PBO files
    /// 
    /// # Returns
    /// 
    /// Vector of validation results, one for each PBO file
    /// 
    /// # Errors
    /// 
    /// Returns an error only if a critical system error occurs.
    /// Individual PBO validation failures are included in the results.
    async fn validate_multiple(&self, pbo_paths: &[&Path]) -> PboOperationResult<Vec<PboValidation>> {
        let mut results = Vec::with_capacity(pbo_paths.len());
        
        for path in pbo_paths {
            match self.validate_pbo(path).await {
                Ok(validation) => results.push(validation),
                Err(e) => {
                    // Create a failed validation result
                    let mut validation = PboValidation::new((*path).to_path_buf());
                    validation.add_critical_error(
                        super::types::ValidationIssueType::Corruption,
                        format!("Failed to validate PBO: {}", e),
                        None,
                    );
                    results.push(validation);
                }
            }
        }
        
        Ok(results)
    }

    /// Get a summary of files by extension
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// 
    /// # Returns
    /// 
    /// A map of file extensions to counts
    async fn get_file_summary(&self, pbo_path: &Path) -> PboOperationResult<std::collections::HashMap<String, usize>> {
        let files = self.list_contents(pbo_path).await?;
        let mut summary = std::collections::HashMap::new();
        
        for file in files {
            let extension = file.extension().unwrap_or("(no extension)");
            *summary.entry(extension.to_string()).or_insert(0) += 1;
        }
        
        Ok(summary)
    }

    /// Check if a PBO contains any files matching a pattern
    /// 
    /// # Arguments
    /// 
    /// * `pbo_path` - Path to the PBO file
    /// * `pattern` - Glob pattern to match
    /// 
    /// # Returns
    /// 
    /// `true` if any files match the pattern
    async fn contains_files_matching(&self, pbo_path: &Path, pattern: &str) -> PboOperationResult<bool> {
        let files = self.list_contents(pbo_path).await?;
        
        // Simple glob matching - in a real implementation, you'd want a proper glob library
        let pattern_regex = pattern
            .replace("**", ".*")
            .replace("*", "[^/]*")
            .replace("?", ".");
        
        if let Ok(regex) = regex::Regex::new(&pattern_regex) {
            Ok(files.iter().any(|file| regex.is_match(&file.file_path)))
        } else {
            // Fallback to simple string matching
            Ok(files.iter().any(|file| file.file_path.contains(pattern)))
        }
    }
}

// Blanket implementation of PboOperationsExt for all types that implement PboOperations
impl<T: PboOperations> PboOperationsExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use super::super::types::*;
    use super::super::error::*;

    // Mock implementation for testing
    struct MockPboOperations;

    #[async_trait]
    impl PboOperations for MockPboOperations {
        async fn list_contents(&self, _pbo_path: &Path) -> PboOperationResult<Vec<PboFileInfo>> {
            Ok(vec![
                PboFileInfo::new(
                    "config.cpp".to_string(),
                    1024,
                    1024,
                    1640995200,
                    1024,
                    0,
                    "text/plain".to_string(),
                ),
                PboFileInfo::new(
                    "data/texture.paa".to_string(),
                    2048,
                    1024,
                    1640995200,
                    2048,
                    0,
                    "image/paa".to_string(),
                ),
            ])
        }

        async fn extract_file(&self, _pbo_path: &Path, _file_path: &str, _output_path: &Path) -> PboOperationResult<()> {
            Ok(())
        }

        async fn extract_all(&self, _pbo_path: &Path, _output_dir: &Path) -> PboOperationResult<()> {
            Ok(())
        }

        async fn extract_filtered(&self, _pbo_path: &Path, _filter: &str, _output_dir: &Path) -> PboOperationResult<()> {
            Ok(())
        }

        async fn get_properties(&self, _pbo_path: &Path) -> PboOperationResult<PboProperties> {
            let mut props = PboProperties::new();
            props.version = Some("1.0.0".to_string());
            props.file_count = 2;
            Ok(props)
        }

        async fn validate_pbo(&self, pbo_path: &Path) -> PboOperationResult<PboValidation> {
            Ok(PboValidation::new(pbo_path.to_path_buf()))
        }

        async fn read_file(&self, _pbo_path: &Path, _file_path: &str) -> PboOperationResult<Vec<u8>> {
            Ok(b"test content".to_vec())
        }

        async fn get_file_reader(&self, _pbo_path: &Path, _file_path: &str) -> PboOperationResult<Box<dyn Read + Send>> {
            Ok(Box::new(std::io::Cursor::new(b"test content".to_vec())))
        }

        async fn file_exists(&self, _pbo_path: &Path, file_path: &str) -> PboOperationResult<bool> {
            Ok(file_path == "config.cpp" || file_path == "data/texture.paa")
        }

        async fn get_file_info(&self, _pbo_path: &Path, file_path: &str) -> PboOperationResult<Option<PboFileInfo>> {
            if file_path == "config.cpp" {
                Ok(Some(PboFileInfo::new(
                    "config.cpp".to_string(),
                    1024,
                    1024,
                    1640995200,
                    1024,
                    0,
                    "text/plain".to_string(),
                )))
            } else {
                Ok(None)
            }
        }
    }

    #[tokio::test]
    async fn test_mock_pbo_operations() {
        let ops = MockPboOperations;
        let path = Path::new("test.pbo");

        // Test list_contents
        let files = ops.list_contents(path).await.unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].file_path, "config.cpp");

        // Test file_exists
        assert!(ops.file_exists(path, "config.cpp").await.unwrap());
        assert!(!ops.file_exists(path, "nonexistent.txt").await.unwrap());

        // Test get_properties
        let props = ops.get_properties(path).await.unwrap();
        assert_eq!(props.version, Some("1.0.0".to_string()));
        assert_eq!(props.file_count, 2);
    }

    #[tokio::test]
    async fn test_extension_methods() {
        let ops = MockPboOperations;
        let path = Path::new("test.pbo");

        // Test get_file_summary
        let summary = ops.get_file_summary(path).await.unwrap();
        assert_eq!(summary.get("cpp"), Some(&1));
        assert_eq!(summary.get("paa"), Some(&1));
    }
}