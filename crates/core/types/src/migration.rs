//! Migration utilities for converting between legacy and unified types

use crate::unified::{Class, Value, ClassMetadata};
use crate::types::PropertyValue as LegacyPropertyValue;
use crate::gamedata::{GameDataClass, GameDataClasses};
use std::collections::HashMap;

/// Convert legacy PropertyValue to unified Value
pub fn convert_legacy_property_value(legacy: &LegacyPropertyValue) -> Value {
    match legacy {
        LegacyPropertyValue::String(s) => Value::String(s.clone()),
        LegacyPropertyValue::Number(n) => Value::Number(*n),
        LegacyPropertyValue::Array(arr) => {
            Value::Array(arr.iter().map(convert_legacy_property_value).collect())
        }
        LegacyPropertyValue::Boolean(b) => Value::Boolean(*b),
        LegacyPropertyValue::Object(obj) => {
            let mut new_obj = HashMap::new();
            for (k, v) in obj {
                new_obj.insert(k.clone(), convert_legacy_property_value(v));
            }
            Value::Object(new_obj)
        }
    }
}

/// Convert legacy GameDataClass to unified Class
pub fn convert_legacy_gamedata_class(legacy: &GameDataClass) -> Class {
    let mut class = Class::new(legacy.name.clone());
    
    if let Some(parent) = &legacy.parent {
        class = class.with_parent(parent.clone());
    }
    
    if let Some(container) = &legacy.container_class {
        class = class.with_container(container.clone());
    }
    
    class.is_forward_declaration = legacy.is_forward_declaration;
    
    // Convert properties
    for (key, value) in &legacy.properties {
        class = class.with_property(key.clone(), convert_legacy_property_value(value));
    }
    
    class
}

/// Convert unified Value to legacy PropertyValue (lossy conversion)
pub fn convert_to_legacy_property_value(unified: &Value) -> LegacyPropertyValue {
    match unified {
        Value::String(s) => LegacyPropertyValue::String(s.clone()),
        Value::Integer(i) => LegacyPropertyValue::Number(*i as f64),
        Value::Number(n) => LegacyPropertyValue::Number(*n),
        Value::Boolean(b) => LegacyPropertyValue::Boolean(*b),
        Value::Array(arr) => {
            LegacyPropertyValue::Array(arr.iter().map(convert_to_legacy_property_value).collect())
        }
        Value::Object(obj) => {
            let mut legacy_obj = HashMap::new();
            for (k, v) in obj {
                legacy_obj.insert(k.clone(), convert_to_legacy_property_value(v));
            }
            LegacyPropertyValue::Object(legacy_obj)
        }
        Value::ClassRef(name) => LegacyPropertyValue::String(name.clone()),
        Value::Expression(expr) => LegacyPropertyValue::String(expr.clone()),
    }
}

/// Convert unified Class to legacy GameDataClass (lossy conversion)
pub fn convert_to_legacy_gamedata_class(unified: &Class) -> GameDataClass {
    let mut legacy = GameDataClass::new(unified.name.clone(), unified.parent.clone());
    
    if let Some(container) = &unified.container_class {
        legacy.set_container_class(container.clone());
    }
    
    legacy.is_forward_declaration = unified.is_forward_declaration;
    
    // Convert properties
    for (key, value) in &unified.properties {
        legacy.add_property(key.clone(), convert_to_legacy_property_value(value));
    }
    
    // Convert arrays as regular properties (flattening)
    for (key, values) in &unified.arrays {
        let array_value = Value::Array(values.clone());
        legacy.add_property(key.clone(), convert_to_legacy_property_value(&array_value));
    }
    
    legacy
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_property_value_conversion_roundtrip() {
        let original = LegacyPropertyValue::String("test".to_string());
        let unified = convert_legacy_property_value(&original);
        let back = convert_to_legacy_property_value(&unified);
        
        assert_eq!(original, back);
    }

    #[test]
    fn test_complex_property_conversion() {
        let mut obj = HashMap::new();
        obj.insert("key1".to_string(), LegacyPropertyValue::String("value1".to_string()));
        obj.insert("key2".to_string(), LegacyPropertyValue::Number(42.0));
        
        let original = LegacyPropertyValue::Object(obj);
        let unified = convert_legacy_property_value(&original);
        
        if let Value::Object(unified_obj) = unified {
            assert_eq!(unified_obj.len(), 2);
            assert_eq!(unified_obj.get("key1").unwrap().as_string(), Some("value1"));
            assert_eq!(unified_obj.get("key2").unwrap().as_number(), Some(42.0));
        } else {
            panic!("Expected Object variant");
        }
    }

    #[test]
    fn test_gamedata_class_conversion() {
        let mut legacy = GameDataClass::new("TestClass".to_string(), Some("Parent".to_string()));
        legacy.add_property("prop1".to_string(), LegacyPropertyValue::String("value1".to_string()));
        legacy.add_property("prop2".to_string(), LegacyPropertyValue::Number(123.0));
        
        let unified = convert_legacy_gamedata_class(&legacy);
        
        assert_eq!(unified.name, "TestClass");
        assert_eq!(unified.parent, Some("Parent".to_string()));
        assert_eq!(unified.properties.len(), 2);
        assert_eq!(unified.get_property("prop1").unwrap().as_string(), Some("value1"));
        assert_eq!(unified.get_property("prop2").unwrap().as_number(), Some(123.0));
    }

    #[test]
    fn test_unified_class_conversion_back() {
        let unified = Class::new("TestClass")
            .with_parent("Parent")
            .with_property("prop1", Value::string("value1"))
            .with_property("prop2", Value::integer(123))
            .with_array("arr", vec![Value::string("item1"), Value::string("item2")]);
        
        let legacy = convert_to_legacy_gamedata_class(&unified);
        
        assert_eq!(legacy.name, "TestClass");
        assert_eq!(legacy.parent, Some("Parent".to_string()));
        assert_eq!(legacy.properties.len(), 3); // 2 properties + 1 array as property
    }
}