use std::collections::HashMap;
use cpp_parser::Value;
use log::trace;

/// Property processor for processing class properties
#[derive(Debug, Default)]
pub struct PropertyProcessor {}

impl PropertyProcessor {
    /// Create a new property processor
    pub fn new() -> Self {
        Self {}
    }
    
    /// Process properties from a class
    pub fn process_properties(&self, properties: &HashMap<String, Value>) -> Vec<(String, String)> {
        // Pre-allocate properties vector with estimated capacity
        let mut processed_properties = Vec::with_capacity(properties.len());
        
        // Process properties
        for (key, value) in properties {
            match value {
                Value::String(s) => {
                    trace!("Processing string property: {} = {}", key, s);
                    processed_properties.push((key.clone(), s.clone()));
                }
                Value::Number(n) => {
                    trace!("Processing number property: {} = {}", key, n);
                    processed_properties.push((key.clone(), n.to_string()));
                }
                Value::Array(arr) => {
                    trace!("Processing array property: {} with {} elements", key, arr.len());
                    // Convert array to string representation
                    let arr_str = arr.iter()
                        .map(|v| self.value_to_string(v))
                        .collect::<Vec<_>>()
                        .join(", ");
                    processed_properties.push((key.clone(), format!("[{}]", arr_str)));
                }
                Value::Class(_) => {
                    // Nested classes are processed separately
                    trace!("Skipping nested class property: {}", key);
                }
                Value::Integer(i) => {
                    trace!("Processing integer property: {} = {}", key, i);
                    processed_properties.push((key.clone(), i.to_string()));
                }
                Value::Expression(expr) => {
                    trace!("Processing expression property: {} = {}", key, expr);
                    processed_properties.push((key.clone(), format!("expr:{}", expr)));
                }
                Value::Reference(ref_name) => {
                    trace!("Processing reference property: {} = {}", key, ref_name);
                    processed_properties.push((key.clone(), format!("ref:{}", ref_name)));
                }
                Value::ListMacro(count, name) => {
                    trace!("Processing list macro property: {} = {}:{}", key, count, name);
                    processed_properties.push((key.clone(), format!("macro:{}:{}", count, name)));
                }
            }
        }
        
        processed_properties
    }
    
    /// Convert a value to a string representation
    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::String(s) => format!("\"{}\"", s),
            Value::Number(n) => n.to_string(),
            Value::Array(arr) => {
                let arr_str = arr.iter()
                    .map(|v| self.value_to_string(v))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", arr_str)
            }
            Value::Class(_) => "class".to_string(),
            Value::Integer(i) => i.to_string(),
            Value::Expression(expr) => format!("expr:{}", expr),
            Value::Reference(ref_name) => format!("ref:{}", ref_name),
            Value::ListMacro(count, name) => format!("macro:{}:{}", count, name),
        }
    }
    
    /// Get the type of a value as a string
    pub fn get_value_type(&self, value: &Value) -> String {
        match value {
            Value::String(_) => "string".to_string(),
            Value::Number(_) => "number".to_string(),
            Value::Array(arr) => {
                if arr.is_empty() {
                    "array".to_string()
                } else {
                    format!("array<{}>", self.get_value_type(&arr[0]))
                }
            },
            Value::Class(_) => "class".to_string(),
            Value::Integer(_) => "integer".to_string(),
            Value::Expression(_) => "expression".to_string(),
            Value::Reference(_) => "reference".to_string(),
            Value::ListMacro(_, _) => "macro".to_string(),
        }
    }
} 