use log::info;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use arma3_database::{
    queries::class_repository::ClassRepository, queries::mission_repository::MissionRepository,
    DatabaseManager,
};

use crate::{
    config::ScanConfig,
    reporter::{
        analyzers::DependencyAnalyzer,
        class_graph::ClassHierarchyWriter,
        error::Result as ReporterResult,
        models::{DependencyReport, FuzzyMissingClassReport},
        writers::ReportWriter,
    },
};

/// Coordinates the dependency analysis and reporting process
pub struct ReportCoordinator<'a> {
    db: &'a DatabaseManager,
    class_repo: ClassRepository<'a>,
    mission_repo: MissionRepository<'a>,
    config: &'a ScanConfig,
}

impl<'a> ReportCoordinator<'a> {
    /// Create a new report coordinator
    pub fn new(db: &'a DatabaseManager, config: &'a ScanConfig) -> Self {
        Self {
            db,
            class_repo: ClassRepository::new(db),
            mission_repo: MissionRepository::new(db),
            config,
        }
    }

    /// Run the complete reporting process
    pub fn run_report(&self, output_dir: &PathBuf) -> ReporterResult<()> {
        info!("Starting dependency analysis and reporting process...");

        // Create analyzer with config
        let analyzer = DependencyAnalyzer::with_config(&self.class_repo, &self.mission_repo, self.config)?;

        // Run analysis
        let analysis = analyzer.analyze_dependencies()?;

        // Create report
        let report = DependencyReport::new(
            analysis.missing_dependencies.clone(),
            analysis.total_missions,
            analysis.total_missing,
            analysis.total_classes,
            analysis.total_dependencies,
        );

        // Create writer
        let writer = ReportWriter::new(output_dir);

        // Write report
        writer.write_report(&report)?;

        info!("Report generation complete");
        Ok(())
    }

    /// Generate a CSV file showing class hierarchy relationships
    pub fn generate_class_graph(&self, output_dir: &PathBuf) -> ReporterResult<()> {
        info!("Starting class hierarchy graph generation...");

        // Create graph writer 
        let writer = ClassHierarchyWriter::new(output_dir);

        // Generate graph
        writer.write_class_graph(&self.class_repo)?;

        info!("Class hierarchy graph generation complete");
        Ok(())
    }

    /// Generate a CSV file listing classes used in missions and their source files
    pub fn generate_mission_class_source_report(&self, output_dir: &PathBuf) -> ReporterResult<()> {
        info!("Starting mission class source report generation...");

        let missions = self.mission_repo.get_all()?;
        info!("Processing {} missions...", missions.len());

        let mut results: Vec<(String, String, Option<String>)> = Vec::new();
        let mut class_source_cache: HashMap<String, Option<String>> = HashMap::new();
        let mut processed_classes_for_mission: HashSet<(String, String)> = HashSet::new();

        for mission in missions {
            let dependencies = self.mission_repo.get_dependencies(&mission.id)?;

            for dep in dependencies {
                let mission_class_key = (mission.id.clone(), dep.class_name.clone());
                if processed_classes_for_mission.contains(&mission_class_key) {
                    continue; // Already processed this class for this mission
                }

                let source_path = match class_source_cache.get(&dep.class_name) {
                    Some(cached_path) => cached_path.clone(),
                    None => {
                        let path = match self.class_repo.get(&dep.class_name)? {
                            Some(class_model) => {
                                if let Some(idx) = class_model.source_file_index {
                                    self.class_repo.get_source_path(idx)?
                                } else {
                                    None
                                }
                            }
                            None => None,
                        };
                        class_source_cache.insert(dep.class_name.clone(), path.clone());
                        path
                    }
                };

                results.push((mission.id.clone(), dep.class_name.clone(), source_path));
                processed_classes_for_mission.insert(mission_class_key);
            }
        }

        // Write to CSV
        let report_path = output_dir.join("mission_class_sources.csv");
        let file =
            File::create(&report_path).map_err(|e| crate::reporter::error::ReporterError::Io(e))?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "mission_id,class_name,source_path")
            .map_err(|e| crate::reporter::error::ReporterError::Io(e))?;

        for (mission_id, class_name, source_path) in results {
            // Basic CSV escaping: wrap fields containing commas or quotes in double quotes, double internal quotes.
            // For simplicity, assuming mission_id and class_name won't have problematic characters.
            // Source path might, but often won't.
            let source_path_str = source_path.unwrap_or_else(|| "".to_string());
            writeln!(writer, "{},{},{}", mission_id, class_name, source_path_str)
                .map_err(|e| crate::reporter::error::ReporterError::Io(e))?;
        }

        writer
            .flush()
            .map_err(|e| crate::reporter::error::ReporterError::Io(e))?;

        info!(
            "Mission class source report generated at: {}",
            report_path.display()
        );
        Ok(())
    }

    /// Get access to the database manager
    pub fn db(&self) -> &'a DatabaseManager {
        self.db
    }

    /// Get access to the class repository
    pub fn class_repo(&self) -> &ClassRepository<'a> {
        &self.class_repo
    }

    /// Get access to the mission repository
    pub fn mission_repo(&self) -> &MissionRepository<'a> {
        &self.mission_repo
    }

    /// Generate a report for missing classes with fuzzy match suggestions
    pub fn generate_fuzzy_missing_class_report(&self, output_dir: &PathBuf) -> ReporterResult<()> {
        info!("Starting fuzzy missing class report generation...");

        // Create analyzer with config
        let analyzer = DependencyAnalyzer::with_config(&self.class_repo, &self.mission_repo, self.config)?;

        // Run fuzzy analysis
        let fuzzy_matches = analyzer.analyze_fuzzy_missing_classes()?;

        // Create report model
        let report = FuzzyMissingClassReport::new(fuzzy_matches);

        // Create writer
        let writer = ReportWriter::new(output_dir);

        // Write report
        writer.write_fuzzy_missing_class_report(&report)?;

        info!("Fuzzy missing class report generation complete.");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arma3_database::{ClassModel, DatabaseManager, MissionDependencyModel, MissionModel};
    use chrono::Utc;
    use tempfile::tempdir;
    use std::fs;

    fn create_basic_config_and_db(dir_path: &std::path::Path) -> (DatabaseManager, ScanConfig) {
        let db_path = dir_path.join("test.db");
        let db = DatabaseManager::new(&db_path).unwrap();
        let config = ScanConfig::default();
        (db, config)
    }

    #[test]
    fn test_report_coordinator() {
        let dir = tempdir().unwrap();
        let (db, config) = create_basic_config_and_db(dir.path());
        let coordinator = ReportCoordinator::new(&db, &config);

        let class_repo = coordinator.class_repo();
        class_repo.create(&ClassModel::new("GameClass".to_string(), None::<String>, None::<String>, Some(1),false)).unwrap();

        let mission_repo = coordinator.mission_repo();
        let mission = MissionModel::new("test_mission".to_string(), "Test Mission".to_string(), PathBuf::from("missions/test.pbo"), Utc::now());
        mission_repo.create(&mission).unwrap();
        mission_repo.add_dependency(&MissionDependencyModel::new("test_mission".to_string(), "GameClass".to_string(), "Direct".to_string(), PathBuf::from("file.sqf"))).unwrap();
        mission_repo.add_dependency(&MissionDependencyModel::new("test_mission".to_string(), "MissingClass".to_string(), "Direct".to_string(), PathBuf::from("file.sqf"))).unwrap();

        let output_dir = dir.path().join("reports");
        coordinator.run_report(&output_dir).unwrap();

        let report_files: Vec<_> = fs::read_dir(&output_dir).unwrap().filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("dependency_report_"))
            .collect();
        assert_eq!(report_files.len(), 1);
    }

    #[test]
    fn test_class_graph_generation() {
        let dir = tempdir().unwrap();
        let (db, config) = create_basic_config_and_db(dir.path());
        let coordinator = ReportCoordinator::new(&db, &config);
        let class_repo = coordinator.class_repo();
        class_repo.create(&ClassModel::new("ParentClass".to_string(), None::<String>, None::<String>, Some(1),false)).unwrap();
        class_repo.create(&ClassModel::new("ChildClass1".to_string(),Some("ParentClass".to_string()), None::<String>, Some(2),false)).unwrap();

        let output_dir = dir.path().join("reports");
        coordinator.generate_class_graph(&output_dir).unwrap();
        assert!(output_dir.join("class_hierarchy.csv").exists());
    }

    #[test]
    fn test_fuzzy_missing_class_report_generation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_fuzzy.db");
        let output_dir = dir.path().join("reports_fuzzy");

        let db = DatabaseManager::new(&db_path).unwrap();
        
        // Config with an ignore file
        let mut config = ScanConfig::default();
        let ignore_file_path = dir.path().join("ignored_fuzzy.txt");
        fs::write(&ignore_file_path, "IgnoredFuzzyClass\n").unwrap();
        config.ignore_classes_file = Some(ignore_file_path);

        let coordinator = ReportCoordinator::new(&db, &config);
        let class_repo = coordinator.class_repo();
        let mission_repo = coordinator.mission_repo();

        // Known classes
        class_repo.create(&ClassModel::new("ActualClass".to_string(), None::<String>, None::<String>, Some(1), false)).unwrap();
        class_repo.create(&ClassModel::new("ActualClasS".to_string(), None::<String>, None::<String>, Some(2), false)).unwrap(); // Similar
        class_repo.create(&ClassModel::new("AnotherActual".to_string(), None::<String>, None::<String>, Some(3), false)).unwrap();

        // Mission with dependencies
        let mission = MissionModel::new("fuzzy_rep_mission".to_string(), "Fuzzy Report Test".to_string(), PathBuf::from("missions/fuzzy_rep.pbo"), Utc::now());
        mission_repo.create(&mission).unwrap();
        mission_repo.add_dependency(&MissionDependencyModel::new("fuzzy_rep_mission".to_string(), "ActualClas".to_string(), "Src".to_string(), PathBuf::from("f1.sqf"))).unwrap(); // Missing, should find ActualClass & ActualClasS
        mission_repo.add_dependency(&MissionDependencyModel::new("fuzzy_rep_mission".to_string(), "NonExistent".to_string(), "Src".to_string(), PathBuf::from("f2.sqf"))).unwrap(); // Missing, no good matches
        mission_repo.add_dependency(&MissionDependencyModel::new("fuzzy_rep_mission".to_string(), "IgnoredFuzzyClass".to_string(), "Src".to_string(), PathBuf::from("f3.sqf"))).unwrap(); // Should be ignored

        coordinator.generate_fuzzy_missing_class_report(&output_dir).unwrap();

        let report_files: Vec<_> = fs::read_dir(&output_dir).unwrap().filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("fuzzy_missing_class_report_"))
            .collect();
        assert_eq!(report_files.len(), 1, "Fuzzy report file should be created");

        let content = fs::read_to_string(report_files[0].path()).unwrap();
        assert!(content.contains("Missing: ActualClas"));
        assert!(content.contains("- ActualClass (Similarity:")); // Check for part of the line
        assert!(content.contains("- ActualClasS (Similarity:"));
        assert!(content.contains("Missing: NonExistent"));
        assert!(content.contains("No potential matches found."));
        assert!(!content.contains("IgnoredFuzzyClass"), "Ignored class should not appear in the fuzzy report's missing list");
    }
}
