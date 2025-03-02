use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::{ClassReportWriter, ReportFormat, ReportConfig};
use crate::reporting::class::{ClassStats, ClassCategoryCount, CategorizedClasses, ClassHierarchy, ClassNode, CircularDependency};
use crate::scanning::classes::processor::{ProcessedClass, ProcessingStats};

/// Manager for class reports
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
        if !circular_deps.is_empty() {
            info!("Detected {} circular dependencies in class hierarchy", circular_deps.len());
            report_writer.write_circular_dependencies_report(&circular_deps)?;
        }
        
        // Write individual category reports
        for (category, class_names) in &categorized.categories {
            let category_classes: Vec<&ProcessedClass> = classes.iter()
                .filter(|c| class_names.contains(&c.name))
                .collect();
            
            if !category_classes.is_empty() {
                report_writer.write_category_report(category, &category_classes)?;
            }
        }
        
        info!("Wrote class reports to {}", self.output_dir.display());
        
        Ok(path)
    }
    
    /// Write class statistics report
    pub fn write_stats_report(&self, stats: &ProcessingStats) -> Result<PathBuf> {
        // Create class stats from processing stats
        let class_stats = self.create_class_stats(stats);
        
        // Use the ClassReportWriter to write the stats report
        let report_writer = if let Some(config) = &self.config {
            ClassReportWriter::with_config(&self.output_dir, ReportFormat::Json, config.clone())
        } else {
            ClassReportWriter::new(&self.output_dir)
        };
        
        let path = report_writer.write_stats_report(&class_stats)?;
        
        info!("Wrote class statistics report to {}", path.display());
        
        Ok(path)
    }
    
    /// Create class stats from processing stats
    fn create_class_stats(&self, stats: &ProcessingStats) -> ClassStats {
        ClassStats {
            total_classes: stats.total_classes,
            total_files: stats.total_files,
            files_with_classes: stats.files_with_classes,
            empty_files: stats.empty_files,
            error_files: stats.error_files,
            class_categories: Vec::new(), // This would be populated based on actual categories
        }
    }
    
    /// Categorize classes based on their properties and parent classes
    fn categorize_classes(&self, classes: &[ProcessedClass]) -> CategorizedClasses {
        let mut categories: HashMap<String, Vec<String>> = HashMap::new();
        
        // Use a single "All" category instead of multiple categories
        categories.insert("All".to_string(), Vec::new());
        
        // Add all classes to the "All" category
        if let Some(class_list) = categories.get_mut("All") {
            for class in classes {
                class_list.push(class.name.clone());
            }
        }
        
        CategorizedClasses { categories }
    }
    
    /// Build class hierarchy from processed classes
    fn build_class_hierarchy(&self, classes: &[ProcessedClass]) -> ClassHierarchy {
        let mut root_classes = Vec::new();
        let mut child_map: HashMap<String, Vec<String>> = HashMap::new();
        
        // First pass: identify parent-child relationships
        for class in classes {
            if let Some(parent) = &class.parent {
                child_map.entry(parent.clone())
                    .or_insert_with(Vec::new)
                    .push(class.name.clone());
            } else {
                // Classes without parents are root classes
                root_classes.push(class.name.clone());
            }
        }
        
        // Second pass: build the hierarchy tree
        let root_nodes = root_classes.iter()
            .map(|name| {
                // Use a HashSet to track visited nodes and prevent circular dependencies
                let mut visited = std::collections::HashSet::new();
                self.build_class_node(name, &child_map, &mut visited)
            })
            .collect();
        
        ClassHierarchy { root_classes: root_nodes }
    }
    
    /// Recursively build a class node and its children
    fn build_class_node(&self, class_name: &str, child_map: &HashMap<String, Vec<String>>, 
                        visited: &mut std::collections::HashSet<String>) -> ClassNode {
        // If we've already visited this node, we have a circular dependency
        // Return a node with circular reference information to break the cycle
        if !visited.insert(class_name.to_string()) {
            debug!("Detected circular dependency for class: {}", class_name);
            return ClassNode {
                name: class_name.to_string(),
                children: Vec::new(),
                circular_ref: None, // We don't know the parent in this context
                is_circular: true,
            };
        }
        
        let children = if let Some(child_names) = child_map.get(class_name) {
            child_names.iter()
                .map(|name| {
                    // Create a new visited set for each child to allow siblings to share parent classes
                    let mut child_visited = visited.clone();
                    let node = self.build_class_node(name, child_map, &mut child_visited);
                    
                    // If this is a circular reference, set the parent reference
                    if node.is_circular && node.circular_ref.is_none() {
                        ClassNode {
                            name: node.name,
                            children: node.children,
                            circular_ref: Some(class_name.to_string()),
                            is_circular: true,
                        }
                    } else {
                        node
                    }
                })
                .collect()
        } else {
            Vec::new()
        };
        
        // Remove this node from visited before returning (not strictly necessary with our clone approach)
        visited.remove(class_name);
        
        ClassNode {
            name: class_name.to_string(),
            children,
            circular_ref: None,
            is_circular: false,
        }
    }

    /// Detect circular dependencies in the class hierarchy
    fn detect_circular_dependencies(&self, classes: &[ProcessedClass]) -> Vec<CircularDependency> {
        let mut circular_deps = Vec::new();
        let mut child_map = HashMap::new();
        
        // First pass: identify parent-child relationships
        for class in classes {
            if let Some(parent) = &class.parent {
                child_map.entry(parent.clone())
                    .or_insert_with(Vec::new)
                    .push(class.name.clone());
            }
        }
        
        // Second pass: detect cycles using DFS
        for class in classes {
            let mut visited = HashSet::new();
            let mut path = Vec::new();
            self.detect_cycles(&class.name, &mut visited, &mut path, classes, &mut circular_deps);
        }
        
        circular_deps
    }

    /// Helper method to detect cycles in the class hierarchy
    fn detect_cycles(
        &self,
        class_name: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        classes: &[ProcessedClass],
        circular_deps: &mut Vec<CircularDependency>
    ) {
        // If we've already visited this class in the current path, we have a cycle
        if path.contains(&class_name.to_string()) {
            // Find the start of the cycle
            let cycle_start_idx = path.iter().position(|c| c == class_name).unwrap_or(0);
            let cycle_path = path[cycle_start_idx..].to_vec();
            
            // Get the parent class (the one that points to this class)
            let parent_class = if !path.is_empty() {
                path.last().unwrap().clone()
            } else {
                "Unknown".to_string()
            };
            
            circular_deps.push(CircularDependency {
                class_name: class_name.to_string(),
                parent_class,
                cycle_path: cycle_path.clone(),
            });
            
            debug!("Detected circular dependency for class: {} (cycle: {:?})", class_name, cycle_path);
            return;
        }
        
        // If we've already processed this class in another path, skip it
        if !visited.insert(class_name.to_string()) {
            return;
        }
        
        // Add this class to the current path
        path.push(class_name.to_string());
        
        // Recursively check the parent class
        if let Some(class) = classes.iter().find(|c| c.name == class_name) {
            if let Some(parent) = &class.parent {
                self.detect_cycles(parent, visited, path, classes, circular_deps);
            }
        }
        
        // Remove this class from the current path
        path.pop();
        
        // Remove from visited to allow this class to be part of other paths
        visited.remove(class_name);
    }
} 