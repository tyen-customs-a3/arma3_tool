use arma3_pbo::{PboApi, PboApiOps, PboOperationError};
use std::path::{Path, PathBuf};
use async_trait::async_trait;
use log::{debug, warn, info};
use walkdir::WalkDir;
use std::fs;

/// Trait for abstracting PBO extraction operations
#[async_trait]
pub trait PboExtractor: Send + Sync {
    /// Extract files matching a filter pattern from a PBO
    async fn extract_filtered(
        &self,
        pbo_path: &Path,
        output_dir: &Path,
        filter: &str,
    ) -> Result<Vec<PathBuf>, PboOperationError>;
    
    /// List all files that were extracted to a directory
    async fn list_extracted_files(
        &self,
        output_dir: &Path,
    ) -> Result<Vec<PathBuf>, PboOperationError>;
    
    /// Convert binary config files to text format
    /// Returns a list of (original_path, converted_path) tuples for successful conversions
    async fn convert_binary_configs(
        &self,
        extracted_files: &[PathBuf],
        output_dir: &Path,
    ) -> Result<Vec<(PathBuf, PathBuf)>, PboOperationError>;
}

/// HEMTT-based PBO extractor implementation
pub struct HemttPboExtractor {
    pbo_api: PboApi,
}

impl HemttPboExtractor {
    /// Create a new HEMTT PBO extractor with the specified timeout
    pub fn new(timeout_seconds: u64) -> Self {
        Self {
            pbo_api: PboApi::new(timeout_seconds as u32),
        }
    }
    
    /// Try to convert a binary config file to text format
    /// Returns Ok(true) if conversion succeeded, Ok(false) if no converter available
    async fn try_convert_binary_config(
        &self,
        binary_path: &Path,
        output_path: &Path,
    ) -> Result<bool, PboOperationError> {
        // Check if the binary file exists
        if !binary_path.exists() {
            return Err(PboOperationError::file_not_found(binary_path));
        }
        
        // Check if we can use hemtt-config to debinarize
        // For now, we'll check if the file is actually a binary config by reading the header
        match fs::read(binary_path) {
            Ok(data) => {
                // Check for raP signature (binary config marker)
                if data.len() >= 4 && &data[0..4] == b"\0raP" {
                    debug!("Detected binary config with raP signature");
                    
                    // TODO: When HEMTT provides debinarize functionality, use it here
                    // For now, we'll create a placeholder file indicating conversion is needed
                    let placeholder_content = format!(
                        "// This file was detected as a binary config (config.bin)\n\
                         // Binary conversion is not yet implemented\n\
                         // Original file: {}\n",
                        binary_path.file_name().unwrap_or_default().to_string_lossy()
                    );
                    
                    // Create parent directories if needed
                    if let Some(parent) = output_path.parent() {
                        fs::create_dir_all(parent)
                            .map_err(|e| PboOperationError::io_error("creating output directory", e))?;
                    }
                    
                    // Write the placeholder file
                    fs::write(output_path, placeholder_content)
                        .map_err(|e| PboOperationError::io_error("writing placeholder file", e))?;
                    
                    // Return false to indicate we didn't actually convert (just created placeholder)
                    // When real conversion is available, return true
                    Ok(false)
                } else {
                    // Not a binary config, might be a text file already
                    debug!("File does not have binary config signature, might be text already");
                    Ok(false)
                }
            }
            Err(e) => {
                Err(PboOperationError::io_error("reading binary config", e))
            }
        }
    }
}

#[async_trait]
impl PboExtractor for HemttPboExtractor {
    async fn extract_filtered(
        &self,
        pbo_path: &Path,
        output_dir: &Path,
        filter: &str,
    ) -> Result<Vec<PathBuf>, PboOperationError> {
        debug!("Extracting PBO {} with filter '{}' to {}", 
               pbo_path.display(), filter, output_dir.display());
        
        // Check if this is a mission PBO (ends with .MapName.pbo)
        let is_mission_pbo = if let Some(file_name) = pbo_path.file_name() {
            let name = file_name.to_string_lossy();
            // Mission PBOs have format: missionName.MapName.pbo
            name.matches('.').count() >= 2 && name.ends_with(".pbo")
        } else {
            false
        };
        
        // For mission PBOs, use the filename (without .pbo) as the subdirectory
        // For mod PBOs, use the internal prefix
        let (extract_base_dir, dir_prefix) = if is_mission_pbo {
            // Extract mission name without .pbo extension
            let mission_name = pbo_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown_mission");
            let mission_dir = output_dir.join(mission_name);
            fs::create_dir_all(&mission_dir)
                .map_err(|e| PboOperationError::io_error("creating mission directory", e))?;
            (mission_dir, Some(mission_name.to_string()))
        } else {
            // For mods, get the PBO properties to check for a prefix
            let properties = self.pbo_api.get_properties(pbo_path).await?;
            let prefix = properties.prefix.clone()
                .map(|p| p.replace('\\', "/")); // Normalize to forward slashes
            (output_dir.to_path_buf(), prefix)
        };
        
        // Create a temporary directory for extraction
        let temp_extract_dir = output_dir.join(".temp_extract");
        fs::create_dir_all(&temp_extract_dir)
            .map_err(|e| PboOperationError::io_error("creating temp directory", e))?;
        
        // Extract files to the temporary directory
        self.pbo_api.extract_filtered(pbo_path, &temp_extract_dir, filter).await?;
        
        // Now move files to the correct location
        let mut extracted_files = Vec::new();
        
        for entry in WalkDir::new(&temp_extract_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(rel_path) = entry.path().strip_prefix(&temp_extract_dir) {
                    // For missions, we already have the mission directory as base
                    // For mods, we need to add the prefix if it exists
                    let final_path = if is_mission_pbo {
                        extract_base_dir.join(rel_path)
                    } else if let Some(ref prefix_str) = dir_prefix {
                        if !prefix_str.is_empty() {
                            output_dir.join(prefix_str).join(rel_path)
                        } else {
                            output_dir.join(rel_path)
                        }
                    } else {
                        output_dir.join(rel_path)
                    };
                    
                    // Create parent directories
                    if let Some(parent) = final_path.parent() {
                        fs::create_dir_all(parent)
                            .map_err(|e| PboOperationError::io_error("creating parent directory", e))?;
                    }
                    
                    // Move the file
                    fs::rename(entry.path(), &final_path)
                        .or_else(|_| {
                            // If rename fails (e.g., across filesystems), try copy and delete
                            fs::copy(entry.path(), &final_path)?;
                            fs::remove_file(entry.path())?;
                            Ok(())
                        })
                        .map_err(|e| PboOperationError::io_error("moving file", e))?;
                    
                    // Calculate the relative path from the output_dir
                    if let Ok(final_rel_path) = final_path.strip_prefix(output_dir) {
                        extracted_files.push(final_rel_path.to_path_buf());
                    } else {
                        warn!("Failed to calculate relative path for {}", final_path.display());
                    }
                }
            }
        }
        
        // Clean up temp directory
        let _ = fs::remove_dir_all(&temp_extract_dir);
        
        if let Some(ref prefix_str) = dir_prefix {
            if !prefix_str.is_empty() {
                if is_mission_pbo {
                    info!("Extracted {} files from mission PBO to '{}'", extracted_files.len(), prefix_str);
                } else {
                    info!("Extracted {} files from mod PBO with prefix '{}'", extracted_files.len(), prefix_str);
                }
            }
        }
        
        Ok(extracted_files)
    }
    
    async fn list_extracted_files(
        &self,
        output_dir: &Path,
    ) -> Result<Vec<PathBuf>, PboOperationError> {
        let mut extracted_files = Vec::new();
        
        // Walk the output directory to find all extracted files
        for entry in WalkDir::new(output_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                // Get the relative path from the output directory
                if let Ok(rel_path) = entry.path().strip_prefix(output_dir) {
                    extracted_files.push(rel_path.to_path_buf());
                } else {
                    warn!("Failed to get relative path for {}", entry.path().display());
                }
            }
        }
        
        debug!("Found {} extracted files in {}", 
               extracted_files.len(), output_dir.display());
        Ok(extracted_files)
    }
    
    async fn convert_binary_configs(
        &self,
        extracted_files: &[PathBuf],
        output_dir: &Path,
    ) -> Result<Vec<(PathBuf, PathBuf)>, PboOperationError> {
        let mut conversions = Vec::new();
        
        for rel_path in extracted_files {
            let full_path = output_dir.join(rel_path);
            
            // Check if this is a binary config file
            if let Some(filename) = rel_path.file_name() {
                if is_binary_config(&filename.to_string_lossy()) {
                    debug!("Found binary config file: {}", rel_path.display());
                    
                    // Get the converted filename
                    let converted_path = get_converted_config_name(rel_path);
                    let full_converted_path = output_dir.join(&converted_path);
                    
                    // Try to convert the binary config
                    match self.try_convert_binary_config(&full_path, &full_converted_path).await {
                        Ok(true) => {
                            info!("Successfully converted {} to {}", 
                                  rel_path.display(), converted_path.display());
                            conversions.push((rel_path.clone(), converted_path));
                            
                            // Remove the original binary file after successful conversion
                            if let Err(e) = fs::remove_file(&full_path) {
                                warn!("Failed to remove original binary file {}: {}", 
                                      full_path.display(), e);
                            }
                        }
                        Ok(false) => {
                            // Conversion not available, keep the original file
                            debug!("Binary conversion not available for {}", rel_path.display());
                        }
                        Err(e) => {
                            warn!("Failed to convert binary config {}: {}", 
                                  rel_path.display(), e);
                            // Keep the original file on error
                        }
                    }
                }
            }
        }
        
        if !conversions.is_empty() {
            info!("Converted {} binary config files", conversions.len());
        }
        
        Ok(conversions)
    }
}

/// Check if a filename should be treated as a binary config that needs conversion
pub fn is_binary_config(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower == "config.bin" || lower.ends_with("/config.bin") || lower.ends_with("\\config.bin")
}

/// Get the converted filename for a binary config (config.bin -> config.cpp)
pub fn get_converted_config_name(path: &Path) -> PathBuf {
    let mut new_path = path.to_path_buf();
    new_path.set_extension("cpp");
    new_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_binary_config_detection() {
        assert!(is_binary_config("config.bin"));
        assert!(is_binary_config("Config.BIN"));
        assert!(is_binary_config("some/path/config.bin"));
        assert!(is_binary_config("some\\path\\config.bin"));
        assert!(!is_binary_config("config.cpp"));
        assert!(!is_binary_config("not_config.bin"));
    }

    #[test]
    fn test_converted_config_name() {
        let path = Path::new("some/path/config.bin");
        let converted = get_converted_config_name(path);
        assert_eq!(converted, Path::new("some/path/config.cpp"));
    }
    
    #[tokio::test]
    async fn test_binary_config_conversion() {
        let temp_dir = TempDir::new().unwrap();
        let extractor = HemttPboExtractor::new(30);
        
        // Create a mock binary config file with raP signature
        let config_bin_path = temp_dir.path().join("config.bin");
        let binary_data = b"\0raP\x00\x00\x00\x00mock binary config data";
        fs::write(&config_bin_path, binary_data).unwrap();
        
        // Test conversion
        let extracted_files = vec![PathBuf::from("config.bin")];
        let result = extractor.convert_binary_configs(&extracted_files, temp_dir.path()).await;
        
        assert!(result.is_ok());
        let conversions = result.unwrap();
        
        // Since we don't have actual debinarize, it should create a placeholder
        // but return Ok(false) from try_convert_binary_config, so no conversions
        assert_eq!(conversions.len(), 0);
        
        // The original file should still exist
        assert!(config_bin_path.exists());
    }
    
    #[tokio::test]
    async fn test_non_binary_file_not_converted() {
        let temp_dir = TempDir::new().unwrap();
        let extractor = HemttPboExtractor::new(30);
        
        // Create a regular text file named config.bin
        let config_bin_path = temp_dir.path().join("config.bin");
        fs::write(&config_bin_path, "class CfgPatches {};").unwrap();
        
        // Test conversion
        let extracted_files = vec![PathBuf::from("config.bin")];
        let result = extractor.convert_binary_configs(&extracted_files, temp_dir.path()).await;
        
        assert!(result.is_ok());
        let conversions = result.unwrap();
        assert_eq!(conversions.len(), 0);
        
        // The original file should still exist
        assert!(config_bin_path.exists());
    }
}