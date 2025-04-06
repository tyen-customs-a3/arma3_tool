mod commands;
mod extract;
mod process;
mod report;

pub use commands::Commands;
pub use extract::run_extract;
pub use process::run_process;
pub use report::run_report;

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