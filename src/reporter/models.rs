use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};

/// Result of dependency analysis
#[derive(Debug)]
pub struct DependencyAnalysis {
    /// Map of mission ID to set of missing class dependencies
    pub missing_dependencies: HashMap<String, HashSet<String>>,
    /// Total number of missions analyzed
    pub total_missions: usize,
    /// Total number of missing dependencies found
    pub total_missing: usize,
    /// Total number of game data classes
    pub total_classes: usize,
    /// Total number of dependencies across all missions
    pub total_dependencies: usize,
}

/// Model for a dependency analysis report
#[derive(Debug)]
pub struct DependencyReport {
    /// Map of mission ID to set of missing class dependencies
    pub missing_dependencies: HashMap<String, HashSet<String>>,
    /// Total number of missions analyzed
    pub total_missions: usize,
    /// Total number of missing dependencies found
    pub total_missing: usize,
    /// Total number of game data classes
    pub total_classes: usize,
    /// Total number of dependencies across all missions
    pub total_dependencies: usize,
    /// When the report was generated
    pub generated_at: DateTime<Utc>,
}

impl DependencyReport {
    /// Create a new dependency report
    pub fn new(
        missing_dependencies: HashMap<String, HashSet<String>>,
        total_missions: usize,
        total_missing: usize,
        total_classes: usize,
        total_dependencies: usize,
    ) -> Self {
        Self {
            missing_dependencies,
            total_missions,
            total_missing,
            total_classes,
            total_dependencies,
            generated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_report() {
        let mut missing_deps = HashMap::new();
        let mut mission_deps = HashSet::new();
        mission_deps.insert("MissingClass".to_string());
        missing_deps.insert("test_mission".to_string(), mission_deps);

        let report = DependencyReport::new(missing_deps, 1, 1, 100, 5);

        assert_eq!(report.total_missions, 1);
        assert_eq!(report.total_missing, 1);
        assert_eq!(report.total_classes, 100);
        assert_eq!(report.total_dependencies, 5);
        assert_eq!(report.missing_dependencies.len(), 1);
        assert!(report.missing_dependencies["test_mission"].contains("MissingClass"));
    }
}
