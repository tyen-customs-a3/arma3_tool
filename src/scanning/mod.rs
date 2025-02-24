use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use log::{info, warn, error};
use rayon::prelude::*;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use pbo_tools::{
    core::api::{PboApi, PboApiOps},
    extract::ExtractOptions,
};

mod types;
mod coordinator;
mod prescanner;
mod processor;
mod utils;

use crate::commands::ScanPboArgs;
use coordinator::ScanCoordinator;

pub async fn scan_pbos(args: ScanPboArgs) -> Result<()> {
    let coordinator = ScanCoordinator::new(args)?;
    coordinator.run().await
}