use log::debug;
use rusqlite::{params, Row, TransactionBehavior, OptionalExtension};
use std::collections::HashSet;

use crate::error::Result;
use crate::models::class::ClassModel;
use crate::DatabaseManager;

/// Repository for class-related database operations
pub struct ClassRepository<'a> {
    db: &'a DatabaseManager,
}

impl<'a> ClassRepository<'a> {
    /// Create a new class repository
    pub fn new(db: &'a DatabaseManager) -> Self {
        Self { db }
    }
    
    /// Create a new class
    pub fn create(&self, class: &ClassModel) -> Result<()> {
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO classes (id, parent_id, source_file_index)
                 VALUES (?1, ?2, ?3)",
                params![
                    class.id,
                    class.parent_id,
                    convert_opt_usize_to_i64(class.source_file_index),
                ],
            )?;
            
            Ok(())
        })
    }
    
    /// Import game data classes with proper foreign key handling
    pub fn import_from_game_data(
        &self, 
        game_data: &arma3_tool_shared_models::GameDataClasses
    ) -> Result<()> {
        debug!("Importing {} classes from GameDataClasses", game_data.classes.len());
        
        // Get a connection and manually manage the transaction
        let mut conn = self.db.connection()?;
        
        // Turn off foreign keys for this connection
        conn.execute("PRAGMA foreign_keys = OFF", [])?;
        
        // Start transaction
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        
        // Clear existing classes
        tx.execute("DELETE FROM classes", [])?;
        
        debug!("Inserting classes into database...");
        
        // Process in two rounds
        {
            // Scope for the statements to ensure they're dropped before committing
            
            // Keep track of already processed class IDs to avoid duplicates
            let mut processed_ids = HashSet::new();
            
            let mut stmt = tx.prepare(
                "INSERT INTO classes (id, parent_id, source_file_index)
                VALUES (?1, ?2, ?3)"
            )?;
            
            // Process in two rounds - first add all classes without parent (roots)
            let mut root_classes = 0;
            let mut child_classes = 0;
            
            // First pass - root classes
            for class in &game_data.classes {
                if class.parent.is_none() && !processed_ids.contains(&class.name) {
                    stmt.execute(params![
                        class.name,
                        class.parent,
                        convert_opt_usize_to_i64(class.source_file_index),
                    ])?;
                    processed_ids.insert(class.name.clone());
                    root_classes += 1;
                }
            }
            
            debug!("Added {} root classes", root_classes);
            
            // Second pass - all other classes
            for class in &game_data.classes {
                if class.parent.is_some() && !processed_ids.contains(&class.name) {
                    stmt.execute(params![
                        class.name,
                        class.parent,
                        convert_opt_usize_to_i64(class.source_file_index),
                    ])?;
                    processed_ids.insert(class.name.clone());
                    child_classes += 1;
                }
            }
            
            debug!("Added {} child classes", child_classes);
            
            // Update file index mapping directly in the same transaction
            debug!("Updating file index mappings...");
            let mut idx_stmt = tx.prepare(
                "INSERT OR REPLACE INTO file_index_mapping (file_index, file_path, normalized_path, pbo_id)
                VALUES (?1, ?2, ?3, ?4)"
            )?;
            
            // Keep track of processed file indices
            let mut processed_file_indices = HashSet::new();
            
            for (idx, path) in game_data.file_sources.iter().enumerate() {
                if processed_file_indices.contains(&idx) {
                    continue;
                }
                
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
                
                idx_stmt.execute(params![
                    idx as i64,
                    path.to_string_lossy().to_string(),
                    normalized_path,
                    pbo_id,
                ])?;
                
                processed_file_indices.insert(idx);
            }
            
            debug!("Successfully imported {} classes ({} roots, {} children) of {} total", 
                processed_ids.len(), root_classes, child_classes, game_data.classes.len());
                
            if processed_ids.len() < game_data.classes.len() {
                debug!("Skipped {} duplicate classes", game_data.classes.len() - processed_ids.len());
            }
        } // end scope for statements
        
        // Commit transaction - now all statements are dropped
        tx.commit()?;
        
        // Re-enable foreign keys - now the transaction is committed
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        
        Ok(())
    }
    
    /// Get all classes
    pub fn get_all(&self) -> Result<Vec<ClassModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, parent_id, source_file_index
                 FROM classes ORDER BY id"
            )?;
            
            let rows = stmt.query_map([], |row| self.map_row_to_class(row))?;
            
            let mut classes = Vec::new();
            for row_result in rows {
                classes.push(row_result?);
            }
            
            Ok(classes)
        })
    }
    
    /// Convert a database row to a ClassModel
    fn map_row_to_class(&self, row: &Row) -> rusqlite::Result<ClassModel> {
        let source_file_index: Option<i64> = row.get(2)?;
        
        Ok(ClassModel {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            source_file_index: source_file_index.map(|idx| idx as usize),
        })
    }
}

// Function that converts Option<usize> to Option<i64> properly
fn convert_opt_usize_to_i64(val: Option<usize>) -> Option<i64> {
    val.map(|v| v as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use arma3_tool_shared_models::{GameDataClasses, GameDataClass};
    use std::collections::HashMap;
    
    #[test]
    fn test_class_repository() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let repo = ClassRepository::new(&db);
        
        // Create test class
        let class = ClassModel::new(
            "Class1", 
            None::<String>, 
            Some(1)
        );
        
        // Insert class
        repo.create(&class).unwrap();
        
        // Create game data classes for import test
        let mut game_data = GameDataClasses::new();
        
        // Add classes in a way that tests foreign key relationships
        let class1 = GameDataClass {
            name: "Parent".to_string(),
            parent: None,
            container_class: None,
            properties: HashMap::new(),
            source_file_index: Some(0),
        };
        
        let class2 = GameDataClass {
            name: "Child".to_string(),
            parent: Some("Parent".to_string()),
            container_class: None,
            properties: HashMap::new(),
            source_file_index: Some(1),
        };
        
        let class3 = GameDataClass {
            name: "GrandChild".to_string(),
            parent: Some("Child".to_string()),
            container_class: None,
            properties: HashMap::new(),
            source_file_index: Some(2),
        };
        
        // Add cyclic dependency to test handling
        let class4 = GameDataClass {
            name: "Cyclic1".to_string(),
            parent: Some("Cyclic2".to_string()),
            container_class: None,
            properties: HashMap::new(),
            source_file_index: Some(3),
        };
        
        let class5 = GameDataClass {
            name: "Cyclic2".to_string(),
            parent: Some("Cyclic1".to_string()),
            container_class: None,
            properties: HashMap::new(),
            source_file_index: Some(4),
        };
        
        // Add duplicate class to test deduplication
        let class6 = GameDataClass {
            name: "Parent".to_string(), // Same name as class1
            parent: None,
            container_class: None,
            properties: HashMap::new(),
            source_file_index: Some(5),
        };
        
        // Add to game data
        game_data.add_class(class1);
        game_data.add_class(class2);
        game_data.add_class(class3);
        game_data.add_class(class4);
        game_data.add_class(class5);
        game_data.add_class(class6); // Duplicate class
        
        // File sources
        game_data.add_file_source("path/to/file1.pbo".into());
        game_data.add_file_source("path/to/file2.pbo".into());
        game_data.add_file_source("path/to/file3.pbo".into());
        game_data.add_file_source("path/to/file4.pbo".into());
        game_data.add_file_source("path/to/file5.pbo".into());
        game_data.add_file_source("path/to/file6.pbo".into());
        
        // Import game data
        repo.import_from_game_data(&game_data).unwrap();
        
        // Get all classes
        let classes = repo.get_all().unwrap();
        
        // There should be 5 new unique classes plus the one we created initially
        assert_eq!(classes.len(), 6);
        
        // Verify file index mappings were created
        let count: i64 = db.with_connection(|conn| -> Result<i64> {
            let count = conn.query_row(
                "SELECT COUNT(*) FROM file_index_mapping",
                [],
                |row| row.get::<_, i64>(0)
            )?;
            
            Ok(count)
        }).unwrap();
        
        // There should be 6 file index mappings
        assert_eq!(count, 6);
    }
} 