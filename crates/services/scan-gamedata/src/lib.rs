//! Game Data Scanner Library
//!
//! This library provides functionality for scanning and parsing game data files in parallel.
//! It can process multiple files simultaneously and extract class definitions and their properties.
//!
//! # Example
//! ```no_run
//! use gamedata_scanner::{Scanner, ScannerConfig};
//! use std::path::Path;
//!
//! // Create a scanner for a specific project root (e.g., current directory)
//! let project_root = Path::new("."); // Or any other path
//! let scanner = Scanner::new(project_root, ScannerConfig::default())
//!     .expect("Failed to create scanner"); // Scanner::new now returns Result
//!
//! // Scan a directory (e.g., the project root itself)
//! match scanner.scan_directory(".") { // Path is relative to project_root
//!     Ok(result) => {
//!         println!("Processed {} files", result.total_files);
//!         println!("Successful: {}", result.successful_files);
//!         println!("Failed: {}", result.failed_files);
//!         
//!         // Process results
//!         for (path, scan_result) in result.results {
//!             println!("File: {}", path.display());
//!             println!("Classes found: {}", scan_result.classes.len());
//!         }
//!     },
//!     Err(e) => eprintln!("Error scanning directory: {}", e),
//! }
//! ```

use std::path::{Path, PathBuf};
use std::sync::Arc;

// Re-export types from arma3_types and arma3_parser_common
pub use arma3_types::{Class as GameClass, Value as PropertyValue};

// Type alias for backward compatibility
pub type ClassProperty = (String, PropertyValue);

// Define a dyn-compatible trait for file parsing
pub trait FileParser: Send + Sync {
    /// Parse a single file and return all classes found
    fn parse_file(&self, file_path: &Path) -> Vec<GameClass>;

    /// Get the name of the parser
    fn name(&self) -> &str;
}
// Do not re-export get_parser directly as its signature changed and is internal to Scanner::new
pub use scanner::{FileScanResult, ScannerConfig, ScannerResult}; // Added FileScanResult

pub mod parser_factory;
pub mod scanner;

/// Main scanner interface for processing game data files
#[derive(Clone)] // Debug might be tricky with Arc<dyn FileParser>
pub struct Scanner {
    project_root_dir: PathBuf,
    parser: Arc<dyn FileParser>,
    config: ScannerConfig,
}

impl Scanner {
    /// Creates a new scanner for a given project root directory and configuration.
    ///
    /// # Arguments
    /// * `project_root_dir` - The root directory of the project to be scanned.
    /// * `config` - The scanner configuration.
    pub fn new<P: AsRef<Path>>(
        project_root_dir: P,
        config: ScannerConfig,
    ) -> Result<Self, String> {
        let root_path = project_root_dir.as_ref().to_path_buf();
        if !root_path.is_dir() {
            return Err(format!(
                "Project root directory does not exist or is not a directory: {}",
                root_path.display()
            ));
        }
        let parser =
            parser_factory::get_parser(&root_path, config.use_advanced_parser)?;
        Ok(Self {
            project_root_dir: root_path,
            parser,
            config,
        })
    }

    /// Creates a new scanner with default configuration for the current directory.
    pub fn default() -> Result<Self, String> {
        Self::new(".", ScannerConfig::default())
    }

    /// Scans a single file.
    ///
    /// # Arguments
    /// * `file_path` - Path to the file to scan. Can be absolute or relative to the project root.
    ///
    /// # Returns
    /// Returns a `Result` containing either a `FileScanResult` or an error string.
    pub fn scan_file<P: AsRef<Path>>(&self, file_path: P) -> Result<FileScanResult, String> {
        let path_to_scan = file_path.as_ref();
        let absolute_path = if path_to_scan.is_absolute() {
            path_to_scan.to_path_buf()
        } else {
            self.project_root_dir.join(path_to_scan)
        };

        if !absolute_path.exists() {
            return Err(format!("File not found: {}", absolute_path.display()));
        }
        if !absolute_path.is_file() {
            return Err(format!("Path is not a file: {}", absolute_path.display()));
        }
        
        // The process_single_file function will be created in scanner.rs
        // It will take the parser, path, timeout and diagnostic_mode.
        scanner::process_single_file(
            self.parser.clone(), // Pass the initialized parser
            &absolute_path,
            self.config.timeout,
            self.config.diagnostic_mode,
        )
        .map_err(|e| e.to_string())
    }


    /// Scans a directory recursively for game data files.
    /// The provided `scan_path` is interpreted relative to the `project_root_dir`
    /// with which the `Scanner` was initialized.
    ///
    /// # Arguments
    ///
    /// * `scan_path` - Path to the directory to scan, relative to the project root.
    ///                 If an absolute path is given, it must be within the project root.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either a `ScannerResult` with the scan results
    /// or an IO error if the scanning process failed.
    pub fn scan_directory<P: AsRef<Path>>(&self, scan_path: P) -> std::io::Result<ScannerResult> {
        let relative_scan_path = scan_path.as_ref();
        let absolute_scan_dir = if relative_scan_path.is_absolute() {
            if !relative_scan_path.starts_with(&self.project_root_dir) {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "Absolute scan path {} is not within project root {}",
                        relative_scan_path.display(),
                        self.project_root_dir.display()
                    ),
                ));
            }
            relative_scan_path.to_path_buf()
        } else {
            self.project_root_dir.join(relative_scan_path)
        };

        if !absolute_scan_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Scan directory not found or is not a directory: {}",
                    absolute_scan_dir.display()
                ),
            ));
        }
        
        // Pass the initialized parser to the underlying scan_directory function
        scanner::scan_directory_with_parser(
            &absolute_scan_dir, // Scan from this absolute path
            self.parser.clone(),
            self.config.clone(),
            &self.project_root_dir, // Provide project root for context if needed by parser
        )
    }

    /// Gets a reference to the scanner's configuration.
    pub fn config(&self) -> &ScannerConfig {
        &self.config
    }

    /// Gets a mutable reference to the scanner's configuration.
    pub fn config_mut(&mut self) -> &mut ScannerConfig {
        &mut self.config
    }

    /// Gets the project root directory.
    pub fn project_root(&self) -> &Path {
        &self.project_root_dir
    }
}

impl Default for Scanner {
    fn default() -> Self {
        // This might panic if parser init fails. Consider making Scanner::default() return Result.
        // For now, assume current dir is valid for default advanced parser.
        Scanner::new(".", ScannerConfig::default())
            .expect("Failed to create default scanner for current directory")
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File}; // Added fs
    use std::io::{self, Write};
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> io::Result<PathBuf> {
        let path = dir.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
        Ok(path)
    }

    #[test]
    fn test_scanner_new_and_scan_file() -> Result<(), String> {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let project_root = temp_dir.path();

        create_test_file(
            project_root,
            "test1.hpp",
            r#"class TestClass1 { displayName = "Test1"; };"#,
        )
        .expect("Failed to create test file");

        let scanner = Scanner::new(project_root, ScannerConfig::default())?;
        
        // Test scanning a file relative to project root
        let result1 = scanner.scan_file("test1.hpp")?;
        assert_eq!(result1.file_path, project_root.join("test1.hpp"));
        assert_eq!(result1.classes.len(), 1);
        assert_eq!(result1.classes[0].name, "TestClass1");

        // Test scanning a file with an absolute path
        let abs_path = project_root.join("test1.hpp");
        let result2 = scanner.scan_file(&abs_path)?;
        assert_eq!(result2.file_path, abs_path);
        assert_eq!(result2.classes.len(), 1);

        // Test scanning a non-existent file
        assert!(scanner.scan_file("nonexistent.hpp").is_err());
        
        Ok(())
    }

    #[test]
    fn test_scanner_scan_directory() -> Result<(), String> {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let project_root = temp_dir.path();

        create_test_file(
            project_root,
            "sub/test2.cpp", // Create in a subdirectory
            r#"class TestClass2 { value = 123; };"#,
        )
        .expect("Failed to create test file");
        create_test_file(
            project_root,
            "test1.hpp",
            r#"class TestClass1 { displayName = "Test1"; };"#,
        )
        .expect("Failed to create test file");


        let mut config = ScannerConfig::default();
        config.show_progress = false; // Disable progress for tests
        let scanner = Scanner::new(project_root, config)?;

        // Scan a subdirectory relative to project root
        let result_subdir = scanner.scan_directory("sub").map_err(|e| e.to_string())?;
        assert_eq!(result_subdir.total_files, 1);
        assert_eq!(result_subdir.successful_files, 1);
        assert!(result_subdir.results.contains_key(&project_root.join("sub/test2.cpp")));

        // Scan the entire project root (by passing ".")
        let result_root = scanner.scan_directory(".").map_err(|e| e.to_string())?;
        assert_eq!(result_root.total_files, 2); // test1.hpp and sub/test2.cpp
        assert_eq!(result_root.successful_files, 2);
        assert!(result_root.results.contains_key(&project_root.join("test1.hpp")));
        assert!(result_root.results.contains_key(&project_root.join("sub/test2.cpp")));
        
        // Test scanning a non-existent directory
        assert!(scanner.scan_directory("nonexistent_subdir").is_err());

        Ok(())
    }
    
    #[test]
    fn test_scanner_default_and_config_access() -> Result<(), String> {
        // Scanner::default() now returns a Result
        let mut scanner = Scanner::default()?;
        assert_eq!(scanner.project_root(), Path::new("."));
        assert_eq!(scanner.config().timeout, ScannerConfig::default().timeout);

        scanner.config_mut().timeout = 99;
        assert_eq!(scanner.config().timeout, 99);
        Ok(())
    }

    
}
