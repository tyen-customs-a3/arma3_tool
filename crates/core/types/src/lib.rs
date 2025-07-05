pub mod gamedata;
pub mod mission;
pub mod types;
pub mod unified;
pub mod migration;

// Export legacy types for backward compatibility
pub use gamedata::*;
pub use mission::*;

// Export new unified types as primary interface
pub use unified::{Class, Value, ClassMetadata, ConversionError};

// Export migration utilities
pub use migration::*;

// Type aliases for backward compatibility during migration
pub type PropertyValue = Value;
pub type GameClass = Class;
pub type GameDataClass = Class; 