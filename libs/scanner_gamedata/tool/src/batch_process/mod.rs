use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use log::{debug, error, info, warn};
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicUsize, AtomicBool};
use std::sync::atomic::Ordering;

pub mod config;
mod error_handler;
mod processor;
mod reporter;
mod scanner;
mod types;

pub use config::Args;
use processor::process_file;
use reporter::generate_report;
use scanner::scan_directory;
use types::{FileFailure, Report};

/// Run the batch parser with the given arguments
pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    args.setup_logging();
    
    let start_time = Instant::now();
    info!("Starting batch parser");
    
    // Validate arguments
    args.validate()?;
    
    // Create output directory if it doesn't exist
    if !args.output_dir.exists() {
        fs::create_dir_all(&args.output_dir)?;
        debug!("Created output directory: {}", args.output_dir.display());
    }
    
    // Scan for files
    info!("Scanning directory: {}", args.input_dir.display());
    let files = scan_directory(&args.input_dir, &args.file_extensions, Some(args.max_files));
    
    if files.is_empty() {
        warn!("No files found to process");
        return Ok(());
    }
    
    info!("Found {} files to process", files.len());
    
    // Process files
    let report = if args.parallel {
        info!("Processing files in parallel");
        process_files_parallel(&files, &args)
    } else {
        info!("Processing files sequentially");
        process_files_sequential(&files, &args)
    };
    
    // Always attempt to generate reports
    info!("Generating reports in: {}", args.output_dir.display());
    if let Err(e) = generate_report(&args.output_dir, &report) {
        error!("Failed to generate reports: {}", e);
        return Err(e.into());
    }
    
    let duration = start_time.elapsed();
    let completion_status = if report.stopped_early {
        "completed with early stopping due to max failures"
    } else {
        "completed successfully"
    };
    info!("Batch processing {} in {:.2}s", completion_status, duration.as_secs_f32());
    
    // Print summary statistics
    print_summary_stats(&report);
    
    // Always return Ok() - we've handled the situation and generated reports
    // Early stopping due to max failures is a normal operational condition
    if report.stopped_early {
        info!("Processing stopped early after reaching maximum failures threshold");
        info!("All collected data has been saved to reports");
    }
    
    Ok(())
}

fn process_files_parallel(files: &[PathBuf], args: &Args) -> Report {
    let report = Arc::new(Mutex::new(Report::new()));
    
    // Create progress bar
    let progress_bar = ProgressBar::new(files.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")
        .unwrap()
        .progress_chars("##-"));
    
    // Create atomic counters
    let failure_count = Arc::new(AtomicUsize::new(0));
    let should_stop = Arc::new(AtomicBool::new(false));
    let processed_count = Arc::new(AtomicUsize::new(0));
    
    // Process files in parallel
    files.par_iter().for_each(|file| {
        if should_stop.load(Ordering::SeqCst) {
            return;
        }
        
        // Process the file - returns (success, hard_failure, warnings)
        // Phase 6 cleanup: success=true even with warnings; only hard errors cause failure=Some()
        let (success, hard_failure, parser_warnings) = process_file(file, Some(&args.output_dir), args);
        
        // Log parser warnings at appropriate level
        if !parser_warnings.is_empty() {
            debug!("Found {} parser warnings for file: {}", parser_warnings.len(), file.display());
            for warning in &parser_warnings {
                debug!("  Warning {}: {}", warning.code, warning.message);
            }
        }
        
        // Update report under lock
        {
            let mut report = report.lock().unwrap();
            if success {
                report.successful_files.push(file.to_string_lossy().to_string());
            }
            if let Some(ref err) = hard_failure {
                report.failures.push(err.clone());
                if err.error_severity == "Error" {
                    let new_failure_count = failure_count.fetch_add(1, Ordering::SeqCst) + 1;
                    if args.max_failures > 0 && new_failure_count >= args.max_failures {
                        should_stop.store(true, Ordering::SeqCst);
                        report.stopped_early = true;
                    }
                }
                progress_bar.set_message(format!("Failed: {} ({})",
                    file.file_name().unwrap_or_default().to_string_lossy(),
                    err.error_category));
            }
            // Store parser warnings if any
            if !parser_warnings.is_empty() {
                let serializable_warnings: Vec<_> = parser_warnings.into_iter()
                    .map(|w| types::SerializableParseWarning::from(w))
                    .collect();
                report.collected_warnings.push((file.to_string_lossy().to_string(), serializable_warnings));
            }
        }
        
        // Update progress
        let current = processed_count.fetch_add(1, Ordering::SeqCst) + 1;
        progress_bar.set_position(current as u64);
        
        if current % 10 == 0 {
            let fail_count = failure_count.load(Ordering::SeqCst);
            progress_bar.set_message(format!("Processed: {}, Failed: {}", current, fail_count));
        }
    });
    
    // Finalize progress bar
    let total_processed = processed_count.load(Ordering::SeqCst);
    let total_failed = failure_count.load(Ordering::SeqCst);
    let stopped_early = should_stop.load(Ordering::SeqCst);
    
    if stopped_early {
        progress_bar.finish_with_message("Stopped - max failures reached");
    } else {
        progress_bar.finish_with_message("Processing complete");
    };
    
    // Finalize report
    let mut report = Arc::try_unwrap(report).unwrap().into_inner().unwrap();
    report.update_stats();
    report.stopped_early = stopped_early;
    
    info!("\nProcessing summary:");
    info!("  Total processed: {}", total_processed);
    info!("  Failed: {}", total_failed);
    info!("  Stopped early: {}", stopped_early);
    
    report
}

fn process_files_sequential(files: &[PathBuf], args: &Args) -> Report {
    let mut report = Report::new();
    let mut failure_count = 0;
    
    // Create progress bar
    let progress_bar = ProgressBar::new(files.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")
        .unwrap()
        .progress_chars("##-"));
    
    for file in files {
        // Check if we should stop due to too many failures
        if args.max_failures > 0 && failure_count >= args.max_failures {
            report.stopped_early = true;
            break;
        }
        
        let (success, hard_failure, parser_warnings) = process_file(file, Some(&args.output_dir), args);
        
        // Log parser warnings at appropriate level
        if !parser_warnings.is_empty() {
            debug!("Found {} parser warnings for file: {}", parser_warnings.len(), file.display());
            for warning in &parser_warnings {
                debug!("  Warning {}: {}", warning.code, warning.message);
            }
        }
        
        if success {
            report.successful_files.push(file.to_string_lossy().to_string());
        }
        if let Some(ref err) = hard_failure {
            report.failures.push(err.clone());
            if err.error_severity == "Error" {
                failure_count += 1;
            }
            progress_bar.set_message(format!("Last: {} ({})",
                file.file_name().unwrap_or_default().to_string_lossy(),
                err.error_category));
        } else {
            progress_bar.set_message(format!("Last: {}",
                file.file_name().unwrap_or_default().to_string_lossy()));
        }
        
        // Store parser warnings if any
        if !parser_warnings.is_empty() {
            let serializable_warnings: Vec<_> = parser_warnings.into_iter()
                .map(|w| types::SerializableParseWarning::from(w))
                .collect();
            report.collected_warnings.push((file.to_string_lossy().to_string(), serializable_warnings));
        }
        
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message(if report.stopped_early {
        "Processing stopped due to too many failures"
    } else {
        "Processing complete"
    });
    
    report.update_stats();
    report
}

fn print_summary_stats(report: &Report) {
    info!("\nProcessing Summary:");
    info!("  Total files processed: {}", report.stats.total_files);
    info!("  Successfully processed (parser): {}", report.stats.successful_files);
    info!("  Failed with errors (hard errors): {}", report.failures.len());
    info!("  Files with parser warnings: {}", report.collected_warnings.len());
    info!("  Total parser warnings: {}", report.stats.total_parser_warnings);
    info!("  Timeouts: {}", report.stats.timeout_files);
    
    if !report.failures.is_empty() {
        info!("\nError Categories:");
        let mut categories = std::collections::HashMap::new();
        for failure in &report.failures {
            *categories.entry(failure.error_category.clone()).or_insert(0) += 1;
        }
        
        for (category, count) in categories {
            info!("  {}: {}", category, count);
        }
    }
    
    if !report.collected_warnings.is_empty() {
        info!("\nParser Warning Summary:");
        info!("  Files with warnings: {}", report.collected_warnings.len());
        info!("  Total warnings: {}", report.stats.total_parser_warnings);
    }
}