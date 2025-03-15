use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fs;
use rayon::prelude::*;
use log::{debug, info, warn, error};
use crate::cache::CacheManager;
use crate::error::{Result, ToolError};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use extraction::extract_pbo_with_options;
use pbo_tools::extract::ExtractOptions;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents the result of prescanning a PBO file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrescanResult {
    /// PBO should be skipped due to permanent error
    Skip(String),
    
    /// PBO can use cached extraction
    UseCached(PathBuf),
    
    /// PBO needs to be extracted
    NeedsExtraction,
}

/// Represents a prescanned PBO file
#[derive(Debug, Clone)]
pub struct PrescannedPbo {
    /// Path to the PBO file
    pub path: PathBuf,
    
    /// Path to the cache directory
    pub cache_path: PathBuf,
    
    /// Result of prescanning
    pub result: PrescanResult,
    
    /// File extensions to extract
    pub extensions: Vec<String>,
    
    /// File filter pattern
    pub file_filter: Option<String>,
}

/// Represents the result of extracting a PBO file
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    /// Path to the PBO file
    pub pbo_path: PathBuf,
    
    /// Path to the extracted cache directory
    pub cache_path: PathBuf,
    
    /// Whether extraction was successful
    pub success: bool,
    
    /// Error message if extraction failed
    pub error_message: Option<String>,
    
    /// Whether this is a permanent error (like bad SHA)
    pub is_permanent_error: bool,
}

/// Prescanner for PBO files
pub struct PboPrescan {
    /// Cache manager
    cache_manager: CacheManager,
    
    /// Whether this is for game data (true) or missions (false)
    is_game_data: bool,
}

impl PboPrescan {
    /// Create a new PBO prescanner for game data
    pub fn for_game_data(cache_manager: CacheManager) -> Self {
        Self {
            cache_manager,
            is_game_data: true,
        }
    }
    
    /// Create a new PBO prescanner for missions
    pub fn for_missions(cache_manager: CacheManager) -> Self {
        Self {
            cache_manager,
            is_game_data: false,
        }
    }
    
    /// Discover PBO files in directories
    pub fn discover_pbos(&self, directories: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut pbo_paths = Vec::new();
        
        // Walk through each directory and find PBO files
        for dir in directories {
            debug!("Scanning directory for PBOs: {}", dir.display());
            
            for entry in WalkDir::new(dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                
                if path.is_file() && path.extension().map_or(false, |ext| ext == "pbo") {
                    debug!("Found PBO: {}", path.display());
                    pbo_paths.push(path.to_path_buf());
                }
            }
        }
        
        Ok(pbo_paths)
    }
    
    /// Prescan PBO files to determine which need extraction
    pub fn prescan(&self, pbo_paths: &[PathBuf], extensions: &[String]) -> Result<Vec<PrescannedPbo>> {
        if pbo_paths.is_empty() {
            return Ok(Vec::new());
        }
        
        info!("Prescanning {} PBO files", pbo_paths.len());
        
        // Create progress bar
        let progress_bar = ProgressBar::new(pbo_paths.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        progress_bar.set_message("Prescanning PBOs");
        
        // Create file filter if extensions are specified
        let file_filter = if !extensions.is_empty() {
            let filter = extensions.iter()
                .map(|ext| format!("*.{}", ext))
                .collect::<Vec<_>>()
                .join(",");
            Some(filter)
        } else {
            None
        };
        
        // Shared counter for statistics
        let stats = Arc::new(Mutex::new(HashMap::new()));
        
        // Process each PBO in parallel
        let results: Vec<PrescannedPbo> = pbo_paths.par_iter()
            .map(|pbo_path| {
                // Get cache path
                let cache_path = if self.is_game_data {
                    self.cache_manager.get_game_data_cache_path(pbo_path)
                } else {
                    self.cache_manager.get_mission_cache_path(pbo_path)
                };
                
                // Check if PBO has a permanent error
                if self.has_permanent_error(pbo_path) {
                    // Update statistics
                    let mut stats_guard = stats.lock().unwrap();
                    *stats_guard.entry("skip_permanent".to_string()).or_insert(0) += 1;
                    
                    // Skip PBO with permanent error
                    progress_bar.inc(1);
                    return PrescannedPbo {
                        path: pbo_path.clone(),
                        cache_path,
                        result: PrescanResult::Skip("Permanent error".to_string()),
                        extensions: extensions.to_vec(),
                        file_filter: file_filter.clone(),
                    };
                }
                
                // Check if PBO has a regular error but patterns have changed
                if self.has_error(pbo_path) && self.has_pattern_changed(pbo_path, extensions, file_filter.as_deref()) {
                    // Update statistics
                    let mut stats_guard = stats.lock().unwrap();
                    *stats_guard.entry("retry_error".to_string()).or_insert(0) += 1;
                    
                    // Need to retry extraction since patterns changed
                    progress_bar.inc(1);
                    return PrescannedPbo {
                        path: pbo_path.clone(),
                        cache_path,
                        result: PrescanResult::NeedsExtraction,
                        extensions: extensions.to_vec(),
                        file_filter: file_filter.clone(),
                    };
                } else if self.has_error(pbo_path) {
                    // Update statistics
                    let mut stats_guard = stats.lock().unwrap();
                    *stats_guard.entry("skip_error".to_string()).or_insert(0) += 1;
                    
                    // Skip PBO with error
                    let error_msg = self.get_error_message(pbo_path)
                        .unwrap_or_else(|| "Unknown error".to_string());
                    
                    progress_bar.inc(1);
                    return PrescannedPbo {
                        path: pbo_path.clone(),
                        cache_path,
                        result: PrescanResult::Skip(error_msg),
                        extensions: extensions.to_vec(),
                        file_filter: file_filter.clone(),
                    };
                }
                
                // Check if already cached and patterns haven't changed
                if self.is_cached(pbo_path, &cache_path) && !self.has_pattern_changed(pbo_path, extensions, file_filter.as_deref()) {
                    // Update statistics
                    let mut stats_guard = stats.lock().unwrap();
                    *stats_guard.entry("use_cached".to_string()).or_insert(0) += 1;
                    
                    // Use cached extraction
                    progress_bar.inc(1);
                    return PrescannedPbo {
                        path: pbo_path.clone(),
                        cache_path: cache_path.clone(),
                        result: PrescanResult::UseCached(cache_path),
                        extensions: extensions.to_vec(),
                        file_filter: file_filter.clone(),
                    };
                }
                
                // Update statistics
                let mut stats_guard = stats.lock().unwrap();
                *stats_guard.entry("needs_extraction".to_string()).or_insert(0) += 1;
                
                // PBO needs extraction
                progress_bar.inc(1);
                PrescannedPbo {
                    path: pbo_path.clone(),
                    cache_path,
                    result: PrescanResult::NeedsExtraction,
                    extensions: extensions.to_vec(),
                    file_filter: file_filter.clone(),
                }
            })
            .collect();
        
        // Finish progress bar
        progress_bar.finish_with_message("Prescanning complete");
        
        // Log statistics
        let stats_guard = stats.lock().unwrap();
        let skip_permanent = stats_guard.get("skip_permanent").unwrap_or(&0);
        let skip_error = stats_guard.get("skip_error").unwrap_or(&0);
        let retry_error = stats_guard.get("retry_error").unwrap_or(&0);
        let use_cached = stats_guard.get("use_cached").unwrap_or(&0);
        let needs_extraction = stats_guard.get("needs_extraction").unwrap_or(&0);
        
        info!("Prescan results: {} PBOs total, {} skipped (permanent errors), {} skipped (errors), {} retrying, {} using cached, {} need extraction",
            pbo_paths.len(), skip_permanent, skip_error, retry_error, use_cached, needs_extraction);
        
        Ok(results)
    }
    
    /// Extract PBOs based on prescan results
    pub fn extract(&mut self, prescanned_pbos: &[PrescannedPbo]) -> Result<Vec<PathBuf>> {
        // Filter out PBOs that need extraction
        let to_extract: Vec<&PrescannedPbo> = prescanned_pbos.iter()
            .filter(|pbo| matches!(pbo.result, PrescanResult::NeedsExtraction))
            .collect();
        
        // Filter out PBOs that can use cached extraction
        let cached: Vec<&PrescannedPbo> = prescanned_pbos.iter()
            .filter(|pbo| matches!(pbo.result, PrescanResult::UseCached(_)))
            .collect();
        
        // Collect paths that will be available after extraction
        let mut extracted_paths = Vec::new();
        
        // Add cached paths - For cached PBOs, we'll add the parent directory
        for pbo in &cached {
            if let PrescanResult::UseCached(path) = &pbo.result {
                // Add base cache directory since files are no longer in PBO-specific directories
                if self.is_game_data {
                    extracted_paths.push(path.parent().unwrap_or(path).to_path_buf());
                } else {
                    // For missions, keep using the old approach
                    extracted_paths.push(path.clone());
                }
            }
        }
        
        // Get base cache directory using getter methods instead of accessing private fields
        let base_cache_dir = if self.is_game_data {
            self.cache_manager.get_game_data_cache_path(&PathBuf::from("dummy")).parent().unwrap_or(&PathBuf::from(".")).to_path_buf()
        } else {
            self.cache_manager.get_mission_cache_path(&PathBuf::from("dummy")).parent().unwrap_or(&PathBuf::from(".")).to_path_buf()
        };
        
        // If we're processing game data and have either cached or to-extract PBOs,
        // add the base directory to extracted_paths if not already there
        if self.is_game_data && (!cached.is_empty() || !to_extract.is_empty()) {
            if !extracted_paths.contains(&base_cache_dir) {
                extracted_paths.push(base_cache_dir.clone());
            }
        }
        
        // If nothing to extract, return early
        if to_extract.is_empty() {
            return Ok(extracted_paths);
        }
        
        info!("Extracting {} PBO files", to_extract.len());
        
        // Create progress bar
        let progress_bar = ProgressBar::new(to_extract.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("#>-")
        );
        progress_bar.set_message("Extracting PBOs");
        
        // Shared counter for statistics
        let stats = Arc::new(Mutex::new(HashMap::new()));
        
        // Process each PBO in parallel
        let results: Vec<ExtractionResult> = to_extract.par_iter()
            .map(|pbo| {
                let pbo_path = &pbo.path;
                let extensions = &pbo.extensions;
                let file_filter = pbo.file_filter.as_deref();
                
                // Determine the extraction directory
                let cache_path = if self.is_game_data {
                    // For game data, extract directly to base cache directory
                    base_cache_dir.clone()
                } else {
                    // For missions, use the original approach with PBO-specific directories
                    pbo.cache_path.clone()
                };
                
                // Create cache directory if it doesn't exist
                if !cache_path.exists() {
                    if let Err(e) = fs::create_dir_all(&cache_path) {
                        let error_msg = format!("Failed to create cache directory: {}", e);
                        error!("{} for {}", error_msg, pbo_path.display());
                        
                        // Update statistics
                        let mut stats_guard = stats.lock().unwrap();
                        *stats_guard.entry("failed".to_string()).or_insert(0) += 1;
                        
                        progress_bar.inc(1);
                        return ExtractionResult {
                            pbo_path: pbo_path.clone(),
                            cache_path: cache_path.clone(),
                            success: false,
                            error_message: Some(error_msg),
                            is_permanent_error: false,
                        };
                    }
                } else if self.has_pattern_changed(pbo_path, extensions, file_filter) && !self.is_game_data {
                    // For missions with changed patterns, clear the cache directory
                    debug!("Extraction patterns changed, clearing cache for: {}", pbo_path.display());
                    if let Err(e) = fs::remove_dir_all(&cache_path) {
                        let error_msg = format!("Failed to clear cache directory: {}", e);
                        error!("{} for {}", error_msg, pbo_path.display());
                        
                        // Update statistics
                        let mut stats_guard = stats.lock().unwrap();
                        *stats_guard.entry("failed".to_string()).or_insert(0) += 1;
                        
                        progress_bar.inc(1);
                        return ExtractionResult {
                            pbo_path: pbo_path.clone(),
                            cache_path: cache_path.clone(),
                            success: false,
                            error_message: Some(error_msg),
                            is_permanent_error: false,
                        };
                    }
                    
                    if let Err(e) = fs::create_dir_all(&cache_path) {
                        let error_msg = format!("Failed to recreate cache directory: {}", e);
                        error!("{} for {}", error_msg, pbo_path.display());
                        
                        // Update statistics
                        let mut stats_guard = stats.lock().unwrap();
                        *stats_guard.entry("failed".to_string()).or_insert(0) += 1;
                        
                        progress_bar.inc(1);
                        return ExtractionResult {
                            pbo_path: pbo_path.clone(),
                            cache_path: cache_path.clone(),
                            success: false,
                            error_message: Some(error_msg),
                            is_permanent_error: false,
                        };
                    }
                }
                
                // Extract PBO
                debug!("Extracting PBO: {} to {}", pbo_path.display(), cache_path.display());
                
                // Create extraction options
                let mut options = ExtractOptions::default();
                
                // Set file filter if specified
                if let Some(filter) = file_filter {
                    options.file_filter = Some(filter.to_string());
                }
                
                // Extract PBO
                match extract_pbo_with_options(pbo_path, &cache_path, options) {
                    Ok(_) => {
                        // Update statistics
                        let mut stats_guard = stats.lock().unwrap();
                        *stats_guard.entry("success".to_string()).or_insert(0) += 1;
                        
                        progress_bar.inc(1);
                        ExtractionResult {
                            pbo_path: pbo_path.clone(),
                            cache_path: cache_path.clone(),
                            success: true,
                            error_message: None,
                            is_permanent_error: false,
                        }
                    },
                    Err(e) => {
                        let error_msg = e.to_string();
                        
                        // Check if this is error code 11 (no files to extract)
                        if error_msg.contains("return code 11") || 
                           error_msg.contains("no file(s) to extract") {
                            debug!("No files to extract (error code 11), treating as success: {}", pbo_path.display());
                            
                            // Update statistics
                            let mut stats_guard = stats.lock().unwrap();
                            *stats_guard.entry("success_no_files".to_string()).or_insert(0) += 1;
                            
                            progress_bar.inc(1);
                            return ExtractionResult {
                                pbo_path: pbo_path.clone(),
                                cache_path: cache_path.clone(),
                                success: true,
                                error_message: None,
                                is_permanent_error: false,
                            };
                        }
                        
                        // Check if this is a bad SHA error
                        let is_bad_sha = self.cache_manager.is_bad_sha_error(&error_msg);
                        if is_bad_sha {
                            warn!("PBO has bad SHA, marking as permanent error: {}", pbo_path.display());
                        }
                        
                        error!("Failed to extract PBO {}: {}", pbo_path.display(), error_msg);
                        
                        // Update statistics
                        let mut stats_guard = stats.lock().unwrap();
                        if is_bad_sha {
                            *stats_guard.entry("failed_bad_sha".to_string()).or_insert(0) += 1;
                        } else {
                            *stats_guard.entry("failed".to_string()).or_insert(0) += 1;
                        }
                        
                        progress_bar.inc(1);
                        ExtractionResult {
                            pbo_path: pbo_path.clone(),
                            cache_path: cache_path.clone(),
                            success: false,
                            error_message: Some(error_msg),
                            is_permanent_error: is_bad_sha,
                        }
                    },
                }
            })
            .collect();
        
        // Finish progress bar
        progress_bar.finish_with_message("Extraction complete");
        
        // Log statistics
        let stats_guard = stats.lock().unwrap();
        let success = stats_guard.get("success").unwrap_or(&0);
        let success_no_files = stats_guard.get("success_no_files").unwrap_or(&0);
        let failed = stats_guard.get("failed").unwrap_or(&0);
        let failed_bad_sha = stats_guard.get("failed_bad_sha").unwrap_or(&0);
        
        info!("Extraction results: {} PBOs total, {} successful, {} successful (no files), {} failed, {} failed (bad SHA)",
            to_extract.len(), success, success_no_files, failed, failed_bad_sha);
        
        // Update cache for each result
        for result in &results {
            if result.success {
                // Update cache for successful extractions
                let pbo = to_extract.iter().find(|p| p.path == result.pbo_path).unwrap();
                if self.is_game_data {
                    if let Err(e) = self.cache_manager.update_game_data_cache(
                        &result.pbo_path,
                        &result.cache_path,
                        &pbo.extensions,
                        pbo.file_filter.as_deref()
                    ) {
                        warn!("Failed to update cache for {}: {}", result.pbo_path.display(), e);
                    }
                } else {
                    if let Err(e) = self.cache_manager.update_mission_cache(
                        &result.pbo_path,
                        &result.cache_path,
                        &pbo.extensions,
                        pbo.file_filter.as_deref()
                    ) {
                        warn!("Failed to update cache for {}: {}", result.pbo_path.display(), e);
                    }
                }
            } else {
                // Mark error in cache
                let pbo = to_extract.iter().find(|p| p.path == result.pbo_path).unwrap();
                let error_msg = result.error_message.as_deref().unwrap_or("Unknown error");
                
                if result.is_permanent_error {
                    // Mark as permanent error
                    if self.is_game_data {
                        if let Err(e) = self.cache_manager.mark_game_data_permanent_error(&result.pbo_path, error_msg) {
                            warn!("Failed to mark permanent error in cache for {}: {}", result.pbo_path.display(), e);
                        }
                    } else {
                        if let Err(e) = self.cache_manager.mark_mission_permanent_error(&result.pbo_path, error_msg) {
                            warn!("Failed to mark permanent error in cache for {}: {}", result.pbo_path.display(), e);
                        }
                    }
                } else {
                    // Mark as regular error
                    if self.is_game_data {
                        if let Err(e) = self.cache_manager.mark_game_data_error(
                            &result.pbo_path,
                            error_msg,
                            &pbo.extensions,
                            pbo.file_filter.as_deref()
                        ) {
                            warn!("Failed to mark error in cache for {}: {}", result.pbo_path.display(), e);
                        }
                    } else {
                        if let Err(e) = self.cache_manager.mark_mission_error(
                            &result.pbo_path,
                            error_msg,
                            &pbo.extensions,
                            pbo.file_filter.as_deref()
                        ) {
                            warn!("Failed to mark error in cache for {}: {}", result.pbo_path.display(), e);
                        }
                    }
                }
            }
        }
        
        // Save all cache updates in a single batch
        if let Err(e) = self.cache_manager.save_all() {
            warn!("Failed to save cache updates: {}", e);
        }
        
        Ok(extracted_paths)
    }
    
    // Helper methods that delegate to the appropriate cache manager methods
    fn has_permanent_error(&self, pbo_path: &Path) -> bool {
        if self.is_game_data {
            self.cache_manager.has_game_data_permanent_error(pbo_path)
        } else {
            self.cache_manager.has_mission_permanent_error(pbo_path)
        }
    }
    
    fn has_error(&self, pbo_path: &Path) -> bool {
        if self.is_game_data {
            self.cache_manager.has_game_data_error(pbo_path)
        } else {
            self.cache_manager.has_mission_error(pbo_path)
        }
    }
    
    fn get_error_message(&self, pbo_path: &Path) -> Option<String> {
        if self.is_game_data {
            self.cache_manager.get_game_data_error_message(pbo_path)
        } else {
            self.cache_manager.get_mission_error_message(pbo_path)
        }
    }
    
    fn is_cached(&self, pbo_path: &Path, cache_path: &Path) -> bool {
        if self.is_game_data {
            self.cache_manager.is_game_data_cached(pbo_path, cache_path)
        } else {
            self.cache_manager.is_mission_cached(pbo_path, cache_path)
        }
    }
    
    fn has_pattern_changed(&self, pbo_path: &Path, extensions: &[String], file_filter: Option<&str>) -> bool {
        if self.is_game_data {
            self.cache_manager.has_game_data_pattern_changed(pbo_path, extensions, file_filter)
        } else {
            self.cache_manager.has_mission_pattern_changed(pbo_path, extensions, file_filter)
        }
    }
} 