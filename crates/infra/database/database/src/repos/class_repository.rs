use log::debug;
use rusqlite::{params, TransactionBehavior, OptionalExtension};
use std::collections::HashSet;
use std::collections::HashMap;
use serde_json;

use crate::error::Result;
use crate::models::class::ClassModel;
use crate::DatabaseManager;
use arma3_models::types::PropertyValue;

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
        self.db.with_transaction(|tx| {
            // Insert class
            tx.execute(
                "INSERT OR REPLACE INTO classes (id, parent_id, container_class, source_file_index)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    class.id,
                    class.parent_id,
                    class.container_class,
                    convert_opt_usize_to_i64(class.source_file_index),
                ],
            )?;
            
            // Insert properties
            for (name, value) in &class.properties {
                let value_type = match value {
                    PropertyValue::String(_) => "string",
                    PropertyValue::Number(_) => "number",
                    PropertyValue::Array(_) => "array",
                    PropertyValue::Boolean(_) => "boolean",
                    PropertyValue::Object(_) => "object",
                };
                
                let serialized_value = serde_json::to_string(value)?;
                
                tx.execute(
                    "INSERT INTO class_properties (class_id, property_name, property_type, property_value)
                     VALUES (?1, ?2, ?3, ?4)",
                    params![
                        class.id,
                        name,
                        value_type,
                        serialized_value,
                    ],
                )?;
            }
            
            Ok(())
        })
    }
    
    /// Import game data classes with proper foreign key handling
    pub fn import_from_game_data(
        &self, 
        game_data: &arma3_models::GameDataClasses
    ) -> Result<()> {
        debug!("Importing {} classes from GameDataClasses", game_data.classes.len());
        
        // Get a connection and manually manage the transaction
        let mut conn = self.db.connection()?;
        
        // Turn off foreign keys for this connection
        conn.execute("PRAGMA foreign_keys = OFF", [])?;
        
        // Start transaction
        let tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        
        // Clear existing classes and properties
        tx.execute("DELETE FROM class_properties", [])?;
        tx.execute("DELETE FROM classes", [])?;
        
        debug!("Inserting classes into database...");
        
        // Process in two rounds
        {
            // Scope for the statements to ensure they're dropped before committing
            
            // Keep track of already processed class IDs to avoid duplicates
            let mut processed_ids = HashSet::new();
            
            let mut class_stmt = tx.prepare(
                "INSERT INTO classes (id, parent_id, container_class, source_file_index)
                VALUES (?1, ?2, ?3, ?4)"
            )?;
            
            let mut prop_stmt = tx.prepare(
                "INSERT INTO class_properties (class_id, property_name, property_type, property_value)
                VALUES (?1, ?2, ?3, ?4)"
            )?;
            
            // Process in two rounds - first add all classes without parent (roots)
            let mut root_classes = 0;
            let mut child_classes = 0;
            
            // First pass - root classes
            for class in &game_data.classes {
                if class.parent.is_none() && !processed_ids.contains(&class.name) {
                    class_stmt.execute(params![
                        class.name,
                        class.parent,
                        class.container_class,
                        convert_opt_usize_to_i64(class.source_file_index),
                    ])?;
                    
                    // Insert properties
                    for (name, value) in &class.properties {
                        let value_type = match value {
                            PropertyValue::String(_) => "string",
                            PropertyValue::Number(_) => "number",
                            PropertyValue::Array(_) => "array",
                            PropertyValue::Boolean(_) => "boolean",
                            PropertyValue::Object(_) => "object",
                        };
                        
                        let serialized_value = serde_json::to_string(value)?;
                        
                        prop_stmt.execute(params![
                            class.name,
                            name,
                            value_type,
                            serialized_value,
                        ])?;
                    }
                    
                    processed_ids.insert(class.name.clone());
                    root_classes += 1;
                }
            }
            
            debug!("Added {} root classes", root_classes);
            
            // Second pass - all other classes
            for class in &game_data.classes {
                if class.parent.is_some() && !processed_ids.contains(&class.name) {
                    class_stmt.execute(params![
                        class.name,
                        class.parent,
                        class.container_class,
                        convert_opt_usize_to_i64(class.source_file_index),
                    ])?;
                    
                    // Insert properties
                    for (name, value) in &class.properties {
                        let value_type = match value {
                            PropertyValue::String(_) => "string",
                            PropertyValue::Number(_) => "number",
                            PropertyValue::Array(_) => "array",
                            PropertyValue::Boolean(_) => "boolean",
                            PropertyValue::Object(_) => "object",
                        };
                        
                        let serialized_value = serde_json::to_string(value)?;
                        
                        prop_stmt.execute(params![
                            class.name,
                            name,
                            value_type,
                            serialized_value,
                        ])?;
                    }
                    
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
                "SELECT c.id, c.parent_id, c.container_class, c.source_file_index,
                        p.property_name, p.property_type, p.property_value
                 FROM classes c
                 LEFT JOIN class_properties p ON c.id = p.class_id
                 ORDER BY c.id"
            )?;
            
            let rows = stmt.query_map([], |row| {
                let class_id: String = row.get(0)?;
                let parent_id: Option<String> = row.get(1)?;
                let container_class: Option<String> = row.get(2)?;
                let source_file_index: Option<i64> = row.get(3)?;
                let property_name: Option<String> = row.get(4)?;
                let property_type: Option<String> = row.get(5)?;
                let property_value: Option<String> = row.get(6)?;
                
                Ok((
                    class_id,
                    parent_id,
                    container_class,
                    source_file_index,
                    property_name,
                    property_type,
                    property_value,
                ))
            })?;
            
            let mut class_map: HashMap<String, ClassModel> = HashMap::new();
            
            for row_result in rows {
                let (
                    class_id,
                    parent_id,
                    container_class,
                    source_file_index,
                    property_name,
                    property_type,
                    property_value,
                ) = row_result?;
                
                let class = class_map.entry(class_id.clone()).or_insert_with(|| {
                    ClassModel::new(
                        class_id,
                        parent_id,
                        container_class,
                        source_file_index.map(|idx| idx as usize),
                        false
                    )
                });
                
                // Add property if present
                if let (Some(name), Some(_type), Some(value)) = (property_name, property_type, property_value) {
                    if let Ok(prop_value) = serde_json::from_str(&value) {
                        class.properties.insert(name, prop_value);
                    }
                }
            }
            
            Ok(class_map.into_values().collect())
        })
    }

    /// Check if a class inherits from any of the specified base classes
    /// Optimized version for checking multiple base classes at once
    pub fn inherits_from_any(&self, class_id: &str, base_classes: &[String], max_depth: Option<i32>) -> Result<bool> {
        if base_classes.is_empty() {
            return Ok(false);
        }

        // Quick check if the class itself is one of the base classes (case-insensitive)
        if base_classes.iter().any(|base| base.to_lowercase() == class_id.to_lowercase()) {
            return Ok(true);
        }

        let max_depth = max_depth.unwrap_or(50);
        
        self.db.with_connection(|conn| {
            // Build placeholders for the base classes
            let placeholders = (2..=base_classes.len() + 1)
                .map(|i| format!("UPPER(?{})", i))
                .collect::<Vec<_>>()
                .join(",");
                
            let query = format!(
                "WITH RECURSIVE inheritance_chain(id, parent_id, depth) AS (
                    -- Base case: start with the given class
                    SELECT id, parent_id, 0 as depth
                    FROM classes
                    WHERE UPPER(id) = UPPER(?1)
                    
                    UNION ALL
                    
                    -- Recursive case: follow parent relationships
                    SELECT c.id, c.parent_id, ic.depth + 1
                    FROM classes c
                    JOIN inheritance_chain ic ON UPPER(ic.parent_id) = UPPER(c.id)
                    WHERE ic.depth < ?{} AND ic.parent_id IS NOT NULL
                )
                SELECT EXISTS(
                    SELECT 1 FROM inheritance_chain 
                    WHERE UPPER(id) IN ({})
                ) as inherits",
                base_classes.len() + 2, // Position for max_depth parameter
                placeholders
            );
            
            let mut stmt = conn.prepare(&query)?;
            
            // Build parameters: class_id, base_classes..., max_depth
            let mut params: Vec<&dyn rusqlite::ToSql> = vec![&class_id as &dyn rusqlite::ToSql];
            params.extend(base_classes.iter().map(|base| base as &dyn rusqlite::ToSql));
            params.push(&max_depth);
            
            let inherits: bool = stmt.query_row(params.as_slice(), |row| row.get(0))?;
            
            Ok(inherits)
        })
    }
}

// Function that converts Option<usize> to Option<i64> properly
fn convert_opt_usize_to_i64(val: Option<usize>) -> Option<i64> {
    val.map(|v| v as i64)
}