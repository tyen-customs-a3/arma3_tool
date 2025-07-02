use anyhow::Result;
use std::path::Path;

/// Detects and chooses optimal serialization format
pub struct FormatDetector;

impl FormatDetector {
    /// Determine if binary format should be used
    pub fn should_use_binary(path: &Path) -> Result<bool> {
        // Check if existing file is binary format
        if path.exists() {
            return Ok(Self::is_binary_format(path)?);
        }
        
        // For new files, prefer binary format by default
        Ok(true)
    }

    /// Check if existing file uses binary format
    fn is_binary_format(path: &Path) -> Result<bool> {
        let data = std::fs::read(path)?;
        
        // Check for LZ4 magic bytes or other binary indicators
        if data.starts_with(&[0x04, 0x22, 0x4d, 0x18]) { // LZ4 magic
            return Ok(true);
        }
        
        // Check if content is valid UTF-8 (likely JSON)
        if std::str::from_utf8(&data).is_ok() {
            return Ok(false);
        }
        
        // If we can't determine, assume binary
        Ok(true)
    }

    /// Get format name from file
    pub fn detect_format(path: &Path) -> Result<String> {
        if !path.exists() {
            return Ok("unknown".to_string());
        }
        
        if Self::is_binary_format(path)? {
            Ok("binary".to_string())
        } else {
            Ok("json".to_string())
        }
    }

    /// Estimate compression ratio for binary vs JSON
    pub fn estimate_compression_ratio(data_size: usize) -> f64 {
        // Binary with LZ4 typically achieves 3-5x compression for config data
        // This is a rough estimate
        match data_size {
            0..=1024 => 2.0,        // Small files: modest compression
            1025..=10240 => 3.5,    // Medium files: good compression
            _ => 4.5,               // Large files: excellent compression
        }
    }

    /// Check if file should be migrated to binary format
    pub fn should_migrate_to_binary(path: &Path) -> Result<bool> {
        if !path.exists() {
            return Ok(false);
        }
        
        // Don't migrate if already binary
        if Self::is_binary_format(path)? {
            return Ok(false);
        }
        
        // Migrate large JSON files to binary for better performance
        let metadata = std::fs::metadata(path)?;
        Ok(metadata.len() > 50 * 1024) // > 50KB
    }
}
