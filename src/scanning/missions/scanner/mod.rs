mod scanner;
mod collector;

use std::path::Path;
use std::sync::{Arc, Mutex};
use anyhow::Result;

use crate::scanning::types::ScanDatabase;
use crate::scanning::missions::extractor::types::MissionExtractionResult;

/// Scanner for mission PBO files
pub struct MissionScanner<'a> {
    input_dir: &'a Path,
    cache_dir: &'a Path,
    threads: usize,
    db: Arc<Mutex<ScanDatabase>>,
}

impl<'a> MissionScanner<'a> {
    /// Create a new mission scanner
    pub fn new(input_dir: &'a Path, cache_dir: &'a Path, threads: usize) -> Self {
        // Create database path
        let db_path = cache_dir.join("mission_scan_db.json");
        let db = Arc::new(Mutex::new(ScanDatabase::load_or_create(&db_path).unwrap_or_else(|_| {
            log::debug!("Creating new mission scan database");
            ScanDatabase::new()
        })));
        
        Self {
            input_dir,
            cache_dir,
            threads,
            db,
        }
    }
    
    /// Scan and extract mission PBO files
    pub async fn scan_and_extract(&self) -> Result<Vec<MissionExtractionResult>> {
        scanner::scan_and_extract(self.input_dir, self.cache_dir, self.threads, &self.db).await
    }
} 