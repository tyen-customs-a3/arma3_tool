use std::path::{Path, PathBuf};
use std::fs;
use super::config::{CONTEXT_LINES, MAX_LINE_LENGTH};
use std::collections::HashMap;
use log::debug;

pub fn categorize_error(error_message: &str) -> String {
    let error_patterns = [
        (vec!["unexpected token", "expected"], "Syntax Error"),
        (vec!["undefined", "not defined", "unknown"], "Undefined Symbol"),
        (vec!["type mismatch", "expected type", "incompatible types"], "Type Error"),
        (vec!["circular", "recursion"], "Circular Reference"),
        (vec!["duplicate", "already defined"], "Duplicate Definition"),
        (vec!["missing", "required"], "Missing Requirement"),
        (vec!["denied", "permission", "access"], "Permission Error"),
        (vec!["timeout", "time limit"], "Timeout Error"),
    ];

    // First try to match against known patterns
    for (patterns, category) in &error_patterns {
        for pattern in patterns {
            if error_message.to_lowercase().contains(pattern) {
                return category.to_string();
            }
        }
    }

    // If no pattern matched, try to extract the category from the first part of the message
    // Example: "ParseError: unexpected token" -> "Parse Error"
    if let Some(first_part) = error_message.split(':').next() {
        // Insert spaces before capital letters to turn camelCase or PascalCase into spaced words
        let mut formatted = String::new();
        let mut prev_is_lower = false;
        
        for c in first_part.chars() {
            if c.is_uppercase() && prev_is_lower {
                formatted.push(' ');
            }
            formatted.push(c);
            prev_is_lower = c.is_lowercase();
        }
        
        // Replace "Error" with "" to avoid redundancy in "Parse Error Error"
        return formatted.replace("Error", "").trim().to_string() + " Error";
    }

    // Default to "Other Error" if no category can be determined
    "Other Error".to_string()
}

pub fn extract_error_context(file_path: &Path, line_number: Option<usize>) -> Option<String> {
    if let Some(target_line) = line_number {
        if let Ok(content) = fs::read_to_string(file_path) {
            let lines: Vec<&str> = content.lines().collect();
            let start = target_line.saturating_sub(CONTEXT_LINES);
            let end = (target_line + CONTEXT_LINES).min(lines.len());
            
            // Calculate the width needed for line numbers
            let max_line_num = end;
            let line_num_width = max_line_num.to_string().len();
            
            let mut context = format!("File: {}\n", file_path.display());
            context.push_str(&format!("Lines {}-{} (error on line {}):\n", start + 1, end, target_line));
            context.push_str(&"-".repeat(80));
            context.push_str("\n");
            
            // Add the context lines with improved formatting
            context.extend(
                lines[start..end]
                    .iter()
                    .enumerate()
                    .map(|(i, &line)| {
                        let line_num = start + i + 1;
                        let (prefix, line_style) = if line_num == target_line {
                            (">", "====> ")
                        } else {
                            (" ", "      ")
                        };
                        
                        // Truncate very long lines
                        let line_content = if line.len() > MAX_LINE_LENGTH {
                            format!("{}... (line truncated, {} characters total)", 
                                  &line[..MAX_LINE_LENGTH], line.len())
                        } else {
                            line.to_string()
                        };
                        
                        format!("{} {:width$} {} {}\n", 
                               prefix,
                               line_num,
                               line_style,
                               line_content,
                               width = line_num_width)
                    })
            );
            
            context.push_str(&"-".repeat(80));
            context.push_str("\n");
            
            return Some(context);
        }
    }
    None
}

pub fn extract_related_context(
    main_file_path: &Path,
    related_files: &[String],
    error_line_number: Option<usize>,
) -> Option<String> {
    const CONTEXT_LINES: usize = 3;
    let mut context = String::new();
    
    // First, extract context from main file if line number is available
    if let Some(line_num) = error_line_number {
        if let Ok(content) = fs::read_to_string(main_file_path) {
            let lines: Vec<&str> = content.lines().collect();
            let start_line = line_num.saturating_sub(CONTEXT_LINES);
            let end_line = (line_num + CONTEXT_LINES).min(lines.len());
            
            context.push_str("Main file context:\n");
            for i in start_line..end_line {
                let line_prefix = if i + 1 == line_num { "▶ " } else { "  " };
                if i < lines.len() {
                    context.push_str(&format!("{}{}: {}\n", line_prefix, i + 1, lines[i]));
                }
            }
        }
    }
    
    // Then extract context from related files (up to 2 to avoid overwhelming output)
    let max_related_files = 2;
    let related_files_to_show = related_files.iter()
        .filter(|&path| path != &main_file_path.to_string_lossy())
        .take(max_related_files);
    
    for (index, related_path) in related_files_to_show.enumerate() {
        if let Ok(content) = fs::read_to_string(related_path) {
            if !context.is_empty() {
                context.push_str("\n");
            }
            
            context.push_str(&format!("Related file {} ({}):\n", index + 1, related_path));
            
            // Only show the first few lines of each related file as a preview
            const PREVIEW_LINES: usize = 5;
            let lines: Vec<&str> = content.lines().collect();
            for i in 0..PREVIEW_LINES.min(lines.len()) {
                context.push_str(&format!("  {}: {}\n", i + 1, lines[i]));
            }
            
            if lines.len() > PREVIEW_LINES {
                context.push_str("  ...\n");
            }
        }
    }
    
    if context.is_empty() {
        None
    } else {
        Some(context)
    }
}

/// Create an annotation from an error, following the format seen in
/// HEMTT/libs/workspace/src/reporting/diagnostic/annotation.rs
pub fn create_annotation(
    file_path: &str,
    line_number: usize,
    message: &str,
    is_error: bool
) -> HashMap<String, String> {
    let mut annotation = HashMap::new();
    
    annotation.insert("path".to_string(), file_path.to_string());
    annotation.insert("start_line".to_string(), line_number.to_string());
    annotation.insert("end_line".to_string(), line_number.to_string());
    annotation.insert("start_column".to_string(), "1".to_string());
    annotation.insert("end_column".to_string(), "1".to_string());
    annotation.insert("level".to_string(), if is_error { "error".to_string() } else { "warning".to_string() });
    annotation.insert("message".to_string(), message.to_string());
    annotation.insert("title".to_string(), categorize_error(message));
    
    annotation
}

/// Generate a formatted report line for console output
pub fn format_report_line(file_path: &str, error_category: &str, message: &str) -> String {
    format!("{}: [{}] {}", file_path, error_category, message)
} 