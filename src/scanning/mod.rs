use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use log::{info, warn, error};
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
    
    // Step 1: Scan Arma 3 base game files
    info!("Step 1/5: Scanning Arma 3 base game files");
    let a3_pbo_args = crate::commands::ScanPboArgs {
        input_dir: args.arma3_dir.clone(),
        cache_dir: a3_cache_dir.clone(),
        extensions: "hpp,cpp,sqf,sqm".to_string(),
        threads: args.threads,
    };
    scan_pbos(a3_pbo_args).await?;
    
    // Step 2: Scan mod files
    info!("Step 2/5: Scanning mod files");
    let mods_pbo_args = crate::commands::ScanPboArgs {
        input_dir: args.mods_dir.clone(),
        cache_dir: mods_cache_dir.clone(),
        extensions: "hpp,cpp,sqf,sqm".to_string(),
        threads: args.threads,
    };
    scan_pbos(mods_pbo_args).await?;
    
    // Step 3: Scan mission files
    info!("Step 3/5: Scanning mission files");
    let mission_reports_dir = reports_dir.join("mission_reports");
    let missions_args = crate::commands::ScanMissionsArgs {
        input_dir: args.missions_dir.clone(),
        cache_dir: missions_cache_dir.clone(),
        output_dir: mission_reports_dir.clone(),
        threads: args.threads,
    };
    scan_missions(missions_args).await?;
    
    // Step 4: Scan class definitions
    info!("Step 4/5: Scanning class definitions");
    let a3_classes_dir = reports_dir.join("a3_base_classes");
    let a3_classes_args = crate::commands::ScanClassesArgs {
        input_dir: a3_cache_dir,
        output_dir: a3_classes_dir,
    };
    scan_classes(a3_classes_args).await?;
    
    let mods_classes_dir = reports_dir.join("mods_classes");
    let mods_classes_args = crate::commands::ScanClassesArgs {
        input_dir: mods_cache_dir,
        output_dir: mods_classes_dir,
    };
    scan_classes(mods_classes_args).await?;
    
    // Step 5: Analyze dependencies
    info!("Step 5/5: Analyzing dependencies");
    let analysis_cache_dir = args.cache_dir.join("analysis");
    let analysis_reports_dir = reports_dir.join("analysis_reports");
    let analysis_args = crate::commands::AnalyzeMissionDependenciesArgs {
        mission_dir: args.missions_dir,
        addon_dir: args.cache_dir.clone(),
        cache_dir: analysis_cache_dir,
        output_dir: analysis_reports_dir,
        threads: args.threads,
    };
    analyze_mission_dependencies(analysis_args).await?;
    
    info!("Full analysis complete! Reports are available in {}", reports_dir.display());
    info!("Summary of reports:");
    info!("  - Mission reports: {}", mission_reports_dir.display());
    info!("  - Class reports: {}/a3_base_classes and {}/mods_classes", reports_dir.display(), reports_dir.display());
    info!("  - Dependency analysis: {}/analysis_reports", reports_dir.display());
    
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