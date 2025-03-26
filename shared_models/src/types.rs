use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Property value for game data and mission components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<PropertyValue>),
    Object(HashMap<String, PropertyValue>),
} 