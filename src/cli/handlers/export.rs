use anyhow::Result;
use std::path::PathBuf;
use log::info;

pub async fn run_export(
    db_path: PathBuf,
    output: PathBuf,
    item_types: Option<String>,
    config_path: Option<PathBuf>,
    limit: usize,
) -> Result<()> {
    info!("Starting export workflow through legacy CLI");
    
    // Call the original export function
    crate::cli::export::run_export(db_path, output, item_types, config_path, limit).await
}