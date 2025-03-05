use std::path::PathBuf;

/// Statistics for class processing
#[derive(Debug, Default, Clone)]
pub struct ProcessingStats {
    /// Total number of files processed
    pub total_files: usize,
    
    /// Total number of classes found
    pub total_classes: usize,
    
    /// Number of empty files encountered
    pub empty_files: usize,
    
    /// Number of files containing classes
    pub files_with_classes: usize,
    
    /// Number of files that failed to parse
    pub error_files: usize,
    
    /// Paths to files that failed to parse
    pub error_file_paths: Vec<PathBuf>,
    
    /// Number of files that timed out during parsing
    pub timeout_files: usize,
    
    /// Paths to files that timed out during parsing
    pub timeout_file_paths: Vec<PathBuf>,
}

impl ProcessingStats {
    /// Create a new processing stats instance
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Merge another stats instance into this one
    pub fn merge(&mut self, other: &Self) {
        self.total_files += other.total_files;
        self.total_classes += other.total_classes;
        self.empty_files += other.empty_files;
        self.files_with_classes += other.files_with_classes;
        self.error_files += other.error_files;
        self.error_file_paths.extend(other.error_file_paths.clone());
        self.timeout_files += other.timeout_files;
        self.timeout_file_paths.extend(other.timeout_file_paths.clone());
    }
    
    /// Calculate the number of files that were skipped (empty + error + timeout)
    pub fn skipped_files(&self) -> usize {
        self.empty_files + self.error_files + self.timeout_files
    }
    
    /// Calculate the percentage of files that were successfully processed
    pub fn success_rate(&self) -> f64 {
        if self.total_files == 0 {
            return 0.0;
        }
        
        let successful = self.total_files - self.skipped_files();
        (successful as f64 / self.total_files as f64) * 100.0
    }
    
    /// Calculate the average number of classes per file
    pub fn avg_classes_per_file(&self) -> f64 {
        if self.files_with_classes == 0 {
            return 0.0;
        }
        
        self.total_classes as f64 / self.files_with_classes as f64
    }
} 