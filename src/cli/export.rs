use anyhow::{anyhow, Result};
use arma3_database::{DatabaseManager, ClassRepository};
use crate::config::ItemFilterConfig;
use log::{info, warn, debug};
use rayon::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

/// Export items from the analysis database to CSV format using inheritance-based categorization
pub async fn run_export(
    db_path: PathBuf,
    output: PathBuf,
    item_types: Option<String>,
    config_path: Option<PathBuf>,
    limit: usize,
) -> Result<()> {
    // Check if database exists
    if !db_path.exists() {
        return Err(anyhow!("Database file not found: {}", db_path.display()));
    }

    info!("Connecting to database: {}", db_path.display());
    let db = DatabaseManager::new(&db_path)
        .map_err(|e| anyhow!("Failed to create database manager: {}", e))?;

    // Create class repository
    let class_repo = ClassRepository::new(&db);

    // Load configuration file
    let config_file = config_path.unwrap_or_else(|| PathBuf::from("item_filter_config.json"));
    info!("Loading configuration from: {}", config_file.display());
    
    let config = ItemFilterConfig::from_json_file(&config_file)
        .map_err(|e| anyhow!("Failed to load configuration: {}", e))?;

    // Determine which item types to process
    let types_to_process = if let Some(types_str) = item_types {
        types_str.split(',').map(|s| s.trim().to_string()).collect::<Vec<_>>()
    } else {
        config.get_item_types().into_iter().map(|s| s.clone()).collect()
    };

    info!("Processing item types: {:?}", types_to_process);

    // Validate that all requested types exist in config
    for item_type in &types_to_process {
        if config.get_base_classes(item_type).is_none() {
            warn!("Item type '{}' not found in configuration, skipping", item_type);
        }
    }

    // Get all classes from database
    info!("Fetching all classes from database...");
    let classes = class_repo.get_all()
        .map_err(|e| anyhow!("Failed to get classes from database: {}", e))?;
    
    info!("Found {} classes in database", classes.len());

    // Create source path cache
    let source_paths = SourcePathCache::new(&db);

    // Process classes in parallel using inheritance-based categorization
    info!("Processing classes with inheritance-based categorization...");
    
    let results = Mutex::new(Vec::new());
    
    classes.par_iter().for_each(|class| {
        // Skip forward declarations
        if class.is_forward_declaration {
            return;
        }

        // Apply exclusion check: skip if scope <= max_scope (typically 1)
        if let Some(scope_prop) = class.properties.get("scope") {
            if let arma3_models::types::PropertyValue::Number(scope) = scope_prop {
                if *scope as i32 <= config.get_max_scope() {
                    debug!("Excluding class {} with scope {}", class.id, scope);
                    return;
                }
            }
        }

        // Check exclusion by prefix
        if config.is_excluded_by_prefix(&class.id) {
            debug!("Excluding class {} by prefix", class.id);
            return;
        }

        // Try to categorize this class using inheritance checking
        let mut class_categories = Vec::new();
        
        for item_type in &types_to_process {
            if let Some(base_classes) = config.get_base_classes(item_type) {
                // Convert to Vec<String> for the inherits_from_any call
                let base_classes_vec = base_classes.clone();
                
                match class_repo.inherits_from_any(&class.id, &base_classes_vec, None) {
                    Ok(true) => {
                        class_categories.push(item_type.clone());
                        debug!("Class {} matches item type {}", class.id, item_type);
                    }
                    Ok(false) => {
                        // Class doesn't inherit from any base classes for this type
                    }
                    Err(e) => {
                        warn!("Error checking inheritance for class {}: {}", class.id, e);
                    }
                }
            }
        }

        // If class matches any categories, add it to results
        if !class_categories.is_empty() {
            // Get source path if available
            let source_path = if let Some(idx) = class.source_file_index {
                source_paths.get_path(idx)
            } else {
                String::new()
            };

            // Create a label (use displayName if available, otherwise use class ID)
            let display_name = class.properties.get("displayName")
                .map(|v| match v {
                    arma3_models::types::PropertyValue::String(s) => s.clone(),
                    _ => class.id.clone(),
                })
                .unwrap_or_else(|| class.id.clone());

            // Convert properties to JSON
            let properties_json = serde_json::to_string(&class.properties)
                .unwrap_or_else(|_| "{}".to_string());

            // Create CSV record
            let csv_line = format!(
                "{},{},{},{},{},{},{}",
                escape_csv(&class.id),
                escape_csv(&display_name),
                escape_csv(&class_categories.join(";")), // Multiple categories separated by semicolon
                escape_csv(&class.parent_id.clone().unwrap_or_default()),
                escape_csv(&class.container_class.clone().unwrap_or_default()),
                escape_csv(&source_path),
                escape_csv(&properties_json)
            );

            results.lock().unwrap().push(csv_line);
        }
    });

    // Write results to CSV file
    info!("Writing results to CSV file: {}", output.display());
    let mut file = File::create(&output)
        .map_err(|e| anyhow!("Failed to create output file: {}", e))?;

    // Write CSV header
    writeln!(file, "id,label,type,parent_id,container_class,source_path,properties")
        .map_err(|e| anyhow!("Failed to write CSV header: {}", e))?;

    // Write results (applying limit if specified)
    let mut results = results.into_inner().unwrap();
    results.sort(); // Sort for consistent output

    let exported_count = if limit > 0 && limit < results.len() {
        results.truncate(limit);
        info!("Applied limit of {} classes", limit);
        limit
    } else {
        results.len()
    };

    for line in results {
        writeln!(file, "{}", line)
            .map_err(|e| anyhow!("Failed to write CSV line: {}", e))?;
    }

    info!(
        "Export completed successfully. Exported {} classes using inheritance-based categorization to {}",
        exported_count, output.display()
    );

    Ok(())
}


/// Map of file indices to source paths
struct SourcePathCache {
    paths: HashMap<usize, String>,
}

impl SourcePathCache {
    fn new(db: &DatabaseManager) -> Self {
        let mut paths = HashMap::new();

        // Try to get a direct connection
        if let Ok(conn) = db.connection() {
            // Query all file mappings at once
            if let Ok(mut stmt) = conn.prepare(
                "SELECT file_index, COALESCE(pbo_id, normalized_path) as source_path 
                 FROM file_index_mapping"
            ) {
                let results = stmt.query_map([], |row| {
                    let idx: i64 = row.get(0)?;
                    let path: String = row.get(1)?;
                    Ok((idx as usize, path))
                });

                if let Ok(rows) = results {
                    for row in rows {
                        if let Ok((idx, path)) = row {
                            paths.insert(idx, path);
                        }
                    }
                }
            }
        }

        Self { paths }
    }

    fn get_path(&self, idx: usize) -> String {
        self.paths.get(&idx).cloned().unwrap_or_default()
    }
}

/// Helper function to escape CSV fields
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
} 