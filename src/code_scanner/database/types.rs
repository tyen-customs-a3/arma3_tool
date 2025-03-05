use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::code_scanner::class::types::ProcessedClass;

/// Entry in the class database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassEntry {
    /// The processed class data
    pub class: ProcessedClass,
    
    /// When this class was first added to the database
    pub added_at: DateTime<Utc>,
    
    /// When this class was last updated in the database
    pub updated_at: DateTime<Utc>,
    
    /// Hash of the file content when this class was processed
    pub file_hash: String,
}

/// Database for storing and querying processed classes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDatabase {
    /// Map of class name to class entry
    pub entries: HashMap<String, ClassEntry>,
    
    /// Map of file path to list of class names in that file
    pub file_classes: HashMap<String, Vec<String>>,
    
    /// When this database was created
    pub created_at: DateTime<Utc>,
    
    /// When this database was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Version of the database schema
    pub version: String,
}

/// Statistics about the class database
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ClassDatabaseStats {
    /// Total number of classes in the database
    pub total_classes: usize,
    
    /// Total number of files referenced in the database
    pub total_files: usize,
    
    /// Number of classes added in the last update
    pub added_classes: usize,
    
    /// Number of classes updated in the last update
    pub updated_classes: usize,
    
    /// Number of classes removed in the last update
    pub removed_classes: usize,
}

impl Default for ClassDatabase {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            file_classes: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
} 