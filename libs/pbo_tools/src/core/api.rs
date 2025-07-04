use std::path::Path;
use std::time::Duration;
use std::sync::Arc;
use log::{debug, warn};
use crate::ops::{
    PboOperations, HemttPboOperations, PboFileInfo, PboProperties, PboValidation,
    PboOperationResult, PboOperationError
};
use super::config::PboConfig;
use super::constants::DEFAULT_TIMEOUT;

/// Core trait defining operations available for PBO files using modern HEMTT backend.
/// 
/// This trait provides the main interface for working with PBO files, including:
/// - Listing contents with detailed metadata
/// - Extracting files (with filtering and pattern matching)
/// - Accessing PBO properties and validation
/// - Native Rust operations without external dependencies
///
/// # Examples
///
/// ```no_run
/// use pbo_tools::core::{PboApi, PboApiOps};
/// use std::path::Path;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let api = PboApi::new(30); // 30 second timeout
/// let pbo_path = Path::new("mission.pbo");
///
/// // List contents
/// let files = api.list_contents(&pbo_path).await?;
/// println!("Found {} files", files.len());
///
/// // Extract specific files
/// let output_dir = Path::new("output");
/// api.extract_filtered(&pbo_path, &output_dir, "*.cpp").await?;
/// # Ok(())
/// # }
/// ```
#[async_trait::async_trait]
pub trait PboApiOps {
    /// List all files in a PBO with detailed metadata
    async fn list_contents(&self, pbo_path: &Path) -> PboOperationResult<Vec<PboFileInfo>>;
    
    /// Extract a specific file from a PBO
    async fn extract_file(&self, pbo_path: &Path, file_path: &str, output_path: &Path) -> PboOperationResult<()>;
    
    /// Extract all files from a PBO
    async fn extract_all(&self, pbo_path: &Path, output_dir: &Path) -> PboOperationResult<()>;
    
    /// Extract files matching a pattern from a PBO
    async fn extract_filtered(&self, pbo_path: &Path, output_dir: &Path, filter: &str) -> PboOperationResult<()>;
    
    /// Get PBO properties and metadata
    async fn get_properties(&self, pbo_path: &Path) -> PboOperationResult<PboProperties>;
    
    /// Validate a PBO file for integrity and correctness
    async fn validate_pbo(&self, pbo_path: &Path) -> PboOperationResult<PboValidation>;
    
    /// Read a file from a PBO into memory
    async fn read_file(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<Vec<u8>>;
    
    /// Check if a file exists in a PBO
    async fn file_exists(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<bool>;
    
    /// Get information about a specific file in a PBO
    async fn get_file_info(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<Option<PboFileInfo>>;
}

/// Main API for working with PBO files using native HEMTT backend.
///
/// PboApi provides a high-level interface for PBO operations with:
/// - Native Rust implementation (no external dependencies)
/// - Async operations for better performance
/// - Comprehensive error handling
/// - Cross-platform compatibility
///
/// # Examples
///
/// Basic usage:
/// ```no_run
/// use pbo_tools::core::PboApi;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let api = PboApi::builder()
///     .with_timeout(30)
///     .build();
///
/// let files = api.list_contents("mission.pbo").await?;
/// println!("Found {} files", files.len());
/// # Ok(())
/// # }
/// ```
///
/// Advanced configuration:
/// ```no_run
/// use pbo_tools::core::{PboApi, PboConfig};
///
/// let config = PboConfig::builder()
///     .case_sensitive(true)
///     .max_retries(5)
///     .build();
///
/// let api = PboApi::builder()
///     .with_config(config)
///     .with_timeout(30)
///     .build();
/// ```
#[derive(Debug)]
pub struct PboApi {
    config: Arc<PboConfig>,
    pbo_ops: HemttPboOperations,
    timeout: Duration,
}

impl PboApi {
    pub fn builder() -> PboApiBuilder {
        PboApiBuilder::new()
    }

    pub fn new(timeout_seconds: u32) -> Self {
        Self::builder()
            .with_timeout(timeout_seconds)
            .build()
    }

    /// Execute an async operation with timeout handling
    async fn with_timeout<T, F>(&self, operation: F) -> PboOperationResult<T>
    where
        F: std::future::Future<Output = PboOperationResult<T>>,
    {
        match tokio::time::timeout(self.timeout, operation).await {
            Ok(result) => result,
            Err(_) => {
                warn!("Operation timed out after {} seconds", self.timeout.as_secs());
                Err(PboOperationError::timeout(format!("Operation timed out after {} seconds", self.timeout.as_secs())))
            }
        }
    }

    /// Validate that a PBO file exists and is accessible
    fn validate_pbo_path(&self, pbo_path: &Path) -> PboOperationResult<()> {
        if !pbo_path.exists() {
            return Err(PboOperationError::file_not_found(pbo_path));
        }
        
        // Check for common PBO extensions
        if let Some(extension) = pbo_path.extension() {
            let ext_str = extension.to_string_lossy().to_lowercase();
            if !["pbo", "xbo", "ifa"].contains(&ext_str.as_str()) {
                return Err(PboOperationError::invalid_format(
                    format!("File does not have a valid PBO extension: {}", pbo_path.display())
                ));
            }
        } else {
            return Err(PboOperationError::invalid_format(
                format!("File has no extension: {}", pbo_path.display())
            ));
        }
        
        Ok(())
    }

    /// Validate and create output directory if needed
    fn ensure_output_dir(&self, output_dir: &Path) -> PboOperationResult<()> {
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)
                .map_err(|e| PboOperationError::io_error("creating output directory", e))?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl PboApiOps for PboApi {
    async fn list_contents(&self, pbo_path: &Path) -> PboOperationResult<Vec<PboFileInfo>> {
        debug!("Listing contents of PBO: {}", pbo_path.display());
        self.validate_pbo_path(pbo_path)?;
        
        self.with_timeout(self.pbo_ops.list_contents(pbo_path)).await
    }

    async fn extract_file(&self, pbo_path: &Path, file_path: &str, output_path: &Path) -> PboOperationResult<()> {
        debug!("Extracting file '{}' from {} to {}", file_path, pbo_path.display(), output_path.display());
        self.validate_pbo_path(pbo_path)?;
        
        if let Some(parent) = output_path.parent() {
            self.ensure_output_dir(parent)?;
        }
        
        self.with_timeout(self.pbo_ops.extract_file(pbo_path, file_path, output_path)).await
    }

    async fn extract_all(&self, pbo_path: &Path, output_dir: &Path) -> PboOperationResult<()> {
        debug!("Extracting all files from {} to {}", pbo_path.display(), output_dir.display());
        self.validate_pbo_path(pbo_path)?;
        self.ensure_output_dir(output_dir)?;
        
        self.with_timeout(self.pbo_ops.extract_all(pbo_path, output_dir)).await
    }

    async fn extract_filtered(&self, pbo_path: &Path, output_dir: &Path, filter: &str) -> PboOperationResult<()> {
        debug!("Extracting filtered files from {} to {} with filter '{}'", 
               pbo_path.display(), output_dir.display(), filter);
        self.validate_pbo_path(pbo_path)?;
        self.ensure_output_dir(output_dir)?;
        
        if filter.trim().is_empty() {
            return Err(PboOperationError::invalid_path("Filter cannot be empty"));
        }
        
        self.with_timeout(self.pbo_ops.extract_filtered(pbo_path, filter, output_dir)).await
    }

    async fn get_properties(&self, pbo_path: &Path) -> PboOperationResult<PboProperties> {
        debug!("Getting properties for PBO: {}", pbo_path.display());
        self.validate_pbo_path(pbo_path)?;
        
        self.with_timeout(self.pbo_ops.get_properties(pbo_path)).await
    }

    async fn validate_pbo(&self, pbo_path: &Path) -> PboOperationResult<PboValidation> {
        debug!("Validating PBO: {}", pbo_path.display());
        self.validate_pbo_path(pbo_path)?;
        
        self.with_timeout(self.pbo_ops.validate_pbo(pbo_path)).await
    }

    async fn read_file(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<Vec<u8>> {
        debug!("Reading file '{}' from {}", file_path, pbo_path.display());
        self.validate_pbo_path(pbo_path)?;
        
        self.with_timeout(self.pbo_ops.read_file(pbo_path, file_path)).await
    }

    async fn file_exists(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<bool> {
        debug!("Checking if file '{}' exists in {}", file_path, pbo_path.display());
        self.validate_pbo_path(pbo_path)?;
        
        self.with_timeout(self.pbo_ops.file_exists(pbo_path, file_path)).await
    }

    async fn get_file_info(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<Option<PboFileInfo>> {
        debug!("Getting file info for '{}' from {}", file_path, pbo_path.display());
        self.validate_pbo_path(pbo_path)?;
        
        self.with_timeout(self.pbo_ops.get_file_info(pbo_path, file_path)).await
    }
}

/// Builder for creating customized PboApi instances.
///
/// The builder pattern allows for flexible configuration of:
/// - Operation timeout
/// - PBO handling configuration
/// - Custom runtime settings
///
/// # Examples
///
/// ```no_run
/// use pbo_tools::core::PboApi;
///
/// let api = PboApi::builder()
///     .with_timeout(60)  // 60 second timeout
///     .build();
/// ```
#[derive(Default)]
pub struct PboApiBuilder {
    config: Option<PboConfig>,
    timeout: Option<Duration>,
}

impl PboApiBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(mut self, config: PboConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_timeout(mut self, seconds: u32) -> Self {
        self.timeout = Some(Duration::from_secs(u64::from(seconds.max(1))));
        self
    }

    pub fn build(self) -> PboApi {
        PboApi {
            config: Arc::new(self.config.unwrap_or_default()),
            pbo_ops: HemttPboOperations::new(),
            timeout: self.timeout.unwrap_or_else(|| Duration::from_secs(u64::from(DEFAULT_TIMEOUT))),
        }
    }
}
