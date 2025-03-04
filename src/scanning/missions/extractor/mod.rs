pub mod types;
mod extractor;

use std::path::Path;
use std::sync::{Arc, Mutex};
use anyhow::Result;
use indicatif::ProgressBar;
use log::info;

pub use types::MissionExtractionResult;
pub use types::MissionDatabase;
pub use types::MissionStats;
use crate::scanning::types::PboScanResult;
use crate::scanning::pbo::calculate_file_hash;

/// Extracts mission files from PBO archives
pub struct MissionExtractor<'a> {
    cache_dir: &'a Path,
    threads: usize,
    db: Arc<Mutex<MissionDatabase>>,
}

impl<'a> MissionExtractor<'a> {
    /// Create a new mission extractor
    pub fn new(cache_dir: &'a Path, threads: usize) -> Result<Self> {
        let db_path = cache_dir.join("mission_scan_db.json");
        let db = Arc::new(Mutex::new(MissionDatabase::load_or_create(&db_path)?));
        
        Ok(Self {
            cache_dir,
            threads,
            db,
        })
    }
    
    /// Extract mission files from PBO archives
    pub fn extract_missions(
        &self,
        scan_results: &[PboScanResult],
        progress: ProgressBar,
    ) -> Result<Vec<MissionExtractionResult>> {
        let results = extractor::extract_missions(
            self.cache_dir, 
            self.threads, 
            scan_results, 
            Arc::clone(&self.db), 
            progress
        )?;
        
        // Save database after extraction
        let db_path = self.cache_dir.join("mission_scan_db.json");
        let mut db = self.db.lock().unwrap();
        db.save(&db_path)?;
        
        // Display statistics
        let stats = db.get_stats();
        info!("Mission extraction statistics:");
        info!("  Total missions: {}", stats.total);
        info!("  Successfully extracted: {}", stats.processed);
        info!("  Failed extractions: {}", stats.failed);
        info!("  Unchanged (skipped): {}", stats.unchanged);
        
        Ok(results)
    }
}