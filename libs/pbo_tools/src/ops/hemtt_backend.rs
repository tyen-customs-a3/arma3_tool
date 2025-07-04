//! HEMTT-based implementation of PBO operations

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use async_trait::async_trait;
use hemtt_pbo::{ReadablePbo, Header, Mime};
use log::{debug, trace, warn};

use super::traits::PboOperations;
use super::types::{
    PboFileInfo, PboProperties, PboValidation, ValidationIssue, ValidationIssueType, ValidationSeverity
};
use super::error::{PboOperationResult, PboOperationError};

/// HEMTT-based PBO operations implementation
/// 
/// This implementation uses the HEMTT PBO crate to provide native Rust
/// PBO operations without external dependencies.
#[derive(Debug, Clone)]
pub struct HemttPboOperations {
    // Configuration options can be added here
}

impl HemttPboOperations {
    /// Create a new HEMTT PBO operations instance
    pub fn new() -> Self {
        Self {}
    }

    /// Convert a HEMTT Header to our PboFileInfo
    fn header_to_file_info(header: &Header) -> PboFileInfo {
        let mime_str = match header.mime() {
            Mime::Vers => "vers",
            Mime::Cprs => "cprs", 
            Mime::Enco => "enco",
            Mime::Blank => "blank",
        };

        PboFileInfo::new(
            header.filename().replace('\\', "/"), // Normalize to forward slashes
            header.size() as u64,
            header.size() as u64, // HEMTT doesn't separate compressed size currently
            header.timestamp() as u64,
            header.original() as u64,
            header.reserved() as u64,
            mime_str.to_string(),
        )
    }

    /// Check if a file path matches a glob pattern
    fn matches_pattern(file_path: &str, pattern: &str) -> bool {
        // Simple glob pattern matching
        // In a production implementation, you'd want to use a proper glob library
        if pattern == "*" || pattern == "**" {
            return true;
        }

        // Handle basic patterns
        if pattern.contains('*') {
            let pattern_regex = pattern
                .replace("**", ".*")
                .replace("*", "[^/]*")
                .replace("?", ".");
            
            if let Ok(regex) = regex::Regex::new(&format!("^{}$", pattern_regex)) {
                return regex.is_match(file_path);
            }
        }

        // Fallback to simple string matching
        file_path.contains(pattern)
    }
}

impl Default for HemttPboOperations {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PboOperations for HemttPboOperations {
    async fn list_contents(&self, pbo_path: &Path) -> PboOperationResult<Vec<PboFileInfo>> {
        debug!("Listing contents of PBO: {}", pbo_path.display());
        
        if !pbo_path.exists() {
            return Err(PboOperationError::file_not_found(pbo_path));
        }

        let file = File::open(pbo_path)
            .map_err(|e| PboOperationError::io_error("opening PBO file", e))?;

        let pbo = ReadablePbo::from(file)
            .map_err(|e| PboOperationError::from(e))?;

        let files = pbo.files_sorted(); // Use sorted for consistent output
        let file_infos: Vec<PboFileInfo> = files.iter()
            .map(Self::header_to_file_info)
            .collect();

        debug!("Found {} files in PBO", file_infos.len());
        Ok(file_infos)
    }

    async fn extract_file(
        &self,
        pbo_path: &Path,
        file_path: &str,
        output_path: &Path,
    ) -> PboOperationResult<()> {
        debug!("Extracting file '{}' from {} to {}", file_path, pbo_path.display(), output_path.display());
        
        if !pbo_path.exists() {
            return Err(PboOperationError::file_not_found(pbo_path));
        }

        let file = File::open(pbo_path)
            .map_err(|e| PboOperationError::io_error("opening PBO file", e))?;

        let mut pbo = ReadablePbo::from(file)
            .map_err(|e| PboOperationError::from(e))?;

        // Create parent directories if they don't exist
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| PboOperationError::io_error("creating output directory", e))?;
        }

        // Read the file from the PBO
        let mut pbo_file = pbo.file(file_path)
            .map_err(|e| PboOperationError::from(e))?
            .ok_or_else(|| PboOperationError::file_not_found_in_pbo(file_path))?;

        // Create the output file
        let mut output_file = File::create(output_path)
            .map_err(|e| PboOperationError::io_error("creating output file", e))?;

        // Copy the data
        std::io::copy(&mut pbo_file, &mut output_file)
            .map_err(|e| PboOperationError::io_error("copying file data", e))?;

        debug!("Successfully extracted '{}' to {}", file_path, output_path.display());
        Ok(())
    }

    async fn extract_all(&self, pbo_path: &Path, output_dir: &Path) -> PboOperationResult<()> {
        debug!("Extracting all files from {} to {}", pbo_path.display(), output_dir.display());
        
        if !pbo_path.exists() {
            return Err(PboOperationError::file_not_found(pbo_path));
        }

        // Create output directory
        std::fs::create_dir_all(output_dir)
            .map_err(|e| PboOperationError::io_error("creating output directory", e))?;

        let file = File::open(pbo_path)
            .map_err(|e| PboOperationError::io_error("opening PBO file", e))?;

        let mut pbo = ReadablePbo::from(file)
            .map_err(|e| PboOperationError::from(e))?;

        let files = pbo.files_sorted();
        let mut extracted_count = 0;
        let mut errors = Vec::new();

        for header in files {
            let file_path = header.filename();
            let output_path = output_dir.join(file_path.replace('\\', "/"));

            // Create parent directories
            if let Some(parent) = output_path.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    warn!("Failed to create directory {}: {}", parent.display(), e);
                    errors.push(PboOperationError::io_error("creating directory", e));
                    continue;
                }
            }

            // Extract the file
            match pbo.file(file_path) {
                Ok(Some(mut pbo_file)) => {
                    match File::create(&output_path) {
                        Ok(mut output_file) => {
                            match std::io::copy(&mut pbo_file, &mut output_file) {
                                Ok(_) => {
                                    extracted_count += 1;
                                    trace!("Extracted: {}", file_path);
                                }
                                Err(e) => {
                                    warn!("Failed to copy file {}: {}", file_path, e);
                                    errors.push(PboOperationError::io_error("copying file", e));
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to create output file {}: {}", output_path.display(), e);
                            errors.push(PboOperationError::io_error("creating output file", e));
                        }
                    }
                }
                Ok(None) => {
                    warn!("File {} not found in PBO", file_path);
                    errors.push(PboOperationError::file_not_found_in_pbo(file_path));
                }
                Err(e) => {
                    warn!("Failed to read file {} from PBO: {}", file_path, e);
                    errors.push(PboOperationError::from(e));
                }
            }
        }

        debug!("Extracted {} files with {} errors", extracted_count, errors.len());

        if !errors.is_empty() && extracted_count == 0 {
            // If no files were extracted, return the first error
            return Err(errors.into_iter().next().unwrap());
        }

        Ok(())
    }

    async fn extract_filtered(
        &self,
        pbo_path: &Path,
        filter: &str,
        output_dir: &Path,
    ) -> PboOperationResult<()> {
        debug!("Extracting filtered files from {} to {} with filter '{}'", 
               pbo_path.display(), output_dir.display(), filter);
        
        if !pbo_path.exists() {
            return Err(PboOperationError::file_not_found(pbo_path));
        }

        // Create output directory
        std::fs::create_dir_all(output_dir)
            .map_err(|e| PboOperationError::io_error("creating output directory", e))?;

        let file = File::open(pbo_path)
            .map_err(|e| PboOperationError::io_error("opening PBO file", e))?;

        let mut pbo = ReadablePbo::from(file)
            .map_err(|e| PboOperationError::from(e))?;

        let files = pbo.files_sorted();
        let mut extracted_count = 0;

        for header in files {
            let file_path = header.filename();
            let normalized_path = file_path.replace('\\', "/");

            // Check if file matches the filter
            if !Self::matches_pattern(&normalized_path, filter) {
                continue;
            }

            let output_path = output_dir.join(&normalized_path);

            // Create parent directories
            if let Some(parent) = output_path.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    warn!("Failed to create directory {}: {}", parent.display(), e);
                    continue;
                }
            }

            // Extract the file
            match pbo.file(file_path) {
                Ok(Some(mut pbo_file)) => {
                    match File::create(&output_path) {
                        Ok(mut output_file) => {
                            match std::io::copy(&mut pbo_file, &mut output_file) {
                                Ok(_) => {
                                    extracted_count += 1;
                                    trace!("Extracted (filtered): {}", file_path);
                                }
                                Err(e) => {
                                    warn!("Failed to copy file {}: {}", file_path, e);
                                }
                            }
                        }
                        Err(e) => {
                            warn!("Failed to create output file {}: {}", output_path.display(), e);
                        }
                    }
                }
                Ok(None) => {
                    warn!("File {} not found in PBO", file_path);
                }
                Err(e) => {
                    warn!("Failed to read file {} from PBO: {}", file_path, e);
                }
            }
        }

        debug!("Extracted {} files matching filter '{}'", extracted_count, filter);
        Ok(())
    }

    async fn get_properties(&self, pbo_path: &Path) -> PboOperationResult<PboProperties> {
        debug!("Getting properties for PBO: {}", pbo_path.display());
        
        if !pbo_path.exists() {
            return Err(PboOperationError::file_not_found(pbo_path));
        }

        let file = File::open(pbo_path)
            .map_err(|e| PboOperationError::io_error("opening PBO file", e))?;

        let pbo = ReadablePbo::from(file)
            .map_err(|e| PboOperationError::from(e))?;

        let mut properties = PboProperties::new();

        // Get basic properties from HEMTT
        let pbo_properties = pbo.properties();
        
        // Extract common properties
        properties.version = pbo_properties.get("version").cloned();
        properties.author = pbo_properties.get("author").cloned();
        properties.prefix = pbo_properties.get("prefix").cloned();
        
        // Copy all properties
        for (key, value) in pbo_properties {
            properties.set_property(key.clone(), value.clone());
        }

        // Get file statistics
        let files = pbo.files();
        properties.file_count = files.len();
        
        let mut total_size = 0u64;
        let mut total_compressed_size = 0u64;
        
        for header in &files {
            total_size += header.size() as u64;
            total_compressed_size += header.size() as u64; // HEMTT doesn't separate compressed size
        }
        
        properties.total_size = total_size;
        properties.total_compressed_size = total_compressed_size;

        // Get checksum if available
        let checksum = pbo.checksum();
        properties.checksum = Some(checksum.hex());

        debug!("Retrieved properties for PBO with {} files", properties.file_count);
        Ok(properties)
    }

    async fn validate_pbo(&self, pbo_path: &Path) -> PboOperationResult<PboValidation> {
        debug!("Validating PBO: {}", pbo_path.display());
        
        let mut validation = PboValidation::new(pbo_path.to_path_buf());

        if !pbo_path.exists() {
            validation.add_critical_error(
                ValidationIssueType::Corruption,
                "PBO file does not exist".to_string(),
                None,
            );
            return Ok(validation);
        }

        let file = match File::open(pbo_path) {
            Ok(f) => f,
            Err(e) => {
                validation.add_critical_error(
                    ValidationIssueType::Corruption,
                    format!("Cannot open PBO file: {}", e),
                    None,
                );
                return Ok(validation);
            }
        };

        let mut pbo = match ReadablePbo::from(file) {
            Ok(pbo) => pbo,
            Err(e) => {
                validation.add_critical_error(
                    ValidationIssueType::CorruptedHeader,
                    format!("Cannot read PBO structure: {}", e),
                    None,
                );
                return Ok(validation);
            }
        };

        // Check if files are sorted
        match pbo.is_sorted() {
            Ok(()) => {
                validation.files_sorted = true;
            }
            Err((original, sorted)) => {
                validation.files_sorted = false;
                validation.add_warning(
                    ValidationIssueType::UnsortedFiles,
                    format!("Files are not sorted properly. Found {} files, expected {} in sorted order", 
                           original.len(), sorted.len()),
                    None,
                );
            }
        }

        // Validate checksum
        match pbo.gen_checksum() {
            Ok(calculated_checksum) => {
                let stored_checksum = pbo.checksum();
                if calculated_checksum.as_bytes() == stored_checksum.as_bytes() {
                    validation.checksum_valid = Some(true);
                } else {
                    validation.checksum_valid = Some(false);
                    validation.add_error(
                        ValidationIssueType::ChecksumMismatch,
                        "Stored checksum does not match calculated checksum".to_string(),
                        None,
                    );
                }
            }
            Err(e) => {
                validation.add_warning(
                    ValidationIssueType::ChecksumMismatch,
                    format!("Cannot calculate checksum: {}", e),
                    None,
                );
            }
        }

        // Check for required prefix
        let properties = pbo.properties();
        if properties.get("prefix").map_or(true, |p| p.is_empty()) {
            validation.add_warning(
                ValidationIssueType::InvalidPrefix,
                "PBO does not have a prefix property".to_string(),
                None,
            );
        }

        // Validate individual files
        let files = pbo.files();
        for header in &files {
            let file_path = header.filename();
            
            // Check for invalid file paths
            if file_path.contains("..") || file_path.starts_with('/') {
                validation.add_warning(
                    ValidationIssueType::InvalidFilePath,
                    format!("Potentially unsafe file path: {}", file_path),
                    Some(file_path.to_string()),
                );
            }

            // Check for zero-byte files (might be intentional)
            if header.size() == 0 {
                validation.add_warning(
                    ValidationIssueType::FileSizeMismatch,
                    format!("File has zero size: {}", file_path),
                    Some(file_path.to_string()),
                );
            }
        }

        debug!("Validation completed: {} errors, {} warnings", 
               validation.errors.len(), validation.warnings.len());
        
        Ok(validation)
    }

    async fn read_file(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<Vec<u8>> {
        debug!("Reading file '{}' from {}", file_path, pbo_path.display());
        
        if !pbo_path.exists() {
            return Err(PboOperationError::file_not_found(pbo_path));
        }

        let file = File::open(pbo_path)
            .map_err(|e| PboOperationError::io_error("opening PBO file", e))?;

        let mut pbo = ReadablePbo::from(file)
            .map_err(|e| PboOperationError::from(e))?;

        let mut pbo_file = pbo.file(file_path)
            .map_err(|e| PboOperationError::from(e))?
            .ok_or_else(|| PboOperationError::file_not_found_in_pbo(file_path))?;

        let mut buffer = Vec::new();
        pbo_file.read_to_end(&mut buffer)
            .map_err(|e| PboOperationError::io_error("reading file data", e))?;

        debug!("Read {} bytes from '{}'", buffer.len(), file_path);
        Ok(buffer)
    }

    async fn get_file_reader(
        &self,
        pbo_path: &Path,
        file_path: &str,
    ) -> PboOperationResult<Box<dyn Read + Send>> {
        debug!("Getting file reader for '{}' from {}", file_path, pbo_path.display());
        
        // For now, we'll read the file into memory and return a cursor
        // In a more sophisticated implementation, we could return a streaming reader
        let data = self.read_file(pbo_path, file_path).await?;
        Ok(Box::new(std::io::Cursor::new(data)))
    }

    async fn file_exists(&self, pbo_path: &Path, file_path: &str) -> PboOperationResult<bool> {
        debug!("Checking if file '{}' exists in {}", file_path, pbo_path.display());
        
        if !pbo_path.exists() {
            return Err(PboOperationError::file_not_found(pbo_path));
        }

        let file = File::open(pbo_path)
            .map_err(|e| PboOperationError::io_error("opening PBO file", e))?;

        let pbo = ReadablePbo::from(file)
            .map_err(|e| PboOperationError::from(e))?;

        let exists = pbo.header(file_path).is_some();
        debug!("File '{}' exists: {}", file_path, exists);
        Ok(exists)
    }

    async fn get_file_info(
        &self,
        pbo_path: &Path,
        file_path: &str,
    ) -> PboOperationResult<Option<PboFileInfo>> {
        debug!("Getting file info for '{}' from {}", file_path, pbo_path.display());
        
        if !pbo_path.exists() {
            return Err(PboOperationError::file_not_found(pbo_path));
        }

        let file = File::open(pbo_path)
            .map_err(|e| PboOperationError::io_error("opening PBO file", e))?;

        let pbo = ReadablePbo::from(file)
            .map_err(|e| PboOperationError::from(e))?;

        if let Some(header) = pbo.header(file_path) {
            let file_info = Self::header_to_file_info(header);
            debug!("Found file info for '{}'", file_path);
            Ok(Some(file_info))
        } else {
            debug!("File '{}' not found", file_path);
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_hemtt_pbo_operations_creation() {
        let ops = HemttPboOperations::new();
        // Just test that we can create the struct
        assert!(format!("{:?}", ops).contains("HemttPboOperations"));
    }

    #[test]
    fn test_default_implementation() {
        let ops = HemttPboOperations::default();
        assert!(format!("{:?}", ops).contains("HemttPboOperations"));
    }

    // Note: Actual functionality tests will be added when the implementation is complete
}