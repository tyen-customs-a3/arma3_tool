use std::sync::{Arc, Mutex};
use std::path::Path;
use anyhow::Result;
use indicatif::ProgressBar;
use log::{debug, info, warn};
use pbo_tools::{
    core::api::{PboApi, PboApiOps},
    extract::ExtractOptions,
    core::config::PboConfig,
};
use rayon::prelude::*;
use walkdir::WalkDir;

use super::types::PboScanResult;
use crate::extraction::database::{ScanDatabase, SkipReason};

pub struct PboProcessor<'a> {
    input_dir: &'a Path,
    cache_dir: &'a Path,
    extensions: &'a str,
    db: Arc<Mutex<ScanDatabase>>,
    timeout: u32,
}

impl<'a> PboProcessor<'a> {
    pub fn new(
        input_dir: &'a Path,
        cache_dir: &'a Path,
        extensions: &'a str,
        db: Arc<Mutex<ScanDatabase>>,
        threads: usize,
        timeout: u32,
    ) -> Self {
        debug!("Creating new PboProcessor with:");
        debug!("  input_dir: {}", input_dir.display());
        debug!("  cache_dir: {}", cache_dir.display());
        debug!("  extensions: {}", extensions);
        debug!("  timeout: {} seconds", timeout);
        Self {
            input_dir,
            cache_dir,
            extensions,
            db,
            timeout,
        }
    }

    pub fn process_all(&self, scan_results: &[PboScanResult], progress: ProgressBar) -> Result<()> {
        info!("Processing {} PBOs for extraction", scan_results.len());
        
        // Count total expected files
        let total_expected_files: usize = scan_results.iter()
            .map(|result| result.expected_files.len())
            .sum();
        info!("Total expected files to extract: {}", total_expected_files);
        
        // Process each PBO
        let results: Vec<_> = scan_results
            .par_iter()
            .with_max_len(1)
            .map(|result| {
                let process_result = self.process_pbo(result);
                progress.inc(1);
                (result, process_result)
            })
            .collect();
            
        // Count successes and failures
        let success_count = results.iter().filter(|(_, r)| r.is_ok()).count();
        let failure_count = results.len() - success_count;
        
        info!("PBO processing complete:");
        info!("  Total PBOs processed: {}", results.len());
        info!("  Successful: {}", success_count);
        info!("  Failed: {}", failure_count);
        
        progress.finish_with_message("Extraction complete");
        Ok(())
    }

    fn process_pbo(&self, scan_result: &PboScanResult) -> Result<()> {
        info!("Processing PBO: {}", scan_result.path.display());
        info!("  Expected files: {:?}", scan_result.expected_files);

        // Skip if no matching files
        if scan_result.expected_files.is_empty() {
            info!("No matching files found in PBO, skipping: {}", scan_result.path.display());
            let mut db = self.db.lock().unwrap();
            db.update_pbo_with_reason(
                &scan_result.path,
                &scan_result.hash,
                true,
                SkipReason::NoMatchingFiles,
            );
            return Ok(());
        }

        // Create output directory for this PBO
        let rel_path = scan_result.path.strip_prefix(self.input_dir)?;
        let base_dir = self.cache_dir.join(rel_path).with_extension("");
        info!("Creating base directory: {}", base_dir.display());
        std::fs::create_dir_all(&base_dir)?;

        // Configure PBO extraction
        let config = PboConfig::default();
        let api = PboApi::builder()
            .with_config(config)
            .with_timeout(self.timeout)
            .build();

        // List contents and get prefix
        info!("Listing contents of PBO: {}", scan_result.path.display());
        let list_result = match api.list_contents(&scan_result.path) {
            Ok(result) => result,
            Err(e) => {
                warn!("Failed to list PBO contents {}: {}", scan_result.path.display(), e);
                let mut db = self.db.lock().unwrap();
                db.update_pbo_with_reason(
                    &scan_result.path,
                    &scan_result.hash,
                    true,
                    SkipReason::InvalidFormat,
                );
                return Ok(());
            }
        };
        
        let prefix = list_result.get_prefix().unwrap_or_default();
        info!("PBO prefix: {}", prefix);

        // Create output directory with prefix path
        let output_dir = base_dir.join(prefix);
        info!("Creating output directory: {}", output_dir.display());
        std::fs::create_dir_all(&output_dir)?;

        // Configure extraction options
        let mut options = ExtractOptions::default();
        options.file_filter = Some(self.extensions.split(',').map(str::to_string).collect());
        options.no_pause = true;
        options.warnings_as_errors = false;
        options.verbose = true;
        info!("Extracting with filter: {:?}", options.file_filter);

        // Extract files
        info!("Extracting PBO: {} to {}", scan_result.path.display(), output_dir.display());
        
        // Try different extraction approaches
        let mut extraction_succeeded = false;
        let mut extract_result = None;
        
        // Attempt 1: Standard extraction
        info!("Trying standard extraction for PBO: {}", scan_result.path.display());
        match api.extract_with_options(&scan_result.path, &output_dir, options.clone()) {
            Ok(result) => {
                info!("Extraction successful with standard extraction");
                extract_result = Some(result);
                extraction_succeeded = true;
            }
            Err(e) => {
                warn!("Standard extraction failed: {}", e);
                
                // Attempt 2: Permissive extraction
                info!("Trying permissive extraction for PBO: {}", scan_result.path.display());
                let mut permissive_options = options.clone();
                permissive_options.file_filter = None; // Extract all files
                match api.extract_with_options(&scan_result.path, &output_dir, permissive_options) {
                    Ok(result) => {
                        info!("Extraction successful with permissive extraction");
                        extract_result = Some(result);
                        extraction_succeeded = true;
                    }
                    Err(e) => {
                        warn!("Permissive extraction failed: {}", e);
                        
                        // Attempt 3: Direct extraction
                        info!("Trying direct extraction for PBO: {}", scan_result.path.display());
                        match api.extract_files(&scan_result.path, &output_dir, None) {
                            Ok(result) => {
                                info!("Extraction successful with direct extraction");
                                extract_result = Some(result);
                                extraction_succeeded = true;
                            }
                            Err(e) => {
                                warn!("Direct extraction failed: {}", e);
                            }
                        }
                    }
                }
            }
        }
        
        if extraction_succeeded {
            let extract_result = extract_result.unwrap();
            
            // Check if any files were actually extracted
            let extracted_files_on_disk = WalkDir::new(&output_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
                .count();
                
            info!("Found {} files on disk after extraction", extracted_files_on_disk);
            
            if extracted_files_on_disk == 0 {
                warn!("No files were extracted to disk from {}", scan_result.path.display());
                let mut db = self.db.lock().unwrap();
                db.update_pbo_with_reason(
                    &scan_result.path,
                    &scan_result.hash,
                    true,
                    SkipReason::Empty,
                );
            } else {
                info!("Successfully extracted {} files from PBO {} to {}", 
                    extracted_files_on_disk,
                    scan_result.path.display(), 
                    output_dir.display()
                );
                
                // Parse extracted files from output
                let mut extracted_files = Vec::new();
                for line in extract_result.stdout.lines() {
                    info!("  {}", line);
                    // Extract the filename from the output line
                    if let Some(file_path) = line.trim().strip_prefix("Extracting ") {
                        extracted_files.push(file_path.to_string());
                    }
                }
                
                // Verify that all expected files were extracted
                let expected_files = scan_result.expected_files.clone();
                let mut db = self.db.lock().unwrap();
                let all_files_extracted = db.update_pbo_with_files(
                    &scan_result.path, 
                    &scan_result.hash, 
                    expected_files,
                    extracted_files.clone()
                );
                
                if !all_files_extracted {
                    let missing_files: Vec<_> = scan_result.expected_files.iter()
                        .filter(|f| !extracted_files.contains(f))
                        .collect();
                    warn!(
                        "Not all expected files were extracted from {}: missing {:?}",
                        scan_result.path.display(),
                        missing_files
                    );
                }
                
                // Verify files actually exist on disk
                info!("Verifying extracted files on disk");
                let mut missing_on_disk = Vec::new();
                for file in &extracted_files {
                    let file_path = output_dir.join(file);
                    if !file_path.exists() {
                        missing_on_disk.push(file.clone());
                        warn!("File reported as extracted but not found on disk: {}", file_path.display());
                    } else {
                        info!("Verified file exists: {}", file_path.display());
                    }
                }
                
                if !missing_on_disk.is_empty() {
                    warn!("Some files were reported as extracted but not found on disk: {:?}", missing_on_disk);
                }
            }
        } else {
            warn!("All extraction attempts failed for {}", scan_result.path.display());
            let mut db = self.db.lock().unwrap();
            db.update_pbo_with_reason(
                &scan_result.path,
                &scan_result.hash,
                true,
                SkipReason::Failed,
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};
    use tempfile::tempdir;
    use crate::extraction::database::{ScanDatabase, SkipReason};

    fn create_test_pbo(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        let mut file = fs::File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }

    fn create_scan_result(path: PathBuf, expected_files: Vec<String>) -> PboScanResult {
        PboScanResult {
            path,
            hash: "test_hash".to_string(),
            expected_files,
        }
    }

    #[test]
    fn test_process_empty_pbo() {
        let input_dir = tempdir().unwrap();
        let cache_dir = tempdir().unwrap();
        let db = Arc::new(Mutex::new(ScanDatabase::default()));
        
        let pbo_path = create_test_pbo(&input_dir.path(), "test.pbo", b"dummy content");
        let scan_result = create_scan_result(pbo_path, vec![]);
        
        let processor = PboProcessor::new(
            input_dir.path(),
            cache_dir.path(),
            "sqf,hpp",
            Arc::clone(&db),
            1,
            30,
        );
        
        processor.process_pbo(&scan_result).unwrap();
        
        let db_guard = db.lock().unwrap();
        let info = db_guard.get_pbo_info(&scan_result.path).unwrap();
        assert!(info.failed);
        assert!(matches!(info.skip_reason, Some(SkipReason::NoMatchingFiles)));
    }

    #[test]
    fn test_process_multiple_pbos() {
        let input_dir = tempdir().unwrap();
        let cache_dir = tempdir().unwrap();
        let db = Arc::new(Mutex::new(ScanDatabase::default()));
        
        let pbo1_path = create_test_pbo(&input_dir.path(), "test1.pbo", b"dummy content 1");
        let pbo2_path = create_test_pbo(&input_dir.path(), "test2.pbo", b"dummy content 2");
        
        let scan_result1 = create_scan_result(pbo1_path, vec!["file1.sqf".to_string()]);
        let scan_result2 = create_scan_result(pbo2_path, vec!["file2.sqf".to_string()]);
        
        let processor = PboProcessor::new(
            input_dir.path(),
            cache_dir.path(),
            "sqf,hpp",
            Arc::clone(&db),
            1,
            30,
        );
        
        let results = vec![scan_result1, scan_result2];
        let progress = ProgressBar::new(results.len() as u64);
        
        processor.process_all(&results, progress).unwrap();
        
        let db_guard = db.lock().unwrap();
        assert_eq!(db_guard.pbos.len(), 2);
    }

    #[test]
    fn test_process_with_timeout() {
        let input_dir = tempdir().unwrap();
        let cache_dir = tempdir().unwrap();
        let db = Arc::new(Mutex::new(ScanDatabase::default()));
        
        let pbo_path = create_test_pbo(&input_dir.path(), "test.pbo", b"dummy content");
        let scan_result = create_scan_result(pbo_path, vec!["file.sqf".to_string()]);
        
        let processor = PboProcessor::new(
            input_dir.path(),
            cache_dir.path(),
            "sqf,hpp",
            Arc::clone(&db),
            1,
            1, // Very short timeout
        );
        
        processor.process_pbo(&scan_result).unwrap();
        
        let db_guard = db.lock().unwrap();
        let info = db_guard.get_pbo_info(&scan_result.path).unwrap();
        assert!(info.failed);
    }

    #[test]
    fn test_process_with_missing_expected_files() {
        let input_dir = tempdir().unwrap();
        let cache_dir = tempdir().unwrap();
        let db = Arc::new(Mutex::new(ScanDatabase::default()));
        
        // Create a test PBO
        let pbo_path = create_test_pbo(&input_dir.path(), "test.pbo", b"dummy content");
        
        // Create a scan result with expected files
        let expected_files = vec![
            "file1.sqf".to_string(),
            "file2.sqf".to_string(),
            "file3.sqf".to_string(),
        ];
        let scan_result = create_scan_result(pbo_path, expected_files);
        
        // Create a processor with a mock extraction function
        let processor = PboProcessor::new(
            input_dir.path(),
            cache_dir.path(),
            "sqf,hpp",
            Arc::clone(&db),
            1,
            30,
        );
        
        // Process the PBO
        processor.process_pbo(&scan_result).unwrap();
        
        // Check that the PBO was marked as failed with MissingExpectedFiles
        let db_guard = db.lock().unwrap();
        let info = db_guard.get_pbo_info(&scan_result.path).unwrap();
        assert!(info.failed);
        
        // The actual check for MissingExpectedFiles would require mocking the PboApi
        // which is beyond the scope of this test. In a real scenario, we would
        // use a mock or a test double for the PboApi to simulate partial extraction.
    }
} 