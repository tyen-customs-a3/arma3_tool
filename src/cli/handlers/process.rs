use anyhow::Result;
use arma3_extractor::ExtractionConfig;
use std::path::PathBuf;
use log::info;

pub async fn run_process(config: ExtractionConfig, db_path: PathBuf) -> Result<()> {
    info!("Starting processing workflow through legacy CLI");
    
    // Call the original processing function
    crate::cli::process::run_process(config, db_path).await
}