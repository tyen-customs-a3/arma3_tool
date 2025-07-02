use std::collections::HashSet;
use log::debug;
use crate::DatabaseManager;

/// Analyze which PBOs would be left empty after class removal
pub fn analyze_pbo_impact(
    db: &DatabaseManager,
    classes_to_remove: &[String],
    orphaned_classes: &[String],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // Create a set of all classes that would be removed
    let mut all_removed = HashSet::new();
    all_removed.extend(classes_to_remove.iter().cloned());
    
    // Only include orphaned classes that aren't already in the remove list
    let filtered_orphans: Vec<String> = orphaned_classes.iter()
        .filter(|class| !all_removed.contains(*class))
        .cloned()
        .collect();
        
    all_removed.extend(filtered_orphans);
    
    debug!("Analyzing PBO impact for {} total removed classes", all_removed.len());
    
    // Query to find PBOs that would be empty
    let empty_pbos = db.with_connection(|conn| {
        // Get classes grouped by PBO
        let query = "
            SELECT COALESCE(fim.pbo_id, fim.normalized_path) as pbo_id, c.id as class_id
            FROM classes c
            JOIN file_index_mapping fim ON c.source_file_index = fim.file_index
            WHERE fim.pbo_id IS NOT NULL OR fim.normalized_path IS NOT NULL
        ";
        
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // pbo_id
                row.get::<_, String>(1)?, // class_id
            ))
        })?;
        
        // Collect classes by PBO
        let mut classes_by_pbo: std::collections::HashMap<String, Vec<String>> = 
            std::collections::HashMap::new();
            
        for row_result in rows {
            let (pbo_id, class_id) = row_result?;
            classes_by_pbo
                .entry(pbo_id)
                .or_default()
                .push(class_id);
        }
        
        debug!("Found {} PBOs with classes", classes_by_pbo.len());
        
        // Find PBOs that would be empty
        let mut empty_pbos = Vec::new();
        
        for (pbo_id, classes) in classes_by_pbo {
            // Check if all classes in this PBO would be removed
            let remaining_classes = classes.iter()
                .filter(|class| !all_removed.contains(*class))
                .count();
                
            if remaining_classes == 0 {
                empty_pbos.push(pbo_id);
            }
        }
        
        empty_pbos.sort();
        Ok(empty_pbos)
    })?;
    
    Ok(empty_pbos)
} 