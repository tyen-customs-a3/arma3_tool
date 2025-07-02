use anyhow::{anyhow, Result};
use arma3_database::{DatabaseManager, ClassRepository};
use log::{info, warn};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Export items from the analysis database to CSV format
pub async fn run_export(
    db_path: PathBuf,
    output: PathBuf,
    item_types: String,
    filter_parents: Option<String>,
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

    // Create source path cache
    let source_paths = SourcePathCache::new(&db);

    // Determine parent filters based on item types or custom filters
    let parent_filters = if let Some(custom_filters) = filter_parents {
        custom_filters.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        get_parent_filters_for_item_types(&item_types)
    };

    info!("Using parent filters: {:?}", parent_filters);

    // Create output file
    let mut file = File::create(&output)
        .map_err(|e| anyhow!("Failed to create output file: {}", e))?;
    
    info!("Exporting to: {}", output.display());

    // Write CSV header
    writeln!(file, "id,label,type,parent_id,container_class,source_path,properties")
        .map_err(|e| anyhow!("Failed to write CSV header: {}", e))?;

    // Get all classes
    let classes = class_repo.get_all()
        .map_err(|e| anyhow!("Failed to get classes from database: {}", e))?;
    
    info!("Found {} classes in database", classes.len());

    // Apply filters and process classes
    let mut exported_count = 0;
    let mut filtered_count = 0;
    
    for class in classes {
        // Skip forward declarations
        if class.is_forward_declaration {
            continue;
        }

        // Apply parent filter if specified and not empty
        if !parent_filters.is_empty() {
            let mut matches_filter = false;
            
            if let Some(parent) = &class.parent_id {
                matches_filter = parent_filters.iter().any(|filter| parent.starts_with(filter));
            }
            
            // Also check the class name itself against filters
            if !matches_filter {
                matches_filter = parent_filters.iter().any(|filter| class.id.starts_with(filter));
            }
            
            if !matches_filter {
                filtered_count += 1;
                continue;
            }
        }

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
            .map_err(|e| anyhow!("Failed to serialize properties: {}", e))?;

        // Write CSV line
        writeln!(
            file,
            "{},{},{},{},{},{},{}",
            escape_csv(&class.id),
            escape_csv(&display_name),
            "class",
            escape_csv(&class.parent_id.unwrap_or_default()),
            escape_csv(&class.container_class.unwrap_or_default()),
            escape_csv(&source_path),
            escape_csv(&properties_json)
        ).map_err(|e| anyhow!("Failed to write CSV line: {}", e))?;

        exported_count += 1;

        // Check limit
        if limit > 0 && exported_count >= limit {
            info!("Reached export limit of {} classes", limit);
            break;
        }
    }

    info!(
        "Export completed successfully. Exported {} classes (filtered out {} classes) to {}",
        exported_count, filtered_count, output.display()
    );

    Ok(())
}

/// Get parent class filters for specified item types
fn get_parent_filters_for_item_types(item_types: &str) -> Vec<String> {
    let types: Vec<String> = item_types.split(',').map(|s| s.trim().to_lowercase()).collect();
    let mut filters = Vec::new();

    for item_type in types {
        match item_type.as_str() {
            "weapons" => {
                filters.extend_from_slice(&[
                    "arifle_", "hgun_", "lmg_", "mmg_", "srifle_", "smg_",
                    "Rifle_Base_F", "Pistol_Base_F", "Launcher_Base_F",
                    "RifleCore", "LauncherCore", "PistolCore",
                ]);
            },
            "uniforms" => {
                filters.extend_from_slice(&[
                    "Uniform_Base", "UniformItem", "U_",
                ]);
            },
            "vests" => {
                filters.extend_from_slice(&[
                    "Vest_", "V_", "VestItem", "Vest_Camo_Base",
                ]);
            },
            "backpacks" => {
                filters.extend_from_slice(&[
                    "Bag_Base", "B_", "Backpack_Base",
                ]);
            },
            "vehicles" => {
                filters.extend_from_slice(&[
                    "Car_F", "Tank_F", "Helicopter_Base_F", "Plane_Base_F",
                    "Ship_F", "StaticWeapon", "AllVehicles",
                ]);
            },
            "items" => {
                filters.extend_from_slice(&[
                    "ItemCore", "ItemInfo", "AccessoryPointer_Base_F",
                    "AccessoryMuzzle_Base_F", "optic_", "muzzle_", "acc_",
                ]);
            },
            "all" => {
                // Return empty filters to include everything
                return Vec::new();
            },
            _ => {
                warn!("Unknown item type: {}", item_type);
            }
        }
    }

    filters.into_iter().map(|s| s.to_string()).collect()
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