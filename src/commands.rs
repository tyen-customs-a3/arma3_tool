use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};
use serde::{Serialize, Deserialize};

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    /// Scan and extract class definitions from files
    ScanClasses(ClassScanArgs),
}

/// Arguments for the class scanning command
#[derive(Args, Debug, Clone)]
pub struct ClassScanArgs {
    /// Input directory containing PBO files
    #[arg(short, long)]
    pub input_dir: PathBuf,

    /// Output directory for scan results and reports
    #[arg(short, long, default_value = "./class_reports")]
    pub output_dir: PathBuf,

    /// Cache directory for extracted files
    #[arg(short, long, default_value = "./cache")]
    pub cache_dir: PathBuf,

    /// File extensions to scan (comma-separated)
    #[arg(short, long, default_value = "cpp,hpp")]
    pub extensions: String,

    /// Maximum number of files to scan (for testing)
    #[arg(long)]
    pub max_files: Option<usize>,
    
    /// Timeout in seconds for PBO extraction operations
    #[arg(short, long, default_value_t = 30)]
    pub timeout: u32,
    
    /// Number of threads to use for processing
    #[arg(short, long, default_value_t = 4)]
    pub threads: usize,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    pub log_level: String,
}

/// Arguments for the full scan command
#[derive(Debug, Parser)]
pub struct FullScanArgs {
    /// Path to the JSON configuration file
    #[arg(short, long)]
    pub config: PathBuf,
    
    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    pub log_level: String,
}

/// Configuration for the full scan process
#[derive(Debug, Serialize, Deserialize)]
pub struct FullScanConfig {
    /// Array of input folders to extract and scan for the class database
    pub class_input_dirs: Vec<PathBuf>,
    /// Array of input folders containing mission PBOs to scan
    pub mission_input_dirs: Vec<PathBuf>,
    /// Output folder for reports
    pub output_dir: PathBuf,
    /// Cache folder for temporary storage
    pub cache_dir: PathBuf,
    /// File extensions to include in extraction (optional)
    #[serde(default = "default_extensions")]
    pub extensions: Vec<String>,
    /// Number of threads to use for processing (optional)
    #[serde(default = "default_threads")]
    pub threads: usize,
    /// Timeout in seconds for PBO extraction operations (optional)
    #[serde(default = "default_timeout")]
    pub timeout: u32,
    /// Log level (trace, debug, info, warn, error) (optional)
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_extensions() -> Vec<String> {
    vec!["hpp".to_string(), "cpp".to_string(), "sqf".to_string(), "xml".to_string()]
}

fn default_threads() -> usize {
    std::thread::available_parallelism().map(|p| p.get()).unwrap_or(1)
}

fn default_timeout() -> u32 {
    30
}

fn default_log_level() -> String {
    "info".to_string()
}
