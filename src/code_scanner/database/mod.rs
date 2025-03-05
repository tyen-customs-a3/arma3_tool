pub mod types;
pub mod storage;
pub mod operations;

// Re-export main types and functions for easier access
pub use types::{ClassDatabase, ClassDatabaseStats, ClassEntry};
pub use operations::{DatabaseOperations, QueryOptions};
pub use storage::DatabaseStorage; 