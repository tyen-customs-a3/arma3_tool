use anyhow::Result;
use crate::config::ScanConfig;
use std::path::PathBuf;
use log::info;

pub async fn run_fuzzy_report(
    db_path: PathBuf,
    output_dir: PathBuf,
    config: &ScanConfig,
) -> Result<()> {
    info!("Starting fuzzy report workflow through legacy CLI");
    
    // Call the original fuzzy report function
    crate::cli::fuzzy_report::run_fuzzy_report(db_path, output_dir, config).await
}