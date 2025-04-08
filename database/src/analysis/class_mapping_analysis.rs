use std::collections::{HashMap, HashSet};
use std::path::Path;
use log::{info, warn, debug};
use csv::Reader;
use serde::Deserialize;
#[cfg(feature = "class_mapping")]
use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs::File;
use std::io::Write;

use crate::DatabaseManager;
use crate::queries::class_repository::ClassRepository;
use crate::queries::mission_repository::MissionRepository;

#[derive(Debug, Deserialize)]
struct ClassMapping {
    original_class: String,
    replacement_class: String,
    notes: String,
}

pub struct ClassMappingAnalysis {
    db: DatabaseManager,
    mappings: HashMap<String, ClassMapping>,
    output_file: Option<String>,
}

impl ClassMappingAnalysis {
    pub fn new(db: DatabaseManager) -> Self {
        Self {
            db,
            mappings: HashMap::new(),
            output_file: None,
        }
    }

    /// Set the output file for analysis results
    pub fn set_output_file(&mut self, output_path: &str) {
        self.output_file = Some(output_path.to_string());
    }

    /// Load class mappings from a CSV file
    pub fn load_mappings(&mut self, csv_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut rdr = Reader::from_path(csv_path)?;
        self.mappings.clear();

        for result in rdr.deserialize() {
            let mapping: ClassMapping = result?;
            self.mappings.insert(mapping.original_class.clone(), mapping);
        }

        info!("Loaded {} class mappings", self.mappings.len());
        Ok(())
    }

    /// Write analysis results to the specified output file or stdout
    fn write_results(&self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(output_path) = &self.output_file {
            let mut file = File::create(output_path)?;
            file.write_all(content.as_bytes())?;
            info!("Analysis results written to {}", output_path);
        } else {
            println!("{}", content);
        }
        Ok(())
    }

    /// Analyze missions for missing classes and generate a report
    pub fn analyze_missions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let class_repo = ClassRepository::new(&self.db);
        let mission_repo = MissionRepository::new(&self.db);

        // Get all missions
        let missions = mission_repo.get_all()?;
        let total_missions = missions.len();
        info!("Analyzing {} missions", total_missions);

        // Track missing classes across all missions
        let mut all_missing_classes = HashSet::new();
        let mut mission_missing_classes = HashMap::new();

        for mission in missions {
            // Get all dependencies for this mission
            let dependencies = mission_repo.get_dependencies(&mission.id)?;
            
            // Track missing classes for this mission
            let mut missing_classes = Vec::new();

            for dep in dependencies {
                // Check if the class exists in the database
                if class_repo.get(&dep.class_name)?.is_none() {
                    missing_classes.push(dep.class_name.clone());
                    all_missing_classes.insert(dep.class_name.clone());
                }
            }

            if !missing_classes.is_empty() {
                mission_missing_classes.insert(mission.name.clone(), missing_classes);
            }
        }

        // Generate report content
        let mut report = String::new();
        report.push_str("\n# Mission Class Dependency Analysis Report\n\n");
        report.push_str("## Summary\n");
        report.push_str(&format!("- Total Missions: {}\n", total_missions));
        report.push_str(&format!("- Missions with Missing Classes: {}\n", mission_missing_classes.len()));
        report.push_str(&format!("- Total Unique Missing Classes: {}\n\n", all_missing_classes.len()));

        report.push_str("## Missing Classes by Mission\n");
        for (mission_name, missing_classes) in mission_missing_classes {
            report.push_str(&format!("\n### {}\n", mission_name));
            for class in missing_classes {
                if let Some(mapping) = self.mappings.get(&class) {
                    report.push_str(&format!("- {} -> {} ({})\n", class, mapping.replacement_class, mapping.notes));
                } else {
                    report.push_str(&format!("- {} (No mapping found)\n", class));
                }
            }
        }

        report.push_str("\n## Unmapped Classes\n");
        for class in all_missing_classes {
            if !self.mappings.contains_key(&class) {
                report.push_str(&format!("- {}\n", class));
            }
        }

        // Write the report to the specified output
        self.write_results(&report)?;

        Ok(())
    }

    /// Watch the CSV file for changes and rerun analysis
    #[cfg(feature = "class_mapping")]
    pub fn watch_and_analyze(&mut self, csv_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Create a channel to receive the events
        let (tx, rx) = channel();

        // Create a watcher object, delivering debounced events
        let mut watcher = notify::watcher(tx, Duration::from_secs(2))?;

        // Add a path to be watched
        watcher.watch(csv_path, RecursiveMode::NonRecursive)?;

        println!("Watching {} for changes...", csv_path.display());
        println!("Press Ctrl+C to exit");

        // Initial analysis
        self.load_mappings(csv_path)?;
        self.analyze_missions()?;

        // Watch for changes
        loop {
            match rx.recv() {
                Ok(Event { kind: EventKind::Modify(_), .. }) => {
                    println!("\nFile changed, reloading mappings and reanalyzing...");
                    self.load_mappings(csv_path)?;
                    self.analyze_missions()?;
                }
                Err(e) => {
                    warn!("Watch error: {:?}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
} 