mod args;

use clap::Parser;
use env_logger::Builder;
use log::{info, warn, error, debug, LevelFilter};
use pbo_tools::{PboApi, ExtractOptions, PboError, PboApiOps};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use thiserror::Error;
use toml;
use walkdir::WalkDir;
use tempfile::Builder as TempBuilder;
use fs_extra::dir as fsx_dir;
use fs_extra::error::ErrorKind as FsxErrorKind;
use path_clean::PathClean;
use std::env;
use rayon::prelude::*;
use regex::Regex;
use std::ffi::OsStr;

use crate::args::Cli;

const TEMPLATE_DIR_NAME: &str = "template/template_project";

// Define text file extensions to scan
const TEXT_FILE_EXTENSIONS: &[&str] = &[
    "cpp", "hpp", "sqf", "txt", "ext", "cfg", "sqm", "inc", "h", "c", "json", "xml", 
    "bikey", "bisign", "bikb", "ksy", "lua", "html", "md", "sqs", "bikb", "fsm", "rvmat"
];

#[derive(Error, Debug)]
enum QuickExtractError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Directory walking error: {0}")]
    WalkDir(#[from] walkdir::Error),
    #[error("PBO Tool error: {0}")]
    Pbo(#[from] PboError),
    #[error("Failed to get PBO filename from path: {0}")]
    FileNameError(PathBuf),
    #[error("Failed to get PBO stem (name without extension) from path: {0}")]
    FileStemError(PathBuf),
    #[error("PBO Extraction failed for {pbo_path}: {reason}")]
    ExtractionFailed {
        pbo_path: PathBuf,
        reason: String,
    },
    #[error("Failed to get PBO prefix for {pbo_path}")]
    PrefixNotFound { pbo_path: PathBuf },
    #[error("Extracted $PBOPREFIX$.txt not found in temp dir for PBO: {0}")]
    ExtractedPrefixNotFound(PathBuf),
    #[error("Could not parse prefix line in {0}: '{1}'")]
    PrefixParseError(PathBuf, String),
    #[error("Filesystem Extra error: {0}")]
    FsExtra(#[from] fs_extra::error::Error),
    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("Template source directory not found: {0}")]
    TemplateNotFound(PathBuf),
    #[error("Cannot determine project name from destination directory: {0}")]
    ProjectNameError(PathBuf),
    #[error("Destination directory already exists: {0}")]
    DestinationExists(PathBuf),
    #[error("Regular expression error: {0}")]
    Regex(#[from] regex::Error),
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectConfig {
    name: String,
    author: String,
    prefix: String,
    mainprefix: String,
}

type Result<T> = std::result::Result<T, QuickExtractError>;

fn main() -> ExitCode {
    if let Err(e) = run() {
        error!("{}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    env_logger::Builder::new()
        .filter_level(if cli.verbose { LevelFilter::Debug } else { LevelFilter::Info })
        .init();

    // --- Resolve Absolute Paths --- 
    let current_dir = env::current_dir()?;
    debug!("Current working directory: {}", current_dir.display());

    let template_path_str = TEMPLATE_DIR_NAME.replace('/', std::path::MAIN_SEPARATOR_STR);
    let absolute_template_path = current_dir.join(template_path_str);
    debug!("Resolved absolute template path: {}", absolute_template_path.display());

    let mut absolute_destination_dir = if cli.destination_dir.is_absolute() {
        cli.destination_dir.clone()
    } else {
        current_dir.join(&cli.destination_dir)
    };
    absolute_destination_dir = absolute_destination_dir.clean();
    debug!("Cleaned absolute destination path: {}", absolute_destination_dir.display());

    info!("Starting HEMTT project creation from template and PBO extraction...");
    debug!("Source PBO directory: {}", cli.source_dir.display());
    debug!("Target HEMTT project directory (absolute): {}", absolute_destination_dir.display());

    if !cli.source_dir.is_dir() {
        error!("Source directory does not exist or is not a directory: {}", cli.source_dir.display());
        return Err(QuickExtractError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Source directory not found")));
    }

    // Remove check for destination existing, we will create it.
    // if absolute_destination_dir.exists() {
    //     return Err(QuickExtractError::DestinationExists(absolute_destination_dir.clone()));
    // }

    // --- Ensure Target Directory Exists --- 
    // Explicitly create the final destination directory
    if !absolute_destination_dir.exists() {
         debug!("Creating final destination directory: {}", absolute_destination_dir.display());
         fs::create_dir_all(&absolute_destination_dir)?;
    } else {
         // If it somehow exists (e.g., from a previous partial run), log it.
         // We might want an option to overwrite or clear it here in the future.
         warn!("Destination directory already exists: {}", absolute_destination_dir.display());
    }
   
    // --- Verify Template Source and Copy Contents --- 
    if !absolute_template_path.is_dir() {
        error!("Template source directory not found or is not a directory: {}", absolute_template_path.display());
        return Err(QuickExtractError::TemplateNotFound(absolute_template_path));
    }
    debug!("Verified template source exists: {}", absolute_template_path.display());

    info!("Copying template project contents from {} to {}", absolute_template_path.display(), absolute_destination_dir.display());
    // Set content_only = true to copy template contents *into* the destination dir
    let mut copy_options = fsx_dir::CopyOptions::new(); 
    copy_options.content_only = true;
    match fsx_dir::copy(&absolute_template_path, &absolute_destination_dir, &copy_options) {
        Ok(_) => info!("Template project contents copied successfully."),
        Err(e) => {
            error!("Filesystem Extra error during copy: {} (Kind: {:?})", e, e.kind);
            // Since destination is guaranteed to exist now, errors are less likely to be path-related
            // but keep specific checks if needed.
            return Err(QuickExtractError::FsExtra(e)); 
            // match e.kind { ... } // Keep previous match block if more specific handling is desired
        }
    }

    let project_name = absolute_destination_dir.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| QuickExtractError::ProjectNameError(absolute_destination_dir.clone()))?;
    
    let project_toml_path = absolute_destination_dir.join(".hemtt").join("project.toml");
    info!("Modifying {}", project_toml_path.display());

    let toml_content_str = fs::read_to_string(&project_toml_path)?;
    let mut config: ProjectConfig = toml::from_str(&toml_content_str)?;

    debug!("Original project.toml config: {:?}", config);
    config.name = project_name.to_string();
    config.prefix = project_name.to_string();
    config.mainprefix = project_name.to_string();
    debug!("Modified project.toml config: {:?}", config);

    let updated_toml_content = toml::to_string_pretty(&config)?;
    fs::write(&project_toml_path, updated_toml_content)?;
    info!("Updated project.toml with name/prefix: {}", project_name);

    let pbo_api = PboApi::builder()
        .with_timeout(cli.timeout)
        .build();

    let mut processed_count = 0;
    let mut skipped_count = 0;
    let target_addons_base_dir = absolute_destination_dir.join("addons"); 

    // --- Collect PBO Paths (Sequential) ---
    info!("Scanning for PBO files...");
    let pbo_paths: Vec<PathBuf> = WalkDir::new(&cli.source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "pbo"))
        .map(|e| e.path().to_path_buf())
        .collect();
    info!("Found {} PBO files to process.", pbo_paths.len());

    // --- Process PBOs (Parallel) ---
    info!("Processing PBOs in parallel...");

    // Use `map` to collect results from each parallel task
    // Each result will be a Result<(), QuickExtractError>
    let results: Vec<Result<()>> = pbo_paths
        .par_iter() // Use parallel iterator
        .map(|pbo_path| {
            // --- Logic for processing a SINGLE PBO --- 
            // Clone or capture necessary data (paths, cli options)
            let pbo_path = pbo_path.clone();
            let target_addons_base_dir = target_addons_base_dir.clone(); 
            let verbose = cli.verbose;
            let timeout = cli.timeout;

            // Process this single PBO
            info!("Processing PBO: {}", pbo_path.display()); // Log might interleave

            let pbo_stem_osstr = pbo_path.file_stem()
                .ok_or_else(|| QuickExtractError::FileStemError(pbo_path.to_path_buf()))?;
            let pbo_stem = pbo_stem_osstr.to_string_lossy().to_string();

            let target_addon_source_dir = target_addons_base_dir.join(&pbo_stem);
            fs::create_dir_all(&target_addon_source_dir)?;

            let temp_dir = TempBuilder::new()
                .prefix(&format!("quick_extract_{}_", pbo_stem))
                .tempdir()?;
            let temp_dir_path = temp_dir.path();
            // Consider logging temp dir path only if verbose
            // debug!("Created temporary directory: {}", temp_dir_path.display());

            // Create PboApi instance per task
            let pbo_api = PboApi::builder()
                .with_timeout(timeout) // Use captured timeout
                .build();

            let extract_options = ExtractOptions {
                no_pause: true,
                warnings_as_errors: false,
                verbose: verbose, // Use captured verbosity
                ..Default::default()
            };

            match pbo_api.extract_with_options(&pbo_path, temp_dir_path, extract_options) {
                Ok(result) => {
                    if result.is_success() {
                        // debug!("Successfully extracted {} to temp dir {}", pbo_path.display(), temp_dir_path.display());
                        match find_prefix_file(temp_dir_path) {
                            Some(temp_prefix_file_path) => {
                                // debug!("Found $PBOPREFIX$.txt at: {}", temp_prefix_file_path.display());
                                let content_root_in_temp = temp_prefix_file_path.parent().unwrap();

                                let move_options = fsx_dir::CopyOptions::new().content_only(true).overwrite(true);
                                fsx_dir::move_dir(content_root_in_temp, &target_addon_source_dir, &move_options)?;
                                // info!("Moved extracted content for {} to {}", pbo_stem, target_addon_source_dir.display());

                                let final_prefix_path = target_addon_source_dir.join("$PBOPREFIX$.txt");
                                process_prefix_file(&final_prefix_path)?;
                                // info!("Processed $PBOPREFIX$.txt for {}", pbo_stem);
                                Ok(())
                            }
                            None => {
                                warn!("$PBOPREFIX$.txt not found within extracted content for {}. Skipping addon.", pbo_path.display());
                                if let Err(e) = fs::remove_dir(&target_addon_source_dir) {
                                    warn!("Failed to clean up empty target dir {}: {}", target_addon_source_dir.display(), e);
                                }
                                Err(QuickExtractError::ExtractedPrefixNotFound(pbo_path.to_path_buf()))
                            }
                        }
                    } else {
                        let reason = result.get_error_message().unwrap_or_else(|| "Unknown extraction error".to_string());
                        error!("Extraction failed for {} (reported by pbo_tools): {}", pbo_path.display(), reason);
                        Err(QuickExtractError::ExtractionFailed{ pbo_path: pbo_path.to_path_buf(), reason })
                    }
                }
                Err(e) => {
                    error!("Error during extraction process for {}: {}", pbo_path.display(), e);
                    Err(e.into())
                }
            }
        })
        .collect(); // Collect all results (Ok or Err)

    // --- Process Results (Sequential) ---
    for result in results {
        match result {
            Ok(_) => processed_count += 1,
            Err(e) => {
                skipped_count += 1;
                // The path information is embedded in the error variants
                error!("Failed to process PBO: {}", e);
            }
        }
    }

    info!("Parallel PBO processing finished.");
    info!("Successfully processed {} PBOs.", processed_count);
    if skipped_count > 0 {
        warn!("Skipped {} PBOs due to errors.", skipped_count);
    }

    // Post-processing: Clean up DeRap headers
    info!("Starting post-processing: removing DeRap headers from text files...");
    match remove_derap_headers(&absolute_destination_dir) {
        Ok(count) => info!("Removed DeRap headers from {} files.", count),
        Err(e) => error!("Error during DeRap header removal: {}", e),
    }
    
    // Final completion message
    info!("HEMTT project creation from template and PBO extraction completed at {}", absolute_destination_dir.display());

    Ok(())
}

fn find_prefix_file(search_dir: &Path) -> Option<PathBuf> {
    WalkDir::new(search_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name().to_string_lossy() == "$PBOPREFIX$.txt")
        .map(|entry| entry.path().to_path_buf())
}

fn process_prefix_file(prefix_file_path: &Path) -> Result<()> {
    let file = fs::File::open(prefix_file_path)?;
    let reader = BufReader::new(file);
    let mut found_prefix_value = None;

    for line_result in reader.lines() {
        let line = line_result?;
        if let Some(stripped) = line.trim().strip_prefix("prefix=") {
            found_prefix_value = Some(stripped.trim_end_matches(';').to_string());
            break;
        }
    }

    match found_prefix_value {
        Some(prefix_value) => {
            let mut file = fs::File::create(prefix_file_path)?;
            writeln!(file, "{}", prefix_value)?;
            Ok(())
        }
        None => Err(QuickExtractError::PrefixParseError(
            prefix_file_path.to_path_buf(),
            "'prefix=' line not found".to_string(),
        )),
    }
}

/// Removes DeRap headers from all text files in the given directory.
/// Returns the number of files processed.
fn remove_derap_headers(dir: &Path) -> Result<usize> {
    debug!("Scanning for text files in {}", dir.display());
    
    // Compile the regex pattern for DeRap headers
    let derap_pattern = Regex::new(r"(?m)^/+\r?\n//DeRap:.*?\r?\n//Produced from.*?\r?\n//.*?\r?\n//.*?\r?\n/+\r?\n")?;
    
    let mut processed_count = 0;
    
    // Walk the directory recursively
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| is_text_file(e.path()))
    {
        let file_path = entry.path();
        debug!("Checking file: {}", file_path.display());
        
        // Read the file content
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                warn!("Could not read file {}: {}", file_path.display(), e);
                continue;
            }
        };
        
        // Check if the file contains a DeRap header
        if derap_pattern.is_match(&content) {
            // Remove the header
            let new_content = derap_pattern.replace_all(&content, "").to_string();
            
            // Write the updated content back to the file
            match fs::write(file_path, new_content) {
                Ok(_) => {
                    debug!("Removed DeRap header from {}", file_path.display());
                    processed_count += 1;
                },
                Err(e) => {
                    warn!("Could not write to file {}: {}", file_path.display(), e);
                }
            }
        }
    }
    
    Ok(processed_count)
}

/// Checks if a file is likely a text file based on its extension
fn is_text_file(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map(|ext| TEXT_FILE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}
