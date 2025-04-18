use std::collections::{HashMap, HashSet};
use std::path::Path;
use log::info;
use csv::Reader;
use serde::Deserialize;
#[cfg(feature = "class_mapping")]
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Event, EventKind, Config};
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
    existing_classes: HashSet<String>,
}

impl ClassMappingAnalysis {
    pub fn new(db: DatabaseManager) -> Self {
        let class_repo = ClassRepository::new(&db);
        // Pre-load all existing classes for case-insensitive lookup
        let existing_classes = class_repo.get_all()
            .unwrap_or_default()
            .into_iter()
            .map(|c| c.id.to_lowercase())
            .collect();

        Self {
            db,
            mappings: HashMap::new(),
            output_file: None,
            existing_classes,
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
            self.mappings.insert(mapping.original_class.to_lowercase(), mapping);
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

    /// Check if a class exists (case-insensitive)
    fn class_exists(&self, class_name: &str) -> bool {
        self.existing_classes.contains(&class_name.to_lowercase())
    }

    /// Check if a class has a mapping (case-insensitive)
    fn has_mapping(&self, class_name: &str) -> bool {
        self.mappings.contains_key(&class_name.to_lowercase())
    }

    /// Analyze missions for missing classes and generate a report
    pub fn analyze_missions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mission_repo = MissionRepository::new(&self.db);

        // Get all missions
        let missions = mission_repo.get_all()?;
        let total_missions = missions.len();
        info!("Analyzing {} missions", total_missions);

        // Track missing classes across all missions
        let mut all_missing_classes = HashSet::new();
        let mut unmapped_classes = HashSet::new();
        let mut mission_unmapped_classes = HashMap::new();

        for mission in missions {
            // Get all dependencies for this mission
            let dependencies = mission_repo.get_dependencies(&mission.id)?;
            
            // Track unmapped classes for this mission
            let mut mission_missing = Vec::new();

            for dep in dependencies {
                // Skip ignored classes

                // Check if the class exists in the database (case-insensitive)
                if !self.class_exists(&dep.class_name) {
                    // Only add to all_missing_classes if it's not already there
                    if all_missing_classes.insert(dep.class_name.clone()) {
                        // Only track classes that don't have a mapping
                        if !self.has_mapping(&dep.class_name) {
                            unmapped_classes.insert(dep.class_name.clone());
                            mission_missing.push(dep.class_name.clone());
                        }
                    }
                }
            }

            if !mission_missing.is_empty() {
                mission_unmapped_classes.insert(mission.name.clone(), mission_missing);
            }
        }

        // Generate report content
        let mut report = String::new();
        report.push_str("\n# Missing Class Dependencies Analysis Report\n\n");
        report.push_str("## Summary\n");
        report.push_str(&format!("- Total Missions: {}\n", total_missions));
        report.push_str(&format!("- Missions with Unmapped Classes: {}\n", mission_unmapped_classes.len()));
        report.push_str(&format!("- Total Unique Missing Classes: {}\n", all_missing_classes.len()));
        report.push_str(&format!("- Unique Unmapped Classes: {}\n", unmapped_classes.len()));
        report.push_str(&format!("- Unique Mapped Classes: {}\n", all_missing_classes.len() - unmapped_classes.len()));

        // Only show missions with unmapped classes
        if !mission_unmapped_classes.is_empty() {
            report.push_str("## Unmapped Classes by Mission\n");
            for (mission_name, classes) in &mission_unmapped_classes {
                report.push_str(&format!("\n### {}\n", mission_name));
                for class in classes {
                    report.push_str(&format!("{}\n", class));
                }
            }
        }

        // List all unmapped classes alphabetically
        report.push_str("\n## All Unmapped Classes\n");
        let mut sorted_unmapped = unmapped_classes.into_iter().collect::<Vec<_>>();
        sorted_unmapped.sort();
        for class in sorted_unmapped {
            report.push_str(&format!("{}\n", class));
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
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default().with_poll_interval(Duration::from_secs(2)))?;

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
                Ok(Ok(Event { kind: EventKind::Modify(_), .. })) => {
                    println!("\nFile changed, reloading mappings and reanalyzing...");
                    self.load_mappings(csv_path)?;
                    self.analyze_missions()?;
                }
                Ok(Err(e)) => {
                    warn!("Watch error: {:?}", e);
                    break;
                }
                Err(e) => {
                    warn!("Channel error: {:?}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
} 