use std::path::PathBuf;
use log::info;

use arma3_database::{
    DatabaseManager,
    queries::class_repository::ClassRepository,
    queries::mission_repository::MissionRepository,
};

use crate::reporter::error::Result as ReporterResult;
use crate::reporter::models::DependencyReport;
use crate::reporter::analyzers::DependencyAnalyzer;
use crate::reporter::writers::ReportWriter;
use crate::reporter::class_graph::ClassHierarchyWriter;

/// Coordinates the dependency analysis and reporting process
pub struct ReportCoordinator<'a> {
    db: &'a DatabaseManager,
    class_repo: ClassRepository<'a>,
    mission_repo: MissionRepository<'a>,
}

impl<'a> ReportCoordinator<'a> {
    /// Create a new report coordinator
    pub fn new(db: &'a DatabaseManager) -> Self {
        Self {
            db,
            class_repo: ClassRepository::new(db),
            mission_repo: MissionRepository::new(db),
        }
    }

    /// Run the complete reporting process
    pub fn run_report(&self, output_dir: &PathBuf) -> ReporterResult<()> {
        info!("Starting dependency analysis and reporting process...");

        // Create analyzer
        let analyzer = DependencyAnalyzer::new(
            &self.class_repo,
            &self.mission_repo,
        );

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use arma3_database::{DatabaseManager, ClassModel, MissionDependencyModel, MissionModel};
    use chrono::Utc;

    #[test]
    fn test_report_coordinator() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let coordinator = ReportCoordinator::new(&db);
        
        // Create test game data classes
        let class_repo = coordinator.class_repo();
        let game_class = ClassModel::new(
            "GameClass".to_string(),
            None::<String>,
            None::<String>,
            Some(1),
        );
        class_repo.create(&game_class).unwrap();
        
        // Create test mission with dependencies
        let mission_repo = coordinator.mission_repo();
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
        
        // Run report
        let output_dir = dir.path().join("reports");
        coordinator.run_report(&output_dir).unwrap();
        
        // Verify report file was created
        let report_files: Vec<_> = std::fs::read_dir(&output_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("dependency_report_"))
            .collect();
        assert_eq!(report_files.len(), 1);
    }
    
    #[test]
    fn test_class_graph_generation() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        
        // Create database
        let db = DatabaseManager::new(&db_path).unwrap();
        let coordinator = ReportCoordinator::new(&db);
        
        // Create test classes with parent-child relationships
        let class_repo = coordinator.class_repo();
        
        // Parent class
        let parent = ClassModel::new(
            "ParentClass".to_string(),
            None::<String>,
            None::<String>,
            Some(1),
        );
        class_repo.create(&parent).unwrap();
        
        // Child classes
        let child1 = ClassModel::new(
            "ChildClass1".to_string(),
            Some("ParentClass".to_string()),
            None::<String>,
            Some(2),
        );
        class_repo.create(&child1).unwrap();
        
        // Run class graph generation
        let output_dir = dir.path().join("reports");
        coordinator.generate_class_graph(&output_dir).unwrap();
        
        // Verify graph file was created
        let graph_file = output_dir.join("class_hierarchy.csv");
        assert!(graph_file.exists());
    }
}