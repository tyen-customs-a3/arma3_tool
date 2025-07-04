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
#[derive(Debug)]
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
    /// Create a new file database manager, loading from db_path if it exists.
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
        let db = match Self::load(db_path) {
            Ok(db) => {
                info!("Successfully loaded existing database from: {}", db_path.display());
                db
            }
            Err(e) => {
                if db_path.exists() {
                     // Log if file exists but failed to load
                    warn!("Failed to load existing database file at {}: {}. Creating a new empty database.", db_path.display(), e);
                } else {
                    // Log if file doesn't exist
                    info!("No existing database found at {}. Creating a new empty database.", db_path.display());
                }
                FileDatabase::default()
            }
        };
        
        let manager = Self {
            db_path: db_path.to_path_buf(),
            db,
        };
        
        info!("File database manager initialized successfully.");
        Ok(manager)
    }
    
    /// Load the database state from a specified JSON file.
    ///
    /// Returns an error if the file doesn't exist or cannot be parsed.
    pub fn load(path: &Path) -> Result<FileDatabase> {
         if !path.exists() {
            return Err(CacheError::IndexOperation(format!("Database file not found: {}", path.display())));
        }

        let mut file = File::open(path)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to open database file '{}': {}", path.display(), e)))?;

        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to read database file '{}': {}", path.display(), e)))?;

        // Attempt to handle empty file gracefully
        if contents.is_empty() {
            warn!("Database file '{}' is empty. Returning default database.", path.display());
            return Ok(FileDatabase::default());
        }

        serde_json::from_slice(&contents)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to parse database file '{}': {}", path.display(), e)))
    }
    
    /// Save the current database state to a JSON file.
    ///
    /// If `target_path` is `None`, saves to the `db_path` configured during initialization.
    /// If `target_path` is `Some`, saves to the specified path.
    pub fn save(&self, target_path: Option<&Path>) -> Result<()> {
        let path_to_save = target_path.unwrap_or(&self.db_path);

        // Ensure parent directory exists for the target path
        if let Some(parent) = path_to_save.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| CacheError::CacheDirectory(format!("Failed to create directory for saving database at '{}': {}", parent.display(), e)))?;
            }
        }

        let contents = serde_json::to_vec_pretty(&self.db)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to serialize database: {}", e)))?;

        let mut file = File::create(path_to_save)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to create database file at '{}': {}", path_to_save.display(), e)))?;

        file.write_all(&contents)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to write database file at '{}': {}", path_to_save.display(), e)))?;

        debug!("Database saved successfully to {}", path_to_save.display());
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
        self.save(None)?;
        
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
        self.save(None)?;
        
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
    use std::time::{Duration, SystemTime};
    
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
        // Sleep briefly to ensure different modification time
        std::thread::sleep(std::time::Duration::from_millis(10));
        
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

    #[test]
    fn test_save_and_load_database() {
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
        
        // Save the database
        db_manager.save(None).unwrap();
        
        // Load the database again
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
    fn test_normalize_path() {
        let path1 = PathBuf::from("C:\\Users\\Test\\File.PBO");
        assert_eq!(normalize_path(&path1), "c:/users/test/file.pbo");

        let path2 = PathBuf::from("/home/user/test.pbo");
        assert_eq!(normalize_path(&path2), "/home/user/test.pbo");

        let path3 = PathBuf::from("Relative/Path/To/File.PBO");
        assert_eq!(normalize_path(&path3), "relative/path/to/file.pbo");
    }

    #[test]
    fn test_failed_extraction() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("failed_test.db");
        let mut db_manager = FileDbManager::new(&db_path).unwrap();

        let pbo_path = create_mock_pbo(temp_dir.path(), "fail.pbo");

        // Initially, no failed record
        assert!(db_manager.is_failed_extraction(&pbo_path).unwrap().is_none());

        // Add a failed extraction record
        let error_msg = "Extraction timed out".to_string();
        db_manager.add_failed_extraction(&pbo_path, error_msg.clone()).unwrap();

        // Verify the record exists
        let failed_info = db_manager.is_failed_extraction(&pbo_path).unwrap();
        assert!(failed_info.is_some());
        let (timestamp_str, message) = failed_info.unwrap();
        assert_eq!(message, error_msg);
        assert!(DateTime::parse_from_rfc3339(&timestamp_str).is_ok());

        // Load again to ensure persistence
        let db_manager2 = FileDbManager::new(&db_path).unwrap();
        let failed_info2 = db_manager2.is_failed_extraction(&pbo_path).unwrap();
        assert!(failed_info2.is_some());
        assert_eq!(failed_info2.unwrap().1, error_msg);
    }

    #[test]
    fn test_find_files_by_extension() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("find_ext.db");
        let mut db_manager = FileDbManager::new(&db_path).unwrap();

        let pbo_path = create_mock_pbo(temp_dir.path(), "ext_test.pbo");
        let file1 = PathBuf::from("script.sqf");
        let file2 = PathBuf::from("config.hpp");
        let file3 = PathBuf::from("another.sqf");

        let metadata = create_test_metadata(
            &pbo_path,
            PboType::GameData,
            vec![file1.clone(), file2.clone(), file3.clone()],
            vec!["sqf".to_string(), "hpp".to_string()],
        );
        db_manager.update_metadata(metadata).unwrap();

        // Find sqf files
        let sqf_files = db_manager.find_files_by_extension("sqf").unwrap();
        assert_eq!(sqf_files.len(), 2);
        assert!(sqf_files.iter().any(|f| f.relative_path == file1));
        assert!(sqf_files.iter().any(|f| f.relative_path == file3));

        // Find hpp files
        let hpp_files = db_manager.find_files_by_extension("hpp").unwrap();
        assert_eq!(hpp_files.len(), 1);
        assert_eq!(hpp_files[0].relative_path, file2);

        // Find non-existent extension
        let txt_files = db_manager.find_files_by_extension("txt").unwrap();
        assert!(txt_files.is_empty());
    }

    #[test]
    fn test_get_metadata_by_type() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("get_type.db");
        let mut db_manager = FileDbManager::new(&db_path).unwrap();

        let game_pbo_path = create_mock_pbo(temp_dir.path(), "game.pbo");
        let mission_pbo_path = create_mock_pbo(temp_dir.path(), "mission.pbo");

        let metadata_game = create_test_metadata(&game_pbo_path, PboType::GameData, vec![], vec![]);
        let metadata_mission = create_test_metadata(&mission_pbo_path, PboType::Mission, vec![], vec![]);

        db_manager.update_metadata(metadata_game.clone()).unwrap();
        db_manager.update_metadata(metadata_mission.clone()).unwrap();

        // Get GameData metadata
        let game_data = db_manager.get_game_data_metadata().unwrap();
        assert_eq!(game_data.len(), 1);
        assert_eq!(game_data[0].id, normalize_path(&game_pbo_path));
        assert_eq!(game_data[0].pbo_type, PboType::GameData);

        // Get Mission metadata
        let mission_data = db_manager.get_mission_metadata().unwrap();
        assert_eq!(mission_data.len(), 1);
        assert_eq!(mission_data[0].id, normalize_path(&mission_pbo_path));
        assert_eq!(mission_data[0].pbo_type, PboType::Mission);
    }

    #[test]
    fn test_update_existing_metadata() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("update_existing.db");
        let mut db_manager = FileDbManager::new(&db_path).unwrap();

        let pbo_path = create_mock_pbo(temp_dir.path(), "update_me.pbo");

        // Initial metadata
        let file1 = PathBuf::from("initial.sqf");
        let initial_metadata = create_test_metadata(
            &pbo_path,
            PboType::GameData,
            vec![file1.clone()],
            vec!["sqf".to_string()],
        );
        db_manager.update_metadata(initial_metadata).unwrap();

        // Verify initial state
        assert_eq!(db_manager.db.pbos.len(), 1);
        assert_eq!(db_manager.db.files.len(), 1);
        assert!(db_manager.db.files.contains_key(&normalize_path(&file1)));
        let initial_record = db_manager.db.pbos.get(&normalize_path(&pbo_path)).unwrap();
        assert_eq!(initial_record.extracted_files.len(), 1);

        // Update metadata for the same PBO with different files/extensions
        let file2 = PathBuf::from("new_config.hpp");
        let updated_metadata = create_test_metadata(
            &pbo_path, // Same PBO path
            PboType::GameData,
            vec![file2.clone()],
            vec!["hpp".to_string()], // Different extension
        );
        // Ensure extraction time is different for update check
        let mut updated_metadata = updated_metadata;
        updated_metadata.extraction_time = SystemTime::now() + Duration::from_secs(1);

        db_manager.update_metadata(updated_metadata.clone()).unwrap();

        // Verify updated state
        let loaded_manager = FileDbManager::new(&db_path).unwrap(); // Reload to check persistence
        assert_eq!(loaded_manager.db.pbos.len(), 1); // Should still be 1 PBO record
        // Note: The file index currently doesn't remove old files, only adds/updates.
        // This might be desired or not depending on exact requirements.
        // For now, we check that the new file is present.
        assert!(loaded_manager.db.files.contains_key(&normalize_path(&file2)));
        // Optionally, check if the old file is still there (current behavior)
        // assert!(loaded_manager.db.files.contains_key(&normalize_path(&file1)));

        let updated_record = loaded_manager.db.pbos.get(&normalize_path(&pbo_path)).unwrap();
        assert_eq!(updated_record.used_extensions, vec!["hpp".to_string()]);
        assert_eq!(updated_record.extracted_files.len(), 1);
        assert_eq!(updated_record.extracted_files[0], file2);
        assert_eq!(updated_record.extraction_time, DateTime::<Utc>::from(updated_metadata.extraction_time));
    }
} 