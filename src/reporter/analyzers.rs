use log::info;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

use crate::config::ScanConfig;

use arma3_database::queries::{
    class_repository::ClassRepository, mission_repository::MissionRepository,
};

use crate::reporter::error::Result as ReporterResult;
use crate::reporter::models::DependencyAnalysis;

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
        let game_data_classes: HashSet<String> = self
            .class_repo
            .get_all()?
            .into_iter()
            .map(|c| c.id)
            .collect();
        let total_classes = game_data_classes.len();
        info!("Found {} total game data classes", total_classes);

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
            // Skip if class is in ignore list
            if self.ignored_classes.contains(&dep.class_name) {
                continue;
            }

            if !game_data_classes.contains(&dep.class_name) {
                // Add to mission's missing dependencies
                missing_dependencies
                    .entry(dep.mission_id.clone())
                    .or_default()
                    .insert(dep.class_name);
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
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use arma3_database::{ClassModel, DatabaseManager, MissionDependencyModel, MissionModel};
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn test_dependency_analyzer() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        // Create database and config
        let db = DatabaseManager::new(&db_path).unwrap();
        let class_repo = ClassRepository::new(&db);
        let mission_repo = MissionRepository::new(&db);

        // Create config with ignore file
        let mut config = ScanConfig::default();
        let ignore_file = dir.path().join("ignored_classes.txt");
        std::fs::write(&ignore_file, "MissingClass").unwrap();
        config.ignore_classes_file = Some(ignore_file);

        // Create analyzer with config
        let analyzer = DependencyAnalyzer::with_config(&class_repo, &mission_repo, &config).unwrap();

        // Create test game data classes
        let game_class = ClassModel::new(
            "GameClass".to_string(),
            None::<String>,
            None::<String>,
            Some(1),
            false,
        );
        class_repo.create(&game_class).unwrap();

        // Create test mission with dependencies
        let mission = MissionModel::new(
            "test_mission",
            "Test Mission",
            PathBuf::from("missions/test.pbo"),
            Utc::now(),
        );
        mission_repo.create(&mission).unwrap();

        // Add dependencies
        let dependency = MissionDependencyModel::new(
            "test_mission".to_string(),
            "GameClass".to_string(), // Existing class
            "DirectClass".to_string(),
            PathBuf::from("mission/dependency.sqf"),
        );
        mission_repo.add_dependency(&dependency).unwrap();

        let missing_dependency = MissionDependencyModel::new(
            "test_mission".to_string(),
            "MissingClass".to_string(), // Non-existent class
            "DirectClass".to_string(),
            PathBuf::from("mission/missing.sqf"),
        );
        mission_repo.add_dependency(&missing_dependency).unwrap();

        // Run analysis
        let analysis = analyzer.analyze_dependencies().unwrap();

        // Verify results
        assert_eq!(analysis.total_missions, 1);
        assert_eq!(analysis.total_missing, 1);

        let mission_missing = analysis.missing_dependencies.get("test_mission");
        assert!(mission_missing.is_none() || mission_missing.unwrap().is_empty(), 
            "Expected no missing dependencies since MissingClass is ignored");
    }
}
