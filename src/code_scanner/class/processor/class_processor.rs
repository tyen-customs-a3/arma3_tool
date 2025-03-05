use std::path::{Path, PathBuf};

use anyhow::{Result, Context};
use log::{warn, info, debug};
use rayon::prelude::*;

use crate::code_scanner::class::types::{ProcessedClass, ClassScanStats, ClassScanOptions, ClassScanResult};
use crate::code_scanner::class::scanner::ClassScanner;
use super::property_processor::PropertyProcessor;

/// Class processor responsible for processing parsed classes
#[derive(Debug)]
pub struct ClassProcessor {
    /// Configuration options for processing
    options: ClassScanOptions,
    
    /// Output directory for logs and temporary files
    output_dir: PathBuf,
    
    /// Property processor for processing class properties
    property_processor: PropertyProcessor,
}

impl ClassProcessor {
    /// Create a new class processor with the given options and output directory
    pub fn new(options: ClassScanOptions, output_dir: impl AsRef<Path>) -> Self {
        Self {
            options: options.clone(),
            output_dir: output_dir.as_ref().to_path_buf(),
            property_processor: PropertyProcessor::new(),
        }
    }
    
    /// Create a new class processor with default options
    pub fn with_defaults(output_dir: impl AsRef<Path>) -> Self {
        Self::new(ClassScanOptions::default(), output_dir)
    }
    
    /// Process a list of files and return the processed classes and statistics
    pub fn process_files(&self, files: &[PathBuf]) -> Result<ClassScanResult> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context("Failed to create output directory")?;
        
        // Create a scanner with the same options
        let scanner = ClassScanner::new(self.options.clone(), &self.output_dir);
        
        // First phase: Scan all files and collect classes
        info!("Phase 1: Scanning files for classes");
        let scanned_files = scanner.scan_files_parallel(files)?;
        
        // Get error files and timeout files from the scanner
        let error_files = ClassScanner::get_error_files();
        let timeout_files = ClassScanner::get_timeout_files();
        
        // Count empty files and error files
        let mut stats = ClassScanStats::default();
        stats.total_files = files.len();
        stats.files_with_classes = scanned_files.len();
        stats.error_files = error_files.len();
        stats.error_file_paths = error_files;
        stats.timeout_files = timeout_files.len();
        stats.timeout_file_paths = timeout_files;
        stats.empty_files = files.len() - scanned_files.len() - stats.error_files - stats.timeout_files;
        
        // Second phase: Process the collected classes
        info!("Phase 2: Processing {} files with classes", scanned_files.len());
        let (processed_classes, processing_stats) = self.process_scanned_classes(scanned_files)?;
        
        // Combine stats
        stats.total_classes = processing_stats.total_classes;
        
        info!("Processing complete:");
        info!("- Total files processed: {}", stats.total_files);
        info!("- Files containing classes: {}", stats.files_with_classes);
        info!("- Empty files: {}", stats.empty_files);
        info!("- Files with errors: {}", stats.error_files);
        info!("- Files that timed out: {}", stats.timeout_files);
        info!("- Total classes found: {}", stats.total_classes);
        
        Ok(ClassScanResult {
            classes: processed_classes,
            stats,
        })
    }
    
    /// Process pre-scanned classes from files
    pub fn process_scanned_classes(
        &self,
        scanned_files: Vec<(PathBuf, Vec<cpp_parser::Class>)>,
    ) -> Result<(Vec<ProcessedClass>, ClassScanStats)> {
        let mut final_processed_classes = Vec::with_capacity(scanned_files.len() * 5); // Pre-allocate with estimated capacity
        let mut final_stats = ClassScanStats::default();
        final_stats.total_files = scanned_files.len();
        
        // Configure thread pool if specified
        if let Some(threads) = self.options.parallel_threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build_global()
                .unwrap_or_else(|e| warn!("Failed to configure thread pool: {}", e));
        }
        
        info!("Processing {} files with classes", scanned_files.len());
        
        // Process all scanned files in parallel
        let processed_results: Vec<Vec<ProcessedClass>> = scanned_files.par_iter()
            .map(|(file_path, classes)| {
                // Process classes for this file
                let mut file_processed_classes = Vec::with_capacity(classes.len() * 2); // Pre-allocate with estimated capacity
                self.process_top_level_classes(classes, file_path, &mut file_processed_classes);
                file_processed_classes
            })
            .collect();
        
        // Combine all processed classes
        for processed in processed_results {
            final_processed_classes.extend(processed);
        }
        
        final_stats.files_with_classes = scanned_files.len();
        final_stats.total_classes = final_processed_classes.len();
        
        // Validate processing results
        if final_stats.total_classes == 0 {
            warn!("No classes were found in any of the files!");
        }
        
        info!("Processing stats:");
        info!("- Files containing classes: {}", final_stats.files_with_classes);
        info!("- Total classes found: {}", final_stats.total_classes);
        
        Ok((final_processed_classes, final_stats))
    }
    
    /// Process top-level classes from a file
    fn process_top_level_classes(&self, classes: &[cpp_parser::Class], file_path: &Path, processed_classes: &mut Vec<ProcessedClass>) {
        for class in classes {
            debug!("Processing top-level class: {:?} from {}", class.name, file_path.display());
            self.process_class(class, file_path, processed_classes);
        }
    }
    
    /// Process a single class and its nested classes
    fn process_class(&self, class: &cpp_parser::Class, file_path: &Path, classes: &mut Vec<ProcessedClass>) {
        // Process properties
        let properties = self.property_processor.process_properties(&class.properties);
        
        // Create processed class
        if let Some(name) = &class.name {
            let processed_class = ProcessedClass {
                name: name.clone(),
                parent: class.parent.clone(),
                properties,
                file_path: Some(file_path.to_path_buf()),
            };
            
            classes.push(processed_class);
        } else {
            debug!("Skipping unnamed class in {}", file_path.display());
        }
        
        // Process nested classes
        for (_, value) in &class.properties {
            if let cpp_parser::Value::Class(nested_class) = value {
                if let Some(name) = &nested_class.name {
                    debug!("Processing nested class: {} in {}", name, file_path.display());
                    self.process_class(nested_class, file_path, classes);
                }
            }
        }
    }
    
    /// Scan a directory for class files and process them
    pub fn scan_directory(&self, input_dir: impl AsRef<Path>) -> Result<ClassScanResult> {
        let input_dir = input_dir.as_ref();
        info!("Scanning class files in {}", input_dir.display());
        
        // Create a scanner with the same options
        let scanner = ClassScanner::new(self.options.clone(), &self.output_dir);
        
        // Collect all .cpp and .hpp files in the input directory
        let files = scanner.collect_files(input_dir)?;
        
        info!("Found {} class files to process", files.len());
        
        // Process the files
        self.process_files(&files)
    }
    
    /// Scan specific class files and process them
    pub fn scan_specific_files(&self, file_paths: &[PathBuf]) -> Result<ClassScanResult> {
        info!("Scanning {} specific class files", file_paths.len());
        
        // Process the files
        self.process_files(file_paths)
    }
} 