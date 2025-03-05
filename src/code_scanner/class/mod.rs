pub mod types;
pub mod scanner;
pub mod processor;

// Re-export the main API for easier access
pub use types::{ProcessedClass, ClassScanStats};
pub use scanner::ClassScanner;
pub use processor::ClassProcessor; 