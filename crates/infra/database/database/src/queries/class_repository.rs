use log::debug;
use rusqlite::{params, Row, OptionalExtension};
use std::collections::HashMap;

use crate::error::{Result, DatabaseError};
use crate::models::class::{ClassModel, ClassHierarchyNode};
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
                "INSERT OR REPLACE INTO classes (id, parent_id, container_class, source_file_index, is_forward_declaration)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    class.id,
                    class.parent_id,
                    class.container_class,
                    convert_opt_usize_to_i64(class.source_file_index),
                    class.is_forward_declaration,
                ],
            )?;
            
            Ok(())
        })
    }
    
    /// Get a class by ID
    pub fn get(&self, id: &str) -> Result<Option<ClassModel>> {
        self.db.with_connection(|conn| {
            let result = conn.query_row(
                "SELECT id, parent_id, container_class, source_file_index, is_forward_declaration
                 FROM classes WHERE id = ?1",
                [id],
                |row| self.map_row_to_class(row),
            ).optional()?;
            
            Ok(result)
        })
    }
    
    /// Update a class
    pub fn update(&self, class: &ClassModel) -> Result<()> {
        self.db.with_connection(|conn| {
            let rows_affected = conn.execute(
                "UPDATE classes SET parent_id = ?2, container_class = ?3, source_file_index = ?4, is_forward_declaration = ?5
                 WHERE id = ?1",
                params![
                    class.id,
                    class.parent_id,
                    class.container_class,
                    convert_opt_usize_to_i64(class.source_file_index),
                    class.is_forward_declaration,
                ],
            )?;
            
            if rows_affected == 0 {
                return Err(DatabaseError::NotFound {
                    entity_type: "Class".to_string(),
                    id: class.id.clone(),
                });
            }
            
            Ok(())
        })
    }
    
    /// Delete a class
    pub fn delete(&self, id: &str) -> Result<()> {
        self.db.with_connection(|conn| {
            let rows_affected = conn.execute(
                "DELETE FROM classes WHERE id = ?1",
                [id],
            )?;
            
            if rows_affected == 0 {
                return Err(DatabaseError::NotFound {
                    entity_type: "Class".to_string(),
                    id: id.to_string(),
                });
            }
            
            Ok(())
        })
    }
    
    /// Get all classes
    pub fn get_all(&self) -> Result<Vec<ClassModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, parent_id, container_class, source_file_index, is_forward_declaration
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
    
    /// Find classes by parent ID
    pub fn find_by_parent(&self, parent_id: &str) -> Result<Vec<ClassModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, parent_id, container_class, source_file_index, is_forward_declaration
                 FROM classes WHERE parent_id = ?1"
            )?;
            
            let rows = stmt.query_map([parent_id], |row| self.map_row_to_class(row))?;
            
            let mut classes = Vec::new();
            for row_result in rows {
                classes.push(row_result?);
            }
            
            Ok(classes)
        })
    }
    
    /// Get direct children of a class
    pub fn get_children(&self, parent_id: &str) -> Result<Vec<ClassModel>> {
        self.find_by_parent(parent_id)
    }
    
    /// Get root classes (classes without a parent)
    pub fn get_root_classes(&self) -> Result<Vec<ClassModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, parent_id, container_class, source_file_index, is_forward_declaration
                 FROM classes WHERE parent_id IS NULL"
            )?;
            
            let rows = stmt.query_map([], |row| self.map_row_to_class(row))?;
            
            let mut classes = Vec::new();
            for row_result in rows {
                classes.push(row_result?);
            }
            
            Ok(classes)
        })
    }
    
    /// Get class hierarchy starting from a root class with max depth
    pub fn get_hierarchy(&self, root_id: &str, max_depth: i32) -> Result<Vec<ClassHierarchyNode>> {
        self.db.with_connection(|conn| {
            // Use recursive CTE to get hierarchy
            let mut stmt = conn.prepare(
                "WITH RECURSIVE hierarchy(id, parent_id, container_class, source_file_index, is_forward_declaration, depth) AS (
                    SELECT id, parent_id, container_class, source_file_index, is_forward_declaration, 0
                    FROM classes
                    WHERE id = ?1
                    
                    UNION ALL
                    
                    SELECT c.id, c.parent_id, c.container_class, c.source_file_index, c.is_forward_declaration, h.depth + 1
                    FROM classes c
                    JOIN hierarchy h ON c.parent_id = h.id
                    WHERE h.depth < ?2
                )
                SELECT id, parent_id, container_class, source_file_index, is_forward_declaration, depth
                FROM hierarchy
                ORDER BY depth, id"
            )?;
            
            let rows = stmt.query_map(params![root_id, max_depth], |row| {
                Ok(ClassHierarchyNode {
                    id: row.get(0)?,
                    parent_id: row.get(1)?,
                    container_class: row.get(2)?,
                    source_file_index: convert_i64_to_usize(row.get(3)?),
                    is_forward_declaration: row.get(4)?,
                    depth: row.get(5)?,
                })
            })?;
            
            let mut nodes = Vec::new();
            for row_result in rows {
                nodes.push(row_result?);
            }
            
            Ok(nodes)
        })
    }
    
    /// Get class hierarchy starting from all root classes with max depth
    pub fn get_full_hierarchy(&self, max_depth: i32) -> Result<Vec<ClassHierarchyNode>> {
        self.db.with_connection(|conn| {
            // Find all root classes first
            let root_classes: Vec<String> = conn.prepare(
                "SELECT id FROM classes WHERE parent_id IS NULL"
            )?
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<std::result::Result<Vec<String>, _>>()?;
            
            // If no roots found, return empty result
            if root_classes.is_empty() {
                return Ok(Vec::new());
            }
            
            // Build query with multiple starting points
            let root_list = root_classes
                .iter()
                .map(|id| format!("'{}'", id.replace('\'', "''")))
                .collect::<Vec<_>>()
                .join(",");
                
            let query = format!(
                "WITH RECURSIVE hierarchy(id, parent_id, container_class, source_file_index, is_forward_declaration, depth) AS (
                    SELECT id, parent_id, container_class, source_file_index, is_forward_declaration, 0
                    FROM classes
                    WHERE id IN ({})
                    
                    UNION ALL
                    
                    SELECT c.id, c.parent_id, c.container_class, c.source_file_index, c.is_forward_declaration, h.depth + 1
                    FROM classes c
                    JOIN hierarchy h ON c.parent_id = h.id
                    WHERE h.depth < ?1
                )
                SELECT id, parent_id, container_class, source_file_index, is_forward_declaration, depth
                FROM hierarchy
                ORDER BY depth, id",
                root_list
            );
            
            let mut stmt = conn.prepare(&query)?;
            
            let rows = stmt.query_map([max_depth], |row| {
                Ok(ClassHierarchyNode {
                    id: row.get(0)?,
                    parent_id: row.get(1)?,
                    container_class: row.get(2)?,
                    source_file_index: convert_i64_to_usize(row.get(3)?),
                    is_forward_declaration: row.get(4)?,
                    depth: row.get(5)?,
                })
            })?;
            
            let mut nodes = Vec::new();
            for row_result in rows {
                nodes.push(row_result?);
            }
            
            Ok(nodes)
        })
    }
    
    /// Find orphaned classes by parent removal
    pub fn find_orphaned_by_parent_removal(&self, parent_ids: &[String]) -> Result<Vec<ClassModel>> {
        if parent_ids.is_empty() {
            return Ok(Vec::new());
        }
        
        self.db.with_connection(|conn| {
            // Build query with placeholders
            let placeholders = (1..=parent_ids.len())
                .map(|i| format!("?{}", i))
                .collect::<Vec<_>>()
                .join(",");
                
            let query = format!(
                "SELECT id, parent_id, container_class, source_file_index, is_forward_declaration
                 FROM classes
                 WHERE parent_id IN ({})",
                placeholders
            );
            
            let mut stmt = conn.prepare(&query)?;
            
            // Convert params to the right type
            let params: Vec<&dyn rusqlite::ToSql> = parent_ids
                .iter()
                .map(|id| id as &dyn rusqlite::ToSql)
                .collect();
            
            let rows = stmt.query_map(params.as_slice(), |row| self.map_row_to_class(row))?;
            
            let mut classes = Vec::new();
            for row_result in rows {
                classes.push(row_result?);
            }
            
            Ok(classes)
        })
    }
    
    /// Find affected classes (recursive children of the given classes)
    pub fn find_affected_children(&self, class_ids: &[String], max_depth: i32) -> Result<Vec<ClassModel>> {
        if class_ids.is_empty() {
            return Ok(Vec::new());
        }
        
        self.db.with_connection(|conn| {
            // Build query with placeholders for starting classes
            let placeholders = (1..=class_ids.len())
                .map(|i| format!("?{}", i))
                .collect::<Vec<_>>()
                .join(",");
                
            let query = format!(
                "WITH RECURSIVE affected_hierarchy(id, parent_id, container_class, source_file_index, is_forward_declaration, depth) AS (
                    SELECT id, parent_id, container_class, source_file_index, is_forward_declaration, 0
                    FROM classes
                    WHERE id IN ({})
                    
                    UNION ALL
                    
                    SELECT c.id, c.parent_id, c.container_class, c.source_file_index, c.is_forward_declaration, a.depth + 1
                    FROM classes c
                    JOIN affected_hierarchy a ON c.parent_id = a.id
                    WHERE a.depth < ?{}
                )
                SELECT id, parent_id, container_class, source_file_index, is_forward_declaration
                FROM affected_hierarchy
                WHERE depth > 0",  // Exclude the starting classes
                placeholders,
                class_ids.len() + 1  // Last parameter is max_depth
            );
            
            let mut stmt = conn.prepare(&query)?;
            
            // Build parameters
            let mut params: Vec<&dyn rusqlite::ToSql> = class_ids
                .iter()
                .map(|id| id as &dyn rusqlite::ToSql)
                .collect();
            
            // Add max_depth parameter
            params.push(&max_depth);
            
            let rows = stmt.query_map(params.as_slice(), |row| self.map_row_to_class(row))?;
            
            let mut classes = Vec::new();
            for row_result in rows {
                classes.push(row_result?);
            }
            
            Ok(classes)
        })
    }
    
    /// Bulk import classes
    pub fn bulk_import(&self, classes: &[ClassModel]) -> Result<()> {
        if classes.is_empty() {
            return Ok(());
        }
        
        debug!("Bulk importing {} classes", classes.len());
        
        self.db.with_transaction(|tx| {
            // Prepare statement outside the loop
            let mut stmt = tx.prepare(
                "INSERT OR REPLACE INTO classes (id, parent_id, container_class, source_file_index, is_forward_declaration)
                 VALUES (?1, ?2, ?3, ?4, ?5)"
            )?;
            
            // Insert all classes in a single transaction
            for class in classes {
                stmt.execute(params![
                    class.id,
                    class.parent_id,
                    class.container_class,
                    convert_opt_usize_to_i64(class.source_file_index),
                    class.is_forward_declaration,
                ])?;
            }
            
            Ok(())
        })
    }
    
    /// Convert from GameDataClasses to database models and import
    pub fn import_from_game_data(
        &self, 
        game_data: &arma3_models::GameDataClasses
    ) -> Result<()> {
        debug!("Importing {} classes from GameDataClasses", game_data.classes.len());
        
        // Convert to ClassModel
        let classes: Vec<ClassModel> = game_data.classes.iter()
            .map(|class| ClassModel::from_game_data_class(class))
            .collect();
        
        // Bulk import classes
        self.bulk_import(&classes)?;
        
        // Update file index mapping
        let cache = crate::queries::cache::Cache::new(self.db);
        cache.import_file_index_mappings(game_data)?;
        
        Ok(())
    }
    
    /// Clear all classes
    pub fn clear_all(&self) -> Result<()> {
        self.db.with_connection(|conn| {
            conn.execute("DELETE FROM classes", [])?;
            Ok(())
        })
    }
    
    /// Convert a database row to a ClassModel
    fn map_row_to_class(&self, row: &Row) -> rusqlite::Result<ClassModel> {
        let source_file_index: Option<i64> = row.get(3)?;
        
        Ok(ClassModel {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            container_class: row.get(2)?,
            source_file_index: source_file_index.map(|idx| idx as usize),
            is_forward_declaration: row.get(4)?,
            properties: HashMap::new(),
        })
    }
    
    /// Get source file path for a given file index, preferring non-forward declarations
    pub fn get_source_path(&self, file_index: usize) -> Result<Option<String>> {
        self.db.with_connection(|conn| {
            conn.query_row(
                "SELECT COALESCE(fim.pbo_id, fim.normalized_path) as source_path 
                 FROM file_index_mapping fim
                 JOIN classes c ON fim.file_index = c.source_file_index
                 WHERE fim.file_index = ?1
                 ORDER BY c.is_forward_declaration ASC -- Prioritize non-forward declarations from classes table
                 LIMIT 1",
                [file_index as i64],
                |row| row.get::<_, String>(0)
            ).optional()
            .map_err(|e| e.into())
        })
    }

    /// Check if a class inherits from a specific base class
    /// This is the core logic for REQ-2.3 - inheritance checking
    pub fn inherits_from(&self, class_id: &str, base_class: &str, max_depth: Option<i32>) -> Result<bool> {
        // If the class is the same as the base class, it trivially "inherits" from itself
        if class_id == base_class {
            return Ok(true);
        }

        let max_depth = max_depth.unwrap_or(50); // Default depth limit from config
        
        self.db.with_connection(|conn| {
            // Use recursive CTE to traverse the inheritance chain efficiently
            let mut stmt = conn.prepare(
                "WITH RECURSIVE inheritance_chain(id, parent_id, depth) AS (
                    -- Base case: start with the given class
                    SELECT id, parent_id, 0 as depth
                    FROM classes
                    WHERE id = ?1
                    
                    UNION ALL
                    
                    -- Recursive case: follow parent relationships
                    SELECT c.id, c.parent_id, ic.depth + 1
                    FROM classes c
                    JOIN inheritance_chain ic ON ic.parent_id = c.id
                    WHERE ic.depth < ?3 AND ic.parent_id IS NOT NULL
                )
                SELECT EXISTS(
                    SELECT 1 FROM inheritance_chain 
                    WHERE id = ?2
                ) as inherits"
            )?;
            
            let inherits: bool = stmt.query_row(
                params![class_id, base_class, max_depth],
                |row| row.get(0)
            )?;
            
            Ok(inherits)
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

    /// Get the full inheritance chain for a class (from class to root)
    /// Useful for debugging and understanding inheritance relationships
    pub fn get_inheritance_chain(&self, class_id: &str, max_depth: Option<i32>) -> Result<Vec<String>> {
        let max_depth = max_depth.unwrap_or(50);
        
        self.db.with_connection(|conn| {
            // Simple approach: start with the class and follow parents iteratively
            let mut chain = Vec::new();
            let mut current_class = class_id.to_string();
            let mut depth = 0;
            
            while depth < max_depth {
                // Add current class to chain
                chain.push(current_class.clone());
                
                // Get parent of current class - handle NULL parent_id properly
                let parent_result: rusqlite::Result<Option<String>> = conn.query_row(
                    "SELECT parent_id FROM classes WHERE id = ?1",
                    [&current_class],
                    |row| {
                        let parent_id: Option<String> = row.get(0)?;
                        Ok(parent_id)
                    }
                );
                
                match parent_result {
                    Ok(Some(parent)) => {
                        current_class = parent;
                        depth += 1;
                    }
                    Ok(None) => break, // Reached root (parent_id is NULL)
                    Err(rusqlite::Error::QueryReturnedNoRows) => break, // Class not found
                    Err(e) => return Err(e.into()), // Other error
                }
            }
            
            Ok(chain)
        })
    }
}

// Function that converts Option<usize> to Option<i64> properly
fn convert_opt_usize_to_i64(val: Option<usize>) -> Option<i64> {
    val.map(|v| v as i64)
}

// Function that converts Option<i64> to Option<usize>
fn convert_i64_to_usize(val: Option<i64>) -> Option<usize> {
    val.map(|v| v as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_class_repository() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let repo = ClassRepository::new(&db);
        
        // Create test classes without foreign key relationships
        let class1 = ClassModel::new(
            "Class1".to_string(), 
            None::<String>,
            None::<String>,
            Some(1),
            false
        );
        
        let class2 = ClassModel::new(
            "Class2".to_string(), 
            Some("Class1".to_string()),
            None::<String>,
            Some(2),
            false
        );
        
        let class3 = ClassModel::new(
            "Class3".to_string(), 
            Some("Class2".to_string()),
            None::<String>,
            Some(3),
            true
        );
        
        // Insert classes
        repo.create(&class1).unwrap();
        repo.create(&class2).unwrap();
        repo.create(&class3).unwrap();
        
        // Test get
        let result = repo.get("Class1").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "Class1");
        
        // Test get_children
        let children = repo.get_children("Class1").unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, "Class2");
        
        // Test get_hierarchy
        let hierarchy = repo.get_hierarchy("Class1", 2).unwrap();
        assert_eq!(hierarchy.len(), 3);
        assert!(hierarchy.iter().any(|n| n.id == "Class3" && n.is_forward_declaration)); // Check flag
        assert!(hierarchy.iter().any(|n| n.id == "Class2" && !n.is_forward_declaration)); // Check flag
        
        // Test find_orphaned_by_parent_removal
        let orphaned = repo.find_orphaned_by_parent_removal(&["Class1".to_string()]).unwrap();
        assert_eq!(orphaned.len(), 1);
        assert_eq!(orphaned[0].id, "Class2");
        
        // Test find_affected_children
        let affected = repo.find_affected_children(&["Class1".to_string()], 10).unwrap();
        assert_eq!(affected.len(), 2); // Should find Class2 and Class3
        assert!(affected.iter().any(|c| c.id == "Class3" && c.is_forward_declaration)); // Check flag
        assert!(affected.iter().any(|c| c.id == "Class2" && !c.is_forward_declaration)); // Check flag
    }

    #[test]
    fn test_inheritance_checking() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_inheritance.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let repo = ClassRepository::new(&db);
        
        // Create simple inheritance chain: A -> B -> C
        let class_a = ClassModel::new(
            "A".to_string(), 
            None::<String>,
            None::<String>,
            Some(1),
            false
        );
        
        let class_b = ClassModel::new(
            "B".to_string(), 
            Some("A".to_string()),
            None::<String>,
            Some(2),
            false
        );
        
        let class_c = ClassModel::new(
            "C".to_string(), 
            Some("B".to_string()),
            None::<String>,
            Some(3),
            false
        );
        
        // Insert all classes
        repo.create(&class_a).unwrap();
        repo.create(&class_b).unwrap();
        repo.create(&class_c).unwrap();
        
        // Test inheritance chain retrieval first
        println!("Testing inheritance chain for C:");
        let chain = repo.get_inheritance_chain("C", None).unwrap();
        println!("Chain: {:?}", chain);
        
        // Should be: ["C", "B", "A"]
        assert_eq!(chain.len(), 3);
        assert_eq!(chain[0], "C"); // Should start with the class itself
        assert_eq!(chain[1], "B");
        assert_eq!(chain[2], "A");
        
        // Test basic inheritance checking
        println!("Testing inheritance checking:");
        
        // C should inherit from A and B
        assert!(repo.inherits_from("C", "A", None).unwrap());
        assert!(repo.inherits_from("C", "B", None).unwrap());
        assert!(repo.inherits_from("C", "C", None).unwrap()); // Self-inheritance
        
        // B should inherit from A
        assert!(repo.inherits_from("B", "A", None).unwrap());
        assert!(repo.inherits_from("B", "B", None).unwrap()); // Self-inheritance
        
        // Negative cases
        assert!(!repo.inherits_from("A", "B", None).unwrap()); // Reverse direction
        assert!(!repo.inherits_from("A", "C", None).unwrap()); // Reverse direction
        
        println!("All basic tests passed!");
    }

    #[test]
    fn test_inherits_from_any() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_inherits_from_any.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let repo = ClassRepository::new(&db);
        
        // Create inheritance hierarchy: 
        // Vehicle -> Car -> SportsCar
        //         -> Truck -> PickupTruck
        let vehicle = ClassModel::new(
            "Vehicle".to_string(), 
            None::<String>,
            None::<String>,
            Some(1),
            false
        );
        
        let car = ClassModel::new(
            "Car".to_string(), 
            Some("Vehicle".to_string()),
            None::<String>,
            Some(2),
            false
        );
        
        let sports_car = ClassModel::new(
            "SportsCar".to_string(), 
            Some("Car".to_string()),
            None::<String>,
            Some(3),
            false
        );
        
        let truck = ClassModel::new(
            "Truck".to_string(), 
            Some("Vehicle".to_string()),
            None::<String>,
            Some(4),
            false
        );
        
        let pickup_truck = ClassModel::new(
            "PickupTruck".to_string(), 
            Some("Truck".to_string()),
            None::<String>,
            Some(5),
            false
        );
        
        // Insert all classes
        repo.create(&vehicle).unwrap();
        repo.create(&car).unwrap();
        repo.create(&sports_car).unwrap();
        repo.create(&truck).unwrap();
        repo.create(&pickup_truck).unwrap();
        
        // Test inherits_from_any with various scenarios
        
        // Test 1: SportsCar should inherit from any of [Vehicle, Car]
        let base_classes = vec!["Vehicle".to_string(), "Car".to_string()];
        assert!(repo.inherits_from_any("SportsCar", &base_classes, None).unwrap());
        
        // Test 2: PickupTruck should inherit from Vehicle but not Car
        let base_classes = vec!["Vehicle".to_string()];
        assert!(repo.inherits_from_any("PickupTruck", &base_classes, None).unwrap());
        
        let base_classes = vec!["Car".to_string()];
        assert!(!repo.inherits_from_any("PickupTruck", &base_classes, None).unwrap());
        
        // Test 3: Multiple base classes - should match any
        let base_classes = vec!["Car".to_string(), "Truck".to_string()];
        assert!(repo.inherits_from_any("SportsCar", &base_classes, None).unwrap()); // Matches Car
        assert!(repo.inherits_from_any("PickupTruck", &base_classes, None).unwrap()); // Matches Truck
        
        // Test 4: Self-inheritance
        let base_classes = vec!["SportsCar".to_string()];
        assert!(repo.inherits_from_any("SportsCar", &base_classes, None).unwrap());
        
        // Test 5: Empty base classes list
        let base_classes = vec![];
        assert!(!repo.inherits_from_any("SportsCar", &base_classes, None).unwrap());
        
        // Test 6: Non-existent class
        let base_classes = vec!["NonExistent".to_string()];
        assert!(!repo.inherits_from_any("SportsCar", &base_classes, None).unwrap());
        
        // Test 7: Case sensitivity - should be case-insensitive according to task
        let base_classes = vec!["vehicle".to_string(), "CAR".to_string()]; // Mixed case
        assert!(repo.inherits_from_any("SportsCar", &base_classes, None).unwrap());
        assert!(repo.inherits_from_any("sportscar", &base_classes, None).unwrap());
        
        // Test 8: Max depth limiting
        let base_classes = vec!["Vehicle".to_string()];
        assert!(repo.inherits_from_any("SportsCar", &base_classes, Some(10)).unwrap());
        assert!(repo.inherits_from_any("SportsCar", &base_classes, Some(2)).unwrap());
        assert!(!repo.inherits_from_any("SportsCar", &base_classes, Some(1)).unwrap()); // Too shallow
        
        println!("All inherits_from_any tests passed!");
    }

    #[test]
    fn test_inherits_from_any_edge_cases() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_inherits_from_any_edge.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let repo = ClassRepository::new(&db);
        
        // Test with empty database
        let base_classes = vec!["NonExistent".to_string()];
        assert!(!repo.inherits_from_any("AlsoNonExistent", &base_classes, None).unwrap());
        
        // Create a single class with no parent
        let standalone = ClassModel::new(
            "Standalone".to_string(), 
            None::<String>,
            None::<String>,
            Some(1),
            false
        );
        repo.create(&standalone).unwrap();
        
        // Test standalone class
        let base_classes = vec!["Standalone".to_string()];
        assert!(repo.inherits_from_any("Standalone", &base_classes, None).unwrap()); // Self
        
        let base_classes = vec!["Other".to_string()];
        assert!(!repo.inherits_from_any("Standalone", &base_classes, None).unwrap());
        
        println!("All edge case tests passed!");
    }
} 