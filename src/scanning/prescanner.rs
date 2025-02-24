use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::Result;
use log::{info, warn};
use rayon::prelude::*;
use walkdir::WalkDir;
use indicatif::ProgressBar;
use pbo_tools::core::api::{PboApi, PboApiOps};

use super::types::{ScanDatabase, PboScanResult, SkipReason};
use super::utils::calculate_file_hash;

pub struct PreScanner<'a> {
    input_dir: &'a Path,
    extensions: &'a str,
    db: Arc<Mutex<ScanDatabase>>,
}

impl<'a> PreScanner<'a> {
    pub fn new(
        input_dir: &'a Path,
        extensions: &'a str,
        db: Arc<Mutex<ScanDatabase>>,
    ) -> Self {
        Self {
            input_dir,
            extensions,
            db,
        }
    }

    pub async fn scan_all(&self, progress: ProgressBar) -> Result<Vec<PboScanResult>> {
        // Find all PBO files and do initial hash check in parallel
        let pbo_files: Vec<(PathBuf, String)> = WalkDir::new(self.input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension()
                    .map(|ext| ext == "pbo")
                    .unwrap_or(false)
            })
            .map(|e| e.path().to_owned())
            .collect::<Vec<_>>()
            .into_par_iter()
            .filter_map(|path| {
                match calculate_file_hash(&path) {
                    Ok(hash) => Some((path, hash)),
                    Err(e) => {
                        warn!("Failed to calculate hash for {}: {}", path.display(), e);
                        let mut db = self.db.lock().unwrap();
                        db.update_pbo_with_reason(&path, "", true, SkipReason::Failed);
                        None
                    }
                }
            })
            .filter(|(path, hash)| {
                let db = self.db.lock().unwrap();
                // Keep only files that either don't exist in DB or have different hash
                if let Some(info) = db.get_pbo_info(path) {
                    if info.hash == *hash {
                        info!("Skipping unchanged PBO: {}", path.display());
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            })
            .collect();

        info!("Found {} PBO files to scan after hash check", pbo_files.len());
        progress.set_length(pbo_files.len() as u64);

        // Process remaining PBOs in parallel with progress tracking
        let results: Vec<_> = pbo_files.into_par_iter()
            .filter_map(|(path, hash)| {
                let result = match self.scan_pbo_contents(&path, &hash) {
                    Ok(Some(result)) => Some(result),
                    Ok(None) => None,
                    Err(e) => {
                        warn!("Failed to scan {}: {}", path.display(), e);
                        None
                    }
                };
                progress.inc(1);
                result
            })
            .collect();

        progress.finish();
        Ok(results)
    }

    fn scan_pbo_contents(&self, path: &Path, hash: &str) -> Result<Option<PboScanResult>> {
        let api = PboApi::builder()
            .with_timeout(30)
            .build();

        // List and validate contents
        match api.list_contents(path) {
            Ok(result) => {
                if !result.is_success() {
                    let mut db = self.db.lock().unwrap();
                    db.update_pbo_with_reason(path, hash, true, SkipReason::InvalidFormat);
                    return Ok(None);
                }

                let ext_list: Vec<&str> = self.extensions.split(',').collect();
                // Add binary extensions that will be converted
                let bin_exts = vec!["bin", "binpbo"];
                
                let matching_files: Vec<String> = result.get_file_list()
                    .into_iter()
                    .filter(|path| {
                        let path_lower = path.to_lowercase();
                        ext_list.iter().any(|ext| {
                            path_lower.ends_with(&format!(".{}", ext.to_lowercase()))
                        }) || 
                        // Also match binary files that can be converted (like config.bin)
                        (bin_exts.iter().any(|&bin_ext| path_lower.ends_with(&format!(".{}", bin_ext))) &&
                         path_lower.contains("config"))
                    })
                    .collect();

                if matching_files.is_empty() {
                    let mut db = self.db.lock().unwrap();
                    db.update_pbo_with_reason(path, hash, false, SkipReason::NoMatchingFiles);
                    return Ok(None);
                }

                Ok(Some(PboScanResult {
                    path: path.to_owned(),
                    expected_files: matching_files,
                    hash: hash.to_owned(),
                }))
            }
            Err(e) => {
                warn!("Failed to list contents of {}: {}", path.display(), e);
                let mut db = self.db.lock().unwrap();
                db.update_pbo_with_reason(path, hash, true, SkipReason::Failed);
                Ok(None)
            }
        }
    }
}