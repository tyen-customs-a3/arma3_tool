use serde::{Serialize, Deserialize};
use super::options::WorkflowOptions;

/// Type for processing workflow
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WorkflowType {
    /// Extract only
    Extract,
    /// Process only
    Process,
    /// Generate reports only
    Report,
    /// Export only
    Export,
    /// Extract and process
    ExtractAndProcess,
    /// Process and report
    ProcessAndReport,
    /// Complete workflow (extract, process, report)
    Complete,
}

impl WorkflowType {
    /// Check if this workflow type includes extraction
    pub fn includes_extraction(&self) -> bool {
        matches!(self, WorkflowType::Extract | WorkflowType::ExtractAndProcess | WorkflowType::Complete)
    }
    
    /// Check if this workflow type includes processing
    pub fn includes_processing(&self) -> bool {
        matches!(self, WorkflowType::Process | WorkflowType::ExtractAndProcess | WorkflowType::ProcessAndReport | WorkflowType::Complete)
    }
    
    /// Check if this workflow type includes reporting
    pub fn includes_reporting(&self) -> bool {
        matches!(self, WorkflowType::Report | WorkflowType::ProcessAndReport | WorkflowType::Complete)
    }
    
    /// Check if this workflow type includes export
    pub fn includes_export(&self) -> bool {
        matches!(self, WorkflowType::Export)
    }
    
    /// Get all workflow types
    pub fn all() -> Vec<WorkflowType> {
        vec![
            WorkflowType::Extract,
            WorkflowType::Process,
            WorkflowType::Report,
            WorkflowType::Export,
            WorkflowType::ExtractAndProcess,
            WorkflowType::ProcessAndReport,
            WorkflowType::Complete,
        ]
    }
}

impl std::fmt::Display for WorkflowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkflowType::Extract => write!(f, "Extract"),
            WorkflowType::Process => write!(f, "Process"),
            WorkflowType::Report => write!(f, "Report"),
            WorkflowType::Export => write!(f, "Export"),
            WorkflowType::ExtractAndProcess => write!(f, "Extract and Process"),
            WorkflowType::ProcessAndReport => write!(f, "Process and Report"),
            WorkflowType::Complete => write!(f, "Complete"),
        }
    }
}

/// Type of content to work with
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ContentType {
    /// Game data
    GameData,
    /// Mission data
    Mission,
    /// Both game data and mission data
    Both,
}

impl ContentType {
    /// Check if this content type includes game data
    pub fn includes_game_data(&self) -> bool {
        matches!(self, ContentType::GameData | ContentType::Both)
    }
    
    /// Check if this content type includes mission data
    pub fn includes_mission_data(&self) -> bool {
        matches!(self, ContentType::Mission | ContentType::Both)
    }
    
    /// Get all content types
    pub fn all() -> Vec<ContentType> {
        vec![
            ContentType::GameData,
            ContentType::Mission,
            ContentType::Both,
        ]
    }
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentType::GameData => write!(f, "Game Data"),
            ContentType::Mission => write!(f, "Mission"),
            ContentType::Both => write!(f, "Both"),
        }
    }
}

/// Workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Name of the workflow
    pub name: String,
    
    /// Type of workflow
    pub workflow_type: WorkflowType,
    
    /// Type of content to process
    pub content_type: ContentType,
    
    /// Processing options
    pub options: WorkflowOptions,
}

impl Workflow {
    /// Create a new workflow
    pub fn new(name: String, workflow_type: WorkflowType, content_type: ContentType, options: WorkflowOptions) -> Self {
        Self {
            name,
            workflow_type,
            content_type,
            options,
        }
    }
    
    /// Create a simple workflow with default options
    pub fn simple(name: String, workflow_type: WorkflowType, content_type: ContentType) -> Self {
        Self {
            name,
            workflow_type,
            content_type,
            options: WorkflowOptions::default(),
        }
    }
    
    /// Get a description of what this workflow will do
    pub fn description(&self) -> String {
        format!("Workflow '{}' will {} on {}", 
                self.name, 
                self.workflow_type, 
                self.content_type)
    }
}

impl Default for Workflow {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            workflow_type: WorkflowType::Complete,
            content_type: ContentType::Both,
            options: WorkflowOptions::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workflow_type_includes() {
        assert!(WorkflowType::Extract.includes_extraction());
        assert!(!WorkflowType::Extract.includes_processing());
        assert!(!WorkflowType::Extract.includes_reporting());
        
        assert!(!WorkflowType::Process.includes_extraction());
        assert!(WorkflowType::Process.includes_processing());
        assert!(!WorkflowType::Process.includes_reporting());
        
        assert!(!WorkflowType::Report.includes_extraction());
        assert!(!WorkflowType::Report.includes_processing());
        assert!(WorkflowType::Report.includes_reporting());
        
        assert!(WorkflowType::Complete.includes_extraction());
        assert!(WorkflowType::Complete.includes_processing());
        assert!(WorkflowType::Complete.includes_reporting());
    }
    
    #[test]
    fn test_content_type_includes() {
        assert!(ContentType::GameData.includes_game_data());
        assert!(!ContentType::GameData.includes_mission_data());
        
        assert!(!ContentType::Mission.includes_game_data());
        assert!(ContentType::Mission.includes_mission_data());
        
        assert!(ContentType::Both.includes_game_data());
        assert!(ContentType::Both.includes_mission_data());
    }
    
    #[test]
    fn test_workflow_creation() {
        let workflow = Workflow::simple(
            "test".to_string(),
            WorkflowType::Extract,
            ContentType::GameData
        );
        
        assert_eq!(workflow.name, "test");
        assert_eq!(workflow.workflow_type, WorkflowType::Extract);
        assert_eq!(workflow.content_type, ContentType::GameData);
    }
    
    #[test]
    fn test_workflow_serialization() {
        let workflow = Workflow::default();
        let json = serde_json::to_string(&workflow).unwrap();
        let deserialized: Workflow = serde_json::from_str(&json).unwrap();
        
        assert_eq!(workflow.name, deserialized.name);
        assert_eq!(workflow.workflow_type, deserialized.workflow_type);
        assert_eq!(workflow.content_type, deserialized.content_type);
    }
}