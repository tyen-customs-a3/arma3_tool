use std::path::{Path, PathBuf};
use std::collections::HashMap;
use log::info;

use crate::models::{ExtractionConfig, PboMetadata, PboType};
use crate::scanner::PboScanner;
use crate::processor::PboProcessor;
use crate::db_manager::DbManager;
use crate::utils;
use crate::error::{Result, CacheError};

use arma3_database::models::pbo::{PboModel, ExtractedFile};

/// Manager for PBO extraction and caching
pub struct ExtractionManager {
    /// Configuration for the extraction process
    config: ExtractionConfig,
    
    /// The database manager for tracking cache state
    db_manager: DbManager,
    
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
        
        // Create database manager
        let db_manager = DbManager::new(
            &config.cache_dir,
            &config.game_data_cache_dir,
            &config.mission_cache_dir,
            &config.db_path,
        ).map_err(|e| CacheError::IndexOperation(format!("Failed to create database manager: {}", e)))?;
        
        // Create processor
        let processor = PboProcessor::new(
            config.timeout,
            config.threads,
        );
        
        Ok(Self {
            config,
            db_manager,
            processor,
        })
    }
    
    /// Get a reference to the database manager
    pub fn get_db_manager(&self) -> &DbManager {
        &self.db_manager
    }
    
    /// Helper method to determine if a PBO needs extraction
    fn needs_extraction(&self, path: &Path, pbo_type: PboType, extensions: &[String]) -> Result<bool> {
        self.db_manager.needs_extraction(path, pbo_type, extensions)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to check extraction status: {}", e)))
    }
    
    /// Helper method to create metadata
    fn create_metadata(
        pbo_path: &Path, 
        extracted_files: &[PathBuf], 
        pbo_type: PboType, 
        extensions: &[String],
        base_dir: Option<&Path>,
    ) -> Result<PboMetadata> {
        // Create metadata with base directory if provided
        let mut metadata = if let Some(base_dir) = base_dir {
            // Create relative path if possible
            let rel_path = if pbo_path.starts_with(base_dir) {
                match pbo_path.strip_prefix(base_dir) {
                    Ok(rel) => rel.to_path_buf(),
                    Err(_) => pbo_path.to_path_buf(),
                }
            } else {
                pbo_path.to_path_buf()
            };
            
            PboMetadata::new_with_base_dir(
                rel_path,
                base_dir.to_path_buf(),
                pbo_type,
                extensions.to_vec(),
            )
        } else {
            PboMetadata::new(
                pbo_path.to_path_buf(),
                pbo_type,
                extensions.to_vec(),
            )
        }.map_err(|e| CacheError::FileOperation(format!("Failed to create metadata: {}", e)))?;
        
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
            &mut |path, error| self.db_manager.add_failed_extraction(path, error),
        ).await.map_err(|e| CacheError::PboExtraction {
            pbo_path: PathBuf::new(), // We don't know which PBO failed
            message: e.to_string(),
        })?;
        
        // Update cache index
        let mut all_extracted_files = Vec::new();
        
        for (pbo_path, extracted_files) in &extraction_results {
            // Find the best base directory for this PBO
            let base_dir = self.find_best_base_dir(pbo_path);
                
            // Create metadata with relative path if possible
            let metadata = Self::create_metadata(
                pbo_path,
                extracted_files,
                PboType::GameData,
                &extensions,
                base_dir.as_deref(),
            )?;
            
            // Update database
            self.db_manager.update_metadata(metadata.clone())?;
            
            // Add to result (full paths)
            all_extracted_files.extend(metadata.get_full_extracted_paths(&cache_dir));
        }
        
        info!("Processed {} game data PBOs with {} extracted files", 
            extraction_results.len(), all_extracted_files.len());
            
        Ok(all_extracted_files)
    }
    
    /// Find the best base directory for a PBO path from the configured dirs
    fn find_best_base_dir(&self, pbo_path: &Path) -> Option<PathBuf> {
        self.config.game_data_dirs.iter()
            .filter(|dir| pbo_path.starts_with(dir))
            .max_by_key(|dir| dir.as_os_str().len())
            .cloned()
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
        let needs_extraction = self.needs_extraction(mission_path, PboType::Mission, &self.config.mission_extensions)?;
        
        if !needs_extraction {
            info!("Mission PBO already extracted and up to date");
            
            // Get the normalized path to query the database
            let pbo_id = mission_path.to_string_lossy().to_string().replace('\\', "/");
            
            // Get extracted files from the database
            let extraction_result = self.db_manager.pbo_repo.get_extracted_files(&pbo_id)
                .map_err(|e| CacheError::IndexOperation(format!("Failed to get extracted files: {}", e)))?;
            
            if !extraction_result.is_empty() {
                // Return the full paths
                return Ok(extraction_result.iter()
                    .map(|file| file.get_full_path(&self.config.mission_cache_dir))
                    .collect());
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
            &mut |path, error| self.db_manager.add_failed_extraction(path, error),
        ).await.map_err(|e| CacheError::PboExtraction {
            pbo_path: mission_path.to_path_buf(),
            message: e.to_string(),
        })?;
        
        if extraction_results.is_empty() {
            return Ok(Vec::new());
        }
        
        let (pbo_path, extracted_files) = &extraction_results[0];
        
        // Find the best base directory for this PBO
        let base_dir = self.find_best_mission_base_dir(pbo_path);
            
        // Create metadata with relative path if possible
        let metadata = Self::create_metadata(
            pbo_path,
            extracted_files,
            PboType::Mission,
            &extensions,
            base_dir.as_deref(),
        )?;
        
        // Update database
        self.db_manager.update_metadata(metadata.clone())?;
        
        info!("Extracted {} files from mission PBO", extracted_files.len());
        
        // Return the full paths
        Ok(metadata.get_full_extracted_paths(&cache_dir))
    }
    
    /// Find the best base directory for a mission PBO
    fn find_best_mission_base_dir(&self, pbo_path: &Path) -> Option<PathBuf> {
        self.config.mission_dirs.iter()
            .filter(|dir| pbo_path.starts_with(dir))
            .max_by_key(|dir| dir.as_os_str().len())
            .cloned()
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
        
        // Save mission cache dir for later use
        let cache_dir = self.config.mission_cache_dir.clone();
        
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
            
            // Get all mission metadata from the database
            let mission_models = self.db_manager.get_mission_metadata()
                .map_err(|e| CacheError::IndexOperation(format!("Failed to get mission metadata: {}", e)))?;
            
            for model in mission_models {
                // Get the extracted files for this PBO
                let extracted_files = self.db_manager.pbo_repo.get_extracted_files(&model.id)
                    .map_err(|e| CacheError::IndexOperation(format!("Failed to get extracted files for {}: {}", model.id, e)))?;
                
                if !extracted_files.is_empty() {
                    // Convert from database model to full paths
                    let full_paths: Vec<PathBuf> = extracted_files.iter()
                        .map(|file| file.get_full_path(&cache_dir))
                        .collect();
                    
                    results.insert(model.full_path, full_paths);
                }
            }
            
            return Ok(results);
        }
        
        // Save extensions and verbose settings before mutable borrow
        let extensions = self.config.mission_extensions.clone();
        let verbose = self.config.verbose;
        
        // Extract PBOs in parallel
        let extraction_results = self.processor.extract_pbos(
            pbos_to_extract,
            &cache_dir,
            extensions.clone(),
            verbose,
            &mut |path, error| self.db_manager.add_failed_extraction(path, error),
        ).await.map_err(|e| CacheError::PboExtraction {
            pbo_path: PathBuf::new(), // We don't know which PBO failed
            message: e.to_string(),
        })?;
        
        // Update cache index and build results
        let mut results = HashMap::new();
        
        // First add the newly extracted PBOs
        for (pbo_path, extracted_files) in &extraction_results {
            // Find the best base directory for this PBO
            let base_dir = self.find_best_mission_base_dir(pbo_path);
                
            // Create metadata with relative path if possible
            let metadata = Self::create_metadata(
                pbo_path,
                extracted_files,
                PboType::Mission,
                &extensions,
                base_dir.as_deref(),
            )?;
            
            // Update database
            self.db_manager.update_metadata(metadata.clone())?;
            
            // Add to result (full paths)
            results.insert(pbo_path.clone(), metadata.get_full_extracted_paths(&cache_dir));
        }
        
        // Then add any already extracted PBOs
        let mission_models = self.db_manager.get_mission_metadata()
            .map_err(|e| CacheError::IndexOperation(format!("Failed to get mission metadata: {}", e)))?;
        
        for model in mission_models {
            let path_buf = model.full_path.clone();
            
            if results.contains_key(&path_buf) {
                continue; // Already processed
            }
            
            // Get the extracted files for this PBO
            let extracted_files = self.db_manager.pbo_repo.get_extracted_files(&model.id)
                .map_err(|e| CacheError::IndexOperation(format!("Failed to get extracted files for {}: {}", model.id, e)))?;
            
            if !extracted_files.is_empty() {
                // Convert from database model to full paths
                let full_paths: Vec<PathBuf> = extracted_files.iter()
                    .map(|file| file.get_full_path(&cache_dir))
                    .collect();
                
                results.insert(path_buf, full_paths);
            }
        }
        
        info!("Processed {} mission PBOs", results.len());
        Ok(results)
    }

    /// Update metadata in the database
    pub fn update_metadata(&mut self, metadata: PboMetadata) -> Result<()> {
        self.db_manager.update_metadata(metadata)
    }

    /// Find the source PBO for a file path
    pub fn find_pbo_for_file(&self, file_path: &Path) -> Result<Option<PboModel>> {
        self.db_manager.find_pbo_for_file(file_path)
    }
    
    /// Find all files with a specific extension
    pub fn find_files_by_extension(&self, extension: &str) -> Result<Vec<ExtractedFile>> {
        self.db_manager.find_files_by_extension(extension)
    }
} 