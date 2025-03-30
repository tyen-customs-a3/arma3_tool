use std::path::{Path, PathBuf};
use log::{debug, info, warn};
use arma3_db::{
    DatabaseManager,
    models::{
        CacheConfig,
        pbo::{PboModel, PboType as DbPboType, normalize_path, ExtractedFile},
    },
    queries::cache::Cache,
};
use crate::models::{PboMetadata, PboType};
use crate::error::{Result, CacheError};

/// Manager for database operations related to PBO caching
pub struct DbManager {
    /// The PBO repository
    pub pbo_repo: Cache<'static>,
}

impl DbManager {
    /// Create a new database manager
    pub fn new(
        cache_dir: &Path,
        game_data_cache_dir: &Path,
        mission_cache_dir: &Path,
        db_path: &Path,
    ) -> Result<Self> {
        debug!("Initializing database manager with path: {}", db_path.display());
        
        let config = CacheConfig::with_cache_dirs(
            db_path.to_path_buf(),
            cache_dir.to_path_buf(),
            game_data_cache_dir.to_path_buf(),
            mission_cache_dir.to_path_buf(),
        );
        
        let db_manager = match DatabaseManager::with_config(config) {
            Ok(manager) => manager,
            Err(e) => return Err(CacheError::IndexOperation(format!("Failed to create database manager: {}", e))),
        };
        
        // We need to leak the DatabaseManager to satisfy the lifetime requirements
        // This is safe because the DbManager owns the DatabaseManager and will live
        // for the entire application lifetime
        let static_db_manager: &'static DatabaseManager = Box::leak(Box::new(db_manager));
        
        let pbo_repo = Cache::new(static_db_manager);
        
        info!("Database manager initialized");
        
        Ok(Self {
            pbo_repo,
        })
    }
    
    /// Create PboModel from PboMetadata
    fn create_pbo_model(metadata: &PboMetadata) -> PboModel {
        PboModel::from_pbo_metadata(metadata)
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
        
        // Normalize the path
        let pbo_id = normalize_path(path);
        
        // Check if extraction is needed
        self.pbo_repo.needs_extraction(&pbo_id, extensions)
            .map_err(|e| {
                let err_msg = format!("Error checking extraction status for {}: {}", path.display(), e);
                warn!("{}", err_msg);
                CacheError::IndexOperation(err_msg)
            })
    }
    
    /// Update metadata for a PBO
    pub fn update_metadata(&self, metadata: PboMetadata) -> Result<()> {
        // Convert to database model
        let pbo_model = Self::create_pbo_model(&metadata);
        
        // Create or update PBO model
        self.pbo_repo.create(&pbo_model)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to update metadata: {}", e)))?;
            
        debug!("Updated metadata for PBO: {}", metadata.path.display());
        
        // Update extraction info (extracted files and extensions)
        let pbo_id = normalize_path(&metadata.path);
        let extracted_files: Vec<PathBuf> = metadata.extracted_files.iter().cloned().collect();
        
        self.pbo_repo.update_extraction_info(
            &pbo_id,
            PboModel::from_pbo_metadata(&metadata).extraction_time,
            &extracted_files,
            &metadata.used_extensions,
        )
        .map_err(|e| CacheError::IndexOperation(format!("Failed to update extracted files: {}", e)))
    }
    
    /// Record a failed extraction
    pub fn add_failed_extraction(&self, path: &Path, error_message: String) -> Result<()> {
        let pbo_id = normalize_path(path);
        
        // First, try to get the PBO record to see if it exists
        let pbo_exists = match self.pbo_repo.get(&pbo_id) {
            Ok(Some(_)) => true,
            _ => false
        };
        
        // If the PBO record doesn't exist, create it
        if !pbo_exists {
            debug!("Creating minimal PBO record for failed extraction: {}", path.display());
            let now = chrono::Utc::now();
            let pbo_type = if path.to_string_lossy().to_lowercase().contains("mission") {
                DbPboType::Mission
            } else {
                DbPboType::GameData
            };
            
            // Create new PBO model with minimal required fields
            let pbo = PboModel {
                id: pbo_id.clone(),
                full_path: path.to_path_buf(),
                base_dir: path.parent().map(|p| p.to_path_buf()),
                file_size: 0,
                last_modified: now,
                extraction_time: now,
                pbo_type,
            };
            
            // Create the PBO record
            if let Err(e) = self.pbo_repo.create(&pbo) {
                warn!("Failed to create PBO record for failed extraction: {}", e);
                return Err(CacheError::IndexOperation(format!("Failed to create PBO record for failed extraction: {}", e)));
            }
            
            // Verify the PBO record was created successfully
            match self.pbo_repo.get(&pbo_id) {
                Ok(Some(_)) => {
                    debug!("Successfully created PBO record for {}", pbo_id);
                },
                Ok(None) => {
                    warn!("PBO record not found after creation for {}", pbo_id);
                    return Err(CacheError::IndexOperation(format!("PBO record not found after creation: {}", pbo_id)));
                },
                Err(e) => {
                    warn!("Error verifying PBO record for {}: {}", pbo_id, e);
                    return Err(CacheError::IndexOperation(format!("Error verifying PBO record: {}", e)));
                }
            }
        }

        // Now record the failed extraction directly using a custom SQL query to avoid foreign key issues
        match self.pbo_repo.record_failed_extraction_direct(&pbo_id, &error_message) {
            Ok(_) => {
                debug!("Successfully recorded failed extraction for {}", pbo_id);
                Ok(())
            },
            Err(e) => {
                warn!("Failed to record extraction failure for {}: {}", path.display(), e);
                Err(CacheError::IndexOperation(format!("Failed to record failed extraction: {}", e)))
            }
        }
    }
    
    /// Check if a PBO has a failed extraction record
    pub fn is_failed_extraction(&self, path: &Path) -> Result<Option<(String, String)>> {
        let pbo_id = normalize_path(path);
        
        match self.pbo_repo.get_failed_extraction(&pbo_id) {
            Ok(Some(failed_extraction)) => {
                let timestamp = failed_extraction.timestamp.to_rfc3339();
                let error_message = failed_extraction.error_message;
                Ok(Some((timestamp, error_message)))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(CacheError::IndexOperation(format!("Failed to check failed extraction: {}", e)))
        }
    }
    
    /// Find the source PBO for a file path
    pub fn find_pbo_for_file(&self, file_path: &Path) -> Result<Option<PboModel>> {
        self.pbo_repo.find_pbo_by_file_path(file_path)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to find PBO for file: {}", e)))
    }
    
    /// Find all files with a specific extension
    pub fn find_files_by_extension(&self, extension: &str) -> Result<Vec<ExtractedFile>> {
        self.pbo_repo.find_files_by_extension(extension)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to find files by extension: {}", e)))
    }
    
    /// Get all game data metadata
    pub fn get_game_data_metadata(&self) -> Result<Vec<PboModel>> {
        self.pbo_repo.get_by_type(DbPboType::GameData)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to get game data metadata: {}", e)))
    }
    
    /// Get all mission metadata
    pub fn get_mission_metadata(&self) -> Result<Vec<PboModel>> {
        self.pbo_repo.get_by_type(DbPboType::Mission)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to get mission metadata: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    
    // Helper function to create a mock PBO file
    fn create_mock_pbo(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(b"Mock PBO content").unwrap();
        path
    }
    
    // Helper function to create PboMetadata with extraction details
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
    fn test_new_db_manager() {
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        let db_path = cache_dir.join("test.db");
        
        // Create cache directories
        fs::create_dir_all(&game_data_cache_dir).unwrap();
        fs::create_dir_all(&mission_cache_dir).unwrap();
        
        // Create manager
        let manager = DbManager::new(
            &cache_dir,
            &game_data_cache_dir,
            &mission_cache_dir,
            &db_path,
        ).unwrap();
        
        // Check that the database was created
        assert!(db_path.exists());
        
        // Check that we can get metadata
        let game_data = manager.get_game_data_metadata().unwrap();
        let missions = manager.get_mission_metadata().unwrap();
        
        assert!(game_data.is_empty());
        assert!(missions.is_empty());
    }
    
    #[test]
    fn test_update_metadata() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        let db_path = cache_dir.join("test.db");
        
        // Create cache directories
        fs::create_dir_all(&game_data_cache_dir).unwrap();
        fs::create_dir_all(&mission_cache_dir).unwrap();
        
        // Create mock PBO file
        let pbo_path = create_mock_pbo(temp_dir.path(), "test_game.pbo");
        
        // Create extracted files (in relative path format)
        let extracted_files = vec![
            PathBuf::from("test_game/file1.sqf"),
            PathBuf::from("test_game/file2.hpp"),
        ];
        
        // Create manager
        let manager = DbManager::new(
            &cache_dir,
            &game_data_cache_dir,
            &mission_cache_dir,
            &db_path,
        ).unwrap();
        
        // Create metadata
        let extensions = vec!["sqf".to_string(), "hpp".to_string()];
        let metadata = create_test_metadata(
            &pbo_path, 
            PboType::GameData, 
            extracted_files,
            extensions.clone(),
        );
        
        // Update metadata in database
        manager.update_metadata(metadata).unwrap();
        
        // Verify that metadata was stored
        let game_data = manager.get_game_data_metadata().unwrap();
        assert_eq!(game_data.len(), 1);
        assert_eq!(normalize_path(&pbo_path), game_data[0].id);
        
        // Verify no mission data was created
        let missions = manager.get_mission_metadata().unwrap();
        assert!(missions.is_empty());
        
        // Verify extracted files were stored
        let pbo_id = normalize_path(&pbo_path);
        let extracted_files_result = manager.pbo_repo.get_extracted_files(&pbo_id).unwrap();
        assert_eq!(extracted_files_result.len(), 2);
    }
    
    #[test]
    fn test_failed_extraction() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        let db_path = cache_dir.join("test.db");
        
        // Create cache directories
        fs::create_dir_all(&game_data_cache_dir).unwrap();
        fs::create_dir_all(&mission_cache_dir).unwrap();
        
        // Create mock PBO file
        let pbo_path = create_mock_pbo(temp_dir.path(), "failed.pbo");
        
        // Create manager
        let manager = DbManager::new(
            &cache_dir,
            &game_data_cache_dir,
            &mission_cache_dir,
            &db_path,
        ).unwrap();
        
        // First create a valid PBO model in the database
        // Create metadata
        let extensions = vec!["sqf".to_string()];
        let metadata = create_test_metadata(
            &pbo_path, 
            PboType::GameData, 
            vec![],
            extensions,
        );
        
        // Update metadata in database to create the PBO entry
        manager.update_metadata(metadata).unwrap();
        
        // Now record failed extraction
        let error_message = "Test extraction failure".to_string();
        manager.add_failed_extraction(&pbo_path, error_message.clone()).unwrap();
    }
    
    #[test]
    fn test_needs_extraction() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        let db_path = cache_dir.join("test.db");
        
        // Create cache directories
        fs::create_dir_all(&game_data_cache_dir).unwrap();
        fs::create_dir_all(&mission_cache_dir).unwrap();
        
        // Create mock PBO file
        let pbo_path = create_mock_pbo(temp_dir.path(), "test_extraction.pbo");
        
        // Create manager
        let manager = DbManager::new(
            &cache_dir,
            &game_data_cache_dir,
            &mission_cache_dir,
            &db_path,
        ).unwrap();
        
        // Test with non-existent PBO
        let non_existent_path = temp_dir.path().join("non_existent.pbo");
        let result = manager.needs_extraction(
            &non_existent_path, 
            PboType::GameData, 
            &["sqf".to_string()]
        ).unwrap();
        assert!(result, "Non-existent PBO should need extraction");
        
        // Test with new PBO (not in database)
        let result = manager.needs_extraction(
            &pbo_path, 
            PboType::GameData, 
            &["sqf".to_string()]
        ).unwrap();
        assert!(result, "New PBO should need extraction");
        
        // Add PBO to database then test again
        let extracted_files = vec![
            PathBuf::from("test_extraction/file1.sqf"),
        ];
        let extensions = vec!["sqf".to_string()];
        let metadata = create_test_metadata(
            &pbo_path, 
            PboType::GameData, 
            extracted_files,
            extensions.clone(),
        );
        
        // Update metadata
        manager.update_metadata(metadata).unwrap();
        
        // Now it shouldn't need extraction with same extensions
        let result = manager.needs_extraction(
            &pbo_path, 
            PboType::GameData, 
            &["sqf".to_string()]
        ).unwrap();
        assert!(!result, "PBO with up-to-date metadata and same extensions should not need extraction");
        
        // But it should need extraction with different extensions
        let result = manager.needs_extraction(
            &pbo_path, 
            PboType::GameData, 
            &["sqf".to_string(), "hpp".to_string()]
        ).unwrap();
        assert!(result, "PBO with different extensions should need extraction");
    }
    
    #[test]
    fn test_add_failed_extraction_foreign_key() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        let db_path = cache_dir.join("test.db");
        
        // Create cache directories
        fs::create_dir_all(&game_data_cache_dir).unwrap();
        fs::create_dir_all(&mission_cache_dir).unwrap();
        
        // Create manager
        let manager = DbManager::new(
            &cache_dir,
            &game_data_cache_dir,
            &mission_cache_dir,
            &db_path,
        ).unwrap();
        
        // Define a path to a non-existent PBO
        let nonexistent_pbo = temp_dir.path().join("does_not_exist.pbo");
        
        // Record a failed extraction for a PBO that doesn't exist in the database
        manager.add_failed_extraction(&nonexistent_pbo, "Test error message".to_string()).unwrap();
        
        // Check that the failed extraction was recorded by querying it
        let result = manager.is_failed_extraction(&nonexistent_pbo).unwrap();
        assert!(result.is_some(), "Failed extraction should be recorded");
        
        if let Some((_, error_msg)) = result {
            assert_eq!(error_msg, "Test error message", "Error message should match");
        }
    }
    
    #[test]
    fn test_add_failed_extraction_with_non_existent_pbo() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        let db_path = cache_dir.join("test.db");
        
        // Create cache directories
        fs::create_dir_all(&game_data_cache_dir).unwrap();
        fs::create_dir_all(&mission_cache_dir).unwrap();
        
        // Create manager
        let manager = DbManager::new(
            &cache_dir,
            &game_data_cache_dir,
            &mission_cache_dir,
            &db_path,
        ).unwrap();
        
        // Define a path to a non-existent PBO with spaces in the path
        let nonexistent_pbo = PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/Arma 3/Addons/sounds_f_environment.pbo");
        
        // Record a failed extraction for a PBO that doesn't exist in the database
        // This should create a minimal PBO record first, then record the failure
        let result = manager.add_failed_extraction(&nonexistent_pbo, "Test error message".to_string());
        assert!(result.is_ok(), "Failed to add extraction failure: {:?}", result);
        
        // Check that the failed extraction was recorded by querying it
        let result = manager.is_failed_extraction(&nonexistent_pbo).unwrap();
        assert!(result.is_some(), "Failed extraction should be recorded");
        
        if let Some((_, error_msg)) = result {
            assert_eq!(error_msg, "Test error message", "Error message should match");
        }
    }
    
    #[test]
    fn test_add_failed_extraction_complex_paths() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        let db_path = cache_dir.join("test.db");
        
        // Create cache directories
        fs::create_dir_all(&game_data_cache_dir).unwrap();
        fs::create_dir_all(&mission_cache_dir).unwrap();
        
        // Create manager
        let manager = DbManager::new(
            &cache_dir,
            &game_data_cache_dir,
            &mission_cache_dir,
            &db_path,
        ).unwrap();
        
        // Test a variety of complex paths
        let test_paths = vec![
            PathBuf::from("C:/Program Files (x86)/Steam/SteamApps/common/Arma 3/Addons/some_content.pbo"),
            PathBuf::from("C:/Users/User Name/Documents/Arma 3/MPMissions/special-mission_v1.2.pbo"),
            PathBuf::from("D:/Games/Arma 3/Missions/test#mission.pbo"),
            PathBuf::from("/mnt/c/Program Files/Arma 3/Addons/addon_with_spaces.pbo"),
        ];
        
        for (i, path) in test_paths.iter().enumerate() {
            let error_message = format!("Test error message {}", i);
            
            // Record a failed extraction
            let result = manager.add_failed_extraction(path, error_message.clone());
            assert!(result.is_ok(), "Failed to add extraction failure for path {}: {:?}", path.display(), result);
            
            // Check that the failed extraction was recorded
            let result = manager.is_failed_extraction(path).unwrap();
            assert!(result.is_some(), "Failed extraction should be recorded for path {}", path.display());
            
            if let Some((_, msg)) = result {
                assert_eq!(msg, error_message, "Error message should match for path {}", path.display());
            }
        }
    }
} 