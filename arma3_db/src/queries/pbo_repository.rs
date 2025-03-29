use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use log::debug;
use rusqlite::{params, Row, OptionalExtension};

use crate::error::{Result, DatabaseError};
use crate::models::pbo::{PboModel, PboType, ExtractedFile, FailedExtraction};
use crate::DatabaseManager;

/// Repository for PBO-related database operations
pub struct PboRepository<'a> {
    db: &'a DatabaseManager,
}

impl<'a> PboRepository<'a> {
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
    
    /// Update a PBO
    pub fn update(&self, pbo: &PboModel) -> Result<()> {
        self.db.with_connection(|conn| {
            let rows_affected = conn.execute(
                "UPDATE pbo_files SET 
                     full_path = ?2, base_dir = ?3, file_size = ?4, 
                     last_modified = ?5, extraction_time = ?6, pbo_type = ?7
                 WHERE id = ?1",
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
            
            if rows_affected == 0 {
                return Err(DatabaseError::NotFound {
                    entity_type: "PBO".to_string(),
                    id: pbo.id.clone(),
                });
            }
            
            Ok(())
        })
    }
    
    /// Delete a PBO
    pub fn delete(&self, id: &str) -> Result<()> {
        self.db.with_transaction(|tx| {
            // Delete all extracted files first
            tx.execute(
                "DELETE FROM extracted_files WHERE pbo_id = ?1",
                [id],
            )?;
            
            // Delete any failed extraction record
            tx.execute(
                "DELETE FROM failed_extractions WHERE pbo_id = ?1",
                [id],
            )?;
            
            // Delete the PBO record
            let rows_affected = tx.execute(
                "DELETE FROM pbo_files WHERE id = ?1",
                [id],
            )?;
            
            if rows_affected == 0 {
                return Err(DatabaseError::NotFound {
                    entity_type: "PBO".to_string(),
                    id: id.to_string(),
                });
            }
            
            Ok(())
        })
    }
    
    /// Get all PBOs
    pub fn get_all(&self) -> Result<Vec<PboModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, full_path, base_dir, file_size, 
                        last_modified, extraction_time, pbo_type
                 FROM pbo_files ORDER BY id"
            )?;
            
            let rows = stmt.query_map([], |row| self.map_row_to_pbo(row))?;
            
            let mut pbos = Vec::new();
            for row_result in rows {
                pbos.push(row_result?);
            }
            
            Ok(pbos)
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
    
    /// Add an extracted file
    pub fn add_extracted_file(&self, pbo_id: &str, relative_path: impl AsRef<Path>) -> Result<()> {
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT INTO extracted_files (pbo_id, relative_path)
                 VALUES (?1, ?2)",
                params![
                    pbo_id,
                    relative_path.as_ref().to_string_lossy(),
                ],
            )?;
            
            Ok(())
        })
    }
    
    /// Add multiple extracted files
    pub fn add_extracted_files(&self, files: &[ExtractedFile]) -> Result<()> {
        if files.is_empty() {
            return Ok(());
        }
        
        self.db.with_transaction(|tx| {
            let mut stmt = tx.prepare(
                "INSERT INTO extracted_files (pbo_id, relative_path)
                 VALUES (?1, ?2)"
            )?;
            
            for file in files {
                stmt.execute(params![
                    file.pbo_id,
                    file.relative_path.to_string_lossy(),
                ])?;
            }
            
            Ok(())
        })
    }
    
    /// Get all extracted files for a PBO
    pub fn get_extracted_files(&self, pbo_id: &str) -> Result<Vec<ExtractedFile>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, pbo_id, relative_path
                 FROM extracted_files
                 WHERE pbo_id = ?1"
            )?;
            
            let rows = stmt.query_map([pbo_id], |row| {
                Ok(ExtractedFile {
                    id: row.get(0)?,
                    pbo_id: row.get(1)?,
                    relative_path: PathBuf::from(row.get::<_, String>(2)?),
                })
            })?;
            
            let mut files = Vec::new();
            for row_result in rows {
                files.push(row_result?);
            }
            
            Ok(files)
        })
    }
    
    /// Clear all extracted files for a PBO
    pub fn clear_extracted_files(&self, pbo_id: &str) -> Result<()> {
        self.db.with_connection(|conn| {
            conn.execute(
                "DELETE FROM extracted_files WHERE pbo_id = ?1",
                [pbo_id],
            )?;
            
            Ok(())
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
        self.db.with_connection(|conn| {
            conn.execute(
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
    
    /// Bulk import PBOs
    pub fn bulk_import(&self, pbos: &[PboModel]) -> Result<()> {
        if pbos.is_empty() {
            return Ok(());
        }
        
        debug!("Bulk importing {} PBOs", pbos.len());
        
        self.db.with_transaction(|tx| {
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO pbo_files (
                     id, full_path, base_dir, file_size, 
                     last_modified, extraction_time, pbo_type
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
            )?;
            
            for pbo in pbos {
                stmt.execute(params![
                    pbo.id,
                    pbo.full_path.to_string_lossy(),
                    pbo.base_dir.as_ref().map(|p| p.to_string_lossy().to_string()),
                    pbo.file_size,
                    pbo.last_modified.to_rfc3339(),
                    pbo.extraction_time.to_rfc3339(),
                    pbo.pbo_type.to_string(),
                ])?;
            }
            
            Ok(())
        })
    }
    
    /// Clear all PBOs
    pub fn clear_all(&self) -> Result<()> {
        self.db.with_transaction(|tx| {
            // Clear related tables
            tx.execute("DELETE FROM extracted_files", [])?;
            tx.execute("DELETE FROM failed_extractions", [])?;
            
            // Clear PBOs
            tx.execute("DELETE FROM pbo_files", [])?;
            
            Ok(())
        })
    }
    
    /// Clear all PBOs of a specific type
    pub fn clear_by_type(&self, pbo_type: PboType) -> Result<()> {
        self.db.with_transaction(|tx| {
            // Get IDs to delete
            let ids: Vec<String> = tx.prepare(
                "SELECT id FROM pbo_files WHERE pbo_type = ?1"
            )?
            .query_map([pbo_type.to_string()], |row| row.get(0))?
            .collect::<std::result::Result<Vec<String>, _>>()?;
            
            if ids.is_empty() {
                return Ok(());
            }
            
            // Build query with placeholders
            let placeholders = (1..=ids.len())
                .map(|i| format!("?{}", i))
                .collect::<Vec<_>>()
                .join(",");
                
            // Build parameter list
            let params: Vec<&dyn rusqlite::ToSql> = ids
                .iter()
                .map(|id| id as &dyn rusqlite::ToSql)
                .collect();
            
            // Delete extracted files
            let query = format!(
                "DELETE FROM extracted_files WHERE pbo_id IN ({})",
                placeholders
            );
            tx.execute(&query, params.as_slice())?;
            
            // Delete failed extractions
            let query = format!(
                "DELETE FROM failed_extractions WHERE pbo_id IN ({})",
                placeholders
            );
            tx.execute(&query, params.as_slice())?;
            
            // Delete PBOs
            tx.execute(
                "DELETE FROM pbo_files WHERE pbo_type = ?1",
                [pbo_type.to_string()],
            )?;
            
            Ok(())
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
        let repo = PboRepository::new(&db);
        
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