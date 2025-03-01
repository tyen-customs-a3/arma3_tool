use std::sync::{Arc, Mutex};
use anyhow::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::{info, warn};
use walkdir::WalkDir;

use crate::commands::ScanPboArgs;
use super::{PreScanner, PboProcessor};
use super::super::types::ScanDatabase;

pub struct ScanCoordinator {
    args: ScanPboArgs,
    db: Arc<Mutex<ScanDatabase>>,
    progress: MultiProgress,
}

impl ScanCoordinator {
    pub fn new(args: ScanPboArgs) -> Result<Self> {
        let db_path = args.cache_dir.join("scan_db.json");
        let db = Arc::new(Mutex::new(ScanDatabase::load_or_create(&db_path)?));
        
        Ok(Self {
            args,
            db,
            progress: MultiProgress::new(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        let scan_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap();

        // Count total PBOs first for reference
        let total_pbo_count = WalkDir::new(&self.args.input_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension()
                    .map(|ext| ext == "pbo")
                    .unwrap_or(false)
            })
            .count();

        // Initialize prescanner
        let prescanner = PreScanner::new(
            &self.args.input_dir,
            &self.args.extensions,
            Arc::clone(&self.db),
        );

        // Run parallel prescan with proper progress bar
        info!("Starting hash check of {} PBOs...", total_pbo_count);
        let prescan_pb = self.progress.add(ProgressBar::new(0));
        prescan_pb.set_style(scan_style.clone());
        prescan_pb.set_message("Checking PBO hashes...");

        let scan_results = prescanner.scan_all(prescan_pb.clone()).await?;
        
        let skipped_count = total_pbo_count - scan_results.len();
        info!("Hash check complete:");
        info!("  Total PBOs found: {}", total_pbo_count);
        info!("  Skipped (unchanged): {}", skipped_count);
        info!("  Need processing: {}", scan_results.len());

        let total_expected_files: usize = scan_results.iter()
            .map(|result| result.expected_files.len())
            .sum();

        if !scan_results.is_empty() {
            // Configure processor for remaining PBOs
            let processor = PboProcessor::new(
                &self.args.input_dir,
                &self.args.cache_dir,
                &self.args.extensions,
                Arc::clone(&self.db),
            );

            // Process remaining PBOs in parallel
            let extract_pb = self.progress.add(ProgressBar::new(scan_results.len() as u64));
            extract_pb.set_style(scan_style);
            extract_pb.set_message("Extracting files...");

            processor.process_all(&scan_results, extract_pb)?;
        }

        // Display final statistics
        self.display_stats(total_pbo_count, total_expected_files)?;
        Ok(())
    }

    fn display_stats(&self, total_pbos: usize, total_expected: usize) -> Result<()> {
        let db = self.db.lock().unwrap();
        let stats = db.get_stats();
        
        info!("Scan complete. Statistics:");
        info!("  Total PBOs found: {}", total_pbos);
        info!("  Total files to extract: {}", total_expected);
        info!("  Successfully extracted: {}", stats.processed);
        info!("  Empty PBOs skipped: {}", stats.empty);
        info!("  No matching files: {}", stats.no_matching_files);
        info!("  Invalid PBO format: {}", stats.invalid_format);
        info!("  Failed extractions: {}", stats.failed);
        info!("  Unchanged (cached): {}", stats.unchanged);

        // Save final database state
        let db_path = self.args.cache_dir.join("scan_db.json");
        db.save(&db_path)?;
        
        Ok(())
    }
}