use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use cpp_parser::{Class, parse_cpp};
use log::{debug, warn, error, trace, info};
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicBool};
use std::time::{Duration, Instant};
use std::sync::Mutex;
use std::thread;
use std::collections::HashSet;

/// Collects all .cpp and .hpp files from the input directory
pub fn collect_files(input_dir: &Path) -> Result<Vec<PathBuf>> {
    debug!("Collecting files from directory: {}", input_dir.display());
    
    // Use a more efficient approach with pre-allocation
    let mut files = Vec::with_capacity(1000); // Pre-allocate with a reasonable capacity
    
    // Define valid extensions once
    let valid_extensions = ["cpp", "hpp"];
    
    for entry in walkdir::WalkDir::new(input_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if let Some(ext) = entry.path().extension() {
            if let Some(ext_str) = ext.to_str() {
                if valid_extensions.iter().any(|&valid_ext| ext_str.eq_ignore_ascii_case(valid_ext)) {
                    trace!("Found file: {}", entry.path().display());
                    files.push(entry.path().to_owned());
                }
            }
        }
    }
    
    debug!("Collected {} files for processing", files.len());
    Ok(files)
}

/// Parses a single file and returns the classes found in it
pub fn parse_single_file(
    file: &Path,
    base_dir: &Path,
    verbose_errors: bool,
    output_dir: &Path,
) -> Result<Vec<Class>> {
    debug!("Processing file: {}", file.display());
    
    // Read the file content
    let content = fs::read_to_string(file)
        .with_context(|| format!("Failed to read file {}", file.display()))?;
    
    if content.trim().is_empty() {
        warn!("Empty file found: {}", file.display());
        return Ok(Vec::new());
    }
    
    trace!("File size: {} bytes, starting parse", content.len());
    
    // Parse the content
    match parse_cpp(&content) {
        Ok(classes) => {
            if classes.is_empty() {
                debug!("No classes found in file: {}", file.display());
            } else {
                debug!("Found {} classes in {}", classes.len(), file.display());
                
                // Only generate class names list for trace level logging
                if log::log_enabled!(log::Level::Trace) {
                    let class_names = classes.iter()
                        .map(|c| c.name.clone().unwrap_or_else(|| "UnnamedClass".to_string()))
                        .collect::<Vec<_>>()
                        .join(", ");
                    trace!("Class names: {}", class_names);
                }
            }
            Ok(classes)
        }
        Err(e) => {
            // Log detailed error information
            let err = anyhow::anyhow!("Failed to parse file {}: {}", file.display(), e);
            error!("{}", err);
            
            // Try to extract a snippet of the problematic content and log to file
            if verbose_errors {
                log_parse_error(file, &e, &content, output_dir);
            }
            
            Err(err)
        }
    }
}

/// Helper function to log parse errors to a file
fn log_parse_error(file: &Path, error: &impl std::fmt::Display, content: &str, output_dir: &Path) {
    if let Some(error_location) = error.to_string().find("line") {
        let error_info = &error.to_string()[error_location..];
        
        // Create a thread-safe error log file
        let error_file_name = format!(
            "parse_error_{}.log",
            file.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .replace(|c: char| !c.is_alphanumeric(), "_")
        );
        
        let error_file_path = output_dir.join(error_file_name);
        
        // Use a more robust approach to write the error file
        match std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&error_file_path)
        {
            Ok(mut file_handle) => {
                use std::io::Write;
                let _ = writeln!(file_handle, "Error parsing file: {}", file.display());
                let _ = writeln!(file_handle, "Error details: {}", error);
                let _ = writeln!(file_handle, "Location: {}", error_info);
                let _ = writeln!(file_handle, "\nFile content:\n{}", content);
                debug!("Wrote detailed error information to {}", error_file_path.display());
            }
            Err(write_err) => {
                error!("Failed to write error log file: {}", write_err);
            }
        }
    }
}

/// Parses a single file with a timeout and returns the classes found in it
pub fn parse_single_file_with_timeout(
    file: &Path,
    base_dir: &Path,
    verbose_errors: bool,
    output_dir: &Path,
    timeout_duration: Duration,
) -> Result<(Vec<Class>, bool)> { // Returns (classes, timed_out)
    debug!("Processing file with timeout: {}", file.display());
    
    // Create a channel to communicate between threads
    let (sender, receiver) = std::sync::mpsc::channel();
    let file_path = file.to_owned();
    let base_dir_path = base_dir.to_owned();
    let output_dir_path = output_dir.to_owned();
    
    // Spawn a thread to parse the file
    let parse_thread = thread::spawn(move || {
        let result = parse_single_file(&file_path, &base_dir_path, verbose_errors, &output_dir_path);
        let _ = sender.send(result);
    });
    
    // Wait for the result with a timeout
    match receiver.recv_timeout(timeout_duration) {
        Ok(result) => {
            // Thread completed within timeout
            match result {
                Ok(classes) => Ok((classes, false)), // Not timed out
                Err(e) => Err(e),
            }
        },
        Err(_) => {
            // Timeout occurred
            warn!("Timeout occurred while processing file: {}", file.display());
            
            // Try to abort the thread (this is not guaranteed to work)
            parse_thread.thread().unpark();
            
            // Return an empty result with timeout flag
            Ok((Vec::new(), true))
        }
    }
}

/// Helper function to log timeout information to a file
fn log_timeout_files(timeout_files: &[PathBuf], output_dir: &Path) {
    let timeout_log_path = output_dir.join("timeout_files.log");
    
    match std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&timeout_log_path)
    {
        Ok(mut file_handle) => {
            use std::io::Write;
            let _ = writeln!(file_handle, "Files that timed out after 10 seconds:");
            let _ = writeln!(file_handle, "Total timed out files: {}", timeout_files.len());
            let _ = writeln!(file_handle, "\nList of files that timed out:");
            
            for file in timeout_files {
                let _ = writeln!(file_handle, "- {}", file.display());
            }
            
            info!("Wrote timeout information to {}", timeout_log_path.display());
        }
        Err(write_err) => {
            error!("Failed to write timeout log file: {}", write_err);
        }
    }
}

/// Scans multiple files in parallel and returns a vector of (file path, classes) pairs
/// This allows separating the scanning phase from the processing phase
pub fn scan_files_parallel(
    files: &[PathBuf],
    verbose_errors: bool,
    output_dir: &Path,
) -> Result<Vec<(PathBuf, Vec<Class>)>> {
    debug!("Starting parallel scan of {} files", files.len());
    
    // Create a progress bar for better user feedback
    let multi_progress = MultiProgress::new();
    let progress_bar = if files.len() > 10 {
        let pb = multi_progress.add(ProgressBar::new(files.len() as u64));
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} files ({eta})")
            .unwrap()
            .progress_chars("#>-"));
        Some(Arc::new(pb))
    } else {
        None
    };
    
    // Create an atomic counter for tracking overall progress
    let processed_count = Arc::new(AtomicUsize::new(0));
    
    // Set the timeout duration (10 seconds per file)
    let timeout_duration = Duration::from_secs(10);
    
    // Pre-allocate results vector
    let results = Arc::new(Mutex::new(Vec::with_capacity(files.len())));
    
    // Track error files
    let error_files = Arc::new(Mutex::new(Vec::new()));
    
    // Track timeout files
    let timeout_files = Arc::new(Mutex::new(Vec::new()));
    
    // Process each file in its own thread
    files.par_iter().for_each(|file| {
        // Update progress bar with atomic counter
        let current_count = processed_count.fetch_add(1, Ordering::SeqCst) + 1;
        
        let pb_clone = progress_bar.as_ref().map(Arc::clone);
        if let Some(pb) = &pb_clone {
            pb.set_position(current_count as u64);
            if current_count % 10 == 0 || current_count == 1 {
                pb.set_message(format!("Scanning: {}", file.file_name().unwrap_or_default().to_string_lossy()));
            }
        }
        
        let base_dir = file.parent().unwrap_or(Path::new(""));
        
        // Parse the file with timeout
        match parse_single_file_with_timeout(file, base_dir, verbose_errors, output_dir, timeout_duration) {
            Ok((classes, timed_out)) => {
                if timed_out {
                    // File processing timed out
                    if let Ok(mut timeout_files_guard) = timeout_files.lock() {
                        timeout_files_guard.push(file.clone());
                    }
                } else if !classes.is_empty() {
                    // File processed successfully with classes
                    if let Ok(mut results_guard) = results.lock() {
                        results_guard.push((file.clone(), classes));
                    }
                }
            },
            Err(_) => {
                // Track files with errors
                if let Ok(mut error_files_guard) = error_files.lock() {
                    error_files_guard.push(file.clone());
                }
            },
        }
    });
    
    // Extract results from the mutex
    let final_results = match results.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => Vec::new(),
    };
    
    // Extract error files from the mutex
    let final_error_files = match error_files.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => Vec::new(),
    };
    
    // Extract timeout files from the mutex
    let final_timeout_files = match timeout_files.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => Vec::new(),
    };
    
    // Log timeout information if any files timed out
    if !final_timeout_files.is_empty() {
        info!("{} files timed out after 10 seconds", final_timeout_files.len());
        log_timeout_files(&final_timeout_files, output_dir);
    }
    
    // Finish the progress bar if it exists
    if let Some(pb) = &progress_bar {
        pb.finish_with_message("Scanning complete");
    }
    
    let error_count = final_error_files.len();
    let timeout_count = final_timeout_files.len();
    debug!("Completed parallel scan:");
    debug!("- Found classes in {} files", final_results.len());
    debug!("- Errors in {} files", error_count);
    debug!("- Timeouts in {} files", timeout_count);
    
    // Store error files in thread-local storage for later retrieval
    thread_local! {
        static ERROR_FILES: std::cell::RefCell<Vec<PathBuf>> = std::cell::RefCell::new(Vec::new());
    }
    
    ERROR_FILES.with(|cell| {
        let mut cell_borrow = cell.borrow_mut();
        cell_borrow.clear();
        cell_borrow.extend(final_error_files);
    });
    
    // Store timeout files in thread-local storage for later retrieval
    thread_local! {
        static TIMEOUT_FILES: std::cell::RefCell<Vec<PathBuf>> = std::cell::RefCell::new(Vec::new());
    }
    
    TIMEOUT_FILES.with(|cell| {
        let mut cell_borrow = cell.borrow_mut();
        cell_borrow.clear();
        cell_borrow.extend(final_timeout_files);
    });
    
    Ok(final_results)
}

/// Retrieves the list of files that had parsing errors during the last scan
pub fn get_error_files() -> Vec<PathBuf> {
    thread_local! {
        static ERROR_FILES: std::cell::RefCell<Vec<PathBuf>> = std::cell::RefCell::new(Vec::new());
    }
    
    ERROR_FILES.with(|cell| cell.borrow().clone())
}

/// Retrieves the list of files that were not processed due to timeout
pub fn get_timeout_files() -> Vec<PathBuf> {
    thread_local! {
        static TIMEOUT_FILES: std::cell::RefCell<Vec<PathBuf>> = std::cell::RefCell::new(Vec::new());
    }
    
    TIMEOUT_FILES.with(|cell| cell.borrow().clone())
}