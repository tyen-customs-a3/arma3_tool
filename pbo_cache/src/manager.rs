use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use log::{info, warn};

use crate::models::{ExtractionConfig, PboMetadata, PboType};
use crate::scanner::PboScanner;
use crate::processor::PboProcessor;
use crate::index::IndexManager;
use crate::utils;
use crate::error::{Result, CacheError};

/// Manager for PBO extraction and caching
pub struct ExtractionManager {
    /// Configuration for the extraction process
    config: ExtractionConfig,
    
    /// The index manager for tracking cache state
    index_manager: IndexManager,
    
    /// The processor for extracting PBOs
    processor: PboProcessor,
}

/// Public functions for convenient extraction
pub async fn extract_game_data(config: ExtractionConfig) -> Result<Vec<PathBuf>> {
    let mut manager = ExtractionManager::new(config)?;
    manager.process_game_data(false).await
}

/// Extract a single mission PBO
pub async fn extract_mission(
    config: ExtractionConfig,
    mission_path: &Path
) -> Result<Vec<PathBuf>> {
    let mut manager = ExtractionManager::new(config)?;
    manager.process_mission(mission_path, false).await
}

impl ExtractionManager {
    /// Create a new extraction manager
    pub fn new(config: ExtractionConfig) -> Result<Self> {
        // Initialize directories
        utils::ensure_dir_exists(&config.cache_dir)
            .map_err(|e| CacheError::CacheDirectory(format!("Failed to create cache directory: {}", e)))?;
        utils::ensure_dir_exists(&config.game_data_cache_dir)
            .map_err(|e| CacheError::CacheDirectory(format!("Failed to create game data cache directory: {}", e)))?;
        utils::ensure_dir_exists(&config.mission_cache_dir)
            .map_err(|e| CacheError::CacheDirectory(format!("Failed to create mission cache directory: {}", e)))?;
        
        // Create index manager
        let index_manager = IndexManager::new(&config.cache_dir)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to create index manager: {}", e)))?;
        
        // Create processor
        let processor = PboProcessor::new(
            config.timeout,
            config.threads,
        );
        
        Ok(Self {
            config,
            index_manager,
            processor,
        })
    }
    
    /// Helper method to determine if a PBO needs extraction
    fn needs_extraction(&self, path: &Path, pbo_type: PboType, extensions: &[String]) -> Result<bool> {
        self.index_manager.needs_extraction(path, pbo_type, extensions)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to check extraction status: {}", e)))
    }
    
    /// Helper method to create metadata
    fn create_metadata(
        pbo_path: &Path, 
        extracted_files: &[PathBuf], 
        pbo_type: PboType, 
        extensions: &[String]
    ) -> Result<PboMetadata> {
        // Create metadata
        let mut metadata = PboMetadata::new(
            pbo_path.to_path_buf(),
            pbo_type,
            extensions.to_vec(),
        ).map_err(|e| CacheError::FileOperation(format!("Failed to create metadata: {}", e)))?;
        
        // Add extracted files
        metadata.extracted_files = extracted_files.to_vec();
        
        Ok(metadata)
    }
    
    /// Process game data PBOs
    pub async fn process_game_data(&mut self, skip_extraction: bool) -> Result<Vec<PathBuf>> {
        info!("Processing game data from {} directories", self.config.game_data_dirs.len());
        
        // Find all game data PBOs
        let all_pbos = PboScanner::find_pbos(&self.config.game_data_dirs)
            .map_err(|e| CacheError::FileOperation(format!("Failed to scan for PBOs: {}", e)))?;
        info!("Found {} game data PBOs", all_pbos.len());
        
        // Skip extraction if requested
        if skip_extraction {
            info!("Skipping extraction as requested");
            return Ok(Vec::new());
        }
        
        // Find PBOs that need extraction
        let needs_extraction = |path: &Path| -> Result<bool> {
            self.needs_extraction(path, PboType::GameData, &self.config.game_data_extensions)
        };
        
        let pbos_to_extract = PboScanner::filter_needs_extraction(
            &all_pbos,
            needs_extraction,
        ).map_err(|e| CacheError::FileOperation(format!("Failed to filter PBOs: {}", e)))?;
        
        if pbos_to_extract.is_empty() {
            info!("No game data PBOs need extraction");
            return Ok(Vec::new());
        }
        
        // Save cache_dir and extensions before the mutable borrow
        let cache_dir = self.config.game_data_cache_dir.clone();
        let extensions = self.config.game_data_extensions.clone();
        let verbose = self.config.verbose;
        
        // Extract PBOs
        let extraction_results = self.processor.extract_pbos(
            pbos_to_extract,
            &cache_dir,
            extensions.clone(),
            verbose,
            self.index_manager.get_index_mut(),
        ).await.map_err(|e| CacheError::PboExtraction {
            pbo_path: PathBuf::new(), // We don't know which PBO failed
            message: e.to_string(),
        })?;
        
        // Update cache index
        let mut all_extracted_files = Vec::new();
        
        for (pbo_path, extracted_files) in &extraction_results {
            // Create metadata
            let metadata = Self::create_metadata(
                pbo_path,
                extracted_files,
                PboType::GameData,
                &extensions,
            )?;
            
            // Update index
            self.index_manager.update_metadata(metadata.clone());
            
            // Add to result
            all_extracted_files.extend(metadata.extracted_files);
        }
        
        // Save the index
        self.index_manager.save()
            .map_err(|e| CacheError::IndexOperation(format!("Failed to save index: {}", e)))?;
        
        info!("Processed {} game data PBOs with {} extracted files", 
            extraction_results.len(), all_extracted_files.len());
            
        Ok(all_extracted_files)
    }
    
    /// Process a single mission PBO
    pub async fn process_mission(
        &mut self,
        mission_path: &Path,
        skip_extraction: bool
    ) -> Result<Vec<PathBuf>> {
        if !mission_path.exists() {
            return Err(CacheError::FileOperation(format!("Mission PBO not found: {}", mission_path.display())));
        }
        
        info!("Processing mission PBO: {}", mission_path.display());
        
        // Skip extraction if requested
        if skip_extraction {
            info!("Skipping extraction as requested");
            return Ok(Vec::new());
        }
        
        // Check if extraction is needed
        let needs_extraction = self.needs_extraction(
            mission_path,
            PboType::Mission,
            &self.config.mission_extensions,
        )?;
        
        if !needs_extraction {
            info!("Mission PBO already extracted and up to date");
            
            // Return the list of cached files
            if let Some(metadata) = self.index_manager.get_index().get_metadata(
                mission_path,
                PboType::Mission,
            ) {
                return Ok(metadata.extracted_files.clone());
            }
            
            return Ok(Vec::new());
        }
        
        // Save cache_dir and extensions before the mutable borrow
        let cache_dir = self.config.mission_cache_dir.clone();
        let extensions = self.config.mission_extensions.clone();
        let verbose = self.config.verbose;
        
        // Extract PBO
        let extraction_results = self.processor.extract_pbos(
            vec![mission_path.to_path_buf()],
            &cache_dir,
            extensions.clone(),
            verbose,
            self.index_manager.get_index_mut(),
        ).await.map_err(|e| CacheError::PboExtraction {
            pbo_path: mission_path.to_path_buf(),
            message: e.to_string(),
        })?;
        
        if extraction_results.is_empty() {
            return Ok(Vec::new());
        }
        
        let (pbo_path, extracted_files) = &extraction_results[0];
        
        // Create metadata
        let metadata = Self::create_metadata(
            pbo_path,
            extracted_files,
            PboType::Mission,
            &extensions,
        )?;
        
        // Update index
        self.index_manager.update_metadata(metadata.clone());
        
        // Save the index
        self.index_manager.save()
            .map_err(|e| CacheError::IndexOperation(format!("Failed to save index: {}", e)))?;
        
        info!("Extracted {} files from mission PBO", extracted_files.len());
        Ok(metadata.extracted_files)
    }
    
    /// Process all mission PBOs
    pub async fn process_all_missions(
        &mut self,
        skip_extraction: bool
    ) -> Result<HashMap<PathBuf, Vec<PathBuf>>> {
        info!("Processing missions from {} directories", self.config.mission_dirs.len());
        
        // Find all mission PBOs
        let all_missions = PboScanner::find_pbos(&self.config.mission_dirs)
            .map_err(|e| CacheError::FileOperation(format!("Failed to scan for PBOs: {}", e)))?;
        info!("Found {} mission PBOs", all_missions.len());
        
        // Skip extraction if requested
        if skip_extraction {
            info!("Skipping extraction as requested");
            return Ok(HashMap::new());
        }
        
        // Find PBOs that need extraction
        let needs_extraction = |path: &Path| -> Result<bool> {
            self.needs_extraction(path, PboType::Mission, &self.config.mission_extensions)
        };
        
        let pbos_to_extract = PboScanner::filter_needs_extraction(
            &all_missions,
            needs_extraction,
        ).map_err(|e| CacheError::FileOperation(format!("Failed to filter PBOs: {}", e)))?;
        
        if pbos_to_extract.is_empty() {
            info!("No mission PBOs need extraction");
            
            // Return already extracted missions
            let mut results = HashMap::new();
            for mission_path in all_missions {
                if let Some(metadata) = self.index_manager.get_index().get_metadata(
                    &mission_path,
                    PboType::Mission,
                ) {
                    results.insert(mission_path, metadata.extracted_files.clone());
                }
            }
            
            return Ok(results);
        }
        
        // Save cache_dir and extensions before the mutable borrow
        let cache_dir = self.config.mission_cache_dir.clone();
        let extensions = self.config.mission_extensions.clone();
        let verbose = self.config.verbose;
        
        // Extract PBOs in parallel
        let extraction_results = self.processor.extract_pbos(
            pbos_to_extract,
            &cache_dir,
            extensions.clone(),
            verbose,
            self.index_manager.get_index_mut(),
        ).await.map_err(|e| CacheError::PboExtraction {
            pbo_path: PathBuf::new(), // We don't know which PBO failed
            message: e.to_string(),
        })?;
        
        // Update cache index and build results
        let mut results = HashMap::new();
        
        // First add the newly extracted PBOs
        for (pbo_path, extracted_files) in &extraction_results {
            // Create metadata
            let metadata = Self::create_metadata(
                pbo_path,
                extracted_files,
                PboType::Mission,
                &extensions,
            )?;
            
            // Update index
            self.index_manager.update_metadata(metadata.clone());
            
            // Add to result
            results.insert(pbo_path.clone(), metadata.extracted_files);
        }
        
        // Then add any already extracted PBOs
        for mission_path in all_missions {
            if results.contains_key(&mission_path) {
                continue; // Already processed
            }
            
            if let Some(metadata) = self.index_manager.get_index().get_metadata(
                &mission_path,
                PboType::Mission,
            ) {
                results.insert(mission_path, metadata.extracted_files.clone());
            }
        }
        
        // Save the index
        self.index_manager.save()
            .map_err(|e| CacheError::IndexOperation(format!("Failed to save index: {}", e)))?;
        
        info!("Processed {} mission PBOs", results.len());
        Ok(results)
    }
    
    /// Cleanup the cache by removing files for PBOs that no longer exist
    /// and removing orphaned cache files
    pub fn cleanup_cache(&mut self) -> Result<()> {
        info!("Cleaning up cache");
        
        let mut removed_count = 0;
        let mut removed_files_count = 0;
        
        // Helper function for removing cache entries
        fn remove_nonexistent_entry(
            metadata: &PboMetadata, 
            pbo_type: PboType, 
            manager: &mut IndexManager,
            removed_count: &mut usize, 
            removed_files_count: &mut usize
        ) -> Result<()> {
            if !metadata.path.exists() {
                info!("Removing cache entry for non-existent PBO: {}", metadata.path.display());
                manager.get_index_mut().remove_metadata(
                    &metadata.path,
                    pbo_type,
                );
                
                // Remove cached files
                for cached_file in &metadata.extracted_files {
                    if cached_file.exists() {
                        if let Err(e) = fs::remove_file(cached_file) {
                            warn!("Failed to remove cached file {}: {}", cached_file.display(), e);
                        } else {
                            *removed_files_count += 1;
                        }
                    }
                }
                
                // For mission PBOs, try to remove empty directories
                if pbo_type == PboType::Mission {
                    if let Some(parent) = metadata.extracted_files.first().and_then(|p| p.parent()) {
                        if parent.exists() {
                            if let Ok(entries) = fs::read_dir(parent) {
                                if entries.count() == 0 {
                                    if let Err(e) = fs::remove_dir(parent) {
                                        warn!("Failed to remove empty mission cache directory {}: {}", parent.display(), e);
                                    }
                                }
                            }
                        }
                    }
                }
                
                *removed_count += 1;
            }
            
            Ok(())
        }
        
        // Check game data PBOs
        let game_data_index = self.index_manager.get_index().game_data.clone();
        for (_key, metadata) in game_data_index {
            remove_nonexistent_entry(
                &metadata, 
                PboType::GameData, 
                &mut self.index_manager,
                &mut removed_count, 
                &mut removed_files_count
            )?;
        }
        
        // Check mission PBOs
        let missions_index = self.index_manager.get_index().missions.clone();
        for (_key, metadata) in missions_index {
            remove_nonexistent_entry(
                &metadata, 
                PboType::Mission, 
                &mut self.index_manager,
                &mut removed_count, 
                &mut removed_files_count
            )?;
        }
        
        // Save the index if changes were made
        if removed_count > 0 {
            self.index_manager.save()
                .map_err(|e| CacheError::IndexOperation(format!("Failed to save index: {}", e)))?;
            info!("Removed {} cache entries and {} cached files for non-existent PBOs", 
                removed_count, removed_files_count);
        } else {
            info!("No cleanup needed, all cached PBOs still exist");
        }
        
        Ok(())
    }

    /// Update metadata in the index
    pub fn update_metadata(&mut self, metadata: PboMetadata) -> Result<()> {
        self.index_manager.update_metadata(metadata);
        Ok(())
    }

    /// Save the index to disk
    pub fn save_index(&mut self) -> Result<()> {
        self.index_manager.save()
            .map_err(|e| CacheError::IndexOperation(format!("Failed to save index: {}", e)))
    }
} 