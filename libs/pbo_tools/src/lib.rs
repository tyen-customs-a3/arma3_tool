pub mod cli;
pub mod core;
pub mod error;
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

// Legacy exports for backward compatibility (deprecated)
#[deprecated(note = "Use PboOperationError instead")]
pub use ops::PboOperationError as PboError;
#[deprecated(note = "Use PboOperationResult instead")]
pub use ops::PboOperationResult as Result;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");