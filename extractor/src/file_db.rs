use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::collections::HashMap;
use log::{debug, info, warn};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::models::{PboMetadata, PboType};
use crate::error::{Result, CacheError};

/// Represents an extracted file in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedFileInfo {
    /// Relative path of the extracted file
    pub relative_path: PathBuf,
    
    /// ID of the PBO containing this file
    pub pbo_id: String,
    
    /// When the file was extracted
    pub extraction_time: DateTime<Utc>,
    
    /// File extension
    pub extension: String,
}

/// Represents a failed extraction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedExtraction {
    /// PBO ID
    pub pbo_id: String,
    
    /// Error message
    pub error_message: String,
    
    /// Timestamp of the failure
    pub timestamp: DateTime<Utc>,
}

/// Represents a PBO record in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PboRecord {
    /// Unique ID for the PBO (normalized path)
    pub id: String,
    
    /// Full path to the PBO file
    pub full_path: PathBuf,
    
    /// Base directory (optional)
    pub base_dir: Option<PathBuf>,
    
    /// File size in bytes
    pub file_size: u64,
    
    /// Last modified timestamp
    pub last_modified: DateTime<Utc>,
    
    /// Last extraction timestamp
    pub extraction_time: DateTime<Utc>,
    
    /// Type of PBO (game data or mission)
    pub pbo_type: PboType,
    
    /// Extensions used for extraction
    pub used_extensions: Vec<String>,
    
    /// Extracted files (relative paths)
    pub extracted_files: Vec<PathBuf>,
}

/// Database container structure
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FileDatabase {
    /// PBO records by ID
    pub pbos: HashMap<String, PboRecord>,
    
    /// Extracted files by relative path
    pub files: HashMap<String, ExtractedFileInfo>,
    
    /// Failed extractions by PBO ID
    pub failed_extractions: HashMap<String, FailedExtraction>,
}

/// Manager for file-based database operations
pub struct FileDbManager {
    /// Path to the database file
    db_path: PathBuf,
    
    /// In-memory database
    pub(crate) db: FileDatabase,
}

/// Normalize a path to use as a consistent ID
pub fn normalize_path(path: &Path) -> String {
    path.to_string_lossy()
        .to_lowercase()
        .replace('\\', "/")
}

impl FileDbManager {
    /// Create a new file database manager
    pub fn new(db_path: &Path) -> Result<Self> {
        debug!("Initializing file database manager with path: {}", db_path.display());
        
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| CacheError::CacheDirectory(format!("Failed to create database directory: {}", e)))?;
            }
        }
        
        // Try to load existing database
        let db = if db_path.exists() {
            match Self::load_database(db_path) {
                Ok(db) => db,
                Err(e) => {
                    warn!("Failed to load database, creating new one: {}", e);
                    FileDatabase::default()
                }
            }
        } else {
            // Create new database
            FileDatabase::default()
        };
        
        let manager = Self {
            db_path: db_path.to_path_buf(),
            db,
        };
        
        info!("File database manager initialized");
        Ok(manager)
    }
    
    /// Load the database from disk
    fn load_database(path: &Path) -> Result<FileDatabase> {
        let mut file = File::open(path)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to open database file: {}", e)))?;
        
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to read database file: {}", e)))?;
        
        serde_json::from_slice(&contents)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to parse database file: {}", e)))
    }
    
    /// Save the database to disk
    fn save_database(&self) -> Result<()> {
        let contents = serde_json::to_vec_pretty(&self.db)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to serialize database: {}", e)))?;
        
        let mut file = File::create(&self.db_path)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to create database file: {}", e)))?;
        
        file.write_all(&contents)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to write database file: {}", e)))?;
        
        debug!("Database saved to {}", self.db_path.display());
        Ok(())
    }
    
    /// Create PboRecord from PboMetadata
    fn create_pbo_record(metadata: &PboMetadata) -> PboRecord {
        // Convert SystemTime to DateTime<Utc>
        let last_modified = DateTime::<Utc>::from(metadata.last_modified);
        let extraction_time = DateTime::<Utc>::from(metadata.extraction_time);
        
        PboRecord {
            id: normalize_path(&metadata.get_full_path()),
            full_path: metadata.get_full_path(),
            base_dir: if metadata.base_dir.as_os_str().is_empty() {
                None
            } else {
                Some(metadata.base_dir.clone())
            },
            file_size: metadata.file_size,
            last_modified,
            extraction_time,
            pbo_type: metadata.pbo_type,
            used_extensions: metadata.used_extensions.clone(),
            extracted_files: metadata.extracted_files.clone(),
        }
    }
    
    /// Check if a PBO needs to be extracted
    pub fn needs_extraction(
        &self,
        path: &Path,
        _pbo_type: PboType,
        extensions: &[String],
    ) -> Result<bool> {
        if !path.exists() {
            return Ok(true);
        }
        
        // Normalize the path to use as ID
        let pbo_id = normalize_path(path);
        
        // Check if we have this PBO in the database
        if let Some(record) = self.db.pbos.get(&pbo_id) {
            // If the list of extensions has changed, we need to extract again
            // Sort both lists to ensure order doesn't matter
            let mut record_extensions = record.used_extensions.clone();
            let mut current_extensions = extensions.to_vec();
            
            record_extensions.sort();
            current_extensions.sort();
            
            if record_extensions != current_extensions {
                debug!("Extensions changed for {}, needs extraction", path.display());
                return Ok(true);
            }
            
            // Check if the file has been modified since last extraction
            let metadata = std::fs::metadata(path)
                .map_err(|e| CacheError::FileOperation(format!("Failed to get metadata: {}", e)))?;
            
            let current_modified = DateTime::<Utc>::from(metadata.modified()
                .map_err(|e| CacheError::FileOperation(format!("Failed to get modified time: {}", e)))?);
            
            if current_modified > record.last_modified {
                debug!("File modified since last extraction: {}", path.display());
                return Ok(true);
            }
            
            // No need to extract
            debug!("PBO does not need extraction: {}", path.display());
            Ok(false)
        } else {
            // Not in database, needs extraction
            debug!("PBO not in database, needs extraction: {}", path.display());
            Ok(true)
        }
    }
    
    /// Update metadata for a PBO
    pub fn update_metadata(&mut self, metadata: PboMetadata) -> Result<()> {
        // Convert to database record
        let pbo_record = Self::create_pbo_record(&metadata);
        let pbo_id = pbo_record.id.clone();
        
        // Add to database
        self.db.pbos.insert(pbo_id.clone(), pbo_record);
        
        // Update extracted files index
        for file_path in &metadata.extracted_files {
            let path_str = file_path.to_string_lossy().to_string();
            let extension = file_path.extension()
                .map(|ext| ext.to_string_lossy().to_string())
                .unwrap_or_default();
            
            let file_info = ExtractedFileInfo {
                relative_path: file_path.clone(),
                pbo_id: pbo_id.clone(),
                extraction_time: DateTime::<Utc>::from(metadata.extraction_time),
                extension,
            };
            
            self.db.files.insert(path_str, file_info);
        }
        
        // Save changes to disk
        self.save_database()?;
        
        debug!("Updated metadata for PBO: {}", metadata.path.display());
        Ok(())
    }
    
    /// Record a failed extraction
    pub fn add_failed_extraction(&mut self, path: &Path, error_message: String) -> Result<()> {
        let pbo_id = normalize_path(path);
        let now = Utc::now();
        
        // Create failed extraction record
        let failed_extraction = FailedExtraction {
            pbo_id: pbo_id.clone(),
            error_message,
            timestamp: now,
        };
        
        // Add to database
        self.db.failed_extractions.insert(pbo_id, failed_extraction);
        
        // Save changes to disk
        self.save_database()?;
        
        debug!("Recorded failed extraction for {}", path.display());
        Ok(())
    }
    
    /// Check if a PBO has a failed extraction record
    pub fn is_failed_extraction(&self, path: &Path) -> Result<Option<(String, String)>> {
        let pbo_id = normalize_path(path);
        
        if let Some(failed) = self.db.failed_extractions.get(&pbo_id) {
            Ok(Some((failed.timestamp.to_rfc3339(), failed.error_message.clone())))
        } else {
            Ok(None)
        }
    }
    
    /// Find the source PBO for a file path
    pub fn find_pbo_for_file(&self, file_path: &Path) -> Result<Option<PboRecord>> {
        // Try to match against relative paths in our file index
        let path_str = file_path.to_string_lossy().to_string();
        
        if let Some(file_info) = self.db.files.get(&path_str) {
            // Return the associated PBO
            if let Some(pbo) = self.db.pbos.get(&file_info.pbo_id) {
                return Ok(Some(pbo.clone()));
            }
        }
        
        // Try matching by suffix if the exact path wasn't found
        for (path, file_info) in &self.db.files {
            if path_str.ends_with(path) {
                if let Some(pbo) = self.db.pbos.get(&file_info.pbo_id) {
                    return Ok(Some(pbo.clone()));
                }
            }
        }
        
        Ok(None)
    }
    
    /// Find all files with a specific extension
    pub fn find_files_by_extension(&self, extension: &str) -> Result<Vec<ExtractedFileInfo>> {
        let mut result = Vec::new();
        
        for file_info in self.db.files.values() {
            if file_info.extension == extension {
                result.push(file_info.clone());
            }
        }
        
        Ok(result)
    }
    
    /// Get all game data PBO records
    pub fn get_game_data_metadata(&self) -> Result<Vec<PboRecord>> {
        let result: Vec<PboRecord> = self.db.pbos.values()
            .filter(|pbo| pbo.pbo_type == PboType::GameData)
            .cloned()
            .collect();
        
        Ok(result)
    }
    
    /// Get all mission PBO records
    pub fn get_mission_metadata(&self) -> Result<Vec<PboRecord>> {
        let result: Vec<PboRecord> = self.db.pbos.values()
            .filter(|pbo| pbo.pbo_type == PboType::Mission)
            .cloned()
            .collect();
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;
    
    // Helper function to create a mock PBO file
    fn create_mock_pbo(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(b"Mock PBO content").unwrap();
        path
    }
    
    // Helper function to create PboMetadata
    fn create_test_metadata(
        pbo_path: &Path, 
        pbo_type: PboType, 
        extracted_files: Vec<PathBuf>, 
        extensions: Vec<String>
    ) -> PboMetadata {
        // Create basic metadata
        let mut metadata = PboMetadata::new(
            pbo_path.to_path_buf(),
            pbo_type,
            extensions,
        ).unwrap();
        
        // Add extracted files
        metadata.extracted_files = extracted_files;
        
        metadata
    }
    
    #[test]
    fn test_update_metadata() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let mut db_manager = FileDbManager::new(&db_path).unwrap();
        
        // Create a mock PBO
        let pbo_path = create_mock_pbo(temp_dir.path(), "test.pbo");
        
        // Create metadata with extracted files
        let extracted_files = vec![
            PathBuf::from("file1.sqf"),
            PathBuf::from("file2.hpp"),
        ];
        let metadata = create_test_metadata(
            &pbo_path,
            PboType::GameData,
            extracted_files.clone(),
            vec!["sqf".to_string(), "hpp".to_string()],
        );
        
        // Update metadata
        db_manager.update_metadata(metadata.clone()).unwrap();
        
        // Verify the database was saved
        assert!(db_path.exists());
        
        // Load the database again to verify persistence
        let db_manager2 = FileDbManager::new(&db_path).unwrap();
        
        // Check if the PBO is in the database
        let pbo_id = normalize_path(&pbo_path);
        assert!(db_manager2.db.pbos.contains_key(&pbo_id));
        
        // Check if extracted files are in the database
        for file in &extracted_files {
            let file_str = file.to_string_lossy().to_string();
            assert!(db_manager2.db.files.contains_key(&file_str));
        }
    }
    
    #[test]
    fn test_needs_extraction() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let mut db_manager = FileDbManager::new(&db_path).unwrap();
        
        // Create a mock PBO
        let pbo_path = create_mock_pbo(temp_dir.path(), "test.pbo");
        
        // Initially should need extraction
        assert!(db_manager.needs_extraction(&pbo_path, PboType::GameData, &["sqf".to_string()]).unwrap());
        
        // Add to database
        let metadata = create_test_metadata(
            &pbo_path,
            PboType::GameData,
            vec![],
            vec!["sqf".to_string()],
        );
        db_manager.update_metadata(metadata).unwrap();
        
        // Now should not need extraction with same extensions
        assert!(!db_manager.needs_extraction(&pbo_path, PboType::GameData, &["sqf".to_string()]).unwrap());
        
        // Should need extraction with different extensions
        assert!(db_manager.needs_extraction(&pbo_path, PboType::GameData, &["hpp".to_string()]).unwrap());
        
        // Test file modification triggering extraction
        // Touch the file to update its modification time
        let mut file = File::create(&pbo_path).unwrap();
        file.write_all(b"Updated content").unwrap();
        
        // Should need extraction again
        assert!(db_manager.needs_extraction(&pbo_path, PboType::GameData, &["sqf".to_string()]).unwrap());
    }
    
    #[test]
    fn test_find_pbo_for_file() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let mut db_manager = FileDbManager::new(&db_path).unwrap();
        
        // Create a mock PBO
        let pbo_path = create_mock_pbo(temp_dir.path(), "test.pbo");
        
        // Create metadata with extracted files
        let extracted_files = vec![
            PathBuf::from("scripts/test.sqf"),
            PathBuf::from("config.hpp"),
        ];
        let metadata = create_test_metadata(
            &pbo_path,
            PboType::GameData,
            extracted_files.clone(),
            vec!["sqf".to_string(), "hpp".to_string()],
        );
        
        // Update metadata
        db_manager.update_metadata(metadata.clone()).unwrap();
        
        // Find PBO for file
        let file_path = PathBuf::from("scripts/test.sqf");
        let found_pbo = db_manager.find_pbo_for_file(&file_path).unwrap();
        
        assert!(found_pbo.is_some());
        let found_pbo = found_pbo.unwrap();
        assert_eq!(found_pbo.full_path, pbo_path);
        
        // Find PBO for file using suffix match
        let file_path = PathBuf::from("some/other/path/scripts/test.sqf");
        let found_pbo = db_manager.find_pbo_for_file(&file_path).unwrap();
        
        assert!(found_pbo.is_some());
        let found_pbo = found_pbo.unwrap();
        assert_eq!(found_pbo.full_path, pbo_path);
        
        // File that doesn't exist
        let file_path = PathBuf::from("nonexistent.sqf");
        let found_pbo = db_manager.find_pbo_for_file(&file_path).unwrap();
        assert!(found_pbo.is_none());
    }
} 