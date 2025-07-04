use crate::config::ScanConfig;
use arma3_reporter::ReportCoordinator;
use anyhow::Result;
use arma3_database::DatabaseManager;
use log::info;
use std::path::PathBuf;

pub async fn run_report(db_path: PathBuf, output_dir: PathBuf, config: &ScanConfig) -> Result<()> {
    // Create report output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| anyhow::anyhow!("Failed to create report directory: {}", e))?;

    // Create database manager
    let db_manager = DatabaseManager::new(&db_path)
        .map_err(|e| anyhow::anyhow!("Failed to create database manager: {}", e))?;

    // Create report coordinator with ignore classes file from config
    let coordinator = ReportCoordinator::new(&db_manager, config.ignore_classes_file.clone());

    // Generate dependency report
    info!("Generating dependency report...");
    coordinator
        .run_report(&output_dir)
        .map_err(|e| anyhow::anyhow!("Failed to generate dependency report: {}", e))?;

    // Generate class hierarchy graph report
    info!("Generating class hierarchy graph...");
    coordinator
        .generate_class_graph(&output_dir)
        .map_err(|e| anyhow::anyhow!("Failed to generate class hierarchy graph: {}", e))?;

    // Generate mission class source report
    info!("Generating mission class source report...");
    coordinator
        .generate_mission_class_source_report(&output_dir)
        .map_err(|e| anyhow::anyhow!("Failed to generate mission class source report: {}", e))?;

    info!("All reports generated successfully");
    Ok(())
}
