use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use chrono::{DateTime, Utc};

/// Model representing a mission in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionModel {
    /// Mission identifier (primary key)
    pub id: String,
    
    /// Mission name
    pub name: String,
    
    /// Path to the mission file
    pub path: PathBuf,
    
    /// Reference to the PBO containing this mission
    pub source_pbo_id: Option<String>,
    
    /// Mission author
    pub author: Option<String>,
    
    /// Mission description
    pub description: Option<String>,
    
    /// When it was last scanned
    pub scan_time: DateTime<Utc>,
}

impl MissionModel {
    /// Create a new mission model
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        path: impl Into<PathBuf>,
        scan_time: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            path: path.into(),
            source_pbo_id: None,
            author: None,
            description: None,
            scan_time,
        }
    }
    
    /// Create a new mission model with additional details
    pub fn with_details(
        id: impl Into<String>,
        name: impl Into<String>,
        path: impl Into<PathBuf>,
        source_pbo_id: Option<impl Into<String>>,
        author: Option<impl Into<String>>,
        description: Option<impl Into<String>>,
        scan_time: DateTime<Utc>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            path: path.into(),
            source_pbo_id: source_pbo_id.map(Into::into),
            author: author.map(Into::into),
            description: description.map(Into::into),
            scan_time,
        }
    }
    
    /// Convert from shared_models::Mission
    pub fn from_mission(mission: &arma3_models::Mission) -> Self {
        Self {
            id: mission.name.clone(),
            name: mission.name.clone(),
            path: mission.path.clone(),
            source_pbo_id: mission.source_pbo.clone(),
            author: mission.author.clone(),
            description: mission.description.clone(),
            scan_time: Utc::now(),
        }
    }
}

/// Model representing a mission component in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionComponentModel {
    /// Component ID (primary key)
    pub id: Option<i64>,
    
    /// Mission ID (foreign key)
    pub mission_id: String,
    
    /// Component name
    pub name: String,
    
    /// Component type
    pub component_type: String,
    
    /// Source file within the mission
    pub source_file: PathBuf,
}

impl MissionComponentModel {
    /// Create a new mission component model
    pub fn new(
        mission_id: impl Into<String>,
        name: impl Into<String>,
        component_type: impl Into<String>,
        source_file: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id: None,
            mission_id: mission_id.into(),
            name: name.into(),
            component_type: component_type.into(),
            source_file: source_file.into(),
        }
    }
    
    /// Convert from shared_models::MissionComponent
    pub fn from_component(
        mission_id: &str,
        component: &arma3_models::MissionComponent,
    ) -> Self {
        Self {
            id: None,
            mission_id: mission_id.to_string(),
            name: component.name.clone(),
            component_type: format!("{:?}", component.component_type),
            source_file: component.source_file.clone(),
        }
    }
}

/// Model representing a mission dependency in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDependencyModel {
    /// Dependency ID (primary key)
    pub id: Option<i64>,
    
    /// Mission ID (foreign key)
    pub mission_id: String,
    
    /// Component ID (optional foreign key, if specific to a component)
    pub component_id: Option<i64>,
    
    /// Class name that is depended on
    pub class_name: String,
    
    /// Reference type
    pub reference_type: String,
    
    /// Source file within the mission
    pub source_file: PathBuf,
    
    /// Line number in the source file
    pub line_number: Option<usize>,
}

impl MissionDependencyModel {
    /// Create a new mission dependency model
    pub fn new(
        mission_id: impl Into<String>,
        class_name: impl Into<String>,
        reference_type: impl Into<String>,
        source_file: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id: None,
            mission_id: mission_id.into(),
            component_id: None,
            class_name: class_name.into(),
            reference_type: reference_type.into(),
            source_file: source_file.into(),
            line_number: None,
        }
    }
    
    /// Create a new mission dependency model for a component
    pub fn for_component(
        mission_id: impl Into<String>,
        component_id: i64,
        class_name: impl Into<String>,
        reference_type: impl Into<String>,
        source_file: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id: None,
            mission_id: mission_id.into(),
            component_id: Some(component_id),
            class_name: class_name.into(),
            reference_type: reference_type.into(),
            source_file: source_file.into(),
            line_number: None,
        }
    }
    
    /// Convert from shared_models::DependencyRef
    pub fn from_dependency(
        mission_id: &str,
        dependency: &arma3_models::DependencyRef,
    ) -> Self {
        Self {
            id: None,
            mission_id: mission_id.to_string(),
            component_id: None,
            class_name: dependency.class_name.clone(),
            reference_type: format!("{:?}", dependency.reference_type),
            source_file: dependency.source_file.clone(),
            line_number: dependency.line_number,
        }
    }
    
    /// Convert from shared_models::DependencyRef for a component
    pub fn from_component_dependency(
        mission_id: &str,
        component_id: i64,
        dependency: &arma3_models::DependencyRef,
    ) -> Self {
        Self {
            id: None,
            mission_id: mission_id.to_string(),
            component_id: Some(component_id),
            class_name: dependency.class_name.clone(),
            reference_type: format!("{:?}", dependency.reference_type),
            source_file: dependency.source_file.clone(),
            line_number: dependency.line_number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mission_model() {
        let now = Utc::now();
        
        let mission = MissionModel::new(
            "test_mission",
            "Test Mission",
            "missions/test.pbo",
            now,
        );
        
        assert_eq!(mission.id, "test_mission");
        assert_eq!(mission.name, "Test Mission");
        assert_eq!(mission.path, PathBuf::from("missions/test.pbo"));
        assert_eq!(mission.scan_time, now);
        assert!(mission.author.is_none());
        
        let mission_with_details = MissionModel::with_details(
            "test_mission_2",
            "Test Mission 2",
            "missions/test2.pbo",
            Some("source.pbo"),
            Some("Test Author"),
            Some("Test Description"),
            now,
        );
        
        assert_eq!(mission_with_details.id, "test_mission_2");
        assert_eq!(mission_with_details.author, Some("Test Author".to_string()));
        assert_eq!(mission_with_details.source_pbo_id, Some("source.pbo".to_string()));
    }
    
    #[test]
    fn test_component_model() {
        let component = MissionComponentModel::new(
            "test_mission",
            "TestComponent",
            "Vehicle",
            "mission/component.sqf",
        );
        
        assert_eq!(component.mission_id, "test_mission");
        assert_eq!(component.name, "TestComponent");
        assert_eq!(component.component_type, "Vehicle");
        assert_eq!(component.source_file, PathBuf::from("mission/component.sqf"));
        assert!(component.id.is_none());
    }
    
    #[test]
    fn test_dependency_model() {
        let dependency = MissionDependencyModel::new(
            "test_mission",
            "Vehicle_F",
            "DirectClass",
            "mission/dependency.sqf",
        );
        
        assert_eq!(dependency.mission_id, "test_mission");
        assert_eq!(dependency.class_name, "Vehicle_F");
        assert_eq!(dependency.reference_type, "DirectClass");
        assert_eq!(dependency.source_file, PathBuf::from("mission/dependency.sqf"));
        assert!(dependency.id.is_none());
        assert!(dependency.component_id.is_none());
        
        let component_dependency = MissionDependencyModel::for_component(
            "test_mission",
            123,
            "Vehicle_F",
            "DirectClass",
            "mission/component_dependency.sqf",
        );
        
        assert_eq!(component_dependency.mission_id, "test_mission");
        assert_eq!(component_dependency.component_id, Some(123));
    }
} 