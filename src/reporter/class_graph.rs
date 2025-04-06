use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::collections::HashMap;
use log::info;

use arma3_database::queries::class_repository::ClassRepository;
use crate::reporter::error::{Result as ReporterResult, ReporterError};

/// Handles writing class hierarchy as a node-edge CSV file
pub struct ClassHierarchyWriter {
    output_dir: PathBuf,
}

impl ClassHierarchyWriter {
    /// Create a new class hierarchy writer
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_path_buf(),
        }
    }

    /// Get color and group type based on first three letters of class name
    fn get_node_metadata(class_name: &str) -> (String, String) {
        let prefix = class_name.chars().take(3).collect::<String>().to_uppercase();
        
        // Map of three-letter prefixes to colors
        let color_map: HashMap<&str, &str> = [
            ("ACE", "red"),
            ("BIS", "blue"),
            ("RHS", "brown"),
            ("HLC", "purple"),
            ("CUP", "green"),
            ("CBA", "lime"),
            ("ACR", "orange"),
            ("USP", "navy"),
            ("USM", "maroon"),
            ("JSR", "yellow"),
            ("ZEN", "cyan"),
        ].iter().map(|(k, v)| (*k, *v)).collect();

        let color = color_map.get(prefix.as_str()).copied().unwrap_or("white");
        (color.to_string(), prefix)
    }

    /// Write metadata for class nodes
    fn write_metadata(&self, classes: &[arma3_database::models::class::ClassModel]) -> ReporterResult<()> {
        let file_path = self.output_dir.join("class_metadata.csv");
        let file = File::create(&file_path)
            .map_err(|e| ReporterError::WriteError {
                path: file_path.clone(),
                source: e,
            })?;

        let mut writer = BufWriter::new(file);

        // Write header
        writeln!(writer, "id;color;type")
            .map_err(|e| ReporterError::WriteError {
                path: file_path.clone(),
                source: e,
            })?;

        // Write metadata for each class
        for class in classes {
            let (color, group_type) = Self::get_node_metadata(&class.id);
            writeln!(writer, "{};{};{}", class.id, color, group_type)
                .map_err(|e| ReporterError::WriteError {
                    path: file_path.clone(),
                    source: e,
                })?;
        }

        writer.flush()
            .map_err(|e| ReporterError::WriteError {
                path: file_path.clone(),
                source: e,
            })?;

        info!("Class metadata written to: {}", file_path.display());
        Ok(())
    }

    /// Write class hierarchy to a CSV file
    pub fn write_class_graph(&self, class_repo: &ClassRepository) -> ReporterResult<()> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .map_err(|e| ReporterError::CreateDirError {
                path: self.output_dir.clone(),
                source: e,
            })?;

        // Get all classes
        let classes = class_repo.get_all()?;
        info!("Found {} classes for graph generation", classes.len());

        // Write metadata first
        self.write_metadata(&classes)?;

        // Create a map for quick lookup of parent relationships
        let mut child_map: HashMap<String, Vec<String>> = HashMap::new();

        // Build child map (invert the parent relationship)
        for class in &classes {
            if let Some(parent_id) = &class.parent_id {
                child_map.entry(parent_id.clone())
                    .or_default()
                    .push(class.id.clone());
            }
        }

        info!("Found {} parent classes with children", child_map.len());

        // Create output file
        let file_path = self.output_dir.join("class_hierarchy.csv");
        let file = File::create(&file_path)
            .map_err(|e| ReporterError::WriteError {
                path: file_path.clone(),
                source: e,
            })?;

        let mut writer = BufWriter::new(file);

        // Write header
        writeln!(writer, "source;target")
            .map_err(|e| ReporterError::WriteError {
                path: file_path.clone(),
                source: e,
            })?;

        // Write edges (only for classes that have children)
        let mut count = 0;
        for (parent, children) in &child_map {
            for child in children {
                writeln!(writer, "{};{}", parent, child)
                    .map_err(|e| ReporterError::WriteError {
                        path: file_path.clone(),
                        source: e,
                    })?;
                count += 1;
            }
        }

        writer.flush()
            .map_err(|e| ReporterError::WriteError {
                path: file_path.clone(),
                source: e,
            })?;

        info!("Class hierarchy graph written to: {} with {} edges", file_path.display(), count);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use arma3_database::{DatabaseManager, models::class::ClassModel};
    
    #[test]
    fn test_class_hierarchy_writer() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let output_dir = dir.path().join("reports");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let class_repo = ClassRepository::new(&db);
        
        // Create test classes with parent-child relationships
        let parent = ClassModel::new(
            "ParentClass".to_string(),
            None::<String>,
            None::<String>,
            Some(1),
        );
        class_repo.create(&parent).unwrap();
        
        let child1 = ClassModel::new(
            "ChildClass1".to_string(),
            Some("ParentClass".to_string()),
            None::<String>,
            Some(2),
        );
        class_repo.create(&child1).unwrap();
        
        let child2 = ClassModel::new(
            "ChildClass2".to_string(),
            Some("ParentClass".to_string()),
            None::<String>,
            Some(3),
        );
        class_repo.create(&child2).unwrap();
        
        let grandchild = ClassModel::new(
            "GrandChildClass".to_string(),
            Some("ChildClass1".to_string()),
            None::<String>,
            Some(4),
        );
        class_repo.create(&grandchild).unwrap();
        
        // Create class with no children
        let orphan = ClassModel::new(
            "OrphanClass".to_string(),
            None::<String>,
            None::<String>,
            Some(5),
        );
        class_repo.create(&orphan).unwrap();
        
        // Write class hierarchy
        let writer = ClassHierarchyWriter::new(&output_dir);
        writer.write_class_graph(&class_repo).unwrap();
        
        // Verify output files exist
        let hierarchy_file = output_dir.join("class_hierarchy.csv");
        let metadata_file = output_dir.join("class_metadata.csv");
        assert!(hierarchy_file.exists());
        assert!(metadata_file.exists());
        
        // Read hierarchy content and verify
        let hierarchy_content = std::fs::read_to_string(hierarchy_file).unwrap();
        let hierarchy_lines: Vec<&str> = hierarchy_content.lines().collect();
        
        assert_eq!(hierarchy_lines[0], "source;target");
        assert!(hierarchy_content.contains("ParentClass;ChildClass1"));
        assert!(hierarchy_content.contains("ParentClass;ChildClass2"));
        assert!(hierarchy_content.contains("ChildClass1;GrandChildClass"));
        
        // Read metadata content and verify
        let metadata_content = std::fs::read_to_string(metadata_file).unwrap();
        let metadata_lines: Vec<&str> = metadata_content.lines().collect();
        
        assert_eq!(metadata_lines[0], "id;color;type");
        assert!(metadata_content.contains("ParentClass;black;PAR"));
        assert!(metadata_content.contains("ChildClass1;black;CHI"));
        assert!(metadata_content.contains("ChildClass2;black;CHI"));
        assert!(metadata_content.contains("GrandChildClass;black;GRA"));
        assert!(metadata_content.contains("OrphanClass;black;ORP"));
    }
}