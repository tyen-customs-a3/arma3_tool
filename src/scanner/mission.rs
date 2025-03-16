use std::path::PathBuf;
use log::{info, warn};
use crate::error::{Result, ToolError};
use pbo_cache::ExtractionManager;
use arma3_tool_models::{MissionData, Mission};

/// Scanner for mission PBOs
pub struct MissionScanner {
    /// Cache directory
    cache_dir: PathBuf,
    /// PBO extractor
    extractor: ExtractionManager,
}

impl MissionScanner {
    /// Create a new mission scanner
    pub fn new(config: pbo_cache::ExtractionConfig) -> Result<Self> {
        let cache_dir = config.cache_dir.clone();
        let extractor = ExtractionManager::new(config)
            .map_err(|e| ToolError::CacheError(format!("Failed to create PBO extractor: {}", e)))?;
            
        Ok(Self { cache_dir, extractor })
    }
    
    /// Extract PBOs without scanning (preparation step)
    pub async fn extract_only(&mut self, dirs: Option<Vec<String>>) -> Result<Vec<PathBuf>> {
        info!("Extracting mission PBOs...");
        
        // Process all mission PBOs
        let extraction_results = self.extractor.process_all_missions(false).await
            .map_err(|e| ToolError::CacheError(format!("Failed to extract mission PBOs: {}", e)))?;
            
        // Flatten results into a single vector of paths
        let mut extracted = Vec::new();
        for (_, paths) in extraction_results {
            extracted.extend(paths);
        }
            
        info!("Extracted {} mission files", extracted.len());
        Ok(extracted)
    }
    
    /// Scan missions from previously extracted PBOs
    pub fn scan_only(&mut self, _dirs: Option<Vec<String>>) -> Result<MissionData> {
        info!("Scanning extracted mission PBOs in {}", self.cache_dir.display());
        
        // For now return dummy data - we'll implement actual scanning later
        let mut data = MissionData::new();
        data.add_mission(Mission::new(
            "DummyMission".to_string(),
            PathBuf::from("dummy/path")
        ));
        
        Ok(data)
    }
    
    /// Scan missions
    pub async fn scan(&mut self, dirs: Option<Vec<String>>) -> Result<MissionData> {
        self.extract_only(dirs).await?;
        self.scan_only(None)
    }
} 