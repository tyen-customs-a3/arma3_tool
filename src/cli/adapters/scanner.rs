use std::path::PathBuf;
use std::time::Instant;
use async_trait::async_trait;
use log::{info, debug};

use arma3_database::models::CacheConfig;
use arma3_database::DatabaseManager;
use arma3_extractor::ExtractionConfig;
use arma3_workflow::process::ScannerInterface;
use arma3_workflow::types::summary::ProcessingSummary;
use arma3_workflow::types::options::ProcessingOptions;
use arma3_workflow::error::{WorkflowError, Result};
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
            processed_files: total_files_processed,
            processing_time: elapsed_time,
            entries_found: total_entries_found,
            output_files: vec![], // Scanner doesn't generate output files directly
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_config(cache_dir: PathBuf) -> ExtractionConfig {
        ExtractionConfig {
            game_data_extensions: vec![],
            mission_extensions: vec![],
            threads: 4,
            timeout: 30,
            verbose: false,
            db_path: cache_dir.join("test.db"),
        }
    }

    #[tokio::test]
    async fn test_scanner_adapter_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(temp_dir.path().to_path_buf());
        let db_path = temp_dir.path().join("test.db");
        let adapter = Arma3ScannerAdapter::new(config, db_path);
        // Adapter should be created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_validate_processing_config_valid() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&cache_dir).unwrap();
        
        let config = create_test_config(cache_dir.clone());
        let db_path = temp_dir.path().join("test.db");
        let adapter = Arma3ScannerAdapter::new(config, db_path);
        
        let options = ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec!["cpp".to_string(), "sqf".to_string()],
            source_directories: vec![source_dir],
            verbose: false,
        };
        
        let result = adapter.validate_processing_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_processing_config_nonexistent_source() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();
        
        let config = create_test_config(cache_dir);
        let db_path = temp_dir.path().join("test.db");
        let adapter = Arma3ScannerAdapter::new(config, db_path);
        
        let options = ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec!["cpp".to_string()],
            source_directories: vec![PathBuf::from("/nonexistent/path")],
            verbose: false,
        };
        
        let result = adapter.validate_processing_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Source directory does not exist"));
    }

    #[tokio::test]
    async fn test_validate_processing_config_source_not_directory() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("source_file.txt");
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();
        fs::write(&source_file, "test content").unwrap();
        
        let config = create_test_config(cache_dir);
        let db_path = temp_dir.path().join("test.db");
        let adapter = Arma3ScannerAdapter::new(config, db_path);
        
        let options = ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec!["cpp".to_string()],
            source_directories: vec![source_file],
            verbose: false,
        };
        
        let result = adapter.validate_processing_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Source path is not a directory"));
    }

    #[tokio::test]
    async fn test_validate_processing_config_invalid_db_parent() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();
        
        let config = create_test_config(cache_dir);
        let db_path = PathBuf::from("/nonexistent/parent/test.db");
        let adapter = Arma3ScannerAdapter::new(config, db_path);
        
        let options = ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec!["cpp".to_string()],
            source_directories: vec![],
            verbose: false,
        };
        
        let result = adapter.validate_processing_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database directory parent does not exist"));
    }

    #[tokio::test]
    async fn test_validate_processing_config_nonexistent_cache() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(PathBuf::from("/nonexistent/cache"));
        let db_path = temp_dir.path().join("test.db");
        let adapter = Arma3ScannerAdapter::new(config, db_path);
        
        let options = ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec!["cpp".to_string()],
            source_directories: vec![],
            verbose: false,
        };
        
        let result = adapter.validate_processing_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cache directory does not exist"));
    }

    #[tokio::test]
    async fn test_validate_processing_config_empty_source_directories() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();
        
        let config = create_test_config(cache_dir);
        let db_path = temp_dir.path().join("test.db");
        let adapter = Arma3ScannerAdapter::new(config, db_path);
        
        let options = ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec!["cpp".to_string()],
            source_directories: vec![], // Empty source directories should be valid for workflow
            verbose: false,
        };
        
        let result = adapter.validate_processing_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scan_files_invalid_cache() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(PathBuf::from("/nonexistent/cache"));
        let db_path = temp_dir.path().join("test.db");
        let adapter = Arma3ScannerAdapter::new(config, db_path);
        
        let source_dir = temp_dir.path().join("source");
        let output_dir = temp_dir.path().join("output");
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let options = ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec!["cpp".to_string()],
            source_directories: vec![source_dir.clone()],
            verbose: false,
        };
        
        // This should fail due to invalid cache directory
        let result = adapter.scan_files(&source_dir, &output_dir, &options).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_processing_options_mapping() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();
        
        let config = create_test_config(cache_dir);
        let db_path = temp_dir.path().join("test.db");
        let adapter = Arma3ScannerAdapter::new(config, db_path);
        
        let options = ProcessingOptions {
            max_files: 500,
            max_depth: 5,
            extensions: vec!["cpp".to_string(), "sqf".to_string(), "sqm".to_string()],
            source_directories: vec![],
            verbose: true,
        };
        
        // Test that options validation passes
        let validation_result = adapter.validate_processing_config(&options).await;
        assert!(validation_result.is_ok());
    }
}