pub mod config;
pub mod cache;
pub mod error;
pub mod scanner;
pub mod database;

// Re-export commonly used types
pub use config::ToolConfig;
pub use cache::CacheManager;
pub use error::{Result, ToolError};
pub use scanner::models::{GameDataClasses, MissionData};
pub use scanner::mission::MissionScanner;
pub use scanner::report::ReportGenerator;
pub use scanner::gamedata::GameDataScanner;
pub use database::DatabaseManager;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
