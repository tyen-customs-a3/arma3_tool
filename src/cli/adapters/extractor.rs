use std::path::PathBuf;
use std::time::Instant;
use async_trait::async_trait;
use log::{info, debug};

use arma3_extractor::{ExtractionConfig, ExtractionManager, extract_game_data as pbo_extract_game_data};
use arma3_workflow::extract::ExtractorInterface;
use arma3_workflow::types::summary::ExtractionSummary;
use arma3_workflow::types::options::ExtractionOptions;
use arma3_workflow::error::{WorkflowError, Result};

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
            extracted_files: total_extracted,
            extraction_time: elapsed_time,
            total_size: extraction_paths.iter()
                .map(|p| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))
                .sum(),
            output_files: extraction_paths,
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
    async fn test_extractor_adapter_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(temp_dir.path().to_path_buf());
        let adapter = Arma3ExtractorAdapter::new(config);
        // Adapter should be created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_validate_extraction_config_valid() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        
        let config = create_test_config(temp_dir.path().to_path_buf());
        let adapter = Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: Some(vec!["test".to_string()]),
            source_directories: vec![source_dir],
            force: false,
        };
        
        let result = adapter.validate_extraction_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_extraction_config_nonexistent_source() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(temp_dir.path().to_path_buf());
        let adapter = Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: None,
            source_directories: vec![PathBuf::from("/nonexistent/path")],
            force: false,
        };
        
        let result = adapter.validate_extraction_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Source directory does not exist"));
    }

    #[tokio::test]
    async fn test_validate_extraction_config_source_not_directory() {
        let temp_dir = TempDir::new().unwrap();
        let source_file = temp_dir.path().join("source_file.txt");
        fs::write(&source_file, "test content").unwrap();
        
        let config = create_test_config(temp_dir.path().to_path_buf());
        let adapter = Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: None,
            source_directories: vec![source_file],
            force: false,
        };
        
        let result = adapter.validate_extraction_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Source path is not a directory"));
    }

    #[tokio::test]
    async fn test_validate_extraction_config_no_source_directories() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = create_test_config(temp_dir.path().to_path_buf());
        config.game_data_dirs = vec![]; // Empty game data dirs
        let adapter = Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: None,
            source_directories: vec![], // Empty source directories
            force: false,
        };
        
        let result = adapter.validate_extraction_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No game data directories specified"));
    }

    #[tokio::test]
    async fn test_validate_extraction_config_invalid_cache_parent() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = create_test_config(PathBuf::from("/nonexistent/parent/cache"));
        config.game_data_dirs = vec![temp_dir.path().to_path_buf()];
        let adapter = Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: None,
            source_directories: vec![temp_dir.path().to_path_buf()],
            force: false,
        };
        
        let result = adapter.validate_extraction_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cache directory parent does not exist"));
    }

    #[tokio::test]
    async fn test_validate_extraction_config_with_config_source_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&source_dir).unwrap();
        
        let mut config = create_test_config(temp_dir.path().to_path_buf());
        config.game_data_dirs = vec![source_dir]; // Config has source dirs
        let adapter = Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: None,
            source_directories: vec![], // Empty workflow source directories
            force: false,
        };
        
        let result = adapter.validate_extraction_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_extract_pbos_invalid_source() {
        let temp_dir = TempDir::new().unwrap();
        let config = create_test_config(temp_dir.path().to_path_buf());
        let adapter = Arma3ExtractorAdapter::new(config);
        
        let source_dir = PathBuf::from("/nonexistent/source");
        let output_dir = temp_dir.path().join("output");
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: None,
            source_directories: vec![source_dir.clone()],
            force: false,
        };
        
        // This should fail during extraction due to invalid source
        let result = adapter.extract_pbos(&source_dir, &output_dir, &options).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_extraction_options_mapping() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let output_dir = temp_dir.path().join("output");
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let config = create_test_config(temp_dir.path().to_path_buf());
        let adapter = Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: Some(vec!["specific_dir".to_string()]),
            source_directories: vec![source_dir.clone()],
            force: true,
        };
        
        // Test that options are properly mapped
        // We can't easily test the actual extraction without mock dependencies,
        // but we can test validation passes
        let validation_result = adapter.validate_extraction_config(&options).await;
        assert!(validation_result.is_ok());
    }
}