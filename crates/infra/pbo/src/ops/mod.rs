//! Modern PBO operations interface using HEMTT's PBO crate
//! 
//! This module provides a trait-based abstraction for PBO operations,
//! allowing for different backend implementations while maintaining
//! a consistent API.

pub mod types;
pub mod error;
pub mod traits;
pub mod hemtt_backend;

pub use types::{PboFileInfo, PboProperties, PboValidation};
pub use error::{PboOperationError, PboOperationResult};
pub use traits::PboOperations;
pub use hemtt_backend::HemttPboOperations;