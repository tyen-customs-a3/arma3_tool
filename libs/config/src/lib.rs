pub mod scan;
pub mod item_filter;
pub mod visualization;
pub mod validation;

// Re-export key types for convenience
pub use scan::ScanConfig;
pub use item_filter::{ItemFilterConfig, ItemTypeConfig, ExclusionRules};
pub use visualization::VisualizationConfig;
pub use validation::ConfigValidation;

// Type alias for backwards compatibility
pub type GlobalConfig = ScanConfig;