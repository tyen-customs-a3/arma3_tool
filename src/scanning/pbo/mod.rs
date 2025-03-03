pub mod coordinator;
mod prescanner;
mod processor;
mod utils;

pub use utils::calculate_file_hash;

use std::path::Path;
use anyhow::Result;
use log::info;
use crate::commands::ScanPboArgs;

/// Scan and extract files from PBO archives
pub async fn scan_pbos(args: ScanPboArgs) -> Result<()> {
    info!("Scanning PBO files in {}", args.input_dir.display());
    
    // Create the scan coordinator and run it
    let coordinator = coordinator::ScanCoordinator::new(args)?;
    coordinator.run().await?;
    
    Ok(())
}