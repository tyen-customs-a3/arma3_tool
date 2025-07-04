use anyhow::Result;
use arma3_database::DatabaseManager;
use async_trait::async_trait;
use std::path::PathBuf;
use std::time::Instant;
use workflow::export::{ExporterInterface, ExportSummary};
use workflow::types::options::ExportOptions;
use workflow::error::WorkflowError;
use log::{info, debug};

/// Adapter for Arma3 export operations implementing the ExporterInterface trait
pub struct Arma3ExporterAdapter {
    db_path: PathBuf,
}

impl Arma3ExporterAdapter {
    /// Create a new Arma3ExporterAdapter
    pub fn new(db_path: PathBuf) -> Self {
        Self { db_path }
    }
}

#[async_trait]
impl ExporterInterface for Arma3ExporterAdapter {
    /// Export data using the legacy export system
    async fn export_data(
        &self,
        source_dir: &PathBuf,
        output_dir: &PathBuf,
        options: &ExportOptions,
    ) -> Result<ExportSummary> {
        let start_time = Instant::now();
        
        debug!("Starting export from {:?} to {:?}", source_dir, output_dir);
        
        // Create output file path based on format
        let output_file = output_dir.join(format!("export.{}", options.format));
        
        // Map export options to legacy parameters
        let item_types = None; // Could be extracted from options if needed
        let config_path = None; // Could be extracted from options if needed
        let limit = 0; // No limit by default
        
        // Call the legacy export function
        crate::cli::export::run_export(
            self.db_path.clone(),
            output_file.clone(),
            item_types,
            config_path,
            limit,
        )
        .await
        .map_err(|e| workflow::error::WorkflowError::export_error(e.to_string()))?;
        
        let export_time = start_time.elapsed();
        
        // Get file size if the output file exists
        let total_size = if output_file.exists() {
            std::fs::metadata(&output_file)
                .map(|m| m.len())
                .unwrap_or(0)
        } else {
            0
        };
        
        // Count exported files (simplified - could be more sophisticated)
        let exported_files = 1;
        
        info!("Export completed in {:?}", export_time);
        
        Ok(ExportSummary {
            exported_files,
            export_format: options.format.clone(),
            export_time,
            total_size,
            output_files: vec![output_file],
        })
    }
    
    /// Validate export configuration
    async fn validate_export_config(&self, options: &ExportOptions) -> Result<()> {
        // Validate export format
        if options.format.is_empty() {
            return Err(workflow::error::WorkflowError::validation_error(
                "Export format not specified"
            ));
        }
        
        // Validate supported formats
        let supported_formats = vec!["csv", "json", "xml"];
        if !supported_formats.contains(&options.format.as_str()) {
            return Err(workflow::error::WorkflowError::validation_error(
                format!("Unsupported export format: {}", options.format)
            ));
        }
        
        // Validate database exists
        if !self.db_path.exists() {
            return Err(workflow::error::WorkflowError::validation_error(
                format!("Database file not found: {}", self.db_path.display())
            ));
        }
        
        // Test database connection
        let _db = DatabaseManager::new(&self.db_path)
            .map_err(|e| workflow::error::WorkflowError::validation_error(
                format!("Failed to connect to database: {}", e)
            ))?;
        
        Ok(())
    }
}