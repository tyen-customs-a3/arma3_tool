mod commands;
mod handlers;
mod adapters;
mod extract;
mod process;
mod report;
mod fuzzy_report;
mod export;
mod export_optimized;

pub use commands::Commands;
pub use handlers::{run_extract, run_process, run_report, run_fuzzy_report, run_export};
pub use export_optimized::run_export_optimized;
pub use adapters::{Arma3ExtractorAdapter, Arma3ScannerAdapter, Arma3ReporterAdapter};

use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    #[arg(short = 'c', long = "config", value_name = "FILE")]
    pub config_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

/// Get the default cache file path
pub fn get_default_cache_path(cache_dir: &Path) -> PathBuf {
    cache_dir.join("arma3.db")
}
