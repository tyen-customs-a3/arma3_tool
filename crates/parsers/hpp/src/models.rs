// Re-export from the new unified types
pub use arma3_types::{Class as GameClass, Value as PropertyValue};
pub use arma3_parser_common::{Parser as FileParser};

// Create a type alias for backward compatibility
pub type ClassProperty = (String, PropertyValue);