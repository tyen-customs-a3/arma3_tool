pub mod config;
pub mod cache;
pub mod error;
pub mod scanner;

// Re-export commonly used types
pub use config::ToolConfig;
pub use cache::CacheManager;
pub use error::{Result, ToolError};
pub use scanner::gamedata::GameDataClasses;
pub use scanner::mission::MissionScanner;
pub use scanner::report::ReportGenerator;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
