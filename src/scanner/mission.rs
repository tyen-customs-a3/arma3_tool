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
    pub fn scan(&self, dirs: Option<Vec<String>>) -> Result<Vec<MissionResults>> {
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
    fn extract_pbos(&self, pbo_paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut extracted_paths = Vec::new();
        
        // Process each PBO in parallel
        let results: Vec<_> = pbo_paths.par_iter()
            .map(|pbo_path| {
                let cache_path = self.cache_manager.get_mission_cache_path(pbo_path);
                
                // Check if already cached
                if self.cache_manager.is_cached(pbo_path, &cache_path) {
                    debug!("Using cached extraction for {}", pbo_path.display());
                    return Ok(cache_path);
                }
                
                // Create cache directory if it doesn't exist
                if !cache_path.exists() {
                    fs::create_dir_all(&cache_path)
                        .map_err(|e| ToolError::CacheError(
                            format!("Failed to create cache directory: {}", e)
                        ))?;
                }
                
                // Extract PBO
                debug!("Extracting mission PBO: {} to {}", pbo_path.display(), cache_path.display());
                
                // Create extraction options
                let mut options = ExtractOptions::default();
                
                // Set file filter if extensions are specified
                if !self.config.mission_extensions.is_empty() {
                    let filter = self.config.mission_extensions.iter()
                        .map(|ext| format!("*.{}", ext))
                        .collect::<Vec<_>>()
                        .join(",");
                    options.file_filter = Some(filter);
                }
                
                // Extract PBO
                match extract_pbo_with_options(pbo_path, &cache_path, options) {
                    Ok(_) => {},
                    Err(e) => return Err(ToolError::ExtractionError(e.to_string())),
                }
                
                // Update cache
                self.cache_manager.update_cache(pbo_path, &cache_path)?;
                
                Ok(cache_path)
            })
            .collect();
        
        // Collect results
        for result in results {
            match result {
                Ok(path) => extracted_paths.push(path),
                Err(e) => error!("Failed to extract mission PBO: {}", e),
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