use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use arma3_models::types::PropertyValue;

/// Model representing a class in the database
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassModel {
    /// Class name (primary identifier)
    pub id: String,
    
    /// Parent class identifier (can be None for root classes)
    pub parent_id: Option<String>,
    
    /// Container class (if this is a nested class)
    pub container_class: Option<String>,
    
    /// Original file index for compatibility with GameDataClass
    pub source_file_index: Option<usize>,

    /// Whether this class represents a forward declaration
    pub is_forward_declaration: bool,

    /// Properties of the class
    pub properties: HashMap<String, PropertyValue>,
}

impl ClassModel {
    /// Create a new class model
    pub fn new(
        id: impl Into<String>,
        parent_id: Option<impl Into<String>>,
        container_class: Option<impl Into<String>>,
        source_file_index: Option<usize>,
        is_forward_declaration: bool,
    ) -> Self {
        Self {
            id: id.into(),
            parent_id: parent_id.map(Into::into),
            container_class: container_class.map(Into::into),
            source_file_index,
            is_forward_declaration,
            properties: HashMap::new(),
        }
    }
    
    /// Convert from a GameDataClass
    pub fn from_game_data_class(
        class: &arma3_models::gamedata::GameDataClass,
    ) -> Self {
        Self {
            id: class.name.clone(),
            parent_id: class.parent.clone(),
            container_class: class.container_class.clone(),
            source_file_index: class.source_file_index,
            is_forward_declaration: class.is_forward_declaration,
            properties: class.properties.clone(),
        }
    }
    
    /// Convert to a GameDataClass
    pub fn to_game_data_class(&self) -> arma3_models::gamedata::GameDataClass {
        arma3_models::gamedata::GameDataClass {
            name: self.id.clone(),
            parent: self.parent_id.clone(),
            container_class: self.container_class.clone(),
            properties: self.properties.clone(),
            source_file_index: self.source_file_index,
            is_forward_declaration: self.is_forward_declaration,
        }
    }
}

/// Node in a class hierarchy with additional depth information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassHierarchyNode {
    /// Class identifier
    pub id: String,
    
    /// Parent class identifier
    pub parent_id: Option<String>,
    
    /// Container class (if this is a nested class)
    pub container_class: Option<String>,
    
    /// Depth in the hierarchy
    pub depth: i32,
    
    /// Source file index
    pub source_file_index: Option<usize>,

    /// Whether this node represents a forward declaration
    pub is_forward_declaration: bool,
}

/// Graph node data structure for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Node identifier (class name)
    pub id: String,
    
    /// Node type/status
    pub node_type: NodeType,
    
    /// Source file index
    pub source_file_index: Option<usize>,

    /// Whether this node represents a forward declaration
    pub is_forward_declaration: bool,

    /// Parent class identifier
    pub parent_id: Option<String>,

    /// Container class (if this is a nested class)
    pub container_class: Option<String>,

    /// Source file path
    pub source_path: Option<String>,
}

/// Node status/type for visualization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeType {
    /// Normal node
    Normal,
    
    /// Node marked for removal
    Removed,
    
    /// Orphaned node (parent will be removed)
    Orphaned,
    
    /// Affected node (depends on removed/orphaned node)
    Affected,
}

/// Graph edge data structure for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Source node identifier
    pub source: String,
    
    /// Target node identifier
    pub target: String,
    
    /// Edge weight
    pub weight: f32,
}

/// Impact analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysisResult {
    /// Classes that will be directly removed
    pub removed_classes: Vec<String>,
    
    /// Classes that will become orphaned
    pub orphaned_classes: Vec<String>,
    
    /// Classes that are affected by removals
    pub affected_classes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_class_model_conversion() {
        // Create a GameDataClass
        let game_data_class = arma3_models::gamedata::GameDataClass {
            name: "TestClass".to_string(),
            parent: Some("ParentClass".to_string()),
            container_class: None,
            properties: std::collections::HashMap::new(),
            source_file_index: Some(1),
            is_forward_declaration: true,
        };
        
        // Convert to ClassModel
        let class_model = ClassModel::from_game_data_class(&game_data_class);
        
        assert_eq!(class_model.id, "TestClass");
        assert_eq!(class_model.parent_id, Some("ParentClass".to_string()));
        assert_eq!(class_model.source_file_index, Some(1));
        assert_eq!(class_model.is_forward_declaration, true);
        
        // Convert back to GameDataClass
        let converted_class = class_model.to_game_data_class();
        
        assert_eq!(converted_class.name, game_data_class.name);
        assert_eq!(converted_class.parent, game_data_class.parent);
        assert_eq!(converted_class.source_file_index, game_data_class.source_file_index);
        assert_eq!(converted_class.is_forward_declaration, game_data_class.is_forward_declaration);
    }
} 