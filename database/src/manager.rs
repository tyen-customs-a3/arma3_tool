use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use log::info;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::error::Result;
use arma3_tool_models::{
    GameDataClasses, MissionData,
    ProcessingSummary, ExtractionSummary,
    ReportFormat,
};

/// Report metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportInfo {
    /// Report name
    pub name: String,
    
    /// Report path
    pub path: PathBuf,
    
    /// Report format
    pub format: ReportFormat,
    
    /// Report size in bytes
    pub size_bytes: u64,
    
    /// Creation timestamp
    pub timestamp: DateTime<Utc>,
}

/// Storage manager for handling persistence
pub struct StorageManager {
    /// Root cache directory
    cache_dir: PathBuf,
    
    /// Extraction directory
    extraction_dir: PathBuf,
    
    /// Database directory
    database_dir: PathBuf,
    
    /// Report directory
    report_dir: PathBuf,
    
    /// In-memory extraction cache
    extraction_cache: HashMap<String, Vec<PathBuf>>,
    
    /// In-memory processing cache
    processing_cache: HashMap<String, ProcessingSummary>,
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new(
        cache_dir: PathBuf,
        extraction_dir: Option<PathBuf>,
        database_dir: Option<PathBuf>,
        report_dir: Option<PathBuf>,
    ) -> Result<Self> {
        // Use provided paths or defaults
        let extraction_dir = extraction_dir.unwrap_or_else(|| cache_dir.join("extracted"));
        let database_dir = database_dir.unwrap_or_else(|| cache_dir.join("database"));
        let report_dir = report_dir.unwrap_or_else(|| cache_dir.join("reports"));
        
        // Ensure directories exist
        for dir in &[&cache_dir, &extraction_dir, &database_dir, &report_dir] {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
        }
        
        Ok(Self {
            cache_dir,
            extraction_dir,
            database_dir,
            report_dir,
            extraction_cache: HashMap::new(),
            processing_cache: HashMap::new(),
        })
    }
    
    /// Get extraction directory
    pub fn get_extraction_dir(&self) -> &Path {
        &self.extraction_dir
    }
    
    /// Get database directory
    pub fn get_database_dir(&self) -> &Path {
        &self.database_dir
    }
    
    /// Get report directory
    pub fn get_report_dir(&self) -> &Path {
        &self.report_dir
    }
    
    /// Save extraction paths to cache
    pub fn save_extraction_paths(&mut self, category: &str, paths: &[PathBuf]) -> Result<()> {
        // Save to memory cache
        self.extraction_cache.insert(category.to_string(), paths.to_vec());
        
        // Get file path
        let file_path = self.database_dir.join(format!("{}_extraction_paths.json", category));
        
        // Serialize and save
        let serialized = serde_json::to_string(paths)?;
            
        fs::write(&file_path, serialized)?;
            
        info!("Saved {} extraction paths to {}", paths.len(), file_path.display());
        Ok(())
    }
    
    /// Load extraction paths from cache
    pub fn load_extraction_paths(&mut self, category: &str) -> Result<Vec<PathBuf>> {
        // Check memory cache first
        if let Some(paths) = self.extraction_cache.get(category) {
            return Ok(paths.clone());
        }
        
        // Get file path
        let file_path = self.database_dir.join(format!("{}_extraction_paths.json", category));
        
        // Check if file exists
        if !file_path.exists() {
            return Ok(Vec::new());
        }
        
        // Read file
        let content = fs::read_to_string(&file_path)?;
            
        // Deserialize
        let paths: Vec<PathBuf> = serde_json::from_str(&content)?;
            
        // Save to memory cache
        self.extraction_cache.insert(category.to_string(), paths.clone());
        
        info!("Loaded {} extraction paths from {}", paths.len(), file_path.display());
        Ok(paths)
    }
    
    /// Save game data to storage
    pub fn save_game_data(&self, game_data: &GameDataClasses) -> Result<()> {
        // Get file path
        let file_path = self.database_dir.join("game_data.json");
        
        // Serialize and save
        let serialized = serde_json::to_string_pretty(game_data)?;
            
        fs::write(&file_path, serialized)?;
            
        info!("Saved {} game data classes to {}", game_data.classes.len(), file_path.display());
        Ok(())
    }
    
    /// Load game data from storage
    pub fn load_game_data(&self) -> Result<GameDataClasses> {
        // Get file path
        let file_path = self.database_dir.join("game_data.json");
        
        // Check if file exists
        if !file_path.exists() {
            return Ok(GameDataClasses::new());
        }
        
        // Read file
        let content = fs::read_to_string(&file_path)?;
            
        // Deserialize
        let game_data: GameDataClasses = serde_json::from_str(&content)?;
            
        info!("Loaded {} game data classes from {}", game_data.classes.len(), file_path.display());
        Ok(game_data)
    }
    
    /// Save mission data to storage
    pub fn save_mission_data(&self, mission_data: &MissionData) -> Result<()> {
        // Get file path
        let file_path = self.database_dir.join("mission_data.json");
        
        // Serialize and save
        let serialized = serde_json::to_string_pretty(mission_data)?;
            
        fs::write(&file_path, serialized)?;
            
        info!("Saved {} missions to {}", mission_data.missions.len(), file_path.display());
        Ok(())
    }
    
    /// Load mission data from storage
    pub fn load_mission_data(&self) -> Result<MissionData> {
        // Get file path
        let file_path = self.database_dir.join("mission_data.json");
        
        // Check if file exists
        if !file_path.exists() {
            return Ok(MissionData::new());
        }
        
        // Read file
        let content = fs::read_to_string(&file_path)?;
            
        // Deserialize
        let mission_data: MissionData = serde_json::from_str(&content)?;
            
        info!("Loaded {} missions from {}", mission_data.missions.len(), file_path.display());
        Ok(mission_data)
    }
    
    /// Save processing summary to storage
    pub fn save_processing_summary(&mut self, category: &str, summary: &ProcessingSummary) -> Result<()> {
        // Save to memory cache
        self.processing_cache.insert(category.to_string(), summary.clone());
        
        // Get file path
        let file_path = self.database_dir.join(format!("{}_processing_summary.json", category));
        
        // Serialize and save
        let serialized = serde_json::to_string_pretty(summary)?;
            
        fs::write(&file_path, serialized)?;
            
        info!("Saved processing summary to {}", file_path.display());
        Ok(())
    }
    
    /// Load processing summary from storage
    pub fn load_processing_summary(&mut self, category: &str) -> Result<Option<ProcessingSummary>> {
        // Check memory cache first
        if let Some(summary) = self.processing_cache.get(category) {
            return Ok(Some(summary.clone()));
        }
        
        // Get file path
        let file_path = self.database_dir.join(format!("{}_processing_summary.json", category));
        
        // Check if file exists
        if !file_path.exists() {
            return Ok(None);
        }
        
        // Read file
        let content = fs::read_to_string(&file_path)?;
            
        // Deserialize
        let summary: ProcessingSummary = serde_json::from_str(&content)?;
            
        // Save to memory cache
        self.processing_cache.insert(category.to_string(), summary.clone());
        
        info!("Loaded processing summary from {}", file_path.display());
        Ok(Some(summary))
    }
    
    /// Save extraction summary to storage
    pub fn save_extraction_summary(&self, category: &str, summary: &ExtractionSummary) -> Result<()> {
        // Get file path
        let file_path = self.database_dir.join(format!("{}_extraction_summary.json", category));
        
        // Serialize and save
        let serialized = serde_json::to_string_pretty(summary)?;
            
        fs::write(&file_path, serialized)?;
            
        info!("Saved extraction summary to {}", file_path.display());
        Ok(())
    }
    
    /// Load extraction summary from storage
    pub fn load_extraction_summary(&self, category: &str) -> Result<Option<ExtractionSummary>> {
        // Get file path
        let file_path = self.database_dir.join(format!("{}_extraction_summary.json", category));
        
        // Check if file exists
        if !file_path.exists() {
            return Ok(None);
        }
        
        // Read file
        let content = fs::read_to_string(&file_path)?;
            
        // Deserialize
        let summary: ExtractionSummary = serde_json::from_str(&content)?;
            
        info!("Loaded extraction summary from {}", file_path.display());
        Ok(Some(summary))
    }
    
    /// Save report to storage
    pub fn save_report(&self, name: &str, format: ReportFormat, content: &[u8]) -> Result<ReportInfo> {
        // Get file extension
        let extension = match format {
            ReportFormat::PlainText => "txt",
            ReportFormat::Markdown => "md",
            ReportFormat::Html => "html",
            ReportFormat::Json => "json",
            ReportFormat::Csv => "csv",
        };
        
        // Get file path
        let file_name = format!("{}_{}.{}", name, chrono::Utc::now().format("%Y%m%d_%H%M%S"), extension);
        let file_path = self.report_dir.join(&file_name);
        
        // Write file
        fs::write(&file_path, content)?;
            
        // Get file size
        let metadata = fs::metadata(&file_path)?;
            
        // Create report info
        let report_info = ReportInfo {
            name: name.to_string(),
            path: file_path.clone(),
            format,
            size_bytes: metadata.len(),
            timestamp: chrono::Utc::now(),
        };
        
        info!("Saved report to {}", file_path.display());
        Ok(report_info)
    }
    
    /// List reports in storage
    pub fn list_reports(&self) -> Result<Vec<ReportInfo>> {
        let mut reports = Vec::new();
        
        // Ensure report directory exists
        if !self.report_dir.exists() {
            return Ok(reports);
        }
        
        // List files in report directory
        for entry in fs::read_dir(&self.report_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            // Skip directories
            if !path.is_file() {
                continue;
            }
            
            // Get file name and extension
            let file_name = match path.file_stem() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            };
            
            let extension = match path.extension() {
                Some(ext) => ext.to_string_lossy().to_string(),
                None => continue,
            };
            
            // Parse name and timestamp
            let parts: Vec<&str> = file_name.split('_').collect();
            if parts.len() < 3 {
                continue;
            }
            
            // Find the timestamp part (format: YYYYMMDD_HHMMSS)
            let mut timestamp_idx = None;
            for (i, part) in parts.iter().enumerate() {
                if part.len() == 8 && part.chars().all(|c| c.is_ascii_digit()) {
                    if i + 1 < parts.len() && parts[i + 1].len() == 6 && parts[i + 1].chars().all(|c| c.is_ascii_digit()) {
                        timestamp_idx = Some(i);
                        break;
                    }
                }
            }
            
            // If we can't find the timestamp, skip this file
            let timestamp_idx = match timestamp_idx {
                Some(idx) => idx,
                None => continue,
            };
            
            // The name is everything before the timestamp
            let name = parts[..timestamp_idx].join("_");
            
            // Parse format
            let format = match extension.as_str() {
                "txt" => ReportFormat::PlainText,
                "md" => ReportFormat::Markdown,
                "html" => ReportFormat::Html,
                "json" => ReportFormat::Json,
                "csv" => ReportFormat::Csv,
                _ => continue,
            };
            
            // Get file size
            let metadata = match fs::metadata(&path) {
                Ok(meta) => meta,
                Err(_) => continue,
            };
            
            // Create report info
            let report_info = ReportInfo {
                name,
                path: path.clone(),
                format,
                size_bytes: metadata.len(),
                timestamp: chrono::Utc::now(), // TODO: Parse timestamp from filename
            };
            
            reports.push(report_info);
        }
        
        // Sort by timestamp (newest first)
        reports.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        info!("Found {} reports in {}", reports.len(), self.report_dir.display());
        Ok(reports)
    }
} 