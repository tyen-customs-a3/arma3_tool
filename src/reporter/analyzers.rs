use log::{debug, info};
use rayon::prelude::*; // Added for parallel processing
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use strsim::normalized_levenshtein; // Changed from jaro_winkler

use crate::config::ScanConfig;

use arma3_database::queries::{
    class_repository::ClassRepository, mission_repository::MissionRepository,
};

use crate::reporter::error::Result as ReporterResult;
use crate::reporter::models::{DependencyAnalysis, MissingClassMatch, PotentialMatch};

const FUZZY_SIMILARITY_THRESHOLD: f64 = 0.6; // Minimum similarity to be considered a match (lowered from 0.7)
pub const MAX_FUZZY_MATCHES: usize = 3;         // Max number of potential matches to report - Made public

/// Analyzes dependencies between missions and game data
pub struct DependencyAnalyzer<'a> {
    class_repo: &'a ClassRepository<'a>,
    mission_repo: &'a MissionRepository<'a>,
    ignored_classes: HashSet<String>,
}

impl<'a> DependencyAnalyzer<'a> {
    /// Create a new dependency analyzer with configuration
    pub fn with_config(
        class_repo: &'a ClassRepository<'a>,
        mission_repo: &'a MissionRepository<'a>,
        config: &ScanConfig,
    ) -> ReporterResult<Self> {
        let mut analyzer = Self {
            class_repo,
            mission_repo,
            ignored_classes: HashSet::new(),
        };

        // Load ignored classes if configured
        if let Some(ignore_file) = &config.ignore_classes_file {
            analyzer.load_ignored_classes(ignore_file)?;
        }

        Ok(analyzer)
    }

    /// Create a new dependency analyzer without config
    pub fn new(
        class_repo: &'a ClassRepository<'a>,
        mission_repo: &'a MissionRepository<'a>,
    ) -> Self {
        Self {
            class_repo,
            mission_repo,
            ignored_classes: HashSet::new(),
        }
    }

    /// Load ignored classes from a file
    fn load_ignored_classes(&mut self, path: &Path) -> ReporterResult<()> {
        info!("Loading ignored classes from {}", path.display());
        
        let file = File::open(path).map_err(|e| crate::reporter::error::ReporterError::Io(e))?;
        let reader = io::BufReader::new(file);
        let mut count = 0;

        for line in reader.lines() {
            let line = line.map_err(|e| crate::reporter::error::ReporterError::Io(e))?;
            let trimmed = line.trim();
            
            // Skip empty lines and comments
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                self.ignored_classes.insert(trimmed.to_string());
                count += 1;
            }
        }

        info!("Loaded {} ignored classes", count);
        Ok(())
    }

    /// Analyze dependencies between missions and game data
    pub fn analyze_dependencies(&self) -> ReporterResult<DependencyAnalysis> {
        info!("Starting dependency analysis...");

        // Get all game data classes
        let all_game_classes_from_db = self.class_repo.get_all()?;
        let total_classes = all_game_classes_from_db.len(); // Count of unique case-sensitive names

        let game_data_classes_lower: HashSet<String> = all_game_classes_from_db
            .into_iter()
            .map(|c| c.id.to_lowercase()) // Store lowercase for case-insensitive lookup
            .collect();
        info!(
            "Found {} total game data classes (unique case-sensitive names). Using {} unique case-insensitive names for lookup.",
            total_classes,
            game_data_classes_lower.len()
        );

        // Get all missions
        let missions = self.mission_repo.get_all()?;
        let total_missions = missions.len();
        info!("Found {} total missions", total_missions);

        // Get all dependencies in bulk
        let all_dependencies = self.mission_repo.get_all_dependencies()?;
        let total_dependencies = all_dependencies.len();
        info!(
            "Found {} total dependencies across all missions",
            total_dependencies
        );

        // Track missing dependencies per mission
        let mut missing_dependencies: HashMap<String, HashSet<String>> = HashMap::new();
        let mut total_missing = 0;

        // Process dependencies in memory
        for dep in all_dependencies {
            // Perform case-insensitive check for existence
            if !game_data_classes_lower.contains(&dep.class_name.to_lowercase()) {
                // Skip if class is in ignore list before adding to missing
                if self.ignored_classes.contains(&dep.class_name) {
                    continue;
                }
                // Add to mission's missing dependencies (store original casing from mission)
                missing_dependencies
                    .entry(dep.mission_id.clone())
                    .or_default()
                    .insert(dep.class_name.clone());
                total_missing += 1;
            }
        }

        info!("Dependency analysis complete");
        info!("Analyzed {} missions", total_missions);
        info!(
            "Found {} total dependencies across all missions",
            total_dependencies
        );
        info!("Found {} total missing dependencies", total_missing);

        Ok(DependencyAnalysis {
            missing_dependencies,
            total_missions,
            total_missing,
            total_classes,
            total_dependencies,
        })
    }

    /// Analyze missing classes and find potential fuzzy matches
    pub fn analyze_fuzzy_missing_classes(&self) -> ReporterResult<Vec<MissingClassMatch>> {
        info!("Starting fuzzy missing class analysis...");

        // Perform standard dependency analysis to get all missing dependencies
        let dependency_analysis = self.analyze_dependencies()?;
        
        // Collect all unique missing class names, excluding ignored ones
        let mut unique_missing_classes: HashSet<String> = HashSet::new();
        for mission_missing_deps in dependency_analysis.missing_dependencies.values() {
            for missing_class in mission_missing_deps {
                if !self.ignored_classes.contains(missing_class) {
                    unique_missing_classes.insert(missing_class.clone());
                }
            }
        }
        info!("Found {} unique missing classes (after ignoring) for fuzzy matching", unique_missing_classes.len());

        // Get all game data classes for comparison
        let game_data_classes: Vec<String> = self
            .class_repo
            .get_all()?
            .into_iter()
            .map(|c| c.id)
            .collect();
        info!("Comparing against {} game data classes", game_data_classes.len());

        // Parallel processing of unique missing classes
        let mut results: Vec<MissingClassMatch> = unique_missing_classes
            .into_par_iter() // Convert to parallel iterator
            .map(|missing_class_name| {
                let mut potential_matches: Vec<PotentialMatch> = Vec::new();
                // It's often better to clone game_data_classes or pass a reference if it's large
                // and accessed many times inside the parallel map.
                // For this case, iterating over it for each missing_class_name is fine.
                for known_class_name in &game_data_classes {
                    let similarity = normalized_levenshtein(
                        &missing_class_name.to_lowercase(), 
                        &known_class_name.to_lowercase()
                    ); // Compare lowercase versions
                    if similarity >= FUZZY_SIMILARITY_THRESHOLD {
                        potential_matches.push(PotentialMatch {
                            class_name: known_class_name.clone(), // Store original casing
                            similarity,
                        });
                    }
                }

                // Sort by similarity (descending) and take top N
                potential_matches.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
                potential_matches.truncate(MAX_FUZZY_MATCHES);

                if !potential_matches.is_empty() {
                    debug!("Found {} potential matches for {}", potential_matches.len(), missing_class_name);
                }
                
                MissingClassMatch {
                    missing_class_name,
                    potential_matches,
                }
            })
            .collect(); // Collect results from parallel processing
        
        // Sort final results by missing class name for consistent output
        results.sort_by(|a, b| a.missing_class_name.cmp(&b.missing_class_name));

        info!("Fuzzy missing class analysis complete. Found suggestions for {} classes.", results.iter().filter(|m| !m.potential_matches.is_empty()).count());
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use arma3_database::{ClassModel, DatabaseManager, MissionDependencyModel, MissionModel};
    use chrono::Utc;
    use tempfile::tempdir;

    fn setup_db_and_config(dir_path: &Path) -> (DatabaseManager, ScanConfig) {
        let db_path = dir_path.join("test.db");
        let db = DatabaseManager::new(&db_path).unwrap();
        
        let mut config = ScanConfig::default();
        let ignore_file_path = dir_path.join("ignored_classes.txt");
        std::fs::write(&ignore_file_path, "IgnoredClass\n#Comment\nAnotherIgnoredClass").unwrap();
        config.ignore_classes_file = Some(ignore_file_path);
        
        (db, config)
    }

    #[test]
    fn test_dependency_analyzer() {
        let dir = tempdir().unwrap();
        let (db, config) = setup_db_and_config(dir.path());
        
        let class_repo = ClassRepository::new(&db);
        let mission_repo = MissionRepository::new(&db);
        let analyzer = DependencyAnalyzer::with_config(&class_repo, &mission_repo, &config).unwrap();

        class_repo.create(&ClassModel::new("GameClass".to_string(), None::<String>, None::<String>, Some(1), false)).unwrap();
        
        let mission = MissionModel::new("test_mission".to_string(), "Test Mission".to_string(), PathBuf::from("missions/test.pbo"), Utc::now());
        mission_repo.create(&mission).unwrap();
        
        mission_repo.add_dependency(&MissionDependencyModel::new("test_mission".to_string(), "GameClass".to_string(), "DirectClass".to_string(), PathBuf::from("mission/dependency.sqf"))).unwrap();
        mission_repo.add_dependency(&MissionDependencyModel::new("test_mission".to_string(), "MissingClassShouldBeReported".to_string(), "DirectClass".to_string(), PathBuf::from("mission/missing.sqf"))).unwrap();
        mission_repo.add_dependency(&MissionDependencyModel::new("test_mission".to_string(), "IgnoredClass".to_string(), "DirectClass".to_string(), PathBuf::from("mission/ignored.sqf"))).unwrap();

        let analysis = analyzer.analyze_dependencies().unwrap();

        assert_eq!(analysis.total_missions, 1);
        // total_missing now excludes ignored classes from the count
        assert_eq!(analysis.total_missing, 1); 
        
        let mission_missing = analysis.missing_dependencies.get("test_mission").unwrap();
        assert_eq!(mission_missing.len(), 1, "Expected only MissingClassShouldBeReported in the list after filtering");
        assert!(mission_missing.contains("MissingClassShouldBeReported"));
        assert!(!mission_missing.contains("IgnoredClass"));
    }

    #[test]
    fn test_fuzzy_missing_class_analyzer() {
        let dir = tempdir().unwrap();
        let (db, config) = setup_db_and_config(dir.path());

        let class_repo = ClassRepository::new(&db);
        let mission_repo = MissionRepository::new(&db);
        let analyzer = DependencyAnalyzer::with_config(&class_repo, &mission_repo, &config).unwrap();

        // Known classes
        class_repo.create(&ClassModel::new("MyExactClass".to_string(), None::<String>, None::<String>, Some(1), false)).unwrap();
        class_repo.create(&ClassModel::new("MyExactClasS".to_string(), None::<String>, None::<String>, Some(2), false)).unwrap(); // Case difference
        class_repo.create(&ClassModel::new("AnotherClass".to_string(), None::<String>, None::<String>, Some(3), false)).unwrap();
        class_repo.create(&ClassModel::new("MyVaguelySimilarClass".to_string(), None::<String>, None::<String>, Some(4), false)).unwrap();
        class_repo.create(&ClassModel::new("CompletelyDifferent".to_string(), None::<String>, None::<String>, Some(5), false)).unwrap();

        let mission = MissionModel::new("fuzzy_mission".to_string(), "Fuzzy Test Mission".to_string(), PathBuf::from("missions/fuzzy.pbo"), Utc::now());
        mission_repo.create(&mission).unwrap();

        // Dependencies that will be missing
        mission_repo.add_dependency(&MissionDependencyModel::new("fuzzy_mission".to_string(), "MyExactClas".to_string(), "Source1".to_string(), PathBuf::from("file1.sqf"))).unwrap(); // Should find MyExactClass and MyExactClasS
        mission_repo.add_dependency(&MissionDependencyModel::new("fuzzy_mission".to_string(), "NonExistent".to_string(), "Source2".to_string(), PathBuf::from("file2.sqf"))).unwrap(); // Should find no matches
        mission_repo.add_dependency(&MissionDependencyModel::new("fuzzy_mission".to_string(), "IgnoredClass".to_string(), "Source3".to_string(), PathBuf::from("file3.sqf"))).unwrap(); // Should be ignored

        let fuzzy_results = analyzer.analyze_fuzzy_missing_classes().unwrap();
        
        assert_eq!(fuzzy_results.len(), 2, "Expected 2 unique missing classes after filtering ignored ones");

        let my_exact_clas_match = fuzzy_results.iter().find(|m| m.missing_class_name == "MyExactClas").unwrap();
        assert_eq!(my_exact_clas_match.potential_matches.len(), 2);
        assert!(my_exact_clas_match.potential_matches.iter().any(|p| p.class_name == "MyExactClass" && p.similarity > 0.9));
        assert!(my_exact_clas_match.potential_matches.iter().any(|p| p.class_name == "MyExactClasS" && p.similarity > 0.9));

        let non_existent_match = fuzzy_results.iter().find(|m| m.missing_class_name == "NonExistent").unwrap();
        assert_eq!(non_existent_match.potential_matches.len(), 0);

        // Ensure IgnoredClass is not in the results
        assert!(fuzzy_results.iter().find(|m| m.missing_class_name == "IgnoredClass").is_none());
    }
     #[test]
    fn test_fuzzy_max_matches() {
        let dir = tempdir().unwrap();
        let (db, config) = setup_db_and_config(dir.path());
        let class_repo = ClassRepository::new(&db);
        let mission_repo = MissionRepository::new(&db);
        let analyzer = DependencyAnalyzer::with_config(&class_repo, &mission_repo, &config).unwrap();

        // Create more than MAX_FUZZY_MATCHES similar classes
        class_repo.create(&ClassModel::new("TestClass1".to_string(), None::<String>, None::<String>, Some(1), false)).unwrap();
        class_repo.create(&ClassModel::new("TestClass2".to_string(), None::<String>, None::<String>, Some(2), false)).unwrap();
        class_repo.create(&ClassModel::new("TestClass3".to_string(), None::<String>, None::<String>, Some(3), false)).unwrap();
        class_repo.create(&ClassModel::new("TestClass4".to_string(), None::<String>, None::<String>, Some(4), false)).unwrap();
        class_repo.create(&ClassModel::new("TestClass5".to_string(), None::<String>, None::<String>, Some(5), false)).unwrap();

        let mission = MissionModel::new("max_match_mission".to_string(), "Max Match Test".to_string(), PathBuf::from("missions/max.pbo"), Utc::now());
        mission_repo.create(&mission).unwrap();
        mission_repo.add_dependency(&MissionDependencyModel::new("max_match_mission".to_string(), "TestClas".to_string(), "Source".to_string(), PathBuf::from("file.sqf"))).unwrap();

        let fuzzy_results = analyzer.analyze_fuzzy_missing_classes().unwrap();
        let test_clas_match = fuzzy_results.iter().find(|m| m.missing_class_name == "TestClas").unwrap();
        
        assert_eq!(test_clas_match.potential_matches.len(), MAX_FUZZY_MATCHES);
        // Verify they are the most similar ones (Jaro-Winkler is sensitive to prefix)
        // All TestClassN should be very similar to TestClas
        for i in 1..=MAX_FUZZY_MATCHES {
            let expected_class = format!("TestClass{}", i);
            assert!(test_clas_match.potential_matches.iter().any(|p| p.class_name == expected_class));
        }
    }
}
