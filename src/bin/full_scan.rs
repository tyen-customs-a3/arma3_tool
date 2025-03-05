use std::path::PathBuf;
use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info, warn};
use serde_json;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

use code_scanner::class::processor::ClassProcessor;
use code_scanner::class::types::{ClassScanOptions, ProcessedClass};
use code_scanner::database::QueryOptions;
use code_scanner::database::operations::DatabaseOperations;
use code_scanner::utils::file_utils;
use extraction::{ExtractionConfig, extract_pbos};

use arma3_tool::commands::FullScanArgs;
use arma3_tool::commands::FullScanConfig;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    args: FullScanArgs,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Initialize logging with specified log level
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(&cli.args.log_level))
        .init();

    // Load configuration from JSON file
    let config_path = &cli.args.config;
    info!("Loading configuration from {}", config_path.display());

    let config_content = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

    let config: FullScanConfig =
        serde_json::from_str(&config_content).with_context(|| "Failed to parse config JSON")?;

    // Create output and cache directories if they don't exist
    file_utils::ensure_dir_exists(&config.output_dir)?;
    file_utils::ensure_dir_exists(&config.cache_dir)?;

    // Process class database inputs
    info!("Processing {} class input directories", config.class_input_dirs.len());
    let class_db_path = config.output_dir.join("class_database.json");
    let mut db_ops = DatabaseOperations::new(&class_db_path)
        .context("Failed to create database operations")?;
    
    // Process each class input directory
    for input_dir in &config.class_input_dirs {
        info!("Processing class input directory: {}", input_dir.display());
        
        // Extract PBOs
        let input_path = Path::new(input_dir);
        if !input_path.exists() {
            warn!("Input directory does not exist: {}", input_path.display());
            continue;
        }

        // Create a unique cache directory for this input folder
        // Get the last component of the path to use as a folder name
        let folder_name = input_path.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| {
                // Fallback to a hash of the path if we can't get the folder name
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                input_path.to_string_lossy().hash(&mut hasher);
                format!("input_{}", hasher.finish())
            });
        
        let unique_cache_dir = config.cache_dir.join(&folder_name);
        file_utils::ensure_dir_exists(&unique_cache_dir)?;
        
        info!("Using unique cache directory for {}: {}", input_path.display(), unique_cache_dir.display());

        // Join extensions with commas to create a single string
        let extensions_str = config.extensions.join(",");
        
        info!("Starting PBO extraction from: {}", input_path.display());
        let extraction_config = ExtractionConfig {
            input_dir: input_path,
            cache_dir: &unique_cache_dir,
            extensions: &extensions_str,
            threads: config.threads,
            timeout: config.timeout,
        };

        if let Err(e) = extract_pbos(extraction_config).await {
            error!("Failed to extract PBOs from {}: {}", input_path.display(), e);
            continue;
        }
        info!("Completed PBO extraction from: {}", input_path.display());
        
        // Scan for classes
        info!("Starting class scanning in: {}", unique_cache_dir.display());
        let options = ClassScanOptions::default();
        let processor = ClassProcessor::new(options, &config.output_dir);
        
        match processor.scan_directory(&unique_cache_dir) {
            Ok(scan_result) => {
                info!("Found {} classes in {} files", 
                    scan_result.classes.len(), 
                    scan_result.stats.files_with_classes);
                
                // Update database with scan results
                match db_ops.update_with_scan_results(scan_result) {
                    Ok(stats) => {
                        info!("Updated database: {} classes total, {} added, {} updated, {} removed",
                            stats.total_classes,
                            stats.added_classes,
                            stats.updated_classes,
                            stats.removed_classes);
                    },
                    Err(e) => {
                        error!("Failed to update database with scan results: {}", e);
                    }
                }
            },
            Err(e) => {
                error!("Failed to scan directory for classes: {}", e);
                continue;
            }
        }
        info!("Completed processing of input directory: {}", input_path.display());
    }

    // Save the class database
    info!("Saving class database to {}", class_db_path.display());
    match db_ops.save() {
        Ok(_) => info!("Successfully saved class database"),
        Err(e) => error!("Failed to save class database: {}", e),
    }

    // Generate class report
    let class_report_path = config.output_dir.join("class_report");
    fs::create_dir_all(&class_report_path).with_context(|| {
        format!(
            "Failed to create class report directory: {}",
            class_report_path.display()
        )
    })?;

    info!("Generating class report in {}", class_report_path.display());
    match generate_class_report(&db_ops, &class_report_path) {
        Ok(_) => info!("Successfully generated class report"),
        Err(e) => error!("Failed to generate class report: {}", e),
    }

    // Process mission input directories
    info!("Processing {} mission input directories", config.mission_input_dirs.len());
    
    // Get all processed classes from the database for validation
    let all_classes = db_ops.query(&QueryOptions {
        parent: None,
        property_name: None,
        property_value: None,
        limit: None,
        sort_by: None,
        descending: false,
    });
    
    let processed_classes = all_classes.iter()
        .map(|entry| entry.class.clone())
        .collect::<Vec<_>>();
    
    // Process each mission input directory
    let mut all_mission_results: Vec<String> = Vec::new();
    for mission_dir in &config.mission_input_dirs {
        info!("Processing mission input directory: {}", mission_dir.display());
        
        let mission_path = Path::new(mission_dir);
        if !mission_path.exists() {
            warn!("Mission input directory does not exist: {}", mission_path.display());
            continue;
        }
        
        // Create a unique cache directory for this mission folder
        let folder_name = mission_path.file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| {
                // Fallback to a hash of the path if we can't get the folder name
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                mission_path.to_string_lossy().hash(&mut hasher);
                format!("mission_{}", hasher.finish())
            });
        
        let mission_cache_dir = config.cache_dir.join(&folder_name);
        file_utils::ensure_dir_exists(&mission_cache_dir)?;
        
        info!("Using unique cache directory for missions: {}", mission_cache_dir.display());
        
        // Extract mission PBOs first
        info!("Starting PBO extraction from mission directory: {}", mission_path.display());
        let extraction_config = ExtractionConfig {
            input_dir: mission_path,
            cache_dir: &mission_cache_dir,
            extensions: &config.extensions.join(","),
            threads: config.threads,
            timeout: config.timeout,
        };

        if let Err(e) = extract_pbos(extraction_config).await {
            error!("Failed to extract mission PBOs from {}: {}", mission_path.display(), e);
            continue;
        }
        info!("Completed mission PBO extraction from: {}", mission_path.display());
        
        // TODO: Implement mission scanning and analysis here
        // For now, we'll just log that we would scan missions
        info!("Mission scanning would be performed here for: {}", mission_path.display());
        info!("Completed processing of mission input directory: {}", mission_path.display());
    }
    
    // TODO: Implement mission validation against class database
    info!("Mission validation would be performed here");

    info!("Full scan completed successfully");
    Ok(())
}

/// Generate a report of the class database
fn generate_class_report(db_ops: &DatabaseOperations, output_dir: &Path) -> Result<()> {
    // Query all classes
    let all_classes = db_ops.query(&QueryOptions {
        parent: None,
        property_name: None,
        property_value: None,
        limit: None,
        sort_by: Some("name".to_string()),
        descending: false,
    });

    // Get database statistics
    let stats = db_ops.get_stats();

    // Count classes with definitions
    let defined_classes = all_classes
        .iter()
        .filter(|entry| entry.class.file_path.is_some())
        .count();

    // Write summary report
    let summary_path = output_dir.join("summary.txt");
    let summary_content = format!(
        "Class Database Summary\n\
        ---------------------\n\
        Total classes: {}\n\
        Classes with definitions: {}\n\
        Classes without definitions: {}\n\
        Total files: {}\n",
        stats.total_classes,
        defined_classes,
        stats.total_classes - defined_classes,
        stats.total_files
    );

    fs::write(&summary_path, summary_content).with_context(|| {
        format!(
            "Failed to write summary report to {}",
            summary_path.display()
        )
    })?;

    // Export the full database as JSON
    let db = db_ops.database();
    let db_json = serde_json::to_string_pretty(&db)
        .with_context(|| "Failed to serialize database to JSON")?;

    let db_json_path = output_dir.join("database.json");
    fs::write(&db_json_path, db_json).with_context(|| {
        format!(
            "Failed to write database JSON to {}",
            db_json_path.display()
        )
    })?;

    // Export list of classes without definitions
    let undefined_classes: Vec<_> = all_classes
        .iter()
        .filter(|entry| entry.class.file_path.is_none())
        .collect();

    let undefined_path = output_dir.join("undefined_classes.txt");
    let mut undefined_content = String::new();

    for entry in undefined_classes {
        undefined_content.push_str(&format!("{}\n", entry.class.name));
    }

    fs::write(&undefined_path, undefined_content).with_context(|| {
        format!(
            "Failed to write undefined classes to {}",
            undefined_path.display()
        )
    })?;

    Ok(())
}
