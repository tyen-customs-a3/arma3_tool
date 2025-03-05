pub mod types;
pub mod commands;

// Export the class scanning command handler
pub mod class_commands {
    use std::path::Path;
    use anyhow::{Result, Context};
    use log::{info, warn};
    use crate::commands::ClassScanArgs;
    
    // Direct imports from the extraction crate
    use extraction::ExtractionConfig;
    use extraction::extract_pbos;
    
    // Direct imports from the code_scanner crate
    use code_scanner::ClassProcessor;
    use code_scanner::ClassScanOptions;
    use code_scanner::DatabaseOperations;
    use code_scanner::QueryOptions;
    use code_scanner::file_utils;
    
    /// Handle the scan classes command
    pub async fn handle_scan_classes(args: ClassScanArgs) -> Result<()> {
        info!("Starting class scanning operation");
        
        // Create output directory
        file_utils::ensure_dir_exists(&args.output_dir)?;
        
        // Always extract PBOs first
        info!("Extracting PBOs before scanning");
        
        // Create cache directory
        file_utils::ensure_dir_exists(&args.cache_dir)?;
        
        // Extract PBOs
        let config = ExtractionConfig {
            input_dir: &args.input_dir,
            cache_dir: &args.cache_dir,
            extensions: &args.extensions,
            threads: args.threads,
            timeout: args.timeout,
        };
        
        info!("Starting PBO extraction with config: {:?}", config);
        match extract_pbos(config).await {
            Ok(_) => info!("PBO extraction completed successfully"),
            Err(e) => {
                warn!("PBO extraction encountered errors: {}", e);
                return Err(anyhow::anyhow!("Failed to extract PBOs: {}", e));
            }
        }
        
        // Use cache directory as input for scanning
        let scan_dir = args.cache_dir.clone();
        
        info!("Scanning for classes in {}", scan_dir.display());
        
        // Create class processor
        let options = ClassScanOptions::default();
        let processor = ClassProcessor::new(options, &args.output_dir);
        
        // Scan directory for classes
        let scan_result = processor.scan_directory(&scan_dir)
            .context("Failed to scan directory for classes")?;
        
        info!("Found {} classes in {} files", 
            scan_result.classes.len(), 
            scan_result.stats.files_with_classes);
        
        // Create database operations
        let db_path = args.output_dir.join("class_database.json");
        let mut db_ops = DatabaseOperations::new(&db_path)
            .context("Failed to create database operations")?;
        
        // Update database with scan results
        let db_stats = db_ops.update_with_scan_results(scan_result)
            .context("Failed to update database with scan results")?;
        
        info!("Updated database: {} classes total, {} added, {} updated, {} removed",
            db_stats.total_classes,
            db_stats.added_classes,
            db_stats.updated_classes,
            db_stats.removed_classes);
        
        // Save database
        db_ops.save()?;
        
        // Generate report
        generate_class_report(&db_ops, &args.output_dir)?;
        
        info!("Class scanning completed successfully");
        Ok(())
    }
    
    /// Generate a report of all classes
    fn generate_class_report(db_ops: &DatabaseOperations, output_dir: &Path) -> Result<()> {
        info!("Generating class report");
        
        // Query all classes
        let all_classes = db_ops.query(&QueryOptions {
            parent: None,
            property_name: None,
            property_value: None,
            limit: None,
            sort_by: Some("name".to_string()),
            descending: false,
        });
        
        // Create report file
        let report_path = output_dir.join("class_report.txt");
        let mut report_content = String::new();
        
        // Add header
        report_content.push_str(&format!("# Class Report\n"));
        report_content.push_str(&format!("Total Classes: {}\n\n", all_classes.len()));
        
        // Add classes
        for entry in all_classes {
            let class = &entry.class;
            
            report_content.push_str(&format!("## {}\n", class.name));
            
            if let Some(parent) = &class.parent {
                report_content.push_str(&format!("Parent: {}\n", parent));
            }
            
            if let Some(path) = &class.file_path {
                report_content.push_str(&format!("File: {}\n", path.to_string_lossy()));
            } else {
                report_content.push_str("File: Unknown\n");
            }
            
            if !class.properties.is_empty() {
                report_content.push_str("Properties:\n");
                for (key, value) in &class.properties {
                    report_content.push_str(&format!("  - {}: {}\n", key, value));
                }
            }
            
            // Check if the class has children (we'll query for them)
            let children = db_ops.query(&QueryOptions {
                parent: Some(class.name.clone()),
                property_name: None,
                property_value: None,
                limit: None,
                sort_by: None,
                descending: false,
            });
            
            if !children.is_empty() {
                let child_names: Vec<String> = children.iter()
                    .map(|entry| entry.class.name.clone())
                    .collect();
                report_content.push_str(&format!("Child Classes: {}\n", child_names.join(", ")));
            }
            
            report_content.push_str("\n");
        }
        
        // Write report to file
        file_utils::write_string_to_file(report_path, &report_content)?;
        
        info!("Class report generated at {}", output_dir.join("class_report.txt").display());
        Ok(())
    }
}