pub mod cli;
pub mod core;
pub mod fs;
pub mod ops;
#[cfg(test)]
pub mod test_utils;

// Re-export commonly used types for easier access
pub use core::{
    api::{PboApi, PboApiOps},
    config::PboConfig,
    constants::{DEFAULT_TIMEOUT, DEFAULT_MAX_RETRIES},
};
pub use ops::{
    PboOperations, PboFileInfo, PboProperties, PboValidation,
    PboOperationError, PboOperationResult, HemttPboOperations,
};

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");