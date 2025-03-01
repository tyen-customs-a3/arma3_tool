use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use indicatif::ProgressBar;
use log::{info, warn};
use rayon::prelude::*;
use pbo_tools::{
    core::api::{PboApi, PboApiOps},
    extract::ExtractOptions,
};

use crate::scanning::types::PboScanResult;
use super::scanner::MissionExtractionResult;

pub struct MissionExtractor<'a> {
    cache_dir: &'a Path,
    threads: usize,
}

impl<'a> MissionExtractor<'a> {
    pub fn new(cache_dir: &'a Path, threads: usize) -> Self {
        Self {
            cache_dir,
            threads,
        }
    }
    
    pub fn extract_missions(
        &self,
        scan_results: &[PboScanResult],
        progress: ProgressBar,
    ) -> Result<Vec<MissionExtractionResult>> {
        // Reset progress bar for extraction phase
        progress.set_length(scan_results.len() as u64);
        progress.set_message("Extracting mission PBOs...");
        progress.reset();
        
        // Process in parallel with thread limit
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.threads)
            .build()
            .context("Failed to build thread pool")?;
        
        let results = pool.install(|| {
            scan_results.par_iter()
                .map(|result| self.extract_single_mission(result))
                .inspect(|_| progress.inc(1))
                .filter_map(|r| r.ok())
                .collect::<Vec<_>>()
        });
        
        progress.finish_with_message("Mission extraction complete");
        
        Ok(results)
    }
    
    fn extract_single_mission(&self, scan_result: &PboScanResult) -> Result<MissionExtractionResult> {
        // Create output directory path (strip .pbo extension)
        let output_dir = self.cache_dir.join(scan_result.path.file_name().unwrap())
            .with_extension("");
        
        // Create output directory
        fs::create_dir_all(&output_dir)
            .context(format!("Failed to create output directory: {}", output_dir.display()))?;
        
        // Configure PBO API
        let api = PboApi::builder()
            .with_timeout(60) // Longer timeout for mission PBOs
            .build();
        
        // Extract with focus on SQM and SQF files
        let options = ExtractOptions {
            no_pause: true,
            warnings_as_errors: false,
            file_filter: Some("*.{sqm,sqf}".to_string()),
            verbose: false,
            ..Default::default()
        };
        
        // Extract the PBO
        let result = api.extract_with_options(&scan_result.path, &output_dir, options)
            .context(format!("Failed to extract PBO: {}", scan_result.path.display()))?;
        
        if !result.is_success() {
            return Err(anyhow::anyhow!(
                "Extraction failed for {}: {}", 
                scan_result.path.display(), 
                result
            ));
        }
        
        // Find mission.sqm file
        let sqm_file = walkdir::WalkDir::new(&output_dir)
            .max_depth(2) // Mission.sqm should be at the root or one level down
            .into_iter()
            .filter_map(|e| e.ok())
            .find(|e| {
                e.file_name().to_string_lossy().eq_ignore_ascii_case("mission.sqm")
            })
            .map(|e| e.path().to_owned());
        
        if sqm_file.is_none() {
            warn!("No mission.sqm file found in extracted PBO: {}", scan_result.path.display());
        }
        
        // Find SQF files
        let sqf_files: Vec<PathBuf> = walkdir::WalkDir::new(&output_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("sqf"))
                    .unwrap_or(false)
            })
            .map(|e| e.path().to_owned())
            .collect();
        
        // Get mission name from PBO filename
        let mission_name = scan_result.path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        info!("Extracted mission '{}' with {} SQF files", 
              mission_name, 
              sqf_files.len());
        
        Ok(MissionExtractionResult {
            mission_name,
            pbo_path: scan_result.path.clone(),
            extracted_path: output_dir,
            sqm_file,
            sqf_files,
        })
    }
} 