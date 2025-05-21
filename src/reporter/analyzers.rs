use std::collections::{HashMap, HashSet};
use log::info;

use arma3_database::queries::{
    class_repository::ClassRepository,
    mission_repository::MissionRepository,
};

use crate::reporter::error::Result as ReporterResult;
use crate::reporter::models::DependencyAnalysis;

/// Analyzes dependencies between missions and game data
pub struct DependencyAnalyzer<'a> {
    class_repo: &'a ClassRepository<'a>,
    mission_repo: &'a MissionRepository<'a>,
}

impl<'a> DependencyAnalyzer<'a> {
    /// Create a new dependency analyzer
    pub fn new(
        class_repo: &'a ClassRepository<'a>,
        mission_repo: &'a MissionRepository<'a>,
    ) -> Self {
        Self {
            class_repo,
            mission_repo,
        }
    }

    /// Analyze dependencies between missions and game data
    pub fn analyze_dependencies(&self) -> ReporterResult<DependencyAnalysis> {
        info!("Starting dependency analysis...");

        // Get all game data classes
        let game_data_classes: HashSet<String> = self.class_repo.get_all()?
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
        info!("Found {} total dependencies across all missions", total_dependencies);

        // Track missing dependencies per mission
        let mut missing_dependencies: HashMap<String, HashSet<String>> = HashMap::new();
        let mut total_missing = 0;

        // Process dependencies in memory
        for dep in all_dependencies {
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
        info!("Found {} total dependencies across all missions", total_dependencies);
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
    use tempfile::tempdir;
    use arma3_database::{DatabaseManager, ClassModel, MissionDependencyModel, MissionModel};
    use chrono::Utc;

    #[test]
    fn test_dependency_analyzer() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let class_repo = ClassRepository::new(&db);
        let mission_repo = MissionRepository::new(&db);
        
        // Create analyzer
        let analyzer = DependencyAnalyzer::new(&class_repo, &mission_repo);
        
        // Create test game data classes
        let game_class = ClassModel::new(
            "GameClass".to_string(),
            None::<String>,
            None::<String>,
            Some(1),
            false
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
            "GameClass".to_string(),  // Existing class
            "DirectClass".to_string(),
            PathBuf::from("mission/dependency.sqf"),
        );
        mission_repo.add_dependency(&dependency).unwrap();
        
        let missing_dependency = MissionDependencyModel::new(
            "test_mission".to_string(),
            "MissingClass".to_string(),  // Non-existent class
            "DirectClass".to_string(),
            PathBuf::from("mission/missing.sqf"),
        );
        mission_repo.add_dependency(&missing_dependency).unwrap();
        
        // Run analysis
        let analysis = analyzer.analyze_dependencies().unwrap();
        
        // Verify results
        assert_eq!(analysis.total_missions, 1);
        assert_eq!(analysis.total_missing, 1);
        
        let mission_missing = analysis.missing_dependencies.get("test_mission").unwrap();
        assert_eq!(mission_missing.len(), 1);
        assert!(mission_missing.contains("MissingClass"));
    }
}