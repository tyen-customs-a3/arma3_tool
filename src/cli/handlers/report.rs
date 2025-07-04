use anyhow::Result;
use crate::config::ScanConfig;
use std::path::PathBuf;
use log::info;

pub async fn run_report(db_path: PathBuf, output_dir: PathBuf, config: &ScanConfig) -> Result<()> {
    info!("Starting reporting workflow through legacy CLI");
    
    // Call the original reporting function
    crate::cli::report::run_report(db_path, output_dir, config).await
}