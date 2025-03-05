use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use rayon::prelude::*;

/// Progress tracker for displaying progress during scanning
#[derive(Debug, Default)]
pub struct ProgressTracker {}

impl ProgressTracker {
    /// Create a new progress tracker
    pub fn new() -> Self {
        Self {}
    }
    
    /// Track progress of parallel operations
    pub fn track_parallel_progress<T, F, R>(&self, items: &[T], operation: F) -> Vec<R>
    where
        T: Sync + Send + Clone,
        F: Fn(&T) -> Option<R> + Sync + Send,
        R: Send,
    {
        // Set up progress bar
        let multi_progress = MultiProgress::new();
        let progress_bar = if items.len() > 10 {
            let pb = multi_progress.add(ProgressBar::new(items.len() as u64));
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} items ({eta})")
                .unwrap()
                .progress_chars("#>-"));
            Some(Arc::new(pb))
        } else {
            None
        };
        
        // Create an atomic counter for tracking progress
        let processed_count = Arc::new(AtomicUsize::new(0));
        
        // Process items in parallel
        let results: Vec<_> = items.par_iter()
            .filter_map(|item| {
                // Update progress
                let current_count = processed_count.fetch_add(1, Ordering::SeqCst) + 1;
                if let Some(pb) = &progress_bar {
                    pb.set_position(current_count as u64);
                    
                    // Update message occasionally to avoid too many updates
                    if current_count % 10 == 0 || current_count == 1 || current_count == items.len() {
                        // Simple progress message
                        pb.set_message(format!("Processing item {}/{}", current_count, items.len()));
                    }
                }
                
                // Perform the operation
                operation(item)
            })
            .collect();
        
        // Finish progress bar
        if let Some(pb) = progress_bar {
            pb.finish_with_message("Processing complete");
        }
        
        results
    }
    
    /// Track progress of parallel operations with paths
    pub fn track_path_progress<F, R>(&self, paths: &[PathBuf], operation: F) -> Vec<R>
    where
        F: Fn(&PathBuf) -> Option<R> + Sync + Send,
        R: Send,
    {
        // Set up progress bar
        let multi_progress = MultiProgress::new();
        let progress_bar = if paths.len() > 10 {
            let pb = multi_progress.add(ProgressBar::new(paths.len() as u64));
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
                .unwrap()
                .progress_chars("#>-"));
            Some(Arc::new(pb))
        } else {
            None
        };
        
        // Create an atomic counter for tracking progress
        let processed_count = Arc::new(AtomicUsize::new(0));
        
        // Process items in parallel
        let results: Vec<_> = paths.par_iter()
            .filter_map(|path| {
                // Update progress
                let current_count = processed_count.fetch_add(1, Ordering::SeqCst) + 1;
                if let Some(pb) = &progress_bar {
                    pb.set_position(current_count as u64);
                    
                    // Update message occasionally to avoid too many updates
                    if current_count % 10 == 0 || current_count == 1 || current_count == paths.len() {
                        if let Some(file_name) = path.file_name() {
                            pb.set_message(format!("Processing: {}", file_name.to_string_lossy()));
                        }
                    }
                }
                
                // Perform the operation
                operation(path)
            })
            .collect();
        
        // Finish progress bar
        if let Some(pb) = progress_bar {
            pb.finish_with_message("Processing complete");
        }
        
        results
    }
} 