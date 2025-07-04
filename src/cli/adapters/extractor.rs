use std::path::PathBuf;
use std::time::Instant;
use async_trait::async_trait;
use log::{info, debug};

use arma3_extractor::{ExtractionConfig, ExtractionManager, extract_game_data as pbo_extract_game_data};
use arma3_workflow::{ExtractorInterface, ExtractionOptions, ExtractionSummary, WorkflowError, Result};

/// Adapter that bridges the legacy arma3_extractor with the new workflow system
pub struct Arma3ExtractorAdapter {
    config: ExtractionConfig,
}

impl Arma3ExtractorAdapter {
    /// Create a new extractor adapter with the given configuration
    pub fn new(config: ExtractionConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ExtractorInterface for Arma3ExtractorAdapter {
    async fn extract_pbos(
        &self,
        source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ExtractionOptions,
    ) -> Result<ExtractionSummary> {
        let start_time = Instant::now();
        
        debug!("Starting PBO extraction from {:?} to {:?}", source_dir, output_dir);
        
        // Create a modified config that uses the workflow's source and output directories
        let mut extraction_config = self.config.clone();
        
        // Map workflow source directories to our config
        if !options.source_directories.is_empty() {
            // Use the workflow's source directories for game data extraction
            extraction_config.game_data_dirs = options.source_directories.clone();
        }
        
        // Ensure output directory is set correctly
        extraction_config.game_data_cache_dir = output_dir.clone();
        extraction_config.mission_cache_dir = output_dir.join("missions");
        extraction_config.cache_dir = output_dir.parent()
            .unwrap_or(output_dir)
            .to_path_buf();
        
        let mut total_extracted = 0;
        let mut extraction_paths = Vec::new();
        let mut errors = Vec::new();
        
        // Extract game data
        info!("Extracting game data...");
        match pbo_extract_game_data(extraction_config.clone()).await {
            Ok(paths) => {
                total_extracted += paths.len();
                extraction_paths.extend(paths);
                info!("Game data extraction complete: {} paths processed", extraction_paths.len());
            }
            Err(e) => {
                let error_msg = format!("Game data extraction failed: {}", e);
                errors.push(error_msg.clone());
                return Err(WorkflowError::extraction_error(error_msg));
            }
        }
        
        // Extract missions using the extraction manager
        info!("Extracting missions...");
        let mut manager = ExtractionManager::new(extraction_config)
            .map_err(|e| WorkflowError::extraction_error(format!("Failed to create extraction manager: {}", e)))?;
        
        match manager.process_all_missions(options.force).await {
            Ok(mission_results) => {
                let mission_count = mission_results.len();
                let mission_files: usize = mission_results.values().map(|files| files.len()).sum();
                total_extracted += mission_files;
                
                // Add mission paths to extraction paths
                for files in mission_results.values() {
                    extraction_paths.extend(files.clone());
                }
                
                info!("Mission extraction complete: {} missions processed with {} total files", 
                      mission_count, mission_files);
            }
            Err(e) => {
                let error_msg = format!("Mission extraction failed: {}", e);
                errors.push(error_msg.clone());
                // Don't fail completely for mission extraction errors, just log them
                log::warn!("{}", error_msg);
            }
        }
        
        let elapsed_time = start_time.elapsed();
        
        info!("PBO extraction completed in {:?}: {} total files extracted", 
              elapsed_time, total_extracted);
        
        Ok(ExtractionSummary {
            extracted_pbos: total_extracted,
            extraction_paths,
            elapsed_time,
            errors,
        })
    }
    
    async fn validate_extraction_config(&self, options: &ExtractionOptions) -> Result<()> {
        debug!("Validating extraction configuration");
        
        // Validate source directories exist
        for source_dir in &options.source_directories {
            if !source_dir.exists() {
                return Err(WorkflowError::validation_error(
                    format!("Source directory does not exist: {:?}", source_dir)
                ));
            }
            
            if !source_dir.is_dir() {
                return Err(WorkflowError::validation_error(
                    format!("Source path is not a directory: {:?}", source_dir)
                ));
            }
        }
        
        // Validate the underlying extraction config
        // Check that essential directories are set
        if self.config.game_data_dirs.is_empty() && options.source_directories.is_empty() {
            return Err(WorkflowError::validation_error(
                "No game data directories specified in config or workflow options"
            ));
        }
        
        // Validate cache directories can be created
        let cache_dir = &self.config.cache_dir;
        if let Some(parent) = cache_dir.parent() {
            if !parent.exists() {
                return Err(WorkflowError::validation_error(
                    format!("Cache directory parent does not exist: {:?}", parent)
                ));
            }
        }
        
        debug!("Extraction configuration validation passed");
        Ok(())
    }
}