use rusqlite::Connection;
use crate::error::{Result, DatabaseError};
use log::{info, debug};

/// Database schema version
pub const SCHEMA_VERSION: i32 = 2;

/// Initialize database schema
pub fn initialize_schema(conn: &mut Connection) -> Result<()> {
    info!("Initializing database schema (version {})", SCHEMA_VERSION);
    
    // Begin transaction for schema creation
    let tx = conn.transaction()?;
    
    // Create metadata table first
    tx.execute(
        "CREATE TABLE IF NOT EXISTS metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;
    
    // Check if schema is already initialized
    let version: Option<i32> = tx
        .query_row(
            "SELECT CAST(value AS INTEGER) FROM metadata WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .ok();
    
    // If the schema version doesn't exist or is different, recreate schema
    match version {
        Some(existing_version) if existing_version == SCHEMA_VERSION => {
            info!("Database schema is up to date (v{})", existing_version);
            return Ok(());
        },
        Some(existing_version) => {
            info!("Schema version mismatch (found v{}, expected v{}). Delete database file to recreate.", 
                existing_version, SCHEMA_VERSION);
            return Err(DatabaseError::SchemaVersionMismatch {
                existing: existing_version,
                required: SCHEMA_VERSION,
            });
        },
        None => {
            // Create core tables
            create_tables(&tx)?;

            // Create indexes
            create_indexes(&tx)?;
            
            // Set schema version
            tx.execute(
                "INSERT OR REPLACE INTO metadata (key, value) VALUES ('schema_version', ?1)",
                [SCHEMA_VERSION.to_string()],
            )?;
            
            // Set creation timestamp
            tx.execute(
                "INSERT OR REPLACE INTO metadata (key, value) VALUES ('created_at', ?1)",
                [chrono::Utc::now().to_rfc3339()],
            )?;
        }
    }
    
    tx.commit()?;
    info!("Database schema initialized successfully");
    
    Ok(())
}

/// Create the database tables
fn create_tables(tx: &rusqlite::Transaction) -> Result<()> {
    debug!("Creating database tables");
    
    // Classes table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS classes (
            id TEXT PRIMARY KEY,
            parent_id TEXT,
            source_pbo_id TEXT,
            source_file_index INTEGER,
            FOREIGN KEY(parent_id) REFERENCES classes(id),
            FOREIGN KEY(source_pbo_id) REFERENCES pbo_files(id)
        )",
        [],
    )?;
    
    // PBO files table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS pbo_files (
            id TEXT PRIMARY KEY,
            full_path TEXT NOT NULL,
            base_dir TEXT,
            file_size INTEGER NOT NULL,
            last_modified TEXT NOT NULL,
            extraction_time TEXT NOT NULL,
            pbo_type TEXT NOT NULL
        )",
        [],
    )?;
    
    // Extracted files table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS extracted_files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pbo_id TEXT NOT NULL,
            relative_path TEXT NOT NULL,
            FOREIGN KEY(pbo_id) REFERENCES pbo_files(id)
        )",
        [],
    )?;
    
    // Failed extractions table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS failed_extractions (
            pbo_id TEXT PRIMARY KEY,
            timestamp TEXT NOT NULL,
            error_message TEXT NOT NULL,
            FOREIGN KEY(pbo_id) REFERENCES pbo_files(id)
        )",
        [],
    )?;
    
    // Mission table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS missions (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            source_pbo_id TEXT,
            author TEXT,
            description TEXT,
            scan_time TEXT NOT NULL,
            FOREIGN KEY(source_pbo_id) REFERENCES pbo_files(id)
        )",
        [],
    )?;
    
    // Mission components table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS mission_components (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mission_id TEXT NOT NULL,
            name TEXT NOT NULL,
            component_type TEXT NOT NULL,
            source_file TEXT NOT NULL,
            FOREIGN KEY(mission_id) REFERENCES missions(id)
        )",
        [],
    )?;
    
    // Mission dependencies table
    tx.execute(
        "CREATE TABLE IF NOT EXISTS mission_dependencies (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            mission_id TEXT NOT NULL,
            component_id INTEGER,
            class_name TEXT NOT NULL,
            reference_type TEXT NOT NULL,
            source_file TEXT NOT NULL,
            line_number INTEGER,
            FOREIGN KEY(mission_id) REFERENCES missions(id),
            FOREIGN KEY(component_id) REFERENCES mission_components(id)
        )",
        [],
    )?;
    
    debug!("Database tables created successfully");
    Ok(())
}

/// Create database indexes
fn create_indexes(tx: &rusqlite::Transaction) -> Result<()> {
    debug!("Creating database indexes");
    
    // Class indexes
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_classes_parent ON classes(parent_id)",
        [],
    )?;
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_classes_pbo ON classes(source_pbo_id)",
        [],
    )?;
    
    // Extracted files index
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_extracted_files_pbo ON extracted_files(pbo_id)",
        [],
    )?;
    
    // Mission component index
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_mission_components_mission ON mission_components(mission_id)",
        [],
    )?;
    
    // Mission dependency indexes
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_mission_dependencies_mission ON mission_dependencies(mission_id)",
        [],
    )?;
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_mission_dependencies_component ON mission_dependencies(component_id)",
        [],
    )?;
    tx.execute(
        "CREATE INDEX IF NOT EXISTS idx_mission_dependencies_class ON mission_dependencies(class_name)",
        [],
    )?;
    
    debug!("Database indexes created successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use tempfile::tempdir;
    
    #[test]
    fn test_schema_initialization() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        
        // Initialize schema
        initialize_schema(&mut conn).unwrap();
        
        // Verify tables were created
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        
        assert!(tables.contains(&"metadata".to_string()));
        assert!(tables.contains(&"classes".to_string()));
        assert!(tables.contains(&"pbo_files".to_string()));
        assert!(tables.contains(&"extracted_files".to_string()));
        assert!(tables.contains(&"failed_extractions".to_string()));
        assert!(tables.contains(&"missions".to_string()));
        assert!(tables.contains(&"mission_components".to_string()));
        assert!(tables.contains(&"mission_dependencies".to_string()));
        
        // Verify schema version
        let version: i32 = conn
            .query_row(
                "SELECT CAST(value AS INTEGER) FROM metadata WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        
        assert_eq!(version, SCHEMA_VERSION);
    }
    
    #[test]
    fn test_schema_already_initialized() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        
        // Initialize schema first time
        initialize_schema(&mut conn).unwrap();
        
        // Initialize schema second time (should be no-op)
        initialize_schema(&mut conn).unwrap();
        
        // Verify schema version is still correct
        let version: i32 = conn
            .query_row(
                "SELECT CAST(value AS INTEGER) FROM metadata WHERE key = 'schema_version'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        
        assert_eq!(version, SCHEMA_VERSION);
    }
    
    #[test]
    fn test_schema_version_mismatch() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        
        // Initialize with wrong version
        conn.execute("CREATE TABLE metadata (key TEXT PRIMARY KEY, value TEXT NOT NULL)", []).unwrap();
        conn.execute("INSERT INTO metadata (key, value) VALUES ('schema_version', '1')", []).unwrap();
        
        // Should fail with version mismatch
        let result = initialize_schema(&mut conn);
        assert!(result.is_err());
        
        match result {
            Err(DatabaseError::SchemaVersionMismatch { existing, required }) => {
                assert_eq!(existing, 1);
                assert_eq!(required, SCHEMA_VERSION);
            },
            _ => panic!("Expected SchemaVersionMismatch error"),
        }
    }
} 