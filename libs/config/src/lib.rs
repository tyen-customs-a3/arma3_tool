pub mod scan;
pub mod item_filter;
pub mod visualization;
pub mod validation;
pub mod error;
pub mod builder;

// Re-export key types for convenience
pub use scan::ScanConfig;
pub use item_filter::{ItemFilterConfig, ItemTypeConfig, ExclusionRules};
pub use visualization::VisualizationConfig;
pub use validation::ConfigValidation;

// Re-export error types
pub use error::{ConfigError, Result};

// Re-export builder preset functions
pub use builder::presets;

// Type alias for backwards compatibility
pub type GlobalConfig = ScanConfig;

/// Create a scan configuration builder
/// 
/// # Examples
/// 
/// ```rust
/// use arma3_config::scan_config;
/// 
/// let config = scan_config()
///     .with_game_data_dir("/opt/arma3")
///     .with_mission_dir("/missions/test")
///     .with_cache_dir("/tmp/cache")
///     .build()?;
/// ```
pub fn scan_config() -> ScanConfig {
    ScanConfig::builder()
}

/// Create an item filter configuration builder
/// 
/// # Examples
/// 
/// ```rust
/// use arma3_config::item_filter_config;
/// 
/// let config = item_filter_config()
///     .with_common_game_data_types()
///     .build()?;
/// ```
pub fn item_filter_config() -> ItemFilterConfig {
    ItemFilterConfig::builder()
}

/// Create a visualization configuration builder
/// 
/// # Examples
/// 
/// ```rust
/// use arma3_config::visualization_config;
/// 
/// let config = visualization_config()
///     .with_preserved_class("MyClass")
///     .build()?;
/// ```
pub fn visualization_config() -> VisualizationConfig {
    VisualizationConfig::builder()
}