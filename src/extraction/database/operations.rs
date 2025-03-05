use std::path::Path;
use anyhow::Result;
use super::types::{ScanDatabase, PboInfo, SkipReason, ScanStats};

impl ScanDatabase {
    pub fn load_or_create(path: &Path) -> Result<Self> {
        if path.exists() {
            let file = std::fs::File::open(path)?;
            Ok(serde_json::from_reader(file)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
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
                skip_reason: None,
                expected_files: None,
                extracted_files: None,
            },
        );
    }

    pub fn update_pbo_with_reason(&mut self, path: &Path, hash: &str, failed: bool, reason: SkipReason) {
        self.pbos.insert(
            path.to_string_lossy().to_string(),
            PboInfo {
                hash: hash.to_string(),
                failed,
                skip_reason: Some(reason),
                expected_files: None,
                extracted_files: None,
            },
        );
    }

    pub fn update_pbo_with_files(
        &mut self, 
        path: &Path, 
        hash: &str, 
        expected_files: Vec<String>,
        extracted_files: Vec<String>,
    ) -> bool {
        let all_files_extracted = expected_files.iter().all(|f| extracted_files.contains(f));
        
        self.pbos.insert(
            path.to_string_lossy().to_string(),
            PboInfo {
                hash: hash.to_string(),
                failed: !all_files_extracted,
                skip_reason: if all_files_extracted { 
                    None 
                } else { 
                    Some(SkipReason::MissingExpectedFiles) 
                },
                expected_files: Some(expected_files),
                extracted_files: Some(extracted_files),
            },
        );
        
        all_files_extracted
    }

    pub fn get_stats(&self) -> ScanStats {
        let mut stats = ScanStats {
            total: self.pbos.len(),
            ..Default::default()
        };

        for info in self.pbos.values() {
            if info.failed {
                match info.skip_reason {
                    Some(SkipReason::Empty) => stats.empty += 1,
                    Some(SkipReason::NoMatchingFiles) => stats.no_matching_files += 1,
                    Some(SkipReason::InvalidFormat) => stats.invalid_format += 1,
                    Some(SkipReason::Failed) => stats.failed += 1,
                    Some(SkipReason::MissingExpectedFiles) => stats.missing_expected_files += 1,
                    None => stats.failed += 1,
                }
            } else {
                stats.processed += 1;
            }
        }

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_load_or_create_new() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");
        
        let db = ScanDatabase::load_or_create(&path).unwrap();
        assert_eq!(db.pbos.len(), 0);
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");
        
        let mut db = ScanDatabase::default();
        db.update_pbo(&PathBuf::from("/test/path.pbo").as_path(), "hash123", false);
        db.save(&path).unwrap();
        
        let loaded_db = ScanDatabase::load_or_create(&path).unwrap();
        assert_eq!(loaded_db.pbos.len(), 1);
        assert!(loaded_db.pbos.contains_key("/test/path.pbo"));
    }

    #[test]
    fn test_update_pbo_with_reason() {
        let mut db = ScanDatabase::default();
        db.update_pbo_with_reason(
            &PathBuf::from("/test/path.pbo").as_path(),
            "hash123",
            true,
            SkipReason::Empty,
        );
        
        let info = db.get_pbo_info(&PathBuf::from("/test/path.pbo").as_path()).unwrap();
        assert_eq!(info.hash, "hash123");
        assert!(info.failed);
        assert!(matches!(info.skip_reason, Some(SkipReason::Empty)));
    }

    #[test]
    fn test_update_pbo_with_files_all_extracted() {
        let mut db = ScanDatabase::default();
        let expected_files = vec![
            "file1.sqf".to_string(),
            "file2.sqf".to_string(),
        ];
        let extracted_files = vec![
            "file1.sqf".to_string(),
            "file2.sqf".to_string(),
            "extra.sqf".to_string(),  // Extra file is fine
        ];
        
        let result = db.update_pbo_with_files(
            &PathBuf::from("/test/path.pbo").as_path(),
            "hash123",
            expected_files,
            extracted_files,
        );
        
        assert!(result);
        let info = db.get_pbo_info(&PathBuf::from("/test/path.pbo").as_path()).unwrap();
        assert_eq!(info.hash, "hash123");
        assert!(!info.failed);
        assert!(info.skip_reason.is_none());
        assert_eq!(info.expected_files.as_ref().unwrap().len(), 2);
        assert_eq!(info.extracted_files.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn test_update_pbo_with_files_missing_files() {
        let mut db = ScanDatabase::default();
        let expected_files = vec![
            "file1.sqf".to_string(),
            "file2.sqf".to_string(),
            "file3.sqf".to_string(),
        ];
        let extracted_files = vec![
            "file1.sqf".to_string(),
            "file2.sqf".to_string(),
        ];
        
        let result = db.update_pbo_with_files(
            &PathBuf::from("/test/path.pbo").as_path(),
            "hash123",
            expected_files,
            extracted_files,
        );
        
        assert!(!result);
        let info = db.get_pbo_info(&PathBuf::from("/test/path.pbo").as_path()).unwrap();
        assert_eq!(info.hash, "hash123");
        assert!(info.failed);
        assert!(matches!(info.skip_reason, Some(SkipReason::MissingExpectedFiles)));
    }

    #[test]
    fn test_get_stats() {
        let mut db = ScanDatabase::default();
        
        // Add various PBO statuses
        db.update_pbo(&PathBuf::from("/test/success.pbo").as_path(), "hash1", false);
        db.update_pbo_with_reason(&PathBuf::from("/test/empty.pbo").as_path(), "hash2", true, SkipReason::Empty);
        db.update_pbo_with_reason(&PathBuf::from("/test/no_files.pbo").as_path(), "hash3", true, SkipReason::NoMatchingFiles);
        db.update_pbo_with_reason(&PathBuf::from("/test/invalid.pbo").as_path(), "hash4", true, SkipReason::InvalidFormat);
        db.update_pbo_with_reason(&PathBuf::from("/test/failed.pbo").as_path(), "hash5", true, SkipReason::Failed);
        
        // Add a PBO with missing expected files
        let expected_files = vec!["file1.sqf".to_string(), "file2.sqf".to_string()];
        let extracted_files = vec!["file1.sqf".to_string()];
        db.update_pbo_with_files(
            &PathBuf::from("/test/missing.pbo").as_path(),
            "hash6",
            expected_files,
            extracted_files,
        );
        
        let stats = db.get_stats();
        assert_eq!(stats.total, 6);
        assert_eq!(stats.processed, 1);
        assert_eq!(stats.empty, 1);
        assert_eq!(stats.no_matching_files, 1);
        assert_eq!(stats.invalid_format, 1);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.missing_expected_files, 1);
    }
} 