use std::path::{Path, PathBuf};
use anyhow::Result;
use log::{debug, trace};
use walkdir::WalkDir;

/// File collector for finding class files
#[derive(Debug, Default)]
pub struct FileCollector {
    /// Valid file extensions to collect
    valid_extensions: Vec<String>,
}

impl FileCollector {
    /// Create a new file collector with default settings
    pub fn new() -> Self {
        Self {
            valid_extensions: vec!["cpp".to_string(), "hpp".to_string()],
        }
    }
    
    /// Create a new file collector with custom file extensions
    pub fn with_extensions(extensions: Vec<String>) -> Self {
        Self {
            valid_extensions: extensions,
        }
    }
    
    /// Collect all files with valid extensions from the input directory
    pub fn collect_files(&self, input_dir: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
        let input_dir = input_dir.as_ref();
        debug!("Collecting files from directory: {}", input_dir.display());
        
        // Use a more efficient approach with pre-allocation
        let mut files = Vec::with_capacity(1000); // Pre-allocate with a reasonable capacity
        
        for entry in WalkDir::new(input_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension() {
                if let Some(ext_str) = ext.to_str() {
                    if self.valid_extensions.iter().any(|valid_ext| ext_str.eq_ignore_ascii_case(valid_ext)) {
                        trace!("Found file: {}", entry.path().display());
                        files.push(entry.path().to_owned());
                    }
                }
            }
        }
        
        debug!("Collected {} files for processing", files.len());
        Ok(files)
    }
    
    /// Add a valid file extension
    pub fn add_extension(&mut self, extension: &str) {
        if !self.valid_extensions.contains(&extension.to_string()) {
            self.valid_extensions.push(extension.to_string());
        }
    }
    
    /// Get the list of valid file extensions
    pub fn extensions(&self) -> &[String] {
        &self.valid_extensions
    }
} 