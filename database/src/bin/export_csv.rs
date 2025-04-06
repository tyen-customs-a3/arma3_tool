use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use arma3_database::{DatabaseManager, ClassRepository};
use clap::Parser;
use serde_json;

/// CSV exporter for Arma 3 Database
/// 
/// This binary exports class data from the Arma 3 database to a CSV file format.
/// The output includes class hierarchy information, source paths, and properties
/// in the specified format.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the database file
    #[arg(short, long)]
    database: String,

    /// Output CSV file path
    #[arg(short, long, default_value = "classes_export.csv")]
    output: String,
    
    /// Include only classes with these parent prefixes (comma-separated)
    #[arg(short, long)]
    filter_parents: Option<String>,
    
    /// Maximum number of classes to export (0 for unlimited)
    #[arg(short, long, default_value = "0")]
    limit: usize,
}

// Map of file indices to source paths
struct SourcePathCache {
    paths: std::collections::HashMap<usize, String>,
}

impl SourcePathCache {
    fn new(db: &DatabaseManager) -> Self {
        let mut paths = std::collections::HashMap::new();
        
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    // Parse command line arguments
    let args = Args::parse();
    
    // Connect to the database
    let db_path = PathBuf::from(&args.database);
    if !db_path.exists() {
        return Err(format!("Database file not found: {}", args.database).into());
    }
    
    println!("Connecting to database: {}", args.database);
    let db = DatabaseManager::new(&db_path)?;
    
    // Create class repository
    let class_repo = ClassRepository::new(&db);
    
    // Create source path cache
    let source_paths = SourcePathCache::new(&db);
    
    // Process parent filters if provided
    let parent_filters: Vec<String> = if let Some(filters) = args.filter_parents {
        filters.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    
    // Create output file
    let mut file = File::create(&args.output)?;
    println!("Exporting to: {}", args.output);
    
    // Write CSV header
    writeln!(file, "id,label,type,parent_id,container_class,source_path,properties")?;
    
    // Get all classes
    let classes = class_repo.get_all()?;
    println!("Found {} classes", classes.len());
    
    // Apply filters and process classes
    let mut exported_count = 0;
    for class in classes {
        // Apply parent filter if specified
        if !parent_filters.is_empty() {
            if let Some(parent) = &class.parent_id {
                if !parent_filters.iter().any(|filter| parent.starts_with(filter)) {
                    continue;
                }
            } else {
                // Skip classes without parents if filtering is enabled
                continue;
            }
        }
        
        // Get source path if available
        let source_path = if let Some(idx) = class.source_file_index {
            source_paths.get_path(idx)
        } else {
            String::new()
        };
        
        // Create a label (use class ID if no display name is available)
        let display_name = class.properties.get("displayName")
            .map(|v| match v {
                arma3_models::types::PropertyValue::String(s) => s.clone(),
                _ => class.id.clone(),
            })
            .unwrap_or_else(|| class.id.clone());
        
        // Convert properties to JSON
        let properties_json = serde_json::to_string(&class.properties)?;
        
        // Write CSV line
        writeln!(
            file,
            "{},{},{},{},{},{},\"{}\"",
            escape_csv(&class.id),
            escape_csv(&display_name),
            "class",
            escape_csv(&class.parent_id.unwrap_or_default()),
            escape_csv(&class.container_class.unwrap_or_default()),
            escape_csv(&source_path),
            escape_csv(&properties_json)
        )?;
        
        exported_count += 1;
        
        // Check limit
        if args.limit > 0 && exported_count >= args.limit {
            println!("Reached export limit of {} classes", args.limit);
            break;
        }
    }
    
    println!("Export completed successfully. Exported {} classes to {}", exported_count, args.output);
    Ok(())
}

// Helper function to escape CSV fields
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
} 