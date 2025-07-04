use anyhow::Result;
use arma3_extractor::ExtractionConfig;
use log::info;

pub async fn run_extract(config: ExtractionConfig) -> Result<()> {
    info!("Starting extraction workflow through legacy CLI");
    
    // Call the original extraction function
    crate::cli::extract::run_extract(config).await
}