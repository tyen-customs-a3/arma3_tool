use std::path::PathBuf;
use std::time::Instant;
use async_trait::async_trait;
use log::{info, debug};

use arma3_database::models::CacheConfig;
use arma3_database::DatabaseManager;
use arma3_extractor::ExtractionConfig;
use arma3_workflow::{ScannerInterface, ProcessingOptions, ProcessingSummary, WorkflowError, Result};
use crate::scanner::gamedata::GameDataScanner;
use crate::scanner::mission::MissionScanner;

/// Adapter that bridges the legacy scanner system with the new workflow system
pub struct Arma3ScannerAdapter {
    config: ExtractionConfig,
    db_path: PathBuf,
}

impl Arma3ScannerAdapter {
    /// Create a new scanner adapter with the given configuration and database path
    pub fn new(config: ExtractionConfig, db_path: PathBuf) -> Self {
        Self { config, db_path }
    }
}

#[async_trait]
impl ScannerInterface for Arma3ScannerAdapter {
    async fn scan_files(
        &self,
        source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ProcessingOptions,
    ) -> Result<ProcessingSummary> {
        let start_time = Instant::now();
        
        debug!("Starting file scanning from {:?} to {:?}", source_dir, output_dir);
        
        // Create database manager
        let db_config = CacheConfig::with_cache_dirs(
            self.db_path.clone(),
            self.config.cache_dir.clone(),
            self.config.game_data_cache_dir.clone(),
            self.config.mission_cache_dir.clone(),
        );
        let db_manager = DatabaseManager::with_config(db_config)
            .map_err(|e| WorkflowError::processing_error(format!("Failed to create database manager: {}", e)))?;

        let mut total_files_processed = 0;
        let mut total_entries_found = 0;
        let mut total_pbos_processed = 0;
        let mut errors = Vec::new();

        // Process extracted game data
        info!("Processing game data files from {}...", self.config.game_data_cache_dir.display());
        match GameDataScanner::with_database(self.config.clone(), db_manager.clone()) {
            Ok(scanner) => {
                match scanner.scan().await {
                    Ok(game_data) => {
                        total_entries_found += game_data.classes.len();
                        info!("Processed {} game data classes", game_data.classes.len());
                        
                        // Save game data to database
                        if let Err(e) = scanner.save_to_database(&game_data) {
                            let error_msg = format!("Failed to save game data to database: {}", e);
                            errors.push(error_msg.clone());
                            log::warn!("{}", error_msg);
                        } else {
                            info!("Saved game data to database");
                        }
                        
                        // Estimate processed files based on typical PBO structure
                        total_files_processed += game_data.classes.len() / 10; // Rough estimate
                        total_pbos_processed += 1; // Treat game data as one logical unit
                    }
                    Err(e) => {
                        let error_msg = format!("Game data scanning failed: {}", e);
                        errors.push(error_msg.clone());
                        log::warn!("{}", error_msg);
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to create game data scanner: {}", e);
                errors.push(error_msg.clone());
                log::warn!("{}", error_msg);
            }
        }

        // Process extracted missions
        info!("Processing mission files from {}...", self.config.mission_cache_dir.display());
        match MissionScanner::with_database(self.config.clone(), db_manager) {
            Ok(scanner) => {
                match scanner.scan().await {
                    Ok(mission_data) => {
                        total_entries_found += mission_data.missions.len();
                        info!("Processed {} missions", mission_data.missions.len());
                        
                        // Estimate processed files based on missions
                        total_files_processed += mission_data.missions.len() * 5; // Rough estimate of files per mission
                        total_pbos_processed += mission_data.missions.len();
                    }
                    Err(e) => {
                        let error_msg = format!("Mission scanning failed: {}", e);
                        errors.push(error_msg.clone());
                        log::warn!("{}", error_msg);
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("Failed to create mission scanner: {}", e);
                errors.push(error_msg.clone());
                log::warn!("{}", error_msg);
            }
        }

        let elapsed_time = start_time.elapsed();
        
        info!("File scanning completed in {:?}: {} files processed, {} entries found", 
              elapsed_time, total_files_processed, total_entries_found);

        // If we have critical errors and no data was processed, fail the operation
        if total_entries_found == 0 && !errors.is_empty() {
            return Err(WorkflowError::processing_error(
                format!("Scanning failed with no data processed. Errors: {}", errors.join("; "))
            ));
        }

        Ok(ProcessingSummary {
            processed_pbos: total_pbos_processed,
            files_processed: total_files_processed,
            entries_found: total_entries_found,
            elapsed_time,
            errors,
        })
    }
    
    async fn validate_processing_config(&self, options: &ProcessingOptions) -> Result<()> {
        debug!("Validating processing configuration");
        
        // Validate source directories if they are specified
        // Note: For workflow integration, source dirs might be empty if this follows extraction
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
        
        // Validate database path can be created
        if let Some(parent) = self.db_path.parent() {
            if !parent.exists() {
                return Err(WorkflowError::validation_error(
                    format!("Database directory parent does not exist: {:?}", parent)
                ));
            }
        }
        
        // Validate cache directories exist (they should have been created by extraction)
        if !self.config.cache_dir.exists() {
            return Err(WorkflowError::validation_error(
                format!("Cache directory does not exist: {:?}", self.config.cache_dir)
            ));
        }
        
        if !self.config.game_data_cache_dir.exists() {
            log::warn!("Game data cache directory does not exist: {:?}", self.config.game_data_cache_dir);
        }
        
        if !self.config.mission_cache_dir.exists() {
            log::warn!("Mission cache directory does not exist: {:?}", self.config.mission_cache_dir);
        }
        
        debug!("Processing configuration validation passed");
        Ok(())
    }
}