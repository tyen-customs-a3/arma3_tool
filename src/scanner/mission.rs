use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use rayon::prelude::*;
use log::{debug, info, warn, error};
use crate::config::ToolConfig;
use crate::cache::CacheManager;
use crate::error::{Result, ToolError};
use extraction::extract_pbo_with_options;
use pbo_tools::extract::ExtractOptions;
use mission_scanner::{scan_mission, MissionScannerConfig, MissionResults};
use tokio::runtime::Runtime;

/// Scanner for mission PBOs
pub struct MissionScanner {
    /// Tool configuration
    config: ToolConfig,
    
    /// Cache manager
    cache_manager: CacheManager,
}

impl MissionScanner {
    /// Create a new mission scanner
    pub fn new(config: ToolConfig, cache_manager: CacheManager) -> Self {
        Self {
            config,
            cache_manager,
        }
    }
    
    /// Scan missions
    pub fn scan(&mut self, dirs: Option<Vec<String>>) -> Result<Vec<MissionResults>> {
        // Discover PBOs
        let pbo_paths = self.discover_pbos(dirs)?;
        info!("Found {} mission PBO files", pbo_paths.len());
        
        // Extract PBOs
        let extracted_paths = self.extract_pbos(&pbo_paths)?;
        info!("Extracted {} mission PBO files", extracted_paths.len());
        
        // Scan missions
        let mission_results = self.scan_missions(&extracted_paths)?;
        info!("Scanned {} missions", mission_results.len());
        
        Ok(mission_results)
    }
    
    /// Discover mission PBO files
    fn discover_pbos(&self, dirs: Option<Vec<String>>) -> Result<Vec<PathBuf>> {
        let mut pbo_paths = Vec::new();
        
        // Use specified directories or all configured directories
        let directories = match dirs {
            Some(dir_strings) => dir_strings.iter()
                .map(|s| PathBuf::from(s))
                .collect::<Vec<_>>(),
            None => self.config.mission_dirs.clone(),
        };
        
        // Walk through each directory and find PBO files
        for dir in directories {
            debug!("Scanning directory for mission PBOs: {}", dir.display());
            
            for entry in WalkDir::new(&dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                
                if path.is_file() && path.extension().map_or(false, |ext| ext == "pbo") {
                    debug!("Found mission PBO: {}", path.display());
                    pbo_paths.push(path.to_path_buf());
                }
            }
        }
        
        Ok(pbo_paths)
    }
    
    /// Extract mission PBO files to cache
    fn extract_pbos(&mut self, pbo_paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut extracted_paths = Vec::new();
        
        // Define a custom result type for the parallel extraction
        type ExtractionResult = std::result::Result<(PathBuf, Vec<String>, Option<String>), (String, Vec<String>, Option<String>)>;
        
        // Process each PBO in parallel
        let results: Vec<ExtractionResult> = pbo_paths.par_iter()
            .map(|pbo_path| {
                let cache_path = self.cache_manager.get_mission_cache_path(pbo_path);
                
                // Create file filter if extensions are specified
                let extensions = self.config.mission_extensions.clone();
                let file_filter = if !extensions.is_empty() {
                    let filter = extensions.iter()
                        .map(|ext| format!("*.{}", ext))
                        .collect::<Vec<_>>()
                        .join(",");
                    Some(filter)
                } else {
                    None
                };
                
                // Check if PBO has a permanent error (like bad SHA)
                if self.cache_manager.has_mission_permanent_error(pbo_path) {
                    debug!("Skipping mission PBO with permanent error: {}", pbo_path.display());
                    return Err(("Skipping mission PBO with permanent error".to_string(), extensions, file_filter));
                }
                
                // Check if PBO has previously had an error but patterns have changed
                if self.cache_manager.has_mission_error(pbo_path) && 
                   self.cache_manager.has_mission_pattern_changed(pbo_path, &extensions, file_filter.as_deref()) {
                    debug!("Extraction patterns changed for mission PBO with previous error, retrying: {}", pbo_path.display());
                    // Continue with extraction since patterns changed
                } else if self.cache_manager.has_mission_error(pbo_path) {
                    // Skip if it had an error and patterns haven't changed
                    let error_msg = self.cache_manager.get_mission_error_message(pbo_path)
                        .unwrap_or_else(|| "Unknown error".to_string());
                    warn!("Skipping mission PBO that previously had an error: {} - {}", pbo_path.display(), error_msg);
                    return Err((format!("Skipping mission PBO that previously had an error: {}", error_msg), extensions, file_filter));
                }
                
                // Check if already cached and patterns haven't changed
                if self.cache_manager.is_mission_cached(pbo_path, &cache_path) && 
                   !self.cache_manager.has_mission_pattern_changed(pbo_path, &extensions, file_filter.as_deref()) {
                    debug!("Using cached extraction for {}", pbo_path.display());
                    return Ok((cache_path, extensions, file_filter));
                }
                
                // Create cache directory if it doesn't exist
                if !cache_path.exists() {
                    fs::create_dir_all(&cache_path)
                        .map_err(|e| (format!("Failed to create cache directory: {}", e), extensions.clone(), file_filter.clone()))?;
                } else if self.cache_manager.has_mission_pattern_changed(pbo_path, &extensions, file_filter.as_deref()) {
                    // If patterns changed, clear the cache directory and recreate it
                    debug!("Extraction patterns changed, clearing cache for: {}", pbo_path.display());
                    fs::remove_dir_all(&cache_path)
                        .map_err(|e| (format!("Failed to clear cache directory: {}", e), extensions.clone(), file_filter.clone()))?;
                    fs::create_dir_all(&cache_path)
                        .map_err(|e| (format!("Failed to recreate cache directory: {}", e), extensions.clone(), file_filter.clone()))?;
                }
                
                // Extract PBO
                debug!("Extracting mission PBO: {} to {}", pbo_path.display(), cache_path.display());
                
                // Create extraction options
                let mut options = ExtractOptions::default();
                
                // Set file filter if extensions are specified
                if let Some(filter) = &file_filter {
                    options.file_filter = Some(filter.clone());
                }
                
                // Extract PBO
                match extract_pbo_with_options(pbo_path, &cache_path, options) {
                    Ok(_) => {
                        // Note: We can't update the cache here because we're in a parallel context
                        // and cache_manager is borrowed immutably. We'll handle this after collecting results.
                        Ok((cache_path, extensions, file_filter))
                    },
                    Err(e) => {
                        let error_msg = e.to_string();
                        
                        // Check if this is error code 11 (no files to extract)
                        if error_msg.contains("return code 11") || 
                           error_msg.contains("no file(s) to extract") {
                            debug!("No files to extract (error code 11), treating as success: {}", pbo_path.display());
                            return Ok((cache_path, extensions, file_filter));
                        }
                        
                        error!("Failed to extract mission PBO {}: {}", pbo_path.display(), error_msg);
                        Err((error_msg, extensions, file_filter))
                    },
                }
            })
            .collect();
        
        // Process results and update cache
        for (i, result) in results.into_iter().enumerate() {
            if let Some(pbo_path) = pbo_paths.get(i) {
                match result {
                    Ok((path, extensions, file_filter)) => {
                        // Update cache for successful extractions
                        if let Err(e) = self.cache_manager.update_mission_cache(
                            pbo_path, 
                            &path, 
                            &extensions, 
                            file_filter.as_deref()
                        ) {
                            warn!("Failed to update cache for {}: {}", pbo_path.display(), e);
                        }
                        extracted_paths.push(path);
                    },
                    Err((error_msg, extensions, file_filter)) => {
                        // Check if this is a bad SHA error
                        if self.cache_manager.is_bad_sha_error(&error_msg) {
                            warn!("Mission PBO has bad SHA, marking as permanent error: {}", pbo_path.display());
                            if let Err(e) = self.cache_manager.mark_mission_permanent_error(pbo_path, &error_msg) {
                                warn!("Failed to mark permanent error in cache for {}: {}", pbo_path.display(), e);
                            }
                        } else {
                            // Mark regular error in cache
                            if let Err(cache_err) = self.cache_manager.mark_mission_error(
                                pbo_path, 
                                &error_msg, 
                                &extensions, 
                                file_filter.as_deref()
                            ) {
                                warn!("Failed to mark error in cache for {}: {}", pbo_path.display(), cache_err);
                            }
                        }
                        error!("Failed to extract mission PBO: {}", error_msg);
                    },
                }
            }
        }
        
        Ok(extracted_paths)
    }
    
    /// Scan extracted missions
    fn scan_missions(&self, extracted_paths: &[PathBuf]) -> Result<Vec<MissionResults>> {
        let mut mission_results = Vec::new();
        
        // Create mission scanner config
        let scanner_config = MissionScannerConfig {
            max_threads: self.config.threads,
            file_extensions: self.config.mission_extensions.clone(),
        };
        
        // Create a Tokio runtime for async operations
        let runtime = Runtime::new()
            .map_err(|e| ToolError::MissionScanError(format!("Failed to create Tokio runtime: {}", e)))?;
        
        // Process each extracted mission
        for path in extracted_paths {
            debug!("Scanning mission: {}", path.display());
            
            // Use the runtime to block on the async scan_mission function
            let result = runtime.block_on(scan_mission(path, self.config.threads, &scanner_config));
            
            match result {
                Ok(result) => {
                    info!("Scanned mission: {} with {} dependencies", 
                        result.mission_name, 
                        result.class_dependencies.len()
                    );
                    mission_results.push(result);
                },
                Err(e) => {
                    error!("Failed to scan mission {}: {}", path.display(), e);
                }
            }
        }
        
        Ok(mission_results)
    }
} 