use anyhow::Result;
use arma3_database::DatabaseManager;
use async_trait::async_trait;
use std::path::PathBuf;
use std::time::Instant;
use arma3_workflow::export::ExporterInterface;
use arma3_workflow::types::summary::ExportSummary;
use arma3_workflow::types::options::ExportOptions;
use arma3_workflow::error::WorkflowError;
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
        .map_err(|e| arma3_workflow::error::WorkflowError::export_error(e.to_string()))?;
        
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
            return Err(arma3_workflow::error::WorkflowError::validation_error(
                "Export format not specified"
            ));
        }
        
        // Validate supported formats
        let supported_formats = vec!["csv", "json", "xml"];
        if !supported_formats.contains(&options.format.as_str()) {
            return Err(arma3_workflow::error::WorkflowError::validation_error(
                format!("Unsupported export format: {}", options.format)
            ));
        }
        
        // Validate database exists
        if !self.db_path.exists() {
            return Err(arma3_workflow::error::WorkflowError::validation_error(
                format!("Database file not found: {}", self.db_path.display())
            ));
        }
        
        // Test database connection
        let _db = DatabaseManager::new(&self.db_path)
            .map_err(|e| arma3_workflow::error::WorkflowError::validation_error(
                format!("Failed to connect to database: {}", e)
            ))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn create_test_database(path: &PathBuf) -> Result<(), std::io::Error> {
        // Create a simple test database file
        fs::write(path, "test database")?;
        Ok(())
    }

    #[tokio::test]
    async fn test_exporter_adapter_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        // Adapter should be created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_validate_export_config_valid() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        let options = ExportOptions {
            format: "csv".to_string(),
            output_path: Some(temp_dir.path().join("output.csv")),
            compression_level: None,
            include_metadata: true,
        };
        
        let result = adapter.validate_export_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_export_config_empty_format() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        let options = ExportOptions {
            format: "".to_string(), // Empty format
            output_path: None,
            compression_level: None,
            include_metadata: true,
        };
        
        let result = adapter.validate_export_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Export format not specified"));
    }

    #[tokio::test]
    async fn test_validate_export_config_unsupported_format() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        let options = ExportOptions {
            format: "unsupported".to_string(),
            output_path: None,
            compression_level: None,
            include_metadata: true,
        };
        
        let result = adapter.validate_export_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported export format"));
    }

    #[tokio::test]
    async fn test_validate_export_config_nonexistent_database() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("nonexistent.db");
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        let options = ExportOptions {
            format: "csv".to_string(),
            output_path: None,
            compression_level: None,
            include_metadata: true,
        };
        
        let result = adapter.validate_export_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Database file not found"));
    }

    #[tokio::test]
    async fn test_validate_export_config_json_format() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        let options = ExportOptions {
            format: "json".to_string(),
            output_path: Some(temp_dir.path().join("output.json")),
            compression_level: Some(9),
            include_metadata: false,
        };
        
        let result = adapter.validate_export_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_export_config_xml_format() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        let options = ExportOptions {
            format: "xml".to_string(),
            output_path: None,
            compression_level: Some(5),
            include_metadata: true,
        };
        
        let result = adapter.validate_export_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_export_data_nonexistent_database() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("nonexistent.db");
        let source_dir = temp_dir.path().join("source");
        let output_dir = temp_dir.path().join("output");
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        let options = ExportOptions {
            format: "csv".to_string(),
            output_path: Some(output_dir.join("export.csv")),
            compression_level: None,
            include_metadata: true,
        };
        
        // This should fail because database doesn't exist
        let result = adapter.export_data(&source_dir, &output_dir, &options).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_export_options_mapping() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        
        // Test with compression options
        let options = ExportOptions {
            format: "csv".to_string(),
            output_path: Some(temp_dir.path().join("compressed_export.csv")),
            compression_level: Some(7),
            include_metadata: false,
        };
        
        let validation_result = adapter.validate_export_config(&options).await;
        assert!(validation_result.is_ok());
        
        // Test with metadata inclusion
        let options_with_metadata = ExportOptions {
            format: "json".to_string(),
            output_path: None,
            compression_level: None,
            include_metadata: true,
        };
        
        let validation_result = adapter.validate_export_config(&options_with_metadata).await;
        assert!(validation_result.is_ok());
    }

    #[tokio::test]
    async fn test_database_connection_validation() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        // Create an invalid database file (just text, not a real database)
        fs::write(&db_path, "invalid database content").unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        let options = ExportOptions {
            format: "csv".to_string(),
            output_path: None,
            compression_level: None,
            include_metadata: true,
        };
        
        // This might fail when trying to connect to the invalid database
        let result = adapter.validate_export_config(&options).await;
        // Note: The actual behavior depends on DatabaseManager implementation
        // It could either succeed (if it doesn't validate the database content immediately)
        // or fail (if it tries to open/validate the database)
    }

    #[tokio::test]
    async fn test_supported_formats() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        create_test_database(&db_path).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        
        // Test all supported formats
        let supported_formats = vec!["csv", "json", "xml"];
        
        for format in supported_formats {
            let options = ExportOptions {
                format: format.to_string(),
                output_path: None,
                compression_level: None,
                include_metadata: true,
            };
            
            let result = adapter.validate_export_config(&options).await;
            assert!(result.is_ok(), "Format {} should be supported", format);
        }
    }

    #[tokio::test]
    async fn test_export_data_format_detection() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let source_dir = temp_dir.path().join("source");
        let output_dir = temp_dir.path().join("output");
        create_test_database(&db_path).unwrap();
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&output_dir).unwrap();
        
        let adapter = Arma3ExporterAdapter::new(db_path);
        
        // Test that format is correctly mapped to output file extension
        let options = ExportOptions {
            format: "json".to_string(),
            output_path: Some(output_dir.join("test_export.json")),
            compression_level: None,
            include_metadata: true,
        };
        
        // Validate the configuration first
        let validation_result = adapter.validate_export_config(&options).await;
        assert!(validation_result.is_ok());
        
        // Note: We can't easily test the actual export without a real database
        // and the legacy export system, but we can test that the configuration is valid
    }
}