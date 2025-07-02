use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Instant, Duration};
use log::{debug, error, trace, warn};
use std::io::Write;
use std::thread;
use std::sync::mpsc;

use gamedata_scanner::{Scanner, ScannerConfig};
use super::error_handler::{categorize_error, extract_related_context};
use super::types::FileFailure;
use super::config::{Args, ParserType};

/// Generate a unique file path by adding a numeric suffix if the file already exists
fn get_unique_path(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }
    
    let file_stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let extension = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    
    let mut counter = 1;
    loop {
        let new_name = if extension.is_empty() {
            format!("{}_{}", file_stem, counter)
        } else {
            format!("{}_{}.{}", file_stem, counter, extension)
        };
        
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

/// Handle a failing file by copying it to the failures directory and logging diagnostics
fn handle_failing_file(
    file_path: &Path,
    failure: &FileFailure,
    output_dir: &Path,
) -> io::Result<()> {
    // Create failures directory if it doesn't exist
    let failures_dir = output_dir.join("failures");
    fs::create_dir_all(&failures_dir)?;
    
    // Create category subdirectory
    let category_dir = failures_dir.join(&failure.error_category.to_lowercase().replace(' ', "_"));
    fs::create_dir_all(&category_dir)?;
    
    // Copy the file to the category directory with a unique name
    let file_name = file_path.file_name().unwrap_or_default();
    let initial_dest_path = category_dir.join(file_name);
    let dest_path = get_unique_path(&initial_dest_path);
    fs::copy(file_path, &dest_path)?;
    
    // Create a diagnostic file next to the copied file with a matching name
    let diag_path = dest_path.with_extension("diag.txt");
    let mut diag_file = fs::File::create(&diag_path)?;
    
    // Write diagnostic information
    writeln!(diag_file, "Error Category: {}", failure.error_category)?;
    writeln!(diag_file, "Error Severity: {}", failure.error_severity)?;
    writeln!(diag_file, "Error Message: {}", failure.error_message)?;
    writeln!(diag_file, "File Size: {} bytes", failure.file_size)?;
    writeln!(diag_file, "Processing Time: {}ms", failure.parse_duration_ms)?;
    writeln!(diag_file, "Original File: {}", file_path.display())?;
    
    if let Some(line) = failure.error_line_number {
        writeln!(diag_file, "Error Line: {}", line)?;
    }
    
    if !failure.diagnostics.is_empty() {
        writeln!(diag_file, "\nDiagnostic Messages:")?;
        for diag in &failure.diagnostics {
            writeln!(diag_file, "  - {}", diag)?;
        }
    }
    
    if let Some(context) = &failure.error_context {
        writeln!(diag_file, "\nError Context:\n{}", context)?;
    }
    
    debug!("Copied failing file to: {}", dest_path.display());
    debug!("Created diagnostic file: {}", diag_path.display());
    
    Ok(())
}

/// Process a single file with timeout and return a tuple containing:
/// - A boolean indicating success (true) or failure (false)
/// - An option containing failure details (None if successful, Some for hard errors only)
/// - A vector of parser warnings (PE12 and other warnings that don't cause failures)
///
/// Phase 6 cleanup: Clear distinction between warnings (PE12, etc.) and hard errors.
/// Only true errors from parser_advanced propagate as FileFailure, not PE12 warnings.
pub fn process_file(file_path: &Path, output_dir: Option<&Path>, args: &Args) -> (bool, Option<FileFailure>, Vec<::parser_advanced::ParseWarning>) {
    let file_path_str = file_path.to_string_lossy().to_string();
    debug!("Processing file: {}", file_path_str);
    
    // Get file metadata
    let file_size = match fs::metadata(file_path) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            error!("Failed to get file size for {}: {}", file_path_str, e);
            let (success, failure) = create_io_error_failure(&e, file_path);
            return (success, failure, Vec::new());
        }
    };
    
    // Record start time for performance metrics
    let start_time = Instant::now();
    
    // Validate file size
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
    if file_size > MAX_FILE_SIZE {
        warn!("File {} is too large ({} bytes), skipping", file_path_str, file_size);
        let failure = FileFailure {
            error_message: format!("File too large: {} bytes (max: {} bytes)", file_size, MAX_FILE_SIZE),
            diagnostics: vec![],
            is_timeout: false,
            file_size,
            error_line_number: None,
            error_context: None,
            error_category: "Size Error".to_string(),
            error_severity: "Error".to_string(),
            related_files: vec![file_path_str],
            parse_duration_ms: 0,
        };
        
        if let Some(output_dir) = output_dir {
            if let Err(e) = handle_failing_file(file_path, &failure, output_dir) {
                error!("Failed to handle failing file: {}", e);
            }
        }
        
        return (false, Some(failure), Vec::new());
    }
    
    // Create channel for result communication
    let (tx, rx) = mpsc::channel();
    let file_path_thread = file_path.to_path_buf();
    let use_advanced = matches!(args.parser_type, ParserType::Advanced);
    
    // Spawn processing in a separate thread
    thread::spawn(move || {
        let result = process_with_scanner(&file_path_thread, use_advanced);
        let _ = tx.send(result); // Ignore send error as receiver might be dropped on timeout
    });
    
    // Wait for the thread with timeout
    let timeout = if args.timeout_secs > 0 {
        Duration::from_secs(args.timeout_secs)
    } else {
        Duration::from_secs(u64::MAX) // Effectively unlimited
    };
    
    let parse_result = match rx.recv_timeout(timeout) {
        Ok(result) => result,
        Err(_) => {
            // Handle timeout
            let duration = start_time.elapsed().as_millis() as u64;
            let failure = FileFailure {
                error_message: format!("Processing timeout after {}ms (limit: {}s)",
                    duration, args.timeout_secs),
                diagnostics: vec![],
                is_timeout: true,
                file_size,
                error_line_number: None,
                error_context: None,
                error_category: "Timeout Error".to_string(),
                error_severity: "Error".to_string(),
                related_files: vec![file_path_str.clone()],
                parse_duration_ms: duration,
            };
            
            if let Some(output_dir) = output_dir {
                if let Err(e) = handle_failing_file(file_path, &failure, output_dir) {
                    error!("Failed to handle failing file: {}", e);
                }
            }
            
            warn!("File {} processing timed out after {}ms (limit: {}s)",
                file_path_str, duration, args.timeout_secs);
            return (false, Some(failure), Vec::new());
        }
    };
    
    let duration = start_time.elapsed().as_millis() as u64;
    
    // Handle parser results: distinguish between warnings and hard errors
    match parse_result {
        Ok((classes, actual_parser_warnings)) => {
            // Parser succeeded, though may have warnings (including PE12)
            debug!("Successfully parsed file {} with {} classes", file_path_str, classes.len());
            trace!("Parsing took {}ms", duration);
            
            // Log parser warnings at appropriate levels
            for warning in &actual_parser_warnings {
                match warning.severity.as_str() {
                    "Error" => warn!("Parser hard failure in {}: {} - {}", file_path_str, warning.code, warning.message),
                    _ => debug!("Parser warning in {}: {} - {}", file_path_str, warning.code, warning.message),
                }
            }
            
            // Check for PE12 warnings specifically
            let pe12_warnings: Vec<_> = actual_parser_warnings.iter().filter(|w| w.code == "PE12").collect();
            if !pe12_warnings.is_empty() {
                debug!("Found {} PE12 warnings (include not found) in {}", pe12_warnings.len(), file_path_str);
                for pe12_warning in &pe12_warnings {
                    debug!("PE12: {}", pe12_warning.message);
                }
            }
            
            // File processed successfully, warnings are collected but don't cause failure
            (true, None, actual_parser_warnings)
        },
        Err(error_message) => {
            let failure = FileFailure {
                error_message: error_message.clone(),
                diagnostics: vec![],
                is_timeout: false,
                file_size,
                error_line_number: None,
                error_context: None,
                error_category: categorize_error(&error_message),
                error_severity: "Error".to_string(),
                related_files: vec![file_path_str.clone()],
                parse_duration_ms: duration,
            };
            
            if let Some(output_dir) = output_dir {
                if let Err(e) = handle_failing_file(file_path, &failure, output_dir) {
                    error!("Failed to handle failing file: {}", e);
                }
            }
            
            error!("File {} failed with hard error: {}", file_path_str, failure.error_message);
            (false, Some(failure), Vec::new())
        }
    }
}

/// Process a file using the gamedata scanner
/// Returns either (classes, warnings) on success or error string on hard failure
/// Phase 6 cleanup: PE12 warnings are included in success case, not error case
fn process_with_scanner(file_path: &Path, use_advanced: bool) -> Result<(Vec<gamedata_scanner_models::GameClass>, Vec<::parser_advanced::ParseWarning>), String> {
    if use_advanced {
        // Find the project root by looking for hemtt.toml, starting from the file's directory
        let project_root = find_project_root(file_path)
            .unwrap_or_else(|| {
                // Fallback to workspace root if no hemtt.toml found in file's hierarchy
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            });
        
        // Check if hemtt.toml exists in the determined project root
        let hemtt_toml_path = project_root.join("hemtt.toml");
        let config_path = if hemtt_toml_path.exists() {
            debug!("Found hemtt.toml at: {}", hemtt_toml_path.display());
            Some(hemtt_toml_path)
        } else {
            debug!("No hemtt.toml found at: {} - linting will be disabled", hemtt_toml_path.display());
            None
        };
        
        // Create the AdvancedProjectParser with consistent project root
        let project_parser = ::parser_advanced::AdvancedProjectParser::new(
            &project_root,
            config_path.as_deref()
        ).map_err(|e| format!("Failed to create AdvancedProjectParser: {:?}", e))?;
        
        // Validate that config was loaded successfully
        if project_parser.has_project_config() {
            debug!("hemtt.toml config loaded successfully - linting rules are active");
        } else {
            debug!("No hemtt.toml config loaded - linting rules are disabled");
        }
        
        // Calculate the relative path from project root to the file
        let relative_path = file_path.strip_prefix(&project_root)
            .map_err(|_| format!("File {} is not within project root {}",
                                file_path.display(), project_root.display()))?;
        
        // Call project_parser.parse_file(relative_path)
        match project_parser.parse_file(relative_path) {
            Ok((classes, parse_warnings_from_advanced_parser)) => {
                Ok((classes, parse_warnings_from_advanced_parser))
            },
            Err(parse_error) => {
                Err(format!("{:?}", parse_error))
            }
        }
    } else {
        // Call ::parser_simple::parse_file(file_path) and return empty warnings
        let simple_classes = ::parser_simple::parse_file(file_path);
        Ok((simple_classes, Vec::new()))
    }
}

/// Find the project root by walking up the directory tree looking for hemtt.toml
fn find_project_root(file_path: &Path) -> Option<PathBuf> {
    let mut current_dir = file_path.parent()?;
    
    loop {
        let hemtt_toml_path = current_dir.join("hemtt.toml");
        if hemtt_toml_path.exists() {
            debug!("Found hemtt.toml in directory: {}", current_dir.display());
            return Some(current_dir.to_path_buf());
        }
        
        // Move up one directory
        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            // Reached filesystem root without finding hemtt.toml
            debug!("Reached filesystem root without finding hemtt.toml");
            break;
        }
    }
    
    None
}

/// Create a failure result for IO errors
fn create_io_error_failure(error: &io::Error, file_path: &Path) -> (bool, Option<FileFailure>) {
    let file_path_str = file_path.to_string_lossy().to_string();
    (false, Some(FileFailure {
        error_message: format!("Failed to read file: {}", error),
        diagnostics: vec![],
        is_timeout: false,
        file_size: 0,
        error_line_number: None,
        error_context: None,
        error_category: "IO Error".to_string(),
        error_severity: "Error".to_string(),
        related_files: vec![file_path_str],
        parse_duration_ms: 0,
    }))
}

/// Utility function to copy a file to the output directory
pub fn copy_file_to_output(source: &Path, dest_dir: &Path) -> io::Result<()> {
    if !dest_dir.exists() {
        fs::create_dir_all(dest_dir)?;
    }
    
    let file_name = source
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid source path"))?;
    
    let initial_dest_path = dest_dir.join(file_name);
    let dest_path = get_unique_path(&initial_dest_path);
    
    debug!("Copying {} to {}", source.display(), dest_path.display());
    fs::copy(source, dest_path)?;
    
    Ok(())
} 