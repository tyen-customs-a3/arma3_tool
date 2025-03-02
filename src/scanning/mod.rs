use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use log::{info, warn, error};
use missions::analyze_mission_dependencies_with_classes;
use rayon::prelude::*;
use walkdir::{WalkDir, DirEntry};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use pbo_tools::{
    core::api::{PboApi, PboApiOps},
    extract::ExtractOptions,
};

// Utility types used by both scanners
mod types;

pub mod pbo;
pub mod classes;
pub mod missions;

// Re-export the main scanning functions for easier access
pub use pbo::scan_pbos;
pub use classes::scan_classes;
pub use missions::scan_missions;
pub use missions::analyze_mission_dependencies;

/// Create a report configuration from command-line arguments
fn create_report_config(disable_reports: Option<&str>, enable_reports: Option<&str>) -> crate::reporting::ReportConfig {
    match (disable_reports, enable_reports) {
        // If enable_reports is specified, create a config with all reports disabled by default
        // and then enable only the specified reports
        (_, Some(enable_list)) => {
            let mut config = crate::reporting::ReportConfig::all_disabled();
            for report_type in enable_list.split(',').map(|s| s.trim()) {
                if !report_type.is_empty() {
                    config.enable(report_type);
                }
            }
            config
        },
        // If only disable_reports is specified, create a config with all reports enabled by default
        // and then disable the specified reports
        (Some(disable_list), None) => {
            let mut config = crate::reporting::ReportConfig::new();
            for report_type in disable_list.split(',').map(|s| s.trim()) {
                if !report_type.is_empty() {
                    config.disable(report_type);
                }
            }
            config
        },
        // If neither is specified, create a config with all reports enabled by default
        (None, None) => crate::reporting::ReportConfig::new(),
    }
}

/// Run a complete analysis pipeline for Arma 3 base game, mods, and missions
pub async fn full_analysis(args: crate::commands::FullAnalysisArgs) -> Result<()> {
    info!("Starting full Arma 3 analysis pipeline");
    
    // Create necessary directories
    let a3_cache_dir = args.cache_dir.join("a3_base");
    let mods_cache_dir = args.cache_dir.join("mods");
    let missions_cache_dir = args.cache_dir.join("missions");
    let reports_dir = args.output_dir.clone();
    
    fs::create_dir_all(&a3_cache_dir)?;
    fs::create_dir_all(&mods_cache_dir)?;
    fs::create_dir_all(&missions_cache_dir)?;
    fs::create_dir_all(&reports_dir)?;
    
    // Create report configuration from command-line arguments
    let report_config = create_report_config(
        args.disable_reports.as_deref(),
        args.enable_reports.as_deref()
    );
    
    // Log which reports are disabled
    if let Some(disable_reports) = &args.disable_reports {
        info!("Disabling reports: {}", disable_reports);
    }
    
    // Log which reports are enabled (if using enable_reports)
    if let Some(enable_reports) = &args.enable_reports {
        info!("Enabling only these reports: {}", enable_reports);
    }
    
    // Check if we have already extracted data
    let a3_extracted = Path::new(&a3_cache_dir).exists() && 
        fs::read_dir(&a3_cache_dir)?.next().is_some();
    let mods_extracted = Path::new(&mods_cache_dir).exists() && 
        fs::read_dir(&mods_cache_dir)?.next().is_some();
    
    let missions_output_dir = reports_dir.join("missions");
    let missions_extracted = Path::new(&missions_cache_dir).exists() && 
        fs::read_dir(&missions_cache_dir)?.next().is_some() &&
        Path::new(&missions_output_dir).exists() &&
        fs::read_dir(&missions_output_dir)?.next().is_some();
    
    let a3_classes_dir = reports_dir.join("a3_base_classes");
    let mods_classes_dir = reports_dir.join("mods_classes");
    
    // Step 1: Scan Arma 3 base game files (only if not already extracted)
    if !a3_extracted {
        info!("Step 1/5: Scanning Arma 3 base game files");
        let a3_pbo_args = crate::commands::ScanPboArgs {
            input_dir: args.arma3_dir.clone(),
            cache_dir: a3_cache_dir.clone(),
            extensions: "hpp,cpp,sqf,sqm".to_string(),
            threads: args.threads,
        };
        scan_pbos(a3_pbo_args).await?;
    } else {
        info!("Step 1/5: Skipping Arma 3 base game extraction (using cached data)");
    }
    
    // Step 2: Scan mod files (only if not already extracted)
    if !mods_extracted {
        info!("Step 2/5: Scanning mod files");
        let mods_pbo_args = crate::commands::ScanPboArgs {
            input_dir: args.mods_dir.clone(),
            cache_dir: mods_cache_dir.clone(),
            extensions: "hpp,cpp,sqf,sqm".to_string(),
            threads: args.threads,
        };
        scan_pbos(mods_pbo_args).await?;
    } else {
        info!("Step 2/5: Skipping mod files extraction (using cached data)");
    }
    
    // Step 3: Scan mission files (only if not already extracted)
    if !missions_extracted {
        info!("Step 3/5: Scanning mission files");
        let missions_args = crate::commands::ScanMissionsArgs {
            input_dir: args.missions_dir.clone(),
            cache_dir: missions_cache_dir.clone(),
            output_dir: reports_dir.join("missions"), // Use a direct "missions" folder instead of "mission_reports"
            threads: args.threads,
            disable_reports: args.disable_reports.clone(),
            enable_reports: args.enable_reports.clone(),
        };
        scan_missions(missions_args).await?;
    } else {
        info!("Step 3/5: Skipping mission files extraction (using cached data)");
    }
    
    // Step 4: Scan class definitions and keep the data in memory
    info!("Step 4a/5: Scanning Arma 3 base game class definitions");
    let a3_classes_args = crate::commands::ScanClassesArgs {
        input_dir: a3_cache_dir.clone(),
        output_dir: a3_classes_dir.clone(),
        max_files: None,
        verbose_errors: false,
        disable_reports: args.disable_reports.clone(),
        enable_reports: args.enable_reports.clone(),
    };
    let a3_classes = scan_classes(a3_classes_args).await?;
    info!("Loaded {} base game classes into memory", a3_classes.len());
    
    info!("Step 4b/5: Scanning mod class definitions");
    let mods_classes_args = crate::commands::ScanClassesArgs {
        input_dir: mods_cache_dir.clone(),
        output_dir: mods_classes_dir.clone(),
        max_files: None,
        verbose_errors: false,
        disable_reports: args.disable_reports.clone(),
        enable_reports: args.enable_reports.clone(),
    };
    let mod_classes = scan_classes(mods_classes_args).await?;
    info!("Loaded {} mod classes into memory", mod_classes.len());
    
    // Combine all classes for validation
    let mut all_classes = Vec::new();
    all_classes.extend(a3_classes);
    all_classes.extend(mod_classes);
    info!("Combined {} total classes for validation", all_classes.len());
    
    // Step 5: Analyze dependencies using in-memory class data
    info!("Step 5/5: Analyzing dependencies");
    
    // Create the missions output directory
    let missions_output_dir = reports_dir.join("missions");
    fs::create_dir_all(&missions_output_dir)?;
    
    let analysis_args = crate::commands::AnalyzeMissionDependenciesArgs {
        mission_dir: args.missions_dir,
        addon_dir: args.cache_dir.clone(),
        cache_dir: args.cache_dir.clone(), // Use the main cache directory instead of creating a separate analysis cache
        output_dir: missions_output_dir.clone(), // Use the missions directory directly
        threads: args.threads,
        class_db_dir: None, // We'll use in-memory classes instead
        disable_reports: args.disable_reports.clone(),
        enable_reports: args.enable_reports.clone(),
    };
    
    // Call the analyze_mission_dependencies function with in-memory classes
    analyze_mission_dependencies_with_classes(analysis_args, all_classes).await?;
    
    info!("Full analysis completed successfully");
    
    Ok(())
}

// File extensions we're interested in
const VALID_EXTENSIONS: [&str; 2] = ["cpp", "hpp"];

// Cache for file hashes to detect changes
#[derive(Debug, Serialize, Deserialize)]
struct FileCache {
    hash: String,
    timestamp: std::time::SystemTime,
}

// Helper function to filter relevant files
fn is_target_file(entry: &DirEntry) -> bool {
    if !entry.file_type().is_file() {
        return false;
    }

    entry.path()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| VALID_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

// Helper function to calculate file hash
fn calculate_file_hash(path: &Path) -> Result<String> {
    let content = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    Ok(format!("{:x}", hasher.finalize()))
}

// Helper function to check if file needs processing
fn needs_processing(path: &Path, cache: &mut HashMap<PathBuf, FileCache>) -> Result<bool> {
    let metadata = fs::metadata(path)?;
    let modified = metadata.modified()?;

    if let Some(cached) = cache.get(path) {
        // Check if file was modified since last scan
        if modified == cached.timestamp {
            return Ok(false);
        }
    }

    let hash = calculate_file_hash(path)?;
    cache.insert(
        path.to_path_buf(),
        FileCache {
            hash,
            timestamp: modified,
        },
    );
    Ok(true)
}

// Helper function to collect files efficiently
fn collect_files(input_dir: &Path) -> Result<Vec<PathBuf>> {
    info!("Collecting files to process from {}", input_dir.display());
    
    let files: Vec<PathBuf> = WalkDir::new(input_dir)
        .min_depth(1)
        .max_depth(32) // Reasonable max depth to prevent infinite recursion
        .into_iter()
        .filter_entry(|e| {
            // Skip hidden directories and files
            !e.file_name()
                .to_str()
                .map(|s| s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter_map(|e| e.ok())
        .filter(is_target_file)
        .map(|e| e.path().to_owned())
        .collect();

    info!("Found {} files to process", files.len());
    Ok(files)
}

// Helper function to create a consistent progress bar style
fn create_progress_bar(len: u64, desc: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg} ({eta})")
            .unwrap()
            .progress_chars("=> ")
    );
    pb.set_message(desc.to_string());
    pb
}

// Add performance monitoring
struct ScanMetrics {
    start_time: std::time::Instant,
    processed_files: usize,
    processed_bytes: usize,
    total_time: std::time::Duration,
}

impl ScanMetrics {
    fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            processed_files: 0,
            processed_bytes: 0,
            total_time: std::time::Duration::default(),
        }
    }

    fn update(&mut self, file_size: u64) {
        self.processed_files += 1;
        self.processed_bytes += file_size as usize;
        self.total_time = self.start_time.elapsed();
    }

    fn log_stats(&self) {
        let mb_processed = self.processed_bytes as f64 / 1_048_576.0;
        let seconds = self.total_time.as_secs_f64();
        let mb_per_second = mb_processed / seconds;
        
        info!(
            "Scan Performance: {:.2} MB/s ({:.2} MB in {:.2}s)",
            mb_per_second, mb_processed, seconds
        );
        info!(
            "Files processed: {} (avg {:.2} ms/file)",
            self.processed_files,
            (seconds * 1000.0) / self.processed_files as f64
        );
    }
}