use anyhow::Result;
use arma3_database::DatabaseManager;
use crate::reporter::ReportCoordinator;
use crate::config::ScanConfig;
use std::path::PathBuf;
use log::info;

/// Runs the fuzzy missing class report generation process
pub async fn run_fuzzy_report(
    db_path: PathBuf,
    output_dir: PathBuf,
    config: &ScanConfig,
) -> Result<()> {
    info!(
        "Starting fuzzy missing class report generation using database: {}",
        db_path.display()
    );
    info!("Report will be saved to: {}", output_dir.display());

    // Initialize database manager
    let db_manager = DatabaseManager::new(&db_path)?;
    info!("Database manager initialized for: {}", db_path.display());

    // Create report coordinator
    let coordinator = ReportCoordinator::new(&db_manager, config);
    info!("Report coordinator created.");

    // Generate the fuzzy missing class report
    coordinator.generate_fuzzy_missing_class_report(&output_dir)?;

    info!(
        "Fuzzy missing class report generation process completed successfully for {}.",
        db_path.display()
    );
    Ok(())
}
