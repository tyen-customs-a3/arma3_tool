use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};

/// PBO file type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PboType {
    /// Game data PBO (addons, etc.)
    GameData,
    
    /// Mission PBO
    Mission,
}

impl std::fmt::Display for PboType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PboType::GameData => write!(f, "GameData"),
            PboType::Mission => write!(f, "Mission"),
        }
    }
}

impl From<&str> for PboType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "gamedata" => PboType::GameData,
            "mission" => PboType::Mission,
            _ => PboType::GameData, // Default to GameData
        }
    }
}

/// Model representing a PBO file in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PboModel {
    /// Normalized path to PBO (primary identifier)
    pub id: String,
    
    /// Full path to the PBO file
    pub full_path: PathBuf,
    
    /// Base directory (for relative paths)
    pub base_dir: Option<PathBuf>,
    
    /// File size in bytes
    pub file_size: u64,
    
    /// Last modified timestamp
    pub last_modified: DateTime<Utc>,
    
    /// When it was last extracted
    pub extraction_time: DateTime<Utc>,
    
    /// Type of PBO (GameData or Mission)
    pub pbo_type: PboType,
}

impl PboModel {
    /// Create a new PBO model with a full path
    pub fn new(
        path: impl AsRef<Path>,
        pbo_type: PboType,
        file_size: u64,
        last_modified: DateTime<Utc>,
        extraction_time: DateTime<Utc>,
    ) -> Self {
        let path_ref = path.as_ref();
        
        Self {
            id: normalize_path(path_ref),
            full_path: path_ref.to_path_buf(),
            base_dir: None,
            file_size,
            last_modified,
            extraction_time,
            pbo_type,
        }
    }
    
    /// Create a new PBO model with a relative path
    pub fn new_with_base_dir(
        path: impl AsRef<Path>,
        base_dir: impl AsRef<Path>,
        pbo_type: PboType,
        file_size: u64,
        last_modified: DateTime<Utc>,
        extraction_time: DateTime<Utc>,
    ) -> Self {
        let path_ref = path.as_ref();
        let base_dir_ref = base_dir.as_ref();
        
        Self {
            id: normalize_path(path_ref),
            full_path: path_ref.to_path_buf(),
            base_dir: Some(base_dir_ref.to_path_buf()),
            file_size,
            last_modified,
            extraction_time,
            pbo_type,
        }
    }
    
    /// Convert from a PboCache PboMetadata
    pub fn from_pbo_metadata(metadata: &impl PboMetadataConversion) -> Self {
        Self {
            id: normalize_path(&metadata.get_path()),
            full_path: metadata.get_full_path(),
            base_dir: metadata.get_base_dir().map(ToOwned::to_owned),
            file_size: metadata.get_file_size(),
            last_modified: metadata.get_last_modified(),
            extraction_time: metadata.get_extraction_time(),
            pbo_type: match metadata.get_pbo_type() {
                "GameData" => PboType::GameData,
                "Mission" => PboType::Mission,
                _ => PboType::GameData,
            },
        }
    }
    
    /// Get the full path
    pub fn get_full_path(&self) -> PathBuf {
        if self.full_path.is_absolute() {
            self.full_path.clone()
        } else if let Some(base_dir) = &self.base_dir {
            base_dir.join(&self.full_path)
        } else {
            self.full_path.clone()
        }
    }
    
    /// Verify if the model is valid for database operations
    pub fn is_valid(&self) -> bool {
        // Check that we have a non-empty ID
        if self.id.trim().is_empty() {
            return false;
        }
        
        // Check that we have a valid path
        if self.full_path.as_os_str().is_empty() {
            return false;
        }
        
        // If we have a base_dir, make sure it's valid
        if let Some(base_dir) = &self.base_dir {
            if base_dir.as_os_str().is_empty() {
                return false;
            }
        }
        
        // All checks passed
        true
    }
}

/// Trait for converting from PBO metadata
pub trait PboMetadataConversion {
    fn get_path(&self) -> PathBuf;
    fn get_full_path(&self) -> PathBuf;
    fn get_base_dir(&self) -> Option<&PathBuf>;
    fn get_file_size(&self) -> u64;
    fn get_last_modified(&self) -> DateTime<Utc>;
    fn get_extraction_time(&self) -> DateTime<Utc>;
    fn get_pbo_type(&self) -> &str;
}

/// Model for extracted files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedFile {
    /// Unique identifier
    pub id: Option<i64>,
    
    /// Reference to source PBO
    pub pbo_id: String,
    
    /// Path relative to cache directory
    pub relative_path: PathBuf,
    
    /// File extension (cached for faster filtering)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    
    /// File name without path (cached for faster lookups)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
}

impl ExtractedFile {
    /// Create a new extracted file
    pub fn new(pbo_id: impl Into<String>, relative_path: impl AsRef<Path>) -> Self {
        let path_ref = relative_path.as_ref();
        let extension = path_ref.extension().map(|e| e.to_string_lossy().to_string());
        let file_name = path_ref.file_name().map(|f| f.to_string_lossy().to_string());
        
        Self {
            id: None,
            pbo_id: pbo_id.into(),
            relative_path: path_ref.to_path_buf(),
            extension,
            file_name,
        }
    }
    
    /// Get the full path by combining with a cache directory
    pub fn get_full_path(&self, cache_dir: impl AsRef<Path>) -> PathBuf {
        cache_dir.as_ref().join(&self.relative_path)
    }
    
    /// Get the file extension (calculating it if not cached)
    pub fn get_extension(&self) -> Option<String> {
        self.extension.clone().or_else(|| {
            self.relative_path.extension().map(|e| e.to_string_lossy().to_string())
        })
    }
    
    /// Get the file name without path (calculating it if not cached)
    pub fn get_file_name(&self) -> Option<String> {
        self.file_name.clone().or_else(|| {
            self.relative_path.file_name().map(|f| f.to_string_lossy().to_string())
        })
    }
}

/// Model for PBO extraction failure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedExtraction {
    /// PBO identifier
    pub pbo_id: String,
    
    /// When it failed
    pub timestamp: DateTime<Utc>,
    
    /// Error message
    pub error_message: String,
}

impl FailedExtraction {
    /// Create a new failed extraction record
    pub fn new(pbo_id: impl Into<String>, error_message: impl Into<String>) -> Self {
        Self {
            pbo_id: pbo_id.into(),
            timestamp: Utc::now(),
            error_message: error_message.into(),
        }
    }
}

/// Normalize a path for consistent database keys
pub fn normalize_path(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .to_string_lossy()
        .to_string()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path(Path::new("C:\\folder\\file.pbo")), "C:/folder/file.pbo");
        assert_eq!(normalize_path("/home/user/file.pbo"), "/home/user/file.pbo");
    }
    
    #[test]
    fn test_pbo_model() {
        let now = Utc::now();
        
        let pbo = PboModel::new(
            "test.pbo",
            PboType::GameData,
            1234,
            now,
            now,
        );
        
        assert_eq!(pbo.id, "test.pbo");
        assert_eq!(pbo.full_path, PathBuf::from("test.pbo"));
        assert_eq!(pbo.file_size, 1234);
        assert_eq!(pbo.pbo_type, PboType::GameData);
        
        let pbo_with_base = PboModel::new_with_base_dir(
            "addon/test.pbo",
            "/game/path",
            PboType::Mission,
            5678,
            now,
            now,
        );
        
        assert_eq!(pbo_with_base.id, "addon/test.pbo");
        assert!(pbo_with_base.base_dir.is_some());
        assert_eq!(pbo_with_base.pbo_type, PboType::Mission);
    }
    
    #[test]
    fn test_pbo_type_conversion() {
        assert_eq!(PboType::from("GameData"), PboType::GameData);
        assert_eq!(PboType::from("gamedata"), PboType::GameData);
        assert_eq!(PboType::from("Mission"), PboType::Mission);
        assert_eq!(PboType::from("mission"), PboType::Mission);
        assert_eq!(PboType::from("unknown"), PboType::GameData); // Default
        
        assert_eq!(PboType::GameData.to_string(), "GameData");
        assert_eq!(PboType::Mission.to_string(), "Mission");
    }
} 