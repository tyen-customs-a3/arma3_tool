use anyhow::{Result, Context};
use crate::models::ScanResult;

/// Trait for different serialization formats
pub trait Serializer: Send + Sync {
    fn serialize(&self, result: &ScanResult) -> Result<Vec<u8>>;
    fn deserialize(&self, data: &[u8]) -> Result<ScanResult>;
    fn format_name(&self) -> &'static str;
}

/// Binary serializer using bincode with LZ4 compression
pub struct BinarySerializer;

impl BinarySerializer {
    pub fn new() -> Self {
        Self
    }
}

impl Serializer for BinarySerializer {
    fn serialize(&self, result: &ScanResult) -> Result<Vec<u8>> {
        let serialized = bincode::serde::encode_to_vec(result, bincode::config::standard())
            .context("Failed to serialize with bincode")?;
        
        // FIX: Use compress_prepend_size to match decompress_size_prepended
        let compressed = lz4_flex::compress_prepend_size(&serialized);
        Ok(compressed)
    }

    fn deserialize(&self, data: &[u8]) -> Result<ScanResult> {
        let decompressed = lz4_flex::decompress_size_prepended(data)
            .context("Failed to decompress LZ4 data")?;
        
        let (result, _) = bincode::serde::decode_from_slice(&decompressed, bincode::config::standard())
            .context("Failed to deserialize with bincode")?;
        
        Ok(result)
    }

    fn format_name(&self) -> &'static str {
        "binary"
    }
}

/// JSON serializer for compatibility and debugging
pub struct JsonSerializer;

impl JsonSerializer {
    pub fn new() -> Self {
        Self
    }
}

impl Serializer for JsonSerializer {
    fn serialize(&self, result: &ScanResult) -> Result<Vec<u8>> {
        let json = serde_json::to_string_pretty(result)
            .context("Failed to serialize to JSON")?;
        Ok(json.into_bytes())
    }

    fn deserialize(&self, data: &[u8]) -> Result<ScanResult> {
        let json = std::str::from_utf8(data)
            .context("Invalid UTF-8 in JSON data")?;
        let result = serde_json::from_str(json)
            .context("Failed to deserialize from JSON")?;
        Ok(result)
    }

    fn format_name(&self) -> &'static str {
        "json"
    }
}

/// Serializer that tries binary first, falls back to JSON
pub struct HybridSerializer {
    binary: BinarySerializer,
    json: JsonSerializer,
}

impl HybridSerializer {
    pub fn new() -> Self {
        Self {
            binary: BinarySerializer::new(),
            json: JsonSerializer::new(),
        }
    }
}

impl Serializer for HybridSerializer {
    fn serialize(&self, result: &ScanResult) -> Result<Vec<u8>> {
        // Try binary first for better performance
        match self.binary.serialize(result) {
            Ok(data) => Ok(data),
            Err(_) => {
                log::warn!("Binary serialization failed, falling back to JSON");
                self.json.serialize(result)
            }
        }
    }

    fn deserialize(&self, data: &[u8]) -> Result<ScanResult> {
        // Try binary first
        match self.binary.deserialize(data) {
            Ok(result) => Ok(result),
            Err(_) => {
                log::debug!("Binary deserialization failed, trying JSON");
                self.json.deserialize(data)
            }
        }
    }

    fn format_name(&self) -> &'static str {
        "hybrid"
    }
}
