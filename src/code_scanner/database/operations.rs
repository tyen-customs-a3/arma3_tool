use std::path::Path;
use std::collections::HashSet;
use anyhow::Result;
use log::{info, warn};
use chrono::Utc;
use sha2::{Sha256, Digest};

use crate::code_scanner::class::types::ClassScanResult;
use super::types::{ClassDatabase, ClassDatabaseStats, ClassEntry};
use super::storage::DatabaseStorage;

/// Options for querying the database
#[derive(Debug, Clone, Default)]
pub struct QueryOptions {
    /// Filter classes by parent class
    pub parent: Option<String>,
    
    /// Filter classes by property name
    pub property_name: Option<String>,
    
    /// Filter classes by property value
    pub property_value: Option<String>,
    
    /// Maximum number of results to return
    pub limit: Option<usize>,
    
    /// Sort results by this field
    pub sort_by: Option<String>,
    
    /// Sort in descending order
    pub descending: bool,
}

/// Database operations for querying and updating the database
#[derive(Debug)]
pub struct DatabaseOperations {
    /// Storage for the database
    storage: DatabaseStorage,
    
    /// The loaded database
    db: ClassDatabase,
}

impl DatabaseOperations {
    /// Create a new database operations instance
    pub fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let storage = DatabaseStorage::new(db_path);
        let db = storage.load()?;
        
        Ok(Self {
            storage,
            db,
        })
    }
    
    /// Get a reference to the database
    pub fn database(&self) -> &ClassDatabase {
        &self.db
    }
    
    /// Get a mutable reference to the database
    pub fn database_mut(&mut self) -> &mut ClassDatabase {
        &mut self.db
    }
    
    /// Save the database to disk
    pub fn save(&self) -> Result<()> {
        self.storage.save(&self.db)
    }
    
    /// Update the database with new scan results
    pub fn update_with_scan_results(&mut self, scan_result: ClassScanResult) -> Result<ClassDatabaseStats> {
        info!("Updating database with {} classes", scan_result.classes.len());
        
        let mut stats = ClassDatabaseStats::default();
        let now = Utc::now();
        
        // Track which classes we've seen in this update
        let mut seen_classes = HashSet::new();
        
        // Track which files we've processed
        let mut processed_files = HashSet::new();
        
        // Process each class
        for class in scan_result.classes {
            // Add class to seen set
            seen_classes.insert(class.name.clone());
            
            // Track the file
            if let Some(file_path) = &class.file_path {
                let path_str = file_path.to_string_lossy().to_string();
                processed_files.insert(path_str.clone());
                
                // Calculate file hash
                let file_hash = if let Ok(content) = std::fs::read_to_string(file_path) {
                    let mut hasher = Sha256::new();
                    hasher.update(content.as_bytes());
                    format!("{:x}", hasher.finalize())
                } else {
                    // If we can't read the file, use a placeholder hash
                    "unknown".to_string()
                };
                
                // Update file_classes map
                let class_names = self.db.file_classes.entry(path_str).or_insert_with(Vec::new);
                if !class_names.contains(&class.name) {
                    class_names.push(class.name.clone());
                }
                
                // Check if class already exists
                if let Some(existing) = self.db.entries.get(&class.name) {
                    // Check if the file hash has changed
                    if existing.file_hash != file_hash {
                        // Update the class
                        self.db.entries.insert(class.name.clone(), ClassEntry {
                            class,
                            added_at: existing.added_at,
                            updated_at: now,
                            file_hash,
                        });
                        stats.updated_classes += 1;
                    }
                } else {
                    // Add new class
                    self.db.entries.insert(class.name.clone(), ClassEntry {
                        class,
                        added_at: now,
                        updated_at: now,
                        file_hash,
                    });
                    stats.added_classes += 1;
                }
            } else {
                // Class has no file path, just add it
                self.db.entries.insert(class.name.clone(), ClassEntry {
                    class,
                    added_at: now,
                    updated_at: now,
                    file_hash: "unknown".to_string(),
                });
                stats.added_classes += 1;
            }
        }
        
        // Update database metadata
        self.db.updated_at = now;
        
        // Calculate stats
        stats.total_classes = self.db.entries.len();
        stats.total_files = self.db.file_classes.len();
        
        info!("Database update complete:");
        info!("- Total classes: {}", stats.total_classes);
        info!("- Total files: {}", stats.total_files);
        info!("- Added classes: {}", stats.added_classes);
        info!("- Updated classes: {}", stats.updated_classes);
        
        Ok(stats)
    }
    
    /// Query the database for classes matching the given options
    pub fn query(&self, options: &QueryOptions) -> Vec<&ClassEntry> {
        let mut results: Vec<&ClassEntry> = self.db.entries.values()
            .filter(|entry| {
                // Filter by parent
                if let Some(parent) = &options.parent {
                    if let Some(entry_parent) = &entry.class.parent {
                        if entry_parent != parent {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                
                // Filter by property name
                if let Some(prop_name) = &options.property_name {
                    if !entry.class.properties.iter().any(|(name, _)| name == prop_name) {
                        return false;
                    }
                }
                
                // Filter by property value
                if let Some(prop_value) = &options.property_value {
                    if !entry.class.properties.iter().any(|(_, value)| value == prop_value) {
                        return false;
                    }
                }
                
                true
            })
            .collect();
        
        // Sort results if requested
        if let Some(sort_by) = &options.sort_by {
            match sort_by.as_str() {
                "name" => {
                    results.sort_by(|a, b| {
                        if options.descending {
                            b.class.name.cmp(&a.class.name)
                        } else {
                            a.class.name.cmp(&b.class.name)
                        }
                    });
                }
                "added_at" => {
                    results.sort_by(|a, b| {
                        if options.descending {
                            b.added_at.cmp(&a.added_at)
                        } else {
                            a.added_at.cmp(&b.added_at)
                        }
                    });
                }
                "updated_at" => {
                    results.sort_by(|a, b| {
                        if options.descending {
                            b.updated_at.cmp(&a.updated_at)
                        } else {
                            a.updated_at.cmp(&b.updated_at)
                        }
                    });
                }
                _ => {
                    warn!("Unknown sort field: {}", sort_by);
                }
            }
        }
        
        // Apply limit if requested
        if let Some(limit) = options.limit {
            if limit < results.len() {
                results.truncate(limit);
            }
        }
        
        results
    }
    
    /// Get a class by name
    pub fn get_class(&self, name: &str) -> Option<&ClassEntry> {
        self.db.entries.get(name)
    }
    
    /// Get all classes in a file
    pub fn get_classes_in_file(&self, file_path: impl AsRef<Path>) -> Vec<&ClassEntry> {
        let path_str = file_path.as_ref().to_string_lossy().to_string();
        
        if let Some(class_names) = self.db.file_classes.get(&path_str) {
            class_names.iter()
                .filter_map(|name| self.db.entries.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get database statistics
    pub fn get_stats(&self) -> ClassDatabaseStats {
        ClassDatabaseStats {
            total_classes: self.db.entries.len(),
            total_files: self.db.file_classes.len(),
            ..ClassDatabaseStats::default()
        }
    }
} 