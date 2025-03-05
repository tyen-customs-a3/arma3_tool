use std::path::{Path, PathBuf};

use anyhow::{Result, Context};
use log::{warn, info, debug};
use rayon::prelude::*;

use crate::code_scanner::class::types::{ProcessedClass, ClassScanStats, ClassScanOptions, ClassScanResult};
use crate::code_scanner::class::scanner::ClassScanner;
use super::property_processor::PropertyProcessor;

/// Class processor responsible for collecting parsed classes
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
    
    /// Process a list of files and return the collected classes and statistics
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
        
        // Second phase: Collect the classes without processing
        info!("Phase 2: Collecting classes from {} files", scanned_files.len());
        let (collected_classes, collection_stats) = self.collect_scanned_classes(scanned_files)?;
        
        // Combine stats
        stats.total_classes = collection_stats.total_classes;
        
        info!("Collection complete:");
        info!("- Total files processed: {}", stats.total_files);
        info!("- Files containing classes: {}", stats.files_with_classes);
        info!("- Empty files: {}", stats.empty_files);
        info!("- Files with errors: {}", stats.error_files);
        info!("- Files that timed out: {}", stats.timeout_files);
        info!("- Total classes found: {}", stats.total_classes);
        
        Ok(ClassScanResult {
            classes: collected_classes,
            stats,
        })
    }
    
    /// Collect pre-scanned classes from files without processing
    pub fn collect_scanned_classes(
        &self,
        scanned_files: Vec<(PathBuf, Vec<cpp_parser::Class>)>,
    ) -> Result<(Vec<ProcessedClass>, ClassScanStats)> {
        let mut final_collected_classes = Vec::with_capacity(scanned_files.len() * 5); // Pre-allocate with estimated capacity
        let mut final_stats = ClassScanStats::default();
        final_stats.total_files = scanned_files.len();
        
        // Configure thread pool if specified
        if let Some(threads) = self.options.parallel_threads {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build_global()
                .unwrap_or_else(|e| warn!("Failed to configure thread pool: {}", e));
        }
        
        info!("Collecting classes from {} files", scanned_files.len());
        
        // Collect all classes from scanned files in parallel
        let collected_results: Vec<Vec<ProcessedClass>> = scanned_files.par_iter()
            .map(|(file_path, classes)| {
                // Collect classes for this file
                let mut file_collected_classes = Vec::with_capacity(classes.len()); // Pre-allocate with estimated capacity
                self.collect_classes(classes, file_path, &mut file_collected_classes);
                file_collected_classes
            })
            .collect();
        
        // Combine all collected classes
        for collected in collected_results {
            final_collected_classes.extend(collected);
        }
        
        final_stats.files_with_classes = scanned_files.len();
        final_stats.total_classes = final_collected_classes.len();
        
        // Validate collection results
        if final_stats.total_classes == 0 {
            warn!("No classes were found in any of the files!");
        }
        
        info!("Collection stats:");
        info!("- Files containing classes: {}", final_stats.files_with_classes);
        info!("- Total classes found: {}", final_stats.total_classes);
        
        Ok((final_collected_classes, final_stats))
    }
    
    /// Collect classes from a file without processing
    fn collect_classes(&self, classes: &[cpp_parser::Class], file_path: &Path, collected_classes: &mut Vec<ProcessedClass>) {
        for class in classes {
            debug!("Collecting class: {:?} from {}", class.name, file_path.display());
            
            // Only collect top-level classes with names
            if let Some(name) = &class.name {
                // Store raw properties without processing
                let properties = self.property_processor.collect_properties(&class.properties);
                
                // Create processed class without evaluating hierarchy
                let processed_class = ProcessedClass {
                    name: name.clone(),
                    parent: class.parent.clone(), // Just store the parent name without evaluating
                    properties,
                    file_path: Some(file_path.to_path_buf()),
                };
                
                collected_classes.push(processed_class);
            } else {
                debug!("Skipping unnamed class in {}", file_path.display());
            }
            
            // Collect nested classes as separate top-level entries
            for (_, value) in &class.properties {
                if let cpp_parser::Value::Class(nested_class) = value {
                    if let Some(nested_name) = &nested_class.name {
                        debug!("Collecting nested class: {} in {}", nested_name, file_path.display());
                        
                        // Store raw properties without processing
                        let properties = self.property_processor.collect_properties(&nested_class.properties);
                        
                        // Create processed class for nested class without evaluating hierarchy
                        let processed_class = ProcessedClass {
                            name: nested_name.clone(),
                            parent: nested_class.parent.clone(), // Just store the parent name without evaluating
                            properties,
                            file_path: Some(file_path.to_path_buf()),
                        };
                        
                        collected_classes.push(processed_class);
                    }
                }
            }
        }
    }
    
    /// Scan a directory for class files and collect them
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
    
    /// Scan specific class files and collect them
    pub fn scan_specific_files(&self, file_paths: &[PathBuf]) -> Result<ClassScanResult> {
        info!("Scanning {} specific class files", file_paths.len());
        
        // Process the files
        self.process_files(file_paths)
    }
} 