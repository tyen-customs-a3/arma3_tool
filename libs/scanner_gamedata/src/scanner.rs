use gamedata_scanner_models::GameClass;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

/// Result of scanning a single file, containing the parsed classes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileScanResult {
    /// Path to the file that was scanned
    pub file_path: PathBuf,
    /// Classes found in the file
    pub classes: Vec<GameClass>,
    /// Time taken to parse the file in milliseconds (only set if diagnostic mode is enabled)
    pub parse_time_ms: Option<u64>,
}

/// Configuration for the scanner
#[derive(Debug, Clone)]
pub struct ScannerConfig {
    /// Maximum number of files to process (for testing)
    pub max_files: Option<usize>,
    /// Whether to show progress bar
    pub show_progress: bool,
    /// File extensions to scan (lowercase)
    pub extensions: Vec<String>,
    /// Timeout in seconds for processing each file (default: 30)
    pub timeout: u64,
    /// Enable diagnostic mode to track parsing time for each file
    pub diagnostic_mode: bool,
    /// Whether to use the advanced parser (true) or simple parser (false)
    pub use_advanced_parser: bool,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            max_files: None,
            show_progress: true,
            extensions: vec!["hpp".to_string(), "cpp".to_string(), "ext".to_string()],
            timeout: 120,
            diagnostic_mode: false,
            use_advanced_parser: false,
        }
    }
}

/// Result of the scanning process
#[derive(Debug)]
pub struct ScannerResult {
    /// Total number of files processed
    pub total_files: usize,
    /// Number of successfully processed files
    pub successful_files: usize,
    /// Number of files that failed processing
    pub failed_files: usize,
    /// Map of file paths to their scan results
    pub results: HashMap<PathBuf, FileScanResult>,
    /// Map of file paths to their error messages
    pub errors: HashMap<PathBuf, String>,
    /// Diagnostic information (only available if diagnostic mode was enabled)
    pub diagnostics: Option<DiagnosticInfo>,
}

/// Diagnostic information about the scanning process
#[derive(Debug)]
pub struct DiagnosticInfo {
    /// Total time taken for the entire scan in milliseconds
    pub total_scan_time_ms: u64,
    /// Map of file paths to their parsing times in milliseconds
    pub file_parse_times: HashMap<PathBuf, u64>,
    /// List of slowest files (sorted by parse time, descending)
    pub slowest_files: Vec<(PathBuf, u64)>,
    /// Number of files that timed out
    pub timeout_count: usize,
}

impl DiagnosticInfo {
    /// Create a new diagnostic info from scan results
    fn new(
        results: &HashMap<PathBuf, FileScanResult>,
        errors: &HashMap<PathBuf, String>,
        total_scan_time_ms: u64,
    ) -> Self {
        // Extract parse times from results
        let mut file_parse_times = HashMap::new();
        for (path, result) in results {
            if let Some(time) = result.parse_time_ms {
                file_parse_times.insert(path.clone(), time);
            }
        }

        // Count timeout errors
        let timeout_count = errors.values().filter(|e| e.contains("timed out")).count();

        // Create sorted list of slowest files
        let mut slowest_files: Vec<(PathBuf, u64)> = file_parse_times
            .iter()
            .map(|(path, &time)| (path.clone(), time))
            .collect();

        // Sort by time (descending)
        slowest_files.sort_by(|a, b| b.1.cmp(&a.1));

        // Limit to top 20 slowest files
        if slowest_files.len() > 20 {
            slowest_files.truncate(20);
        }

        Self {
            total_scan_time_ms,
            file_parse_times,
            slowest_files,
            timeout_count,
        }
    }

    /// Generate a formatted diagnostic report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str(&format!("=== Diagnostic Report ===\n"));
        report.push_str(&format!(
            "Total scan time: {:.2} seconds\n",
            self.total_scan_time_ms as f64 / 1000.0
        ));
        report.push_str(&format!(
            "Files processed: {}\n",
            self.file_parse_times.len()
        ));
        report.push_str(&format!("Files timed out: {}\n", self.timeout_count));

        if !self.file_parse_times.is_empty() {
            let total_parse_time: u64 = self.file_parse_times.values().sum();
            let avg_parse_time = total_parse_time as f64 / self.file_parse_times.len() as f64;

            report.push_str(&format!("Average parse time: {:.2} ms\n", avg_parse_time));

            if let Some((_, max_time)) = self.slowest_files.first() {
                report.push_str(&format!("Maximum parse time: {} ms\n", max_time));
            }
        }

        if !self.slowest_files.is_empty() {
            report.push_str("\nTop 20 slowest files:\n");
            for (i, (path, time)) in self.slowest_files.iter().enumerate() {
                report.push_str(&format!("{}. {} - {} ms\n", i + 1, path.display(), time));
            }
        }

        report
    }
}

/// Scans a directory recursively for game data files and processes them in parallel.
/// The `scan_dir` is the specific directory to start scanning from (can be project_root or a sub-directory).
/// The `project_root_dir` is used by the parser for context if needed (e.g. for advanced parsing).
pub(crate) fn scan_directory_with_parser(
    scan_dir: &Path, // The directory to actually scan (absolute)
    parser: Arc<dyn gamedata_scanner_models::FileParser>,
    config: ScannerConfig,
    _project_root_dir: &Path, // Keep for potential future use by parser or file collection logic
) -> io::Result<ScannerResult> {
    // Start timing the entire scan if diagnostic mode is enabled
    let scan_start_time = if config.diagnostic_mode {
        Some(Instant::now())
    } else {
        None
    };

    // Verify input directory exists
    if !scan_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory not found: {}", scan_dir.display()),
        ));
    }
    if !scan_dir.is_dir() {
         return Err(io::Error::new(
            io::ErrorKind::InvalidInput, // Changed from NotFound
            format!("Path is not a directory: {}", scan_dir.display()),
        ));
    }


    // First, collect all files to process from the scan_dir
    let mut files_to_scan = Vec::new();
    collect_files_recursive(scan_dir, &mut files_to_scan, &config)?;

    // Apply max_files limit if specified
    if let Some(max) = config.max_files {
        files_to_scan.truncate(max);
    }

    // Create shared result containers
    let results = Arc::new(Mutex::new(HashMap::new()));
    let errors = Arc::new(Mutex::new(HashMap::new()));
    let successful_count = Arc::new(Mutex::new(0usize));
    let failed_count = Arc::new(Mutex::new(0usize));

    // Create progress bar if enabled
    let progress_bar = if config.show_progress {
        let pb = ProgressBar::new(files_to_scan.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        Some(pb)
    } else {
        None
    };

    // Process files in parallel
    files_to_scan.par_iter().for_each(|file_path| {
        // Clone parser for each thread
        let thread_parser = parser.clone();
        match process_single_file( // Renamed from process_file
            thread_parser, // Pass the parser
            file_path,
            config.timeout,
            config.diagnostic_mode,
            // use_advanced_parser is no longer needed here as parser is pre-initialized
        ) {
            Ok(scan_result) => {
                results
                    .lock()
                    .unwrap()
                    .insert(file_path.clone(), scan_result);
                *successful_count.lock().unwrap() += 1;
            }
            Err(e) => {
                errors
                    .lock()
                    .unwrap()
                    .insert(file_path.clone(), e.to_string());
                *failed_count.lock().unwrap() += 1;
            }
        }

        if let Some(pb) = &progress_bar {
            pb.inc(1);
        }
    });

    // Finish progress bar
    if let Some(pb) = progress_bar {
        pb.finish_with_message("Scan complete");
    }

    // Calculate total scan time if diagnostic mode is enabled
    let diagnostics = if config.diagnostic_mode {
        scan_start_time.map(|start| {
            let total_scan_time_ms = start.elapsed().as_millis() as u64;
            // Ensure results and errors are properly accessed after Arc::try_unwrap
            let results_guard = results.lock().unwrap();
            let errors_guard = errors.lock().unwrap();
            DiagnosticInfo::new(&results_guard, &errors_guard, total_scan_time_ms)
        })
    } else {
        None
    };
    
    // Build final result
    // Ensure results and errors are properly accessed after Arc::try_unwrap
    let final_results = Arc::try_unwrap(results)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to unwrap Arc for results"))?
        .into_inner()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to unwrap Mutex for results"))?;
    let final_errors = Arc::try_unwrap(errors)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to unwrap Arc for errors"))?
        .into_inner()
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to unwrap Mutex for errors"))?;


    // Get the counts before creating the result
    let successful = *successful_count.lock().unwrap();
    let failed = *failed_count.lock().unwrap();

    Ok(ScannerResult {
        total_files: files_to_scan.len(),
        successful_files: successful,
        failed_files: failed,
        results: final_results,
        errors: final_errors,
        diagnostics,
    })
}

/// Recursively collects files to process
fn collect_files_recursive(
    dir: &Path,
    files: &mut Vec<PathBuf>,
    config: &ScannerConfig,
) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_files_recursive(&path, files, config)?;
        } else if is_target_file(&path, &config.extensions) {
            files.push(path);
        }
    }

    Ok(())
}

/// Checks if a file should be processed based on its extension
fn is_target_file(path: &Path, extensions: &[String]) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase()))
        .unwrap_or(false)
}

/// Process a single file using the provided parser.
pub(crate) fn process_single_file(
    parser: Arc<dyn gamedata_scanner_models::FileParser>, // Accept initialized parser
    file_path: &Path,
    timeout_secs: u64,
    diagnostic_mode: bool,
) -> io::Result<FileScanResult> {
    // Create a channel for the worker thread to send results back
    let (tx, rx) = mpsc::channel();

    // Clone the path for the worker thread
    let file_path_clone = file_path.to_path_buf();
    // Parser is already Arc'd, clone it for the thread
    let thread_parser = parser.clone();


    // Spawn a worker thread to process the file
    thread::spawn(move || {
        // Start timing if diagnostic mode is enabled
        let start_time = if diagnostic_mode {
            Some(Instant::now())
        } else {
            None
        };

        let result = {
            // Use the passed-in parser
            let classes = thread_parser.parse_file(&file_path_clone);
            // Calculate parsing time if diagnostic mode is enabled
            let parse_time_ms = start_time.map(|start| start.elapsed().as_millis() as u64);

            Ok(FileScanResult {
                file_path: file_path_clone.clone(),
                classes,
                parse_time_ms,
            })
        };

        // Send the result back to the main thread
        let _ = tx.send(result);
    });

    // Wait for the result with a timeout
    match rx.recv_timeout(Duration::from_secs(timeout_secs)) {
        Ok(result) => result,
        Err(RecvTimeoutError::Timeout) => Err(io::Error::new(
            io::ErrorKind::TimedOut,
            format!(
                "Processing timed out after {} seconds for file: {}",
                timeout_secs,
                file_path.display()
            ),
        )),
        Err(RecvTimeoutError::Disconnected) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Worker thread disconnected while processing file: {}",
                file_path.display()
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> io::Result<PathBuf> {
        let path = dir.join(name);
        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
        Ok(path)
    }

    #[test]
    fn test_scanner_basic() -> io::Result<()> {
        let temp_dir = TempDir::new()?;

        // Create test files
        create_test_file(
            temp_dir.path(),
            "test1.hpp",
            r#"
            class TestClass1 {
                displayName = "Test1";
            };
        "#,
        )?;

        create_test_file(
            temp_dir.path(),
            "test2.cpp",
            r#"
            class TestClass2 {
                displayName = "Test2";
            };
        "#,
        )?;

        let config = ScannerConfig {
            show_progress: false,
            timeout: 30,
            ..Default::default()
        };

        // Tests will need to be updated to use Scanner::new(project_root, config).scan_directory(...)
        // For now, this test will likely fail or need adjustment.
        // We'll use the new Scanner API for testing.
        let scanner = crate::Scanner::new(temp_dir.path(), config)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let result = scanner.scan_directory(".")?;

        assert_eq!(result.total_files, 2);
        assert!(result.successful_files > 0);

        Ok(())
    }

    #[test]
    fn test_scanner_with_max_files() -> io::Result<()> {
        let temp_dir = TempDir::new()?;

        // Create multiple test files
        for i in 0..5 {
            create_test_file(
                temp_dir.path(),
                &format!("test{}.hpp", i),
                &format!(
                    r#"
                class TestClass{} {{
                    displayName = "Test{}";
                }};
            "#,
                    i, i
                ),
            )?;
        }

        let config = ScannerConfig {
            max_files: Some(3),
            show_progress: false,
            timeout: 30,
            ..Default::default()
        };

        let scanner = crate::Scanner::new(temp_dir.path(), config)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let result = scanner.scan_directory(".")?;

        assert_eq!(result.total_files, 3);

        Ok(())
    }
}
