use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::scanning::classes::processor::{ProcessedClass, ProcessingStats};
use crate::reporting::{ReportConfig, ReportFormat};
use crate::reporting::class::{
    ClassReportWriter, ClassStats, CategorizedClasses, ClassHierarchy, 
    ClassNode, CircularDependency, ClassCategoryCount
};

/// Class report manager
pub struct ClassReportManager {
    output_dir: PathBuf,
    config: Option<ReportConfig>,
}

impl ClassReportManager {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: None,
        }
    }
    
    /// Create a new ClassReportManager with a specific configuration
    pub fn with_config(output_dir: &Path, config: ReportConfig) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
            config: Some(config),
        }
    }
    
    /// Write all class reports
    pub fn write_all_reports(&self, classes: &[ProcessedClass]) -> Result<PathBuf> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the ClassReportWriter to write reports
        let report_writer = if let Some(config) = &self.config {
            ClassReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            ClassReportWriter::new(&self.output_dir)
        };
        
        // Write the main classes report
        let path = report_writer.write_all_reports(classes)?;
        
        // Generate and write categorized classes report
        let categorized = self.categorize_classes(classes);
        report_writer.write_categorized_report(&categorized)?;
        
        // Detect and report circular dependencies
        let circular_deps = self.detect_circular_dependencies(classes);
        report_writer.write_circular_dependencies_report(&circular_deps)?;
        
        // Build and write class hierarchy
        let hierarchy = self.build_class_hierarchy(classes);
        report_writer.write_hierarchy_report(&hierarchy)?;
        
        // Calculate and write statistics
        let stats = self.create_processing_stats(classes);
        self.write_stats_report(&stats)?;
        
        info!("Wrote class reports to {}", self.output_dir.display());
        
        Ok(path)
    }
    
    /// Create processing stats from processed classes
    fn create_processing_stats(&self, classes: &[ProcessedClass]) -> ProcessingStats {
        let total_classes = classes.len();
        let mut file_paths = HashSet::new();
        
        // Collect unique file paths
        for class in classes {
            if let Some(path) = &class.file_path {
                file_paths.insert(path.clone());
            }
        }
        
        let total_files = file_paths.len();
        
        // For simplicity, we'll set these to default values
        // In a real implementation, you'd calculate these properly
        ProcessingStats {
            total_classes,
            total_files,
            files_with_classes: total_files,
            empty_files: 0,
            error_files: 0,
            error_file_paths: Vec::new(),
        }
    }
    
    /// Write statistics report
    pub fn write_stats_report(&self, stats: &ProcessingStats) -> Result<PathBuf> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the ClassReportWriter to write the stats report
        let report_writer = if let Some(config) = &self.config {
            ClassReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            ClassReportWriter::new(&self.output_dir)
        };
        
        // Create class stats from processing stats
        let class_stats = self.create_class_stats(stats);
        
        // Write the stats report
        let path = report_writer.write_stats_report(&class_stats)?;
        
        Ok(path)
    }
    
    /// Create class statistics from processing stats
    fn create_class_stats(&self, stats: &ProcessingStats) -> ClassStats {
        // For simplicity, we'll create some dummy category counts
        // In a real implementation, you'd calculate these from the actual classes
        let class_categories = vec![
            ClassCategoryCount { 
                category: "Vehicles".to_string(), 
                count: stats.total_classes / 3 
            },
            ClassCategoryCount { 
                category: "Weapons".to_string(), 
                count: stats.total_classes / 3 
            },
            ClassCategoryCount { 
                category: "Other".to_string(), 
                count: stats.total_classes / 3 
            },
        ];
        
        ClassStats {
            total_classes: stats.total_classes,
            total_files: stats.total_files,
            files_with_classes: stats.files_with_classes,
            empty_files: stats.empty_files,
            error_files: stats.error_files,
            class_categories,
        }
    }
    
    /// Categorize classes by their category
    fn categorize_classes(&self, classes: &[ProcessedClass]) -> CategorizedClasses {
        let mut categories = HashMap::new();
        
        for class in classes {
            // In this simplified version, we'll determine category based on class name
            // In a real implementation, you'd use the actual category field
            let category = if class.name.contains("Vehicle") || class.name.contains("Car") || class.name.contains("Tank") {
                "Vehicles".to_string()
            } else if class.name.contains("Weapon") || class.name.contains("Rifle") || class.name.contains("Gun") {
                "Weapons".to_string()
            } else {
                "Other".to_string()
            };
            
            let class_list = categories.entry(category).or_insert_with(Vec::new);
            class_list.push(class.name.clone());
        }
        
        CategorizedClasses { categories }
    }
    
    /// Build a class hierarchy from processed classes
    fn build_class_hierarchy(&self, classes: &[ProcessedClass]) -> ClassHierarchy {
        // Create a map of parent classes to their children
        let mut child_map: HashMap<String, Vec<String>> = HashMap::new();
        
        // Add all classes to the map
        for class in classes {
            if let Some(parent) = &class.parent {
                let children = child_map.entry(parent.clone()).or_insert_with(Vec::new);
                children.push(class.name.clone());
            }
            
            // Ensure every class is in the map, even if it has no children
            if !child_map.contains_key(&class.name) {
                child_map.insert(class.name.clone(), Vec::new());
            }
        }
        
        // Find root classes (those that have no parent)
        let mut root_classes = Vec::new();
        let mut visited = HashSet::new();
        
        for class in classes {
            if class.parent.is_none() && !visited.contains(&class.name) {
                let node = self.build_class_node(&class.name, &child_map, &mut visited);
                root_classes.push(node);
            }
        }
        
        // Add any remaining classes that weren't visited
        for class in classes {
            if !visited.contains(&class.name) {
                let node = self.build_class_node(&class.name, &child_map, &mut visited);
                root_classes.push(node);
            }
        }
        
        ClassHierarchy { root_classes }
    }
    
    /// Build a class node for the hierarchy
    fn build_class_node(&self, class_name: &str, child_map: &HashMap<String, Vec<String>>, 
                        visited: &mut std::collections::HashSet<String>) -> ClassNode {
        visited.insert(class_name.to_string());
        
        let mut children = Vec::new();
        let mut is_circular = false;
        let mut circular_ref = None;
        
        if let Some(child_names) = child_map.get(class_name) {
            for child_name in child_names {
                if visited.contains(child_name) {
                    // Circular reference detected
                    is_circular = true;
                    circular_ref = Some(child_name.clone());
                } else {
                    let child_node = self.build_class_node(child_name, child_map, visited);
                    children.push(child_node);
                }
            }
        }
        
        ClassNode {
            name: class_name.to_string(),
            children,
            circular_ref,
            is_circular,
        }
    }
    
    /// Detect circular dependencies in class inheritance
    fn detect_circular_dependencies(&self, classes: &[ProcessedClass]) -> Vec<CircularDependency> {
        let mut circular_deps = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        // Check each class for circular dependencies
        for class in classes {
            if !visited.contains(&class.name) {
                self.detect_cycles(
                    &class.name,
                    &mut visited,
                    &mut path,
                    classes,
                    &mut circular_deps
                );
            }
        }
        
        circular_deps
    }
    
    /// Detect cycles in class inheritance using DFS
    fn detect_cycles(
        &self,
        class_name: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        classes: &[ProcessedClass],
        circular_deps: &mut Vec<CircularDependency>
    ) {
        // Mark the current class as visited and add it to the path
        visited.insert(class_name.to_string());
        path.push(class_name.to_string());
        
        // Find the class in the processed classes
        if let Some(class) = classes.iter().find(|c| c.name == class_name) {
            // Check the parent class
            if let Some(parent) = &class.parent {
                if !visited.contains(parent) {
                    // Recursively check the parent
                    self.detect_cycles(parent, visited, path, classes, circular_deps);
                } else if path.contains(parent) {
                    // Circular dependency detected
                    let cycle_start = path.iter().position(|c| c == parent).unwrap();
                    let cycle_path = path[cycle_start..].to_vec();
                    
                    circular_deps.push(CircularDependency {
                        class_name: class_name.to_string(),
                        parent_class: parent.clone(),
                        cycle_path,
                    });
                }
            }
        }
        
        // Remove the current class from the path
        path.pop();
    }
} 