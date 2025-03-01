mod coordinator;
mod prescanner;
mod processor;
mod utils;

pub use coordinator::*;
pub(crate) use prescanner::*;
pub(crate) use processor::*;
pub(crate) use utils::*;

use super::types::*;
use crate::commands::ScanPboArgs;

pub async fn scan_pbos(args: ScanPboArgs) -> anyhow::Result<()> {
    let coordinator = ScanCoordinator::new(args)?;
    coordinator.run().await
}