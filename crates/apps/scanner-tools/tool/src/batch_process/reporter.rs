use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use log::{debug, error, info};
use serde_json;
use super::types::{Report, FileFailure};

/// Generate reports for the batch parsing results
pub fn generate_report(output_dir: &Path, report: &Report) -> io::Result<()> {
    // Create reports directory inside output dir
    let reports_dir = output_dir.join("reports");
    fs::create_dir_all(&reports_dir)?;
    debug!("Created reports directory at: {}", reports_dir.display());

    // Generate JSON report
    let json_path = reports_dir.join("report.json");
    save_json_report(report, &json_path)?;
    info!("JSON report saved to: {}", json_path.display());
    
    // Generate diagnostic report with detailed failure information
    let diagnostic_path = reports_dir.join("diagnostics.log");
    write_diagnostic_report(report, &diagnostic_path)?;
    info!("Diagnostic report saved to: {}", diagnostic_path.display());
    
    // Generate summary report
    let summary_path = reports_dir.join("summary.txt");
    write_summary_report(report, &summary_path)?;
    info!("Summary report saved to: {}", summary_path.display());

    // Ensure all reports were created
    if !json_path.exists() || !diagnostic_path.exists() || !summary_path.exists() {
        error!("One or more reports failed to generate");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to generate all reports"
        ));
    }
    
    info!("All reports generated successfully in: {}", reports_dir.display());
    Ok(())
}

/// Save the report in JSON format
fn save_json_report(report: &Report, path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(report)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    
    fs::write(path, json)?;
    debug!("JSON report saved to: {}", path.display());
    
    Ok(())
}

/// Write a detailed diagnostic report
fn write_diagnostic_report(report: &Report, path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    
    // Write header
    writeln!(file, "Batch Parser Diagnostic Report")?;
    writeln!(file, "Generated at: {}", report.timestamp)?;
    writeln!(file, "{}", "-".repeat(80))?;
    writeln!(file)?;
    
    // Write statistics
    writeln!(file, "Statistics:")?;
    writeln!(file, "  Total files processed: {}", report.stats.total_files)?;
    writeln!(file, "  Successfully processed (parser): {}", report.stats.successful_files)?;
    writeln!(file, "  Failed with errors (hard errors): {}", report.failures.len())?;
    writeln!(file, "  Files with parser warnings: {}", report.collected_warnings.len())?;
    writeln!(file, "  Timeouts: {}", report.stats.timeout_files)?;
    if report.stopped_early {
        writeln!(file, "  Note: Processing stopped early due to too many failures")?;
    }
    writeln!(file)?;
    
    // Write failures grouped by category
    if !report.failures.is_empty() {
        writeln!(file, "Detailed Failure Analysis")?;
        writeln!(file, "{}", "-".repeat(80))?;
        writeln!(file)?;
        
        // Group failures by category
        let mut failures_by_category: std::collections::HashMap<String, Vec<&FileFailure>> =
            std::collections::HashMap::new();
        
        for failure in &report.failures {
            failures_by_category
                .entry(failure.error_category.clone())
                .or_default()
                .push(failure);
        }
        
        // Process each category
        for (category, failures) in failures_by_category.iter() {
            writeln!(file, "Category: {} ({} failures)", category, failures.len())?;
            writeln!(file, "{}", "-".repeat(40))?;
            
            for failure in failures {
                writeln!(file, "\nFile: {}", failure.related_files.first().unwrap_or(&String::new()))?;
                writeln!(file, "Severity: {}", failure.error_severity)?;
                
                if failure.is_timeout {
                    writeln!(file, "Status: Timeout after {}ms", failure.parse_duration_ms)?;
                } else {
                    writeln!(file, "Processing time: {}ms", failure.parse_duration_ms)?;
                }
                
                if let Some(line) = failure.error_line_number {
                    writeln!(file, "Error on line: {}", line)?;
                }
                
                writeln!(file, "\nError message:")?;
                writeln!(file, "{}", failure.error_message)?;
                
                if !failure.diagnostics.is_empty() {
                    writeln!(file, "\nAdditional diagnostics:")?;
                    for diagnostic in &failure.diagnostics {
                        writeln!(file, "  - {}", diagnostic)?;
                    }
                }
                
                if let Some(context) = &failure.error_context {
                    writeln!(file, "\nError context:")?;
                    writeln!(file, "{}", context)?;
                }
                
                if failure.related_files.len() > 1 {
                    writeln!(file, "\nRelated files:")?;
                    for related_file in &failure.related_files[1..] {
                        writeln!(file, "  - {}", related_file)?;
                    }
                }
                
                writeln!(file, "\n{}", "-".repeat(40))?;
            }
            writeln!(file)?;
        }
    }
    
    // Write parser warnings section
    if !report.collected_warnings.is_empty() {
        writeln!(file, "Parser Warnings")?;
        writeln!(file, "{}", "-".repeat(80))?;
        writeln!(file)?;
        
        for (file_path, warnings) in &report.collected_warnings {
            writeln!(file, "File: {}", file_path)?;
            writeln!(file, "{}", "-".repeat(40))?;
            
            for warning in warnings {
                writeln!(file, "  Code: {}", warning.code)?;
                writeln!(file, "  Message: {}", warning.message)?;
                writeln!(file, "  Severity: {}", warning.severity)?;
                writeln!(file)?;
            }
            writeln!(file)?;
        }
    }
    
    debug!("Diagnostic report saved to: {}", path.display());
    Ok(())
}

/// Write a summary report
fn write_summary_report(report: &Report, path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    
    writeln!(file, "Batch Parser Summary Report")?;
    writeln!(file, "Generated at: {}", report.timestamp)?;
    writeln!(file, "{}", "-".repeat(80))?;
    writeln!(file)?;
    
    // Write statistics
    writeln!(file, "Processing Summary")?;
    writeln!(file, "  Total files processed: {}", report.stats.total_files)?;
    writeln!(file, "  Successfully processed (parser): {}", report.stats.successful_files)?;
    writeln!(file, "  Failed with errors (hard errors): {}", report.failures.len())?;
    writeln!(file, "  Files with parser warnings: {}", report.collected_warnings.len())?;
    writeln!(file, "  Timeouts: {}", report.stats.timeout_files)?;
    if report.stopped_early {
        writeln!(file, "  Note: Processing stopped early due to too many failures")?;
    }
    writeln!(file)?;
    
    // Write error categories summary with file counts
    if !report.failures.is_empty() {
        let mut categories = std::collections::HashMap::new();
        let mut category_files = std::collections::HashMap::new();
        
        for failure in &report.failures {
            let category = &failure.error_category;
            *categories.entry(category.clone()).or_insert(0) += 1;
            
            // Store files by category
            category_files
                .entry(category.clone())
                .or_insert_with(Vec::new)
                .extend(failure.related_files.iter().cloned());
        }
        
        writeln!(file, "Failed Files by Category:")?;
        writeln!(file, "{}", "-".repeat(80))?;
        
        // Sort categories for consistent output
        let mut sorted_categories: Vec<_> = categories.keys().collect();
        sorted_categories.sort();
        
        for category in sorted_categories {
            let count = categories[category];
            writeln!(file, "\n{} ({} files):", category, count)?;
            
            if let Some(files) = category_files.get(category) {
                // Sort files for consistent output
                let mut sorted_files: Vec<_> = files.iter().collect();
                sorted_files.sort();
                
                for file_path in sorted_files {
                    // Find the failure details for this file
                    if let Some(failure) = report.failures.iter().find(|f| f.related_files.contains(file_path)) {
                        let reason = if failure.is_timeout {
                            "Timeout".to_string()
                        } else {
                            failure.error_message.lines().next()
                                .unwrap_or(&failure.error_message)
                                .to_string()
                        };
                        
                        writeln!(file, "  - {}", file_path)?;
                        writeln!(file, "    Reason: {}", reason)?;
                        if let Some(line) = failure.error_line_number {
                            writeln!(file, "    Line: {}", line)?;
                        }
                        if failure.is_timeout {
                            writeln!(file, "    Duration: {}ms", failure.parse_duration_ms)?;
                        }
                    }
                }
            }
        }
        writeln!(file)?;
    }
    
    // Write parser warnings section
    if !report.collected_warnings.is_empty() {
        writeln!(file, "Parser Warnings")?;
        writeln!(file, "{}", "-".repeat(80))?;
        writeln!(file)?;
        
        for (file_path, warnings) in &report.collected_warnings {
            writeln!(file, "File: {}", file_path)?;
            for warning in warnings {
                writeln!(file, "  - {} ({}): {}", warning.code, warning.severity, warning.message)?;
            }
            writeln!(file)?;
        }
    }
    
    debug!("Summary report saved to: {}", path.display());
    Ok(())
}
