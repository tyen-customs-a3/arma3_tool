use serde::{Serialize, Deserialize};

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
pub struct DatabaseQuery {
    pub query_type: QueryType,
    pub parameters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum QueryType {
    GetClassHierarchy,
    GetPboDependencies,
    GetMissionDependencies,
    GetClassImpact,
    GetDatabaseStats,
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