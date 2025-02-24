use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub enum SkipReason {
    Empty,
    NoMatchingFiles,
    InvalidFormat,
    Failed,
    None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PboInfo {
    pub hash: String,
    pub failed: bool,
    pub skip_reason: SkipReason,
}

impl Default for SkipReason {
    fn default() -> Self {
        SkipReason::None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanDatabase {
    pbos: HashMap<String, PboInfo>,
}

#[derive(Debug)]
pub struct ScanStats {
    pub total: usize,
    pub failed: usize,
    pub unchanged: usize,
    pub processed: usize,
    pub empty: usize,
    pub no_matching_files: usize,
    pub invalid_format: usize,
}

#[derive(Debug)]
pub struct PboScanResult {
    pub path: PathBuf,
    pub expected_files: Vec<String>,
    pub hash: String,
}

impl ScanDatabase {
    pub fn new() -> Self {
        Self {
            pbos: HashMap::new(),
        }
    }

    pub fn load_or_create(path: &Path) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_pbo_info(&self, path: &Path) -> Option<&PboInfo> {
        self.pbos.get(&path.to_string_lossy().to_string())
    }

    pub fn update_pbo(&mut self, path: &Path, hash: &str, failed: bool) {
        self.pbos.insert(
            path.to_string_lossy().to_string(),
            PboInfo {
                hash: hash.to_string(),
                failed,
                skip_reason: SkipReason::None,
            },
        );
    }

    pub fn update_pbo_with_reason(&mut self, path: &Path, hash: &str, failed: bool, reason: SkipReason) {
        self.pbos.insert(
            path.to_string_lossy().to_string(),
            PboInfo {
                hash: hash.to_string(),
                failed,
                skip_reason: reason,
            },
        );
    }

    pub fn mark_failed(&mut self, path: &Path) {
        if let Some(info) = self.pbos.get_mut(&path.to_string_lossy().to_string()) {
            info.failed = true;
        }
    }

    pub fn get_stats(&self) -> ScanStats {
        let mut stats = ScanStats {
            total: self.pbos.len(),
            failed: 0,
            unchanged: 0,
            processed: 0,
            empty: 0,
            no_matching_files: 0,
            invalid_format: 0,
        };

        for info in self.pbos.values() {
            match info.skip_reason {
                SkipReason::Empty => stats.empty += 1,
                SkipReason::NoMatchingFiles => stats.no_matching_files += 1,
                SkipReason::InvalidFormat => stats.invalid_format += 1,
                SkipReason::Failed => stats.failed += 1,
                SkipReason::None => {
                    if info.failed {
                        stats.failed += 1;
                    } else {
                        stats.processed += 1;
                    }
                }
            }
        }

        stats.unchanged = stats.total - stats.failed - stats.processed;
        stats
    }
}