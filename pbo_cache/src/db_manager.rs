use std::path::{Path, PathBuf};
use log::{debug, info, warn};
use arma3_db::{
    DatabaseManager,
    models::{
        DatabaseConfig,
        pbo::{PboModel, PboType as DbPboType, normalize_path},
    },
    queries::pbo_repository::PboRepository,
};
use crate::models::{PboMetadata, PboType};
use crate::error::{Result, CacheError};

/// Manager for database operations related to PBO caching
pub struct DbManager {
    /// The database manager
    db_manager: DatabaseManager,
    
    /// The PBO repository
    pub pbo_repo: PboRepository<'static>,
    
    /// Path to the game data cache directory
    game_data_cache_dir: PathBuf,
    
    /// Path to the mission cache directory
    mission_cache_dir: PathBuf,
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
        
        let config = DatabaseConfig::with_cache_dirs(
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
        
        let pbo_repo = PboRepository::new(static_db_manager);
        
        info!("Database manager initialized");
        
        Ok(Self {
            db_manager: static_db_manager.clone(),
            pbo_repo,
            game_data_cache_dir: game_data_cache_dir.to_path_buf(),
            mission_cache_dir: mission_cache_dir.to_path_buf(),
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
            .or_else(|_| Ok(true)) // On error, assume extraction is needed
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
        
        self.pbo_repo.record_failed_extraction(&pbo_id, &error_message)
            .map_err(|e| CacheError::IndexOperation(format!("Failed to record failed extraction: {}", e)))
    }
    
    /// Check if a PBO has previously failed extraction
    pub fn is_failed_extraction(&self, path: &Path) -> Result<Option<(String, String)>> {
        let pbo_id = normalize_path(path);
        
        self.pbo_repo.get_failed_extraction(&pbo_id)
            .map(|opt| {
                opt.map(|failed| (
                    failed.timestamp.to_rfc3339(),
                    failed.error_message,
                ))
            })
            .map_err(|e| CacheError::IndexOperation(format!("Failed to check failed extraction: {}", e)))
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
        
        // Check if extraction failed
        let result = manager.is_failed_extraction(&pbo_path).unwrap();
        assert!(result.is_some());
        
        // Check error message
        let (_, actual_error) = result.unwrap();
        assert_eq!(actual_error, error_message);
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
    fn test_get_metadata_by_type() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();
        let game_data_cache_dir = cache_dir.join("gamedata");
        let mission_cache_dir = cache_dir.join("missions");
        let db_path = cache_dir.join("test.db");
        
        // Create cache directories
        fs::create_dir_all(&game_data_cache_dir).unwrap();
        fs::create_dir_all(&mission_cache_dir).unwrap();
        
        // Create mock PBO files
        let game_data_pbo = create_mock_pbo(temp_dir.path(), "game_data.pbo");
        let mission_pbo = create_mock_pbo(temp_dir.path(), "mission.pbo");
        
        // Create manager
        let manager = DbManager::new(
            &cache_dir,
            &game_data_cache_dir,
            &mission_cache_dir,
            &db_path,
        ).unwrap();
        
        // Create and add metadata for game data PBO
        let game_data_metadata = create_test_metadata(
            &game_data_pbo,
            PboType::GameData,
            vec![PathBuf::from("game_data/file.sqf")],
            vec!["sqf".to_string()],
        );
        manager.update_metadata(game_data_metadata).unwrap();
        
        // Create and add metadata for mission PBO
        let mission_metadata = create_test_metadata(
            &mission_pbo,
            PboType::Mission,
            vec![PathBuf::from("mission/file.sqf")],
            vec!["sqf".to_string()],
        );
        manager.update_metadata(mission_metadata).unwrap();
        
        // Test get_game_data_metadata
        let game_data = manager.get_game_data_metadata().unwrap();
        assert_eq!(game_data.len(), 1);
        assert_eq!(normalize_path(&game_data_pbo), game_data[0].id);
        
        // Test get_mission_metadata
        let missions = manager.get_mission_metadata().unwrap();
        assert_eq!(missions.len(), 1);
        assert_eq!(normalize_path(&mission_pbo), missions[0].id);
    }
} 