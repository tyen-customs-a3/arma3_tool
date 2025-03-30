use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Property value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Array(Vec<PropertyValue>),
    Boolean(bool),
    Object(HashMap<String, PropertyValue>),
} 