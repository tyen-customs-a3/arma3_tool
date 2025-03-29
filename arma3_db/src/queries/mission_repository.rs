use std::path::Path;
use log::debug;
use rusqlite::{params, Row, OptionalExtension};

use crate::error::{Result, DatabaseError};
use crate::models::mission::{MissionModel, MissionComponentModel, MissionDependencyModel};
use crate::DatabaseManager;
use chrono::Utc;

/// Repository for mission-related database operations
pub struct MissionRepository<'a> {
    db: &'a DatabaseManager,
}

impl<'a> MissionRepository<'a> {
    /// Create a new mission repository
    pub fn new(db: &'a DatabaseManager) -> Self {
        Self { db }
    }
    
    /// Create a new mission
    pub fn create(&self, mission: &MissionModel) -> Result<()> {
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT OR REPLACE INTO missions (id, name, path, source_pbo_id, author, description, scan_time)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    mission.id,
                    mission.name,
                    mission.path.to_string_lossy(),
                    mission.source_pbo_id,
                    mission.author,
                    mission.description,
                    mission.scan_time.to_rfc3339(),
                ],
            )?;
            
            Ok(())
        })
    }
    
    /// Get a mission by ID
    pub fn get(&self, id: &str) -> Result<Option<MissionModel>> {
        self.db.with_connection(|conn| {
            let result = conn.query_row(
                "SELECT id, name, path, source_pbo_id, author, description, scan_time
                 FROM missions WHERE id = ?1",
                [id],
                |row| self.map_row_to_mission(row),
            ).optional()?;
            
            Ok(result)
        })
    }
    
    /// Update a mission
    pub fn update(&self, mission: &MissionModel) -> Result<()> {
        self.db.with_connection(|conn| {
            let rows_affected = conn.execute(
                "UPDATE missions 
                 SET name = ?2, path = ?3, source_pbo_id = ?4, author = ?5, description = ?6, scan_time = ?7
                 WHERE id = ?1",
                params![
                    mission.id,
                    mission.name,
                    mission.path.to_string_lossy(),
                    mission.source_pbo_id,
                    mission.author,
                    mission.description,
                    mission.scan_time.to_rfc3339(),
                ],
            )?;
            
            if rows_affected == 0 {
                return Err(DatabaseError::NotFound {
                    entity_type: "Mission".to_string(),
                    id: mission.id.clone(),
                });
            }
            
            Ok(())
        })
    }
    
    /// Delete a mission
    pub fn delete(&self, id: &str) -> Result<()> {
        self.db.with_transaction(|tx| {
            // Delete associated dependencies first
            tx.execute(
                "DELETE FROM mission_dependencies WHERE mission_id = ?1",
                [id],
            )?;
            
            // Delete associated components
            tx.execute(
                "DELETE FROM mission_components WHERE mission_id = ?1",
                [id],
            )?;
            
            // Delete the mission
            let rows_affected = tx.execute(
                "DELETE FROM missions WHERE id = ?1",
                [id],
            )?;
            
            if rows_affected == 0 {
                return Err(DatabaseError::NotFound {
                    entity_type: "Mission".to_string(),
                    id: id.to_string(),
                });
            }
            
            Ok(())
        })
    }
    
    /// Get all missions
    pub fn get_all(&self) -> Result<Vec<MissionModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, path, source_pbo_id, author, description, scan_time
                 FROM missions ORDER BY name"
            )?;
            
            let rows = stmt.query_map([], |row| self.map_row_to_mission(row))?;
            
            let mut missions = Vec::new();
            for row_result in rows {
                missions.push(row_result?);
            }
            
            Ok(missions)
        })
    }
    
    /// Add a component to a mission
    pub fn add_component(&self, component: &MissionComponentModel) -> Result<i64> {
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT INTO mission_components (mission_id, name, component_type, source_file)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    component.mission_id,
                    component.name,
                    component.component_type,
                    component.source_file.to_string_lossy(),
                ],
            )?;
            
            let id = conn.last_insert_rowid();
            Ok(id)
        })
    }
    
    /// Get components for a mission
    pub fn get_components(&self, mission_id: &str) -> Result<Vec<MissionComponentModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, mission_id, name, component_type, source_file
                 FROM mission_components
                 WHERE mission_id = ?1
                 ORDER BY name"
            )?;
            
            let rows = stmt.query_map([mission_id], |row| self.map_row_to_component(row))?;
            
            let mut components = Vec::new();
            for row_result in rows {
                components.push(row_result?);
            }
            
            Ok(components)
        })
    }
    
    /// Delete all components for a mission
    pub fn delete_components(&self, mission_id: &str) -> Result<()> {
        self.db.with_connection(|conn| {
            // Delete dependencies associated with components first
            conn.execute(
                "DELETE FROM mission_dependencies 
                 WHERE mission_id = ?1 AND component_id IS NOT NULL",
                [mission_id],
            )?;
            
            // Delete the components
            conn.execute(
                "DELETE FROM mission_components WHERE mission_id = ?1",
                [mission_id],
            )?;
            
            Ok(())
        })
    }
    
    /// Add a dependency to a mission
    pub fn add_dependency(&self, dependency: &MissionDependencyModel) -> Result<i64> {
        self.db.with_connection(|conn| {
            conn.execute(
                "INSERT INTO mission_dependencies 
                 (mission_id, component_id, class_name, reference_type, source_file, line_number)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    dependency.mission_id,
                    dependency.component_id,
                    dependency.class_name,
                    dependency.reference_type,
                    dependency.source_file.to_string_lossy(),
                    convert_opt_usize_to_i64(dependency.line_number),
                ],
            )?;
            
            let id = conn.last_insert_rowid();
            Ok(id)
        })
    }
    
    /// Add multiple dependencies
    pub fn add_dependencies(&self, dependencies: &[MissionDependencyModel]) -> Result<()> {
        if dependencies.is_empty() {
            return Ok(());
        }
        
        self.db.with_transaction(|tx| {
            let mut stmt = tx.prepare(
                "INSERT INTO mission_dependencies 
                 (mission_id, component_id, class_name, reference_type, source_file, line_number)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
            )?;
            
            for dep in dependencies {
                stmt.execute(params![
                    dep.mission_id,
                    dep.component_id,
                    dep.class_name,
                    dep.reference_type,
                    dep.source_file.to_string_lossy(),
                    convert_opt_usize_to_i64(dep.line_number),
                ])?;
            }
            
            Ok(())
        })
    }
    
    /// Get dependencies for a mission
    pub fn get_dependencies(&self, mission_id: &str) -> Result<Vec<MissionDependencyModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, mission_id, component_id, class_name, reference_type, source_file, line_number
                 FROM mission_dependencies
                 WHERE mission_id = ?1
                 ORDER BY class_name"
            )?;
            
            let rows = stmt.query_map([mission_id], |row| self.map_row_to_dependency(row))?;
            
            let mut dependencies = Vec::new();
            for row_result in rows {
                dependencies.push(row_result?);
            }
            
            Ok(dependencies)
        })
    }
    
    /// Get dependencies for a specific component
    pub fn get_component_dependencies(&self, component_id: i64) -> Result<Vec<MissionDependencyModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT id, mission_id, component_id, class_name, reference_type, source_file, line_number
                 FROM mission_dependencies
                 WHERE component_id = ?1
                 ORDER BY class_name"
            )?;
            
            let rows = stmt.query_map([component_id], |row| self.map_row_to_dependency(row))?;
            
            let mut dependencies = Vec::new();
            for row_result in rows {
                dependencies.push(row_result?);
            }
            
            Ok(dependencies)
        })
    }
    
    /// Delete all dependencies for a mission
    pub fn delete_dependencies(&self, mission_id: &str) -> Result<()> {
        self.db.with_connection(|conn| {
            conn.execute(
                "DELETE FROM mission_dependencies WHERE mission_id = ?1",
                [mission_id],
            )?;
            
            Ok(())
        })
    }
    
    /// Import mission data from a shared model
    pub fn import_from_shared_model(&self, mission: &arma3_tool_shared_models::Mission) -> Result<()> {
        debug!("Importing mission: {}", mission.name);
        
        // Create transaction
        self.db.with_transaction(|tx| {
            // Convert mission model
            let mission_model = MissionModel::from_mission(mission);
            
            // Insert mission
            tx.execute(
                "INSERT OR REPLACE INTO missions (id, name, path, source_pbo_id, author, description, scan_time)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    mission_model.id,
                    mission_model.name,
                    mission_model.path.to_string_lossy(),
                    mission_model.source_pbo_id,
                    mission_model.author,
                    mission_model.description,
                    mission_model.scan_time.to_rfc3339(),
                ],
            )?;
            
            // Delete existing components and dependencies
            tx.execute(
                "DELETE FROM mission_dependencies WHERE mission_id = ?1",
                [&mission_model.id],
            )?;
            
            tx.execute(
                "DELETE FROM mission_components WHERE mission_id = ?1",
                [&mission_model.id],
            )?;
            
            // Store mission-level dependencies
            let mut stmt_dep = tx.prepare(
                "INSERT INTO mission_dependencies 
                 (mission_id, component_id, class_name, reference_type, source_file, line_number)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
            )?;
            
            for dep in &mission.dependencies {
                let model = MissionDependencyModel::from_dependency(&mission_model.id, dep);
                stmt_dep.execute(params![
                    model.mission_id,
                    model.component_id,
                    model.class_name,
                    model.reference_type,
                    model.source_file.to_string_lossy(),
                    convert_opt_usize_to_i64(model.line_number),
                ])?;
            }
            
            // Store components
            let mut stmt_comp = tx.prepare(
                "INSERT INTO mission_components (mission_id, name, component_type, source_file)
                 VALUES (?1, ?2, ?3, ?4)"
            )?;
            
            for comp in &mission.components {
                let model = MissionComponentModel::from_component(&mission_model.id, comp);
                stmt_comp.execute(params![
                    model.mission_id,
                    model.name,
                    model.component_type,
                    model.source_file.to_string_lossy(),
                ])?;
                
                let component_id = tx.last_insert_rowid();
                
                // Store component dependencies
                for dep in &comp.dependencies {
                    let dep_model = MissionDependencyModel::from_component_dependency(
                        &mission_model.id,
                        component_id,
                        dep,
                    );
                    stmt_dep.execute(params![
                        dep_model.mission_id,
                        dep_model.component_id,
                        dep_model.class_name,
                        dep_model.reference_type,
                        dep_model.source_file.to_string_lossy(),
                        convert_opt_usize_to_i64(dep_model.line_number),
                    ])?;
                }
            }
            
            Ok(())
        })
    }
    
    /// Import all missions from a shared model collection
    pub fn import_from_collection(&self, mission_data: &arma3_tool_shared_models::MissionData) -> Result<()> {
        debug!("Importing {} missions", mission_data.missions.len());
        
        for mission in &mission_data.missions {
            self.import_from_shared_model(mission)?;
        }
        
        Ok(())
    }
    
    /// Clear all missions
    pub fn clear_all(&self) -> Result<()> {
        self.db.with_transaction(|tx| {
            tx.execute("DELETE FROM mission_dependencies", [])?;
            tx.execute("DELETE FROM mission_components", [])?;
            tx.execute("DELETE FROM missions", [])?;
            Ok(())
        })
    }
    
    /// Find missions by class dependency
    pub fn find_by_class_dependency(&self, class_name: &str) -> Result<Vec<MissionModel>> {
        self.db.with_connection(|conn| {
            let mut stmt = conn.prepare(
                "SELECT DISTINCT m.id, m.name, m.path, m.source_pbo_id, m.author, m.description, m.scan_time
                 FROM missions m
                 JOIN mission_dependencies d ON m.id = d.mission_id
                 WHERE d.class_name = ?1
                 ORDER BY m.name"
            )?;
            
            let rows = stmt.query_map([class_name], |row| self.map_row_to_mission(row))?;
            
            let mut missions = Vec::new();
            for row_result in rows {
                missions.push(row_result?);
            }
            
            Ok(missions)
        })
    }
    
    /// Count dependencies on a class
    pub fn count_class_dependencies(&self, class_name: &str) -> Result<i64> {
        self.db.with_connection(|conn| {
            let count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM mission_dependencies WHERE class_name = ?1",
                [class_name],
                |row| row.get(0),
            )?;
            
            Ok(count)
        })
    }
    
    /// Convert a database row to a MissionModel
    fn map_row_to_mission(&self, row: &Row) -> rusqlite::Result<MissionModel> {
        let scan_time_str: String = row.get(6)?;
        let scan_time = chrono::DateTime::parse_from_rfc3339(&scan_time_str)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        
        Ok(MissionModel {
            id: row.get(0)?,
            name: row.get(1)?,
            path: Path::new(&row.get::<_, String>(2)?).to_path_buf(),
            source_pbo_id: row.get(3)?,
            author: row.get(4)?,
            description: row.get(5)?,
            scan_time,
        })
    }
    
    /// Convert a database row to a MissionComponentModel
    fn map_row_to_component(&self, row: &Row) -> rusqlite::Result<MissionComponentModel> {
        Ok(MissionComponentModel {
            id: row.get(0)?,
            mission_id: row.get(1)?,
            name: row.get(2)?,
            component_type: row.get(3)?,
            source_file: Path::new(&row.get::<_, String>(4)?).to_path_buf(),
        })
    }
    
    /// Convert a database row to a MissionDependencyModel
    fn map_row_to_dependency(&self, row: &Row) -> rusqlite::Result<MissionDependencyModel> {
        let line_number: Option<i64> = row.get(6)?;
        
        Ok(MissionDependencyModel {
            id: row.get(0)?,
            mission_id: row.get(1)?,
            component_id: row.get(2)?,
            class_name: row.get(3)?,
            reference_type: row.get(4)?,
            source_file: Path::new(&row.get::<_, String>(5)?).to_path_buf(),
            line_number: line_number.map(|n| n as usize),
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
    use chrono::Utc;
    
    #[test]
    fn test_mission_repository() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let repo = MissionRepository::new(&db);
        
        // Create test mission
        let mission = MissionModel::new(
            "test_mission",
            "Test Mission",
            "missions/test.pbo",
            Utc::now(),
        );
        
        // Insert mission
        repo.create(&mission).unwrap();
        
        // Test get
        let result = repo.get("test_mission").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "Test Mission");
        
        // Test components
        let component = MissionComponentModel::new(
            "test_mission",
            "TestComponent",
            "Vehicle",
            "mission/component.sqf",
        );
        
        let component_id = repo.add_component(&component).unwrap();
        assert!(component_id > 0);
        
        let components = repo.get_components("test_mission").unwrap();
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].name, "TestComponent");
        
        // Test dependencies
        let dependency = MissionDependencyModel::new(
            "test_mission",
            "Vehicle_F",
            "DirectClass",
            "mission/dependency.sqf",
        );
        
        let dependency_id = repo.add_dependency(&dependency).unwrap();
        assert!(dependency_id > 0);
        
        let dependencies = repo.get_dependencies("test_mission").unwrap();
        assert_eq!(dependencies.len(), 1);
        assert_eq!(dependencies[0].class_name, "Vehicle_F");
        
        // Test component dependency
        let component_dependency = MissionDependencyModel::for_component(
            "test_mission",
            component_id,
            "Car_F",
            "InheritedClass",
            "mission/component_dependency.sqf",
        );
        
        repo.add_dependency(&component_dependency).unwrap();
        
        let comp_dependencies = repo.get_component_dependencies(component_id).unwrap();
        assert_eq!(comp_dependencies.len(), 1);
        assert_eq!(comp_dependencies[0].class_name, "Car_F");
        
        // Test deletion
        repo.delete("test_mission").unwrap();
        
        let result = repo.get("test_mission").unwrap();
        assert!(result.is_none());
        
        let components = repo.get_components("test_mission").unwrap();
        assert_eq!(components.len(), 0);
        
        let dependencies = repo.get_dependencies("test_mission").unwrap();
        assert_eq!(dependencies.len(), 0);
    }
} 