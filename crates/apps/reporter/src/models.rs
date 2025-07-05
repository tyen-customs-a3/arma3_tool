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

/// Represents a missing class and its potential fuzzy matches
#[derive(Debug, Clone, PartialEq)]
pub struct MissingClassMatch {
    pub missing_class_name: String,
    pub potential_matches: Vec<PotentialMatch>,
}

/// Represents a potential fuzzy match for a missing class
#[derive(Debug, Clone, PartialEq)]
pub struct PotentialMatch {
    pub class_name: String,
    pub similarity: f64, // Jaro-Winkler similarity score (0.0 to 1.0)
}

/// Model for a report detailing missing classes and their potential fuzzy matches
#[derive(Debug)]
pub struct FuzzyMissingClassReport {
    /// List of missing classes with their potential matches
    pub missing_class_matches: Vec<MissingClassMatch>,
    /// Total number of unique missing classes found
    pub total_unique_missing: usize,
    /// When the report was generated
    pub generated_at: DateTime<Utc>,
}

impl FuzzyMissingClassReport {
    /// Create a new fuzzy missing class report
    pub fn new(
        missing_class_matches: Vec<MissingClassMatch>,
    ) -> Self {
        let total_unique_missing = missing_class_matches.len();
        Self {
            missing_class_matches,
            total_unique_missing,
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

    #[test]
    fn test_fuzzy_missing_class_report() {
        let matches = vec![
            MissingClassMatch {
                missing_class_name: "MyClas".to_string(),
                potential_matches: vec![
                    PotentialMatch { class_name: "MyClass".to_string(), similarity: 0.9 },
                    PotentialMatch { class_name: "MyClassEx".to_string(), similarity: 0.8 },
                ]
            }
        ];
        let report = FuzzyMissingClassReport::new(matches.clone());
        assert_eq!(report.total_unique_missing, 1);
        assert_eq!(report.missing_class_matches.len(), 1);
        assert_eq!(report.missing_class_matches[0].missing_class_name, "MyClas");
        assert_eq!(report.missing_class_matches[0].potential_matches.len(), 2);
    }
}
