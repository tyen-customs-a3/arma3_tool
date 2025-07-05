use crate::PropertyValue;

// Implement conversions from strings to PropertyValue
impl From<String> for PropertyValue {
    fn from(value: String) -> Self {
        PropertyValue::String(value)
    }
}

impl From<&str> for PropertyValue {
    fn from(value: &str) -> Self {
        PropertyValue::String(value.to_string())
    }
}

// Implement conversions from numbers to PropertyValue
impl From<i64> for PropertyValue {
    fn from(value: i64) -> Self {
        PropertyValue::Number(value)
    }
}

impl From<i32> for PropertyValue {
    fn from(value: i32) -> Self {
        PropertyValue::Number(value as i64)
    }
}

// Implement conversions from arrays to PropertyValue
impl From<Vec<String>> for PropertyValue {
    fn from(value: Vec<String>) -> Self {
        PropertyValue::Array(value)
    }
}

impl<'a> From<&'a [String]> for PropertyValue {
    fn from(value: &'a [String]) -> Self {
        PropertyValue::Array(value.to_vec())
    }
}

impl<'a> From<&'a [&str]> for PropertyValue {
    fn from(value: &'a [&str]) -> Self {
        PropertyValue::Array(value.iter().map(|s| s.to_string()).collect())
    }
}
