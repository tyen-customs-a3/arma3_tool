// Configuration has been moved to libs/config
// Re-export from arma3_config for backward compatibility
pub use arma3_config::{
    ScanConfig, GlobalConfig, VisualizationConfig,
    ItemFilterConfig, ItemTypeConfig, ExclusionRules,
    ConfigValidation,
};

// Note: This file can be removed once all code has been updated to import
// config types directly from arma3_config instead of crate::config
