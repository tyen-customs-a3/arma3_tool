use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use num_cpus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Target folder to scan for weapon and magazine data
    pub target: PathBuf,
    
    /// Output file for the report
    #[serde(default = "default_output")]
    pub output: PathBuf,
    
    /// Database file for caching scan results
    #[serde(default = "default_database")]
    pub database: PathBuf,
    
    /// Force rescan even if files haven't changed
    #[serde(default)]
    pub force: bool,
    
    /// Number of threads to use for parallel scanning
    #[serde(default = "default_threads")]
    pub threads: usize,
    
    /// Output format (json, yaml, csv, text)
    #[serde(default = "default_format")]
    pub format: String,
    
    /// Verbose output
    #[serde(default)]
    pub verbose: bool,
    
    /// Timeout per file in seconds
    #[serde(default = "default_timeout")]
    pub timeout: u64,
    
    /// Project root directory for parser (auto-detected if not specified)
    pub project_root: Option<PathBuf>,

    /// Output file for weapons grouped by mod (optional)
    pub weapons_by_mod: Option<PathBuf>,
}

fn default_output() -> PathBuf {
    PathBuf::from("weapons_magazines_report.json")
}

fn default_database() -> PathBuf {
    PathBuf::from("weapon_magazine_cache.db")
}

fn default_threads() -> usize {
    num_cpus::get()
}

fn default_format() -> String {
    "json".to_string()
}

fn default_timeout() -> u64 {
    30
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            target: PathBuf::new(), // Must be provided
            output: default_output(),
            database: default_database(),
            force: false,
            threads: default_threads(),
            format: default_format(),
            verbose: false,
            timeout: default_timeout(),
            project_root: None,
            weapons_by_mod: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponInfo {
    pub name: String,
    pub parent: Option<String>,
    pub file_path: PathBuf,
    pub magazine_wells: Vec<String>,
    pub compatible_magazines: Vec<String>,
    pub mod_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagazineWellInfo {
    pub name: String,
    pub file_path: PathBuf,
    pub magazines: HashMap<String, Vec<String>>, // prefix -> magazines
    pub mod_source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub weapons: Vec<WeaponInfo>,
    pub magazine_wells: HashMap<String, MagazineWellInfo>,
    pub scan_timestamp: chrono::DateTime<chrono::Utc>,
    pub folder_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderHash {
    pub hash: String,
    pub file_hashes: HashMap<PathBuf, String>,
}
