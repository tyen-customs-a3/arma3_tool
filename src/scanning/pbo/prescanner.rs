use std::path::Path;
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use anyhow::Result;
use indicatif::{ProgressBar, ParallelProgressIterator};
use log::{debug, warn, info};
use pbo_tools::core::api::{PboApi, PboApiOps};
use walkdir::WalkDir;
use rayon::prelude::*;

use super::utils::calculate_file_hash;
use super::super::types::{ScanDatabase, PboScanResult, SkipReason};

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
        // Collect all PBO files first
        let pbo_files: Vec<_> = WalkDir::new(self.input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "pbo")
                    .unwrap_or(false)
            })
            .map(|e| e.path().to_owned())
            .collect();

        progress.set_length(pbo_files.len() as u64);
        
        // Counter for previously failed PBOs
        let previously_failed_count = Arc::new(AtomicUsize::new(0));

        // Process PBOs in parallel
        let results: Vec<PboScanResult> = pbo_files.par_iter()
            .progress_with(progress)
            .filter_map(|path| {
                let hash = match calculate_file_hash(path) {
                    Ok(h) => h,
                    Err(_) => return None,
                };
                
                // Check if we've seen this PBO before
                let needs_processing = {
                    let db = self.db.lock().unwrap();
                    match db.get_pbo_info(path) {
                        // Skip if the PBO has the same hash and didn't fail before
                        Some(info) if info.hash == hash && !info.failed => {
                            debug!("Skipping unchanged PBO: {}", path.display());
                            false
                        },
                        // Skip if the PBO previously failed to parse, regardless of hash
                        Some(info) if info.failed => {
                            info!("Skipping previously failed PBO: {}", path.display());
                            
                            // Update the hash if it changed but keep the failed status
                            if info.hash != hash {
                                let mut db = self.db.lock().unwrap();
                                db.update_pbo_with_reason(
                                    path,
                                    &hash,
                                    true, // Keep failed status
                                    info.skip_reason.clone() // Keep the same skip reason
                                );
                            }
                            
                            // Increment the counter for previously failed PBOs
                            previously_failed_count.fetch_add(1, Ordering::Relaxed);
                            
                            false
                        },
                        _ => true
                    }
                };

                if !needs_processing {
                    return None;
                }

                let api = PboApi::builder()
                    .with_timeout(30)
                    .build();

                // List PBO contents to check for relevant files
                match api.list_contents(path) {
                    Ok(result) => {
                        if result.is_success() {
                            let files: Vec<String> = result.get_file_list()
                                .into_iter()
                                .filter(|f| {
                                    let ext = Path::new(f)
                                        .extension()
                                        .and_then(|e| e.to_str())
                                        .unwrap_or("");
                                    self.extensions.split(',').any(|valid_ext| 
                                        ext.eq_ignore_ascii_case(valid_ext)
                                    )
                                })
                                .collect();

                            if files.is_empty() {
                                let mut db = self.db.lock().unwrap();
                                db.update_pbo_with_reason(
                                    path,
                                    &hash,
                                    false,
                                    SkipReason::NoMatchingFiles
                                );
                                None
                            } else {
                                Some(PboScanResult {
                                    path: path.clone(),
                                    expected_files: files,
                                    hash,
                                })
                            }
                        } else {
                            warn!("Failed to list contents of {}: {}", path.display(), result);
                            let mut db = self.db.lock().unwrap();
                            db.update_pbo_with_reason(
                                path,
                                &hash,
                                true,
                                SkipReason::InvalidFormat
                            );
                            None
                        }
                    }
                    Err(e) => {
                        warn!("Error listing contents of {}: {}", path.display(), e);
                        let mut db = self.db.lock().unwrap();
                        db.update_pbo_with_reason(path, &hash, true, SkipReason::Failed);
                        None
                    }
                }
            })
            .collect();

        let failed_count = previously_failed_count.load(Ordering::Relaxed);
        if failed_count > 0 {
            info!("Skipped {} PBOs that previously failed to parse", failed_count);
        }

        Ok(results)
    }
}