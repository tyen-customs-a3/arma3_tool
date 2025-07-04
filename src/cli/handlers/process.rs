use anyhow::Result;
use arma3_extractor::ExtractionConfig;
use gamedata_scanner::{Scanner, ScannerConfig};
use arma3_workflow::{
    WorkflowOrchestrator, ProcessWorkflowHandler, ScannerInterface,
    Workflow, WorkflowType, ContentType, WorkflowOptions, ProcessingOptions,
    ProcessingSummary, WorkflowError
};
use async_trait::async_trait;
use std::path::PathBuf;
use std::time::Instant;
use log::info;

/// Direct implementation of ScannerInterface using gamedata_scanner library
pub struct DirectScannerImpl {
    _config: ExtractionConfig,
    _db_path: PathBuf,
}

impl DirectScannerImpl {
    pub fn new(config: ExtractionConfig, db_path: PathBuf) -> Self {
        Self { _config: config, _db_path: db_path }
    }
}

#[async_trait]
impl ScannerInterface for DirectScannerImpl {
    async fn scan_files(
        &self,
        source_dir: &PathBuf,
        _output_dir: &PathBuf,
        options: &ProcessingOptions,
    ) -> arma3_workflow::Result<ProcessingSummary> {
        let start_time = Instant::now();
        
        // Create scanner config
        let mut scanner_config = ScannerConfig::default();
        scanner_config.show_progress = options.verbose;
        scanner_config.timeout = 60; // seconds
        
        // Create scanner for the source directory
        let scanner = Scanner::new(source_dir, scanner_config)
            .map_err(|e| WorkflowError::processing_error(format!("Failed to create scanner: {}", e)))?;
        
        // Scan the directory
        let scan_result = scanner.scan_directory(".")
            .map_err(|e| WorkflowError::processing_error(format!("Failed to scan directory: {}", e)))?;
        
        let elapsed_time = start_time.elapsed();
        
        // Calculate totals from scan result
        let total_classes: usize = scan_result.results.values()
            .map(|file_result| file_result.classes.len())
            .sum();
        
        Ok(ProcessingSummary {
            processed_pbos: 1, // We processed one directory
            files_processed: scan_result.total_files,
            entries_found: total_classes,
            elapsed_time,
            errors: Vec::new(), // TODO: Include any errors from scan result
        })
    }
    
    async fn validate_processing_config(&self, options: &ProcessingOptions) -> arma3_workflow::Result<()> {
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
        
        if options.max_files == 0 {
            return Err(WorkflowError::validation_error("Max files must be greater than 0"));
        }
        
        if options.max_depth == 0 {
            return Err(WorkflowError::validation_error("Max depth must be greater than 0"));
        }
        
        Ok(())
    }
}

pub async fn run_process(config: ExtractionConfig, db_path: PathBuf) -> Result<()> {
    info!("Starting processing using workflow orchestration");
    
    // Create the direct scanner implementation
    let scanner = Box::new(DirectScannerImpl::new(config.clone(), db_path.clone()));
    
    // Create the workflow handler
    let process_handler = ProcessWorkflowHandler::new(scanner);
    
    // Create the workflow orchestrator
    let mut orchestrator = WorkflowOrchestrator::new();
    orchestrator.register_handler(Box::new(process_handler));
    
    // Convert ExtractionConfig to workflow options
    let processing_options = ProcessingOptions {
        max_files: 1000, // Default reasonable limit
        max_depth: 10,   // Default reasonable depth
        extensions: config.game_data_extensions.clone(),
        source_directories: vec![config.game_data_cache_dir.clone(), config.mission_cache_dir.clone()],
        verbose: false,  // Could be made configurable
    };
    
    let workflow_options = WorkflowOptions::default()
        .with_processing(processing_options);
    
    // Create the workflow
    let workflow = Workflow::new(
        "process".to_string(),
        WorkflowType::Process,
        ContentType::GameData,
        workflow_options,
    );
    
    // Execute the workflow
    let output_dir = db_path.parent()
        .unwrap_or_else(|| std::path::Path::new("."))
        .to_path_buf();
    
    let result = orchestrator.execute_workflow(workflow, output_dir).await
        .map_err(|e| anyhow::anyhow!("Workflow execution failed: {}", e))?;
    
    // Report results
    if let Some(processing_summary) = result.processing_summary {
        info!(
            "Processing completed successfully: {} files processed, {} entries found in {:?}",
            processing_summary.files_processed,
            processing_summary.entries_found,
            result.duration
        );
        
        if !processing_summary.errors.is_empty() {
            info!("Processing completed with {} warnings/errors:", processing_summary.errors.len());
            for error in &processing_summary.errors {
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
    
    info!("Processing workflow completed successfully");
    Ok(())
}