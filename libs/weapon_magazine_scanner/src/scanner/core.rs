use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;

use crate::models::{WeaponInfo, MagazineWellInfo, ScanResult}; // Removed FolderHash
use crate::database::Database;
use super::{FileProcessor, HashCalculator, CompatibilityResolver};

/// Convert Windows UNC path to clean display format
fn clean_path_display(path: &Path) -> String {
    let path_str = path.to_string_lossy();
    if path_str.starts_with(r"\\?\") {
        path_str.strip_prefix(r"\\?\").unwrap_or(&path_str).to_string()
    } else {
        path_str.to_string()
    }
}

/// Core scanner that orchestrates the scanning process
pub struct WeaponMagazineScanner {
    thread_count: usize,
    timeout_seconds: u64,
    file_processor: FileProcessor,
    hash_calculator: HashCalculator
}

impl WeaponMagazineScanner {
    pub fn new(project_root: &Path, thread_count: usize, _timeout_seconds: u64) -> Result<Self> {
        const DEFAULT_TIMEOUT: u64 = 30; // Enforce consistent 30 second timeout
        Ok(Self {
            thread_count,
            timeout_seconds: DEFAULT_TIMEOUT,
            file_processor: FileProcessor::new(project_root, DEFAULT_TIMEOUT)?,
            hash_calculator: HashCalculator::new(),
        })
    }

    /// Check if a rescan is needed by comparing folder hashes
    pub fn should_rescan(&self, target: &Path, db: &mut Database) -> Result<bool> {
        let current_hash = self.hash_calculator.calculate_folder_hash(target)?;
        
        match db.get_folder_hash()? {
            Some(stored_hash) => Ok(current_hash.hash != stored_hash.hash),
            None => Ok(true), // No hash stored, need to scan
        }
    }

    /// Main scanning method
    pub fn scan(&self, target: &Path) -> Result<ScanResult> {
        log::info!("Starting optimized scan of {} with {} threads", clean_path_display(target), self.thread_count);
        
        // Set up thread pool with better configuration
        log::info!("Initializing thread pool...");
        if let Err(_) = rayon::ThreadPoolBuilder::new()
            .num_threads(self.thread_count)
            .thread_name(|i| format!("scanner-{}", i))
            .build_global()
        {
            log::debug!("Global thread pool already initialized, using existing configuration");
        }
        
        log::info!("Starting file processing...");
        // Process all files and collect results
        let results = self.file_processor.process_directory(target)?;
        log::info!("File processing completed, found {} files with relevant content", results.len());
        
        // Merge all results
        log::info!("Merging results...");
        let (mut weapons, magazine_wells) = self.merge_results(results);
        log::info!("Initial merge complete: {} weapons and {} magazine wells found",
                  weapons.len(), magazine_wells.len());
        
        // Perform global compatibility resolution
        log::info!("Starting compatibility resolution...");
        let resolver = CompatibilityResolver::new(&magazine_wells);
        resolver.resolve_compatibility(&mut weapons);
        log::info!("Compatibility resolution completed");
        
        // Calculate folder hash for change detection
        log::info!("Calculating folder hash...");
        let folder_hash = self.hash_calculator.calculate_folder_hash(target)?.hash;
        log::info!("Folder hash calculated");
        
        log::info!("Scan completed successfully. Found {} weapons and {} magazine wells",
                  weapons.len(), magazine_wells.len());

        Ok(ScanResult {
            weapons,
            magazine_wells,
            scan_timestamp: chrono::Utc::now(),
            folder_hash,
        })
    }

    /// Merge results from parallel processing
    fn merge_results(&self, results: Vec<super::FileProcessingResult>) -> (Vec<WeaponInfo>, HashMap<String, MagazineWellInfo>) {
        let mut all_weapons = Vec::new();
        let mut all_magazine_wells = HashMap::new();
        
        for result in results {
            all_weapons.extend(result.weapons);
            for (name, well) in result.magazine_wells {
                all_magazine_wells.insert(name, well);
            }
        }
        
        (all_weapons, all_magazine_wells)
    }

    /// Get performance statistics
    pub fn get_performance_stats(&self) -> HashMap<String, String> {
        let mut stats = HashMap::new();
        stats.insert("thread_count".to_string(), self.thread_count.to_string());
        stats.insert("timeout_seconds".to_string(), self.timeout_seconds.to_string());
        stats
    }
}
