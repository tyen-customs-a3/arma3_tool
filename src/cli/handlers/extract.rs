use anyhow::Result;
use arma3_extract::ExtractionConfig;
use arma3_workflow::{
    WorkflowOrchestrator, ExtractWorkflowHandler, ExtractorInterface,
    Workflow, WorkflowType, ContentType, WorkflowOptions, ExtractionOptions,
    ExtractionSummary, WorkflowError
};
use arma3_extract::{extract_game_data, ExtractionConfig as ExtractorExtractionConfig};
use async_trait::async_trait;
use log::info;
use std::path::PathBuf;
use std::time::Instant;

/// Direct implementation of ExtractorInterface using arma3_extractor library
pub struct DirectExtractorImpl {
    config: ExtractionConfig,
}

impl DirectExtractorImpl {
    pub fn new(config: ExtractionConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl ExtractorInterface for DirectExtractorImpl {
    async fn extract_pbos(
        &self,
        source_dir: &PathBuf,
        output_dir: &PathBuf,
        _options: &ExtractionOptions,
    ) -> arma3_workflow::Result<ExtractionSummary> {
        let start_time = Instant::now();
        
        // Create extractor config using the proper constructor
        let mut extractor_config = ExtractorExtractionConfig::new(output_dir.clone());
        extractor_config.game_data_dirs = vec![source_dir.clone()];
        extractor_config.game_data_extensions = self.config.game_data_extensions.clone();
        extractor_config.game_data_cache_dir = output_dir.clone();
        
        // Use arma3_extractor directly
        let result = extract_game_data(extractor_config)
            .await
            .map_err(|e| WorkflowError::extraction_error(e.to_string()))?;
        
        let elapsed_time = start_time.elapsed();
        
        Ok(ExtractionSummary {
            extracted_pbos: result.len(),
            extraction_paths: result,
            elapsed_time,
            errors: Vec::new(), // TODO: Include any errors from extraction result
        })
    }
    
    async fn validate_extraction_config(&self, options: &ExtractionOptions) -> arma3_workflow::Result<()> {
        if options.source_directories.is_empty() {
            return Err(WorkflowError::validation_error("No source directories specified"));
        }
        
        for dir in &options.source_directories {
            if !dir.exists() {
                return Err(WorkflowError::validation_error(
                    format!("Source directory does not exist: {:?}", dir)
                ));
            }
        }
        
        Ok(())
    }
}

pub async fn run_extract(config: ExtractionConfig) -> Result<()> {
    info!("Starting extraction using workflow orchestration");
    
    // Create the direct extractor implementation
    let extractor = Box::new(DirectExtractorImpl::new(config.clone()));
    
    // Create the workflow handler
    let extract_handler = ExtractWorkflowHandler::new(extractor);
    
    // Create the workflow orchestrator
    let mut orchestrator = WorkflowOrchestrator::new();
    orchestrator.register_handler(Box::new(extract_handler));
    
    // Convert ExtractionConfig to workflow options
    let extraction_options = ExtractionOptions {
        source_directories: config.game_data_dirs.iter()
            .map(|dir| PathBuf::from(dir))
            .collect(),
        ..Default::default()
    };
    
    let workflow_options = WorkflowOptions::default()
        .with_extraction(extraction_options);
    
    // Create the workflow
    let workflow = Workflow::new(
        "extract".to_string(),
        WorkflowType::Extract,
        ContentType::GameData,
        workflow_options,
    );
    
    // Execute the workflow
    let output_dir = config.game_data_cache_dir.parent()
        .unwrap_or(&config.game_data_cache_dir)
        .to_path_buf();
    
    let result = orchestrator.execute_workflow(workflow, output_dir).await
        .map_err(|e| anyhow::anyhow!("Workflow execution failed: {}", e))?;
    
    // Report results
    if let Some(extraction_summary) = result.extraction_summary {
        info!(
            "Extraction completed successfully: {} PBOs extracted in {:?}",
            extraction_summary.extracted_pbos,
            result.duration
        );
        
        if !extraction_summary.errors.is_empty() {
            info!("Extraction completed with {} warnings/errors:", extraction_summary.errors.len());
            for error in &extraction_summary.errors {
                log::warn!("  {}", error);
            }
        }
    }
    
    if !result.errors.is_empty() {
        for error in &result.errors {
            log::error!("Workflow error: {}", error);
        }
        return Err(anyhow::anyhow!("Workflow completed with {} errors", result.errors.len()));
    }
    
    info!("Extraction workflow completed successfully");
    Ok(())
}