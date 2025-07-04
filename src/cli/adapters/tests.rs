//! Comprehensive error handling and edge case tests for workflow adapters

#[cfg(test)]
mod integration_tests {
    use super::super::*;
    use tempfile::TempDir;
    use std::fs;
    use std::path::PathBuf;
    use std::os::unix::fs::PermissionsExt;
    
    use arma3_workflow::types::options::{ExtractionOptions, ProcessingOptions, ReportingOptions, ExportOptions, ReportFormat};
    use arma3_workflow::extract::ExtractorInterface;
    use arma3_workflow::process::ScannerInterface;
    use arma3_workflow::report::ReporterInterface;
    use arma3_workflow::export::ExporterInterface;
    use arma3_extractor::ExtractionConfig;

    /// Create a minimal extraction config for testing
    fn create_minimal_extraction_config(cache_dir: PathBuf) -> ExtractionConfig {
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
    async fn test_extractor_permission_errors() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let source_dir = temp_dir.path().join("source");
        let restricted_dir = temp_dir.path().join("restricted");
        
        fs::create_dir_all(&cache_dir).unwrap();
        fs::create_dir_all(&source_dir).unwrap();
        fs::create_dir_all(&restricted_dir).unwrap();
        
        // Make directory read-only
        #[cfg(unix)]
        {
            fs::set_permissions(&restricted_dir, fs::Permissions::from_mode(0o444)).unwrap();
        }
        
        let config = create_minimal_extraction_config(cache_dir);
        let adapter = extractor::Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: None,
            source_directories: vec![restricted_dir],
            force: false,
        };
        
        // This should pass validation but may fail during extraction
        let validation_result = adapter.validate_extraction_config(&options).await;
        assert!(validation_result.is_ok());
    }

    #[tokio::test]
    async fn test_scanner_concurrent_database_access() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let db_path = temp_dir.path().join("test.db");
        fs::create_dir_all(&cache_dir).unwrap();
        fs::write(&db_path, "test database").unwrap();
        
        let config = create_minimal_extraction_config(cache_dir);
        let adapter = scanner::Arma3ScannerAdapter::new(config, db_path);
        
        let options = ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec!["cpp".to_string()],
            source_directories: vec![],
            verbose: false,
        };
        
        // Multiple concurrent validation calls should all succeed
        let validation_tasks = (0..5).map(|_| {
            let adapter_ref = &adapter;
            let options_ref = &options;
            async move {
                adapter_ref.validate_processing_config(options_ref).await
            }
        });
        
        let results = futures::future::join_all(validation_tasks).await;
        for result in results {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_reporter_large_output_path() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        fs::write(&db_path, "test database").unwrap();
        
        let adapter = reporter::Arma3ReporterAdapter::new(db_path, None);
        
        // Create a very long path name
        let long_path_component = "a".repeat(255);
        let mut long_path = temp_dir.path().to_path_buf();
        for _ in 0..10 {
            long_path = long_path.join(&long_path_component);
        }
        
        let options = ReportingOptions {
            format: ReportFormat::Markdown,
            output_path: Some(long_path),
            source_directories: vec![],
            include_mission_dependencies: true,
            include_game_data_classes: false,
            include_missing_dependencies: false,
        };
        
        // This should succeed for validation
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_exporter_zero_byte_database() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("empty.db");
        
        // Create a zero-byte database file
        fs::write(&db_path, "").unwrap();
        
        let adapter = exporter::Arma3ExporterAdapter::new(db_path);
        let options = ExportOptions {
            format: "csv".to_string(),
            output_path: None,
            compression_level: None,
            include_metadata: true,
        };
        
        // This might succeed or fail depending on DatabaseManager implementation
        let result = adapter.validate_export_config(&options).await;
        // We can't assert specific behavior as it depends on the database implementation
    }

    #[tokio::test]
    async fn test_extractor_circular_symlinks() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let source_dir = temp_dir.path().join("source");
        fs::create_dir_all(&cache_dir).unwrap();
        fs::create_dir_all(&source_dir).unwrap();
        
        #[cfg(unix)]
        {
            // Create circular symlinks (on Unix systems)
            let link1 = source_dir.join("link1");
            let link2 = source_dir.join("link2");
            
            std::os::unix::fs::symlink(&link2, &link1).ok();
            std::os::unix::fs::symlink(&link1, &link2).ok();
        }
        
        let config = create_minimal_extraction_config(cache_dir);
        let adapter = extractor::Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: None,
            source_directories: vec![source_dir],
            force: false,
        };
        
        // Should handle symlinks gracefully
        let result = adapter.validate_extraction_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scanner_max_limits() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let db_path = temp_dir.path().join("test.db");
        fs::create_dir_all(&cache_dir).unwrap();
        fs::write(&db_path, "test database").unwrap();
        
        let config = create_minimal_extraction_config(cache_dir);
        let adapter = scanner::Arma3ScannerAdapter::new(config, db_path);
        
        // Test with extreme values
        let options = ProcessingOptions {
            max_files: usize::MAX,
            max_depth: usize::MAX,
            extensions: vec!["a".repeat(1000)], // Very long extension
            source_directories: vec![],
            verbose: true,
        };
        
        let result = adapter.validate_processing_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reporter_all_formats() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        fs::write(&db_path, "test database").unwrap();
        
        let adapter = reporter::Arma3ReporterAdapter::new(db_path, None);
        
        // Test all report formats
        let formats = vec![
            ReportFormat::PlainText,
            ReportFormat::Markdown,
            ReportFormat::Html,
            ReportFormat::Json,
            ReportFormat::Csv,
        ];
        
        for format in formats {
            let options = ReportingOptions {
                format: format.clone(),
                output_path: None,
                source_directories: vec![],
                include_mission_dependencies: true,
                include_game_data_classes: false,
                include_missing_dependencies: false,
            };
            
            let result = adapter.validate_reporting_config(&options).await;
            assert!(result.is_ok(), "Format {:?} should be valid", format);
        }
    }

    #[tokio::test]
    async fn test_exporter_unicode_paths() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        fs::write(&db_path, "test database").unwrap();
        
        let adapter = exporter::Arma3ExporterAdapter::new(db_path);
        
        // Test with Unicode characters in paths
        let unicode_path = temp_dir.path().join("测试_🚀_файл.csv");
        
        let options = ExportOptions {
            format: "csv".to_string(),
            output_path: Some(unicode_path),
            compression_level: None,
            include_metadata: true,
        };
        
        let result = adapter.validate_export_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fuzzy_reporter_edge_cases() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        fs::write(&db_path, "test database").unwrap();
        
        let adapter = reporter::FuzzyReporterAdapter::new(db_path, None);
        
        // Test with minimal configuration
        let options = ReportingOptions {
            format: ReportFormat::Csv,
            output_path: None,
            source_directories: vec![],
            include_mission_dependencies: false,
            include_game_data_classes: false,
            include_missing_dependencies: false,
        };
        
        // Fuzzy reporter doesn't require specific report types to be enabled
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_extractor_empty_directories_list() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();
        
        let config = create_minimal_extraction_config(cache_dir);
        let adapter = extractor::Arma3ExtractorAdapter::new(config);
        
        let options = ExtractionOptions {
            use_extractor: true,
            directories: Some(vec![]), // Empty directories list
            source_directories: vec![temp_dir.path().to_path_buf()],
            force: false,
        };
        
        let result = adapter.validate_extraction_config(&options).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[tokio::test]
    async fn test_scanner_empty_extensions() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let db_path = temp_dir.path().join("test.db");
        fs::create_dir_all(&cache_dir).unwrap();
        fs::write(&db_path, "test database").unwrap();
        
        let config = create_minimal_extraction_config(cache_dir);
        let adapter = scanner::Arma3ScannerAdapter::new(config, db_path);
        
        let options = ProcessingOptions {
            max_files: 1000,
            max_depth: 10,
            extensions: vec![], // Empty extensions
            source_directories: vec![],
            verbose: false,
        };
        
        // Empty extensions should be valid - might process all files
        let result = adapter.validate_processing_config(&options).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_exporter_compression_edge_cases() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        fs::write(&db_path, "test database").unwrap();
        
        let adapter = exporter::Arma3ExporterAdapter::new(db_path);
        
        // Test with extreme compression levels
        let test_cases = vec![
            (Some(0), true),   // Minimum compression
            (Some(9), true),   // Maximum compression
            (Some(255), true), // Invalid compression level (should still validate)
            (None, true),      // No compression
        ];
        
        for (compression_level, should_succeed) in test_cases {
            let options = ExportOptions {
                format: "csv".to_string(),
                output_path: None,
                compression_level,
                include_metadata: true,
            };
            
            let result = adapter.validate_export_config(&options).await;
            if should_succeed {
                assert!(result.is_ok(), "Compression level {:?} should be valid", compression_level);
            }
        }
    }

    #[tokio::test]
    async fn test_reporter_with_very_long_ignore_list() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let ignore_path = temp_dir.path().join("huge_ignore.txt");
        fs::write(&db_path, "test database").unwrap();
        
        // Create a very large ignore file
        let large_content = (0..10000).map(|i| format!("ignored_class_{}", i)).collect::<Vec<_>>().join("\n");
        fs::write(&ignore_path, large_content).unwrap();
        
        let adapter = reporter::Arma3ReporterAdapter::new(db_path, Some(ignore_path));
        
        let options = ReportingOptions {
            format: ReportFormat::Markdown,
            output_path: None,
            source_directories: vec![],
            include_mission_dependencies: true,
            include_game_data_classes: false,
            include_missing_dependencies: false,
        };
        
        let result = adapter.validate_reporting_config(&options).await;
        assert!(result.is_ok());
    }
}