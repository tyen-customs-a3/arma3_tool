use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use log::debug;
use rusqlite::{params, Row, OptionalExtension};

use crate::error::{Result, DatabaseError};
use crate::models::pbo::{PboModel, PboType, ExtractedFile, FailedExtraction};
use crate::DatabaseManager;

/// Repository for PBO-related database operations
pub struct Cache<'a> {
    db: &'a DatabaseManager,
}

// Helper function to find the length of the common suffix between two paths
fn common_suffix_length(path1: &str, path2: &str) -> usize {
    let path1_components: Vec<&str> = path1.split('/').collect();
    let path2_components: Vec<&str> = path2.split('/').collect();
    
    let mut common_length = 0;
    let mut i = 1;
    
    // Start from the end of both paths and count matching components
    while i <= path1_components.len() && i <= path2_components.len() {
        let p1_idx = path1_components.len() - i;
        let p2_idx = path2_components.len() - i;
        
        if path1_components[p1_idx] == path2_components[p2_idx] {
            // Add the length of this component plus 1 for the slash
            common_length += path1_components[p1_idx].len() + 1;
            i += 1;
        } else {
            break;
        }
    }
    
    // Remove the leading slash from the count if there was a match
    if common_length > 0 {
        common_length -= 1;
    }
    
    common_length
}

impl<'a> Cache<'a> {
    /// Create a new PBO repository
    pub fn new(db: &'a DatabaseManager) -> Self {
        Self { db }
    }
    
    /// Create a new PBO record
    pub fn create(&self, pbo: &PboModel) -> Result<()> {
        // Validate the PBO model
        if !pbo.is_valid() {
            return Err(DatabaseError::InvalidData(
                format!("Invalid PBO model: {}", pbo.id)
            ));
        }
        
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO pbo_files (
                     id, full_path, base_dir, file_size, 
                     last_modified, extraction_time, pbo_type
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    pbo.id,
                    pbo.full_path.to_string_lossy(),
                    pbo.base_dir.as_ref().map(|p| p.to_string_lossy().to_string()),
                    pbo.file_size,
                    pbo.last_modified.to_rfc3339(),
                    pbo.extraction_time.to_rfc3339(),
                    pbo.pbo_type.to_string(),
                ],
            )?;
            
            debug!("Created or updated PBO record: {}", pbo.id);
            Ok(())
        })
    }
    
    /// Get a PBO by ID
    pub fn get(&self, id: &str) -> Result<Option<PboModel>> {
        self.db.with_connection(|conn| {
            let result = conn.query_row(
                "SELECT id, full_path, base_dir, file_size, 
                        last_modified, extraction_time, pbo_type
                 FROM pbo_files WHERE id = ?1",
                [id],
                |row| self.map_row_to_pbo(row),
            ).optional()?;
            
            Ok(result)
        })
    }
    
    /// Get PBOs by type
    pub fn get_by_type(&self, pbo_type: PboType) -> Result<Vec<PboModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, full_path, base_dir, file_size, 
                        last_modified, extraction_time, pbo_type
                 FROM pbo_files WHERE pbo_type = ?1 ORDER BY id"
            )?;
            
            let rows = stmt.query_map([pbo_type.to_string()], |row| self.map_row_to_pbo(row))?;
            
            let mut pbos = Vec::new();
            for row_result in rows {
                pbos.push(row_result?);
            }
            
            Ok(pbos)
        })
    }
    
    /// Add multiple extracted files
    pub fn add_extracted_files(&self, files: &[ExtractedFile]) -> Result<()> {
        if files.is_empty() {
            return Ok(());
        }
        
        self.db.with_transaction(|tx| {
            let mut stmt = tx.prepare(
                "INSERT INTO extracted_files (pbo_id, relative_path, extension, file_name)
                 VALUES (?1, ?2, ?3, ?4)"
            )?;
            
            for file in files {
                // Get extension and file name if not already cached
                let extension = file.extension.clone().or_else(|| {
                    file.relative_path.extension().map(|e| e.to_string_lossy().to_string())
                });
                
                let file_name = file.file_name.clone().or_else(|| {
                    file.relative_path.file_name().map(|f| f.to_string_lossy().to_string())
                });
                
                stmt.execute(params![
                    file.pbo_id,
                    file.relative_path.to_string_lossy(),
                    extension,
                    file_name,
                ])?;
            }
            
            Ok(())
        })
    }
    
    /// Get extracted files for a PBO
    pub fn get_extracted_files(&self, pbo_id: &str) -> Result<Vec<ExtractedFile>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, pbo_id, relative_path, extension, file_name
                 FROM extracted_files
                 WHERE pbo_id = ?1
                 ORDER BY relative_path"
            )?;
            
            let rows = stmt.query_map([pbo_id], |row| {
                Ok(ExtractedFile {
                    id: row.get(0)?,
                    pbo_id: row.get(1)?,
                    relative_path: PathBuf::from(row.get::<_, String>(2)?),
                    extension: row.get(3)?,
                    file_name: row.get(4)?,
                })
            })?;
            
            let mut files = Vec::new();
            for row_result in rows {
                files.push(row_result?);
            }
            
            Ok(files)
        })
    }
    
    /// Find the source PBO for a given file path
    pub fn find_pbo_by_file_path(&self, file_path: impl AsRef<Path>) -> Result<Option<PboModel>> {
        // Convert path to string and normalize slashes
        let path_str = file_path.as_ref().to_string_lossy().replace('\\', "/");
        
        // Try exact match first (useful for relative paths)
        let exact_match_result = self.find_pbo_by_exact_file_path(&path_str)?;
        if exact_match_result.is_some() {
            return Ok(exact_match_result);
        }
        
        // If not found, try as full path by finding the file name component
        if let Some(file_name) = file_path.as_ref().file_name() {
            let file_name_str = file_name.to_string_lossy();
            return self.find_pbo_by_file_name(&path_str, &file_name_str);
        }
        
        Ok(None)
    }
    
    // Helper function to find PBO by exact file path
    fn find_pbo_by_exact_file_path(&self, file_path: &str) -> Result<Option<PboModel>> {
        self.db.with_connection(|conn| {
            // First find the extracted file entry
            let pbo_id: Option<String> = conn.query_row(
                "SELECT pbo_id FROM extracted_files WHERE relative_path = ?1 LIMIT 1",
                [file_path],
                |row| row.get(0)
            ).optional()?;
            
            match pbo_id {
                Some(id) => self.get(&id),
                None => Ok(None),
            }
        })
    }
    
    // Helper function to find PBO by file name and partial path
    fn find_pbo_by_file_name(&self, full_path: &str, file_name: &str) -> Result<Option<PboModel>> {
        self.db.with_connection(|conn| {
            // First try to find by exact filename match (fast path)
            let mut stmt = conn.prepare(
                "SELECT pbo_id, relative_path FROM extracted_files 
                 WHERE relative_path LIKE ?1 LIMIT 100"
            )?;
            
            let pattern = format!("%/{}", file_name);
            let rows = stmt.query_map([pattern], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                ))
            })?;
            
            // Collect potential matches
            let mut matches = Vec::new();
            for row_result in rows {
                matches.push(row_result?);
            }
            
            if matches.is_empty() {
                return Ok(None);
            }
            
            // If we have multiple matches, find the best one
            if matches.len() > 1 {
                // Sort by length of common suffix with the target path
                matches.sort_by(|(_, path_a), (_, path_b)| {
                    let common_a = common_suffix_length(full_path, path_a);
                    let common_b = common_suffix_length(full_path, path_b);
                    common_b.cmp(&common_a) // Sort descending by common length
                });
            }
            
            // Get the PBO for the best match
            let (best_pbo_id, _) = &matches[0];
            self.get(best_pbo_id)
        })
    }
    
    /// Check if a PBO needs extraction
    pub fn needs_extraction(&self, pbo_id: &str, extensions: &[String]) -> Result<bool> {
        // Get the PBO
        let pbo_result = self.get(pbo_id)?;
        
        // If not found, needs extraction
        if pbo_result.is_none() {
            debug!("PBO {} not found in database, needs extraction", pbo_id);
            return Ok(true);
        }
        
        let pbo = pbo_result.unwrap();
        
        // Check if file exists
        let full_path = pbo.get_full_path();
        if !full_path.exists() {
            debug!("PBO {} physical file does not exist, needs extraction", pbo_id);
            return Ok(true);
        }
        
        // Check if file metadata has changed
        if let Ok(metadata) = std::fs::metadata(&full_path) {
            // Check file size
            if metadata.len() != pbo.file_size {
                debug!("PBO {} size changed (was: {}, now: {}), needs extraction", 
                    pbo_id, pbo.file_size, metadata.len());
                return Ok(true);
            }
            
            // Check last modified time
            if let Ok(last_modified) = metadata.modified() {
                let last_modified_time = DateTime::<Utc>::from(last_modified);
                if last_modified_time > pbo.last_modified {
                    debug!("PBO {} modified time changed, needs extraction", pbo_id);
                    return Ok(true);
                }
            } else {
                debug!("Unable to get PBO {} modified time, assuming needs extraction", pbo_id);
                return Ok(true);
            }
        } else {
            debug!("Unable to get PBO {} metadata, assuming needs extraction", pbo_id);
            return Ok(true);
        }
        
        // Check if any extracted files exist
        let extracted_files = self.get_extracted_files(pbo_id)?;
        if extracted_files.is_empty() {
            debug!("PBO {} has no extracted files in database, needs extraction", pbo_id);
            return Ok(true);
        }
        
        // Check if extensions match
        let extensions_str = extensions.join(",");
        let cached_extensions = match self.db.get_metadata(&format!("pbo_{}_extensions", pbo_id))? {
            Some(ext) => ext,
            None => {
                debug!("PBO {} has no cached extensions, needs extraction", pbo_id);
                return Ok(true);
            }
        };
        
        // Create sorted sets for comparing extension lists
        let mut sorted_cached_extensions: Vec<&str> = cached_extensions.split(',').collect();
        sorted_cached_extensions.sort();
        
        let mut sorted_requested_extensions: Vec<&str> = extensions.iter().map(|s| s.as_str()).collect();
        sorted_requested_extensions.sort();
        
        if sorted_cached_extensions != sorted_requested_extensions {
            debug!("PBO {} extensions changed (was: {}, now: {}), needs extraction", 
                pbo_id, cached_extensions, extensions_str);
            return Ok(true);
        }
        
        // No need to extract
        debug!("PBO {} is up-to-date, no need for extraction", pbo_id);
        Ok(false)
    }
    
    /// Record a failed extraction
    pub fn record_failed_extraction(&self, pbo_id: &str, error_message: &str) -> Result<()> {
        self.db.with_transaction(|tx| {
            // First check if the PBO record exists
            let pbo_exists: bool = tx.query_row(
                "SELECT 1 FROM pbo_files WHERE id = ?1 LIMIT 1",
                [pbo_id],
                |_| Ok(true)
            ).optional()?.unwrap_or(false);
            
            // If the PBO record doesn't exist, we can't record the failure due to foreign key constraint
            if !pbo_exists {
                return Err(crate::error::DatabaseError::InvalidData(
                    format!("Cannot record failed extraction: PBO {} not found in database", pbo_id)
                ));
            }
            
            // Record the failed extraction
            tx.execute(
                "INSERT OR REPLACE INTO failed_extractions (pbo_id, timestamp, error_message)
                 VALUES (?1, ?2, ?3)",
                params![
                    pbo_id,
                    Utc::now().to_rfc3339(),
                    error_message,
                ],
            )?;
            
            Ok(())
        })
    }
    
    /// Record a failed extraction directly, bypassing foreign key checks
    /// This should only be used after ensuring the PBO record exists
    pub fn record_failed_extraction_direct(&self, pbo_id: &str, error_message: &str) -> Result<()> {
        self.db.with_connection(|conn| {
            // Temporarily disable foreign key constraints
            conn.execute("PRAGMA foreign_keys = OFF", [])?;
            
            // Record the failed extraction
            let result = conn.execute(
                "INSERT OR REPLACE INTO failed_extractions (pbo_id, timestamp, error_message)
                 VALUES (?1, ?2, ?3)",
                params![
                    pbo_id,
                    Utc::now().to_rfc3339(),
                    error_message,
                ],
            );
            
            // Re-enable foreign key constraints
            conn.execute("PRAGMA foreign_keys = ON", [])?;
            
            // Check result
            result?;
            
            debug!("Directly recorded failed extraction for {}", pbo_id);
            Ok(())
        })
    }
    
    /// Get failed extraction
    pub fn get_failed_extraction(&self, pbo_id: &str) -> Result<Option<FailedExtraction>> {
        self.db.with_connection(|conn| {
            let result = conn.query_row(
                "SELECT pbo_id, timestamp, error_message
                 FROM failed_extractions WHERE pbo_id = ?1",
                [pbo_id],
                |row| {
                    let timestamp_str: String = row.get(1)?;
                    let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                        .map(|dt| dt.with_timezone(&Utc))
                        .map_err(|_| rusqlite::Error::InvalidColumnType(1, "timestamp".to_string(), rusqlite::types::Type::Text))?;
                    
                    Ok(FailedExtraction {
                        pbo_id: row.get(0)?,
                        timestamp,
                        error_message: row.get(2)?,
                    })
                },
            ).optional()?;
            
            Ok(result)
        })
    }
    
    /// Find extracted files by extension
    pub fn find_files_by_extension(&self, extension: &str) -> Result<Vec<ExtractedFile>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, pbo_id, relative_path, extension, file_name
                 FROM extracted_files
                 WHERE extension = ?1"
            )?;
            
            let rows = stmt.query_map([extension], |row| {
                Ok(ExtractedFile {
                    id: row.get(0)?,
                    pbo_id: row.get(1)?,
                    relative_path: PathBuf::from(row.get::<_, String>(2)?),
                    extension: row.get(3)?,
                    file_name: row.get(4)?,
                })
            })?;
            
            let mut files = Vec::new();
            for row_result in rows {
                files.push(row_result?);
            }
            
            Ok(files)
        })
    }
    
    /// Update extraction information for a PBO
    pub fn update_extraction_info(
        &self, 
        pbo_id: &str, 
        extraction_time: DateTime<Utc>,
        extracted_files: &[PathBuf],
        extensions: &[String],
    ) -> Result<()> {
        self.db.with_transaction(|tx| {
            // Update PBO extraction time
            tx.execute(
                "UPDATE pbo_files SET extraction_time = ?1 WHERE id = ?2",
                params![extraction_time.to_rfc3339(), pbo_id],
            )?;
            
            // Clear existing extracted files
            tx.execute(
                "DELETE FROM extracted_files WHERE pbo_id = ?1",
                [pbo_id],
            )?;
            
            // Add new extracted files
            let mut stmt = tx.prepare(
                "INSERT INTO extracted_files (pbo_id, relative_path)
                 VALUES (?1, ?2)"
            )?;
            
            for path in extracted_files {
                stmt.execute(params![
                    pbo_id,
                    path.to_string_lossy(),
                ])?;
            }
            
            // Store extensions metadata
            tx.execute(
                "INSERT OR REPLACE INTO metadata (key, value)
                 VALUES (?1, ?2)",
                params![
                    format!("pbo_{}_extensions", pbo_id),
                    extensions.join(","),
                ],
            )?;
            
            // Remove from failed extractions if it exists
            tx.execute(
                "DELETE FROM failed_extractions WHERE pbo_id = ?1",
                [pbo_id],
            )?;
            
            Ok(())
        })
    }
    
    /// Convert a database row to a PboModel
    fn map_row_to_pbo(&self, row: &Row) -> rusqlite::Result<PboModel> {
        let last_modified_str: String = row.get(4)?;
        let extraction_time_str: String = row.get(5)?;
        
        let last_modified = DateTime::parse_from_rfc3339(&last_modified_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|_| rusqlite::Error::InvalidColumnType(4, "last_modified".to_string(), rusqlite::types::Type::Text))?;
            
        let extraction_time = DateTime::parse_from_rfc3339(&extraction_time_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|_| rusqlite::Error::InvalidColumnType(5, "extraction_time".to_string(), rusqlite::types::Type::Text))?;
        
        let base_dir_str: Option<String> = row.get(2)?;
        let base_dir = base_dir_str.map(PathBuf::from);
        
        let pbo_type_str: String = row.get(6)?;
        let pbo_type = match pbo_type_str.as_str() {
            "GameData" => PboType::GameData,
            "Mission" => PboType::Mission,
            _ => PboType::GameData, // Default
        };
        
        Ok(PboModel {
            id: row.get(0)?,
            full_path: PathBuf::from(row.get::<_, String>(1)?),
            base_dir,
            file_size: row.get(3)?,
            last_modified,
            extraction_time,
            pbo_type,
        })
    }
    
    /// Import file index mappings from GameDataClasses
    pub fn import_file_index_mappings(&self, game_data: &arma3_models::GameDataClasses) -> Result<()> {
        if game_data.file_sources.is_empty() {
            return Ok(());
        }
        
        debug!("Importing {} file index mappings", game_data.file_sources.len());
        
        self.db.with_transaction(|tx| {
            // Prepare statement outside the loop
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
                 VALUES (?1, ?2, ?3, ?4)"
            )?;
            
            // Insert all mappings in a single transaction
            for (idx, path) in game_data.file_sources.iter().enumerate() {
                let normalized_path = crate::models::pbo::normalize_path(path);
                
                // Try to find matching PBO in the database
                let pbo_id = match tx.query_row(
                    "SELECT id FROM pbo_files WHERE id = ?1",
                    [&normalized_path],
                    |row| row.get::<_, String>(0)
                ).optional() {
                    Ok(Some(id)) => Some(id),
                    _ => None,
                };
                
                stmt.execute(params![
                    idx as i64,
                    path.to_string_lossy().to_string(),
                    normalized_path,
                    pbo_id,
                ])?;
            }
            
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    
    #[test]
    fn test_pbo_repository() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let repo = Cache::new(&db);
        
        // Create test PBO
        let now = Utc::now();
        let pbo = PboModel::new(
            "test.pbo",
            PboType::GameData,
            1234,
            now,
            now,
        );
        
        // Insert PBO
        repo.create(&pbo).unwrap();
        
        // Test get
        let result = repo.get("test.pbo").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "test.pbo");
        
        // Test add extracted files
        let files = vec![
            ExtractedFile::new("test.pbo", "file1.sqf"),
            ExtractedFile::new("test.pbo", "file2.sqf"),
        ];
        
        repo.add_extracted_files(&files).unwrap();
        
        // Test get extracted files
        let extracted = repo.get_extracted_files("test.pbo").unwrap();
        assert_eq!(extracted.len(), 2);
        
        // Create another PBO for failed extraction test
        let failed_pbo = PboModel::new(
            "failed.pbo",
            PboType::GameData,
            1234,
            Utc::now(),
            Utc::now(),
        );
        repo.create(&failed_pbo).unwrap();
        
        // Test record failed extraction
        repo.record_failed_extraction("failed.pbo", "Test error").unwrap();
        
        let failed = repo.get_failed_extraction("failed.pbo").unwrap();
        assert!(failed.is_some());
        assert_eq!(failed.unwrap().error_message, "Test error");
        
        // Test needs extraction
        let needs = repo.needs_extraction("test.pbo", &["sqf".to_string()]).unwrap();
        // Note: This will return true since the PBO doesn't exist on disk
        assert!(needs);
    }
} 