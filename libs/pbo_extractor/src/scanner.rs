use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use log::{debug, info, warn};
use rayon::prelude::*;
use crate::error::Result;

/// Scanner for finding PBO files in directories
pub struct PboScanner;

impl PboScanner {
    /// Find all PBO files in the given directories
    pub fn find_pbos(dirs: &[PathBuf]) -> Result<Vec<PathBuf>> {
        debug!("Scanning {} directories for PBO files", dirs.len());
        
        let pbos: Vec<PathBuf> = dirs.par_iter()
            .filter_map(|dir| {
                if !dir.exists() {
                    warn!("Directory does not exist: {}", dir.display());
                    return None;
                }
                
                // Find all .pbo files
                let mut files = Vec::new();
                for entry in WalkDir::new(dir)
                    .follow_links(true)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    let path = entry.path();
                    if path.is_file() && 
                       path.extension().is_some_and(|ext| ext == "pbo") {
                        debug!("Found PBO: {}", path.display());
                        files.push(path.to_path_buf());
                    }
                }
                
                Some(files)
            })
            .flatten()
            .collect();
        
        info!("Found {} PBO files", pbos.len());
        Ok(pbos)
    }
    
    /// Filter PBOs that need extraction based on the provided function
    pub fn filter_needs_extraction<F>(
        pbos: &[PathBuf],
        needs_extraction_fn: F,
    ) -> Result<Vec<PathBuf>> 
    where
        F: Fn(&Path) -> Result<bool> + Sync + Send,
    {
        debug!("Checking {} PBOs for extraction", pbos.len());
        
        // Use try_fold to collect results and propagate errors if all checks fail
        let filtered: Vec<PathBuf> = pbos.par_iter()
            .filter_map(|path| {
                match needs_extraction_fn(path) {
                    Ok(true) => Some(path.clone()),
                    Ok(false) => None,
                    Err(e) => {
                        // Only log a warning but don't fail the entire operation
                        // This allows some PBOs to fail checks while others succeed
                        warn!("Error checking PBO {}: {}", path.display(), e);
                        None
                    }
                }
            })
            .collect();
        
        info!("{} PBOs need extraction", filtered.len());
        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;
    
    fn create_test_pbo(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(b"PBO content").unwrap();
        path
    }
    
    #[test]
    fn test_find_pbos() {
        let temp_dir = tempdir().unwrap();
        let path1 = create_test_pbo(temp_dir.path(), "test1.pbo");
        let path2 = create_test_pbo(temp_dir.path(), "test2.pbo");
        
        // Create a subdirectory
        let sub_dir = temp_dir.path().join("subdir");
        std::fs::create_dir(&sub_dir).unwrap();
        let path3 = create_test_pbo(&sub_dir, "test3.pbo");
        
        // Create a non-PBO file
        let non_pbo = temp_dir.path().join("not_a_pbo.txt");
        let mut file = File::create(&non_pbo).unwrap();
        file.write_all(b"Not a PBO").unwrap();
        
        let dirs = vec![temp_dir.path().to_path_buf()];
        let pbos = PboScanner::find_pbos(&dirs).unwrap();
        
        assert_eq!(pbos.len(), 3);
        assert!(pbos.contains(&path1));
        assert!(pbos.contains(&path2));
        assert!(pbos.contains(&path3));
        assert!(!pbos.contains(&non_pbo));
    }
    
    #[test]
    fn test_filter_needs_extraction() {
        let temp_dir = tempdir().unwrap();
        let path1 = create_test_pbo(temp_dir.path(), "test1.pbo");
        let path2 = create_test_pbo(temp_dir.path(), "test2.pbo");
        let path3 = create_test_pbo(temp_dir.path(), "test3.pbo");
        
        let pbos = vec![path1.clone(), path2.clone(), path3.clone()];
        
        // Function that says only test1 and test3 need extraction
        let needs_extraction = |path: &Path| -> Result<bool> {
            Ok(path.file_name().unwrap() != path2.file_name().unwrap())
        };
        
        let filtered = PboScanner::filter_needs_extraction(&pbos, needs_extraction).unwrap();
        
        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains(&path1));
        assert!(!filtered.contains(&path2));
        assert!(filtered.contains(&path3));
    }
} 