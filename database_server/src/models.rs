use arma3_models::PropertyValue;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Graph data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source: String,
    pub target: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

// WebSocket message structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GenerateRequest {
    pub node_count: usize,
    pub grid_size: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CsvData {
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileRequest {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebSocketMessage {
    pub action: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeDetailsRequest {
    pub node_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeMetadata {
    pub id: String,
    pub parent_id: Option<String>,
    pub container_class: Option<String>,
    pub source_path: Option<String>,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseQuery {
    pub query_type: QueryType,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QueryType {
    ClassHierarchy,
    PboDependencies,
    MissionDependencies,
    ClassImpact,
    DatabaseStats,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassHierarchyRequest {
    pub root_class: Option<String>,
    pub max_depth: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassImpactRequest {
    pub classes_to_remove: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorizeChildrenRequest {
    pub parent_class: String,
    pub color: String,
}