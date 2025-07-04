use anyhow::Result;
use arma3_extractor::ExtractionConfig;
use arma3_workflow::{
    WorkflowOrchestrator, ExtractWorkflowHandler, Workflow, WorkflowType, 
    ContentType, WorkflowOptions, ExtractionOptions
};
use crate::cli::adapters::Arma3ExtractorAdapter;
use log::info;
use std::path::PathBuf;

pub async fn run_extract(config: ExtractionConfig) -> Result<()> {
    info!("Starting extraction using workflow orchestration");
    
    // Create the extractor adapter
    let extractor_adapter = Box::new(Arma3ExtractorAdapter::new(config.clone()));
    
    // Create the workflow handler
    let extract_handler = ExtractWorkflowHandler::new(extractor_adapter);
    
    // Create the workflow orchestrator
    let mut orchestrator = WorkflowOrchestrator::new();
    orchestrator.register_handler(Box::new(extract_handler));
    
    // Convert ExtractionConfig to workflow options
    let extraction_options = ExtractionOptions {
        use_extractor: true,
        directories: None,
        source_directories: config.game_data_dirs.iter()
            .map(|dir| PathBuf::from(dir))
            .collect(),
        force: false, // Could be made configurable
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