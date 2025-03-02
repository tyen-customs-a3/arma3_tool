use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, Context};
use log::{info, debug};
use serde::Serialize;

use crate::reporting::ClassReportWriter;
use crate::reporting::class::{ClassStats, ClassCategoryCount, CategorizedClasses, ClassHierarchy, ClassNode};
use super::processor::{ProcessedClass, ProcessingStats};

/// Writer for class reports
pub struct ReportWriter {
    output_dir: PathBuf,
}

impl ReportWriter {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
        }
    }
    
    /// Write all class reports
    pub fn write_all_reports(&self, classes: &[ProcessedClass]) -> Result<PathBuf> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context(format!("Failed to create output directory: {}", self.output_dir.display()))?;
        
        // Use the ClassReportWriter to write reports
        let report_writer = ClassReportWriter::new(&self.output_dir);
        
        // Write the main classes report
        let path = report_writer.write_all_reports(classes)?;
        
        // Generate and write categorized classes report
        let categorized = self.categorize_classes(classes);
        report_writer.write_categorized_report(&categorized)?;
        
        // Generate and write class hierarchy report
        let hierarchy = self.build_class_hierarchy(classes);
        report_writer.write_hierarchy_report(&hierarchy)?;
        
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
        let report_writer = ClassReportWriter::new(&self.output_dir);
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
        
        // Common categories in Arma 3
        categories.insert("Vehicle".to_string(), Vec::new());
        categories.insert("Weapon".to_string(), Vec::new());
        categories.insert("Magazine".to_string(), Vec::new());
        categories.insert("Item".to_string(), Vec::new());
        categories.insert("Backpack".to_string(), Vec::new());
        categories.insert("Other".to_string(), Vec::new());
        
        for class in classes {
            let category = self.determine_class_category(class);
            if let Some(class_list) = categories.get_mut(&category) {
                class_list.push(class.name.clone());
            }
        }
        
        CategorizedClasses { categories }
    }
    
    /// Determine the category of a class based on its properties and parent
    fn determine_class_category(&self, class: &ProcessedClass) -> String {
        // Check parent class for common base classes
        if let Some(parent) = &class.parent {
            let parent_lower = parent.to_lowercase();
            
            if parent_lower.contains("vehicle") || parent_lower.contains("car") || 
               parent_lower.contains("tank") || parent_lower.contains("air") {
                return "Vehicle".to_string();
            }
            
            if parent_lower.contains("weapon") || parent_lower.contains("rifle") || 
               parent_lower.contains("launcher") || parent_lower.contains("pistol") {
                return "Weapon".to_string();
            }
            
            if parent_lower.contains("magazine") || parent_lower.contains("ammo") {
                return "Magazine".to_string();
            }
            
            if parent_lower.contains("item") || parent_lower.contains("equipment") {
                return "Item".to_string();
            }
            
            if parent_lower.contains("backpack") {
                return "Backpack".to_string();
            }
        }
        
        // Check properties for hints about the class type
        for (key, value) in &class.properties {
            let key_lower = key.to_lowercase();
            let value_lower = value.to_lowercase();
            
            if key_lower == "vehicleclass" {
                return "Vehicle".to_string();
            }
            
            if key_lower.contains("weapon") || key_lower.contains("rifle") || 
               key_lower.contains("gun") || key_lower.contains("firearm") {
                return "Weapon".to_string();
            }
            
            if key_lower.contains("magazine") || key_lower.contains("ammo") || 
               key_lower.contains("rounds") {
                return "Magazine".to_string();
            }
            
            if key_lower.contains("item") || key_lower.contains("equipment") {
                return "Item".to_string();
            }
            
            if key_lower.contains("backpack") || key_lower.contains("bag") {
                return "Backpack".to_string();
            }
        }
        
        // Default category if no specific category is determined
        "Other".to_string()
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
            .map(|name| self.build_class_node(name, &child_map))
            .collect();
        
        ClassHierarchy { root_classes: root_nodes }
    }
    
    /// Recursively build a class node and its children
    fn build_class_node(&self, class_name: &str, child_map: &HashMap<String, Vec<String>>) -> ClassNode {
        let children = if let Some(child_names) = child_map.get(class_name) {
            child_names.iter()
                .map(|name| self.build_class_node(name, child_map))
                .collect()
        } else {
            Vec::new()
        };
        
        ClassNode {
            name: class_name.to_string(),
            children,
        }
    }
} 