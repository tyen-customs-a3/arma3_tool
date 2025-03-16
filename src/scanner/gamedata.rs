use std::path::PathBuf;
use log::{info, warn};
use crate::error::{Result, ToolError};
use crate::scanner::models::{GameDataClasses, GameDataClassDefinition};
use pbo_cache::ExtractionManager;

/// Scanner for game data PBOs
pub struct GameDataScanner {
    /// Cache directory
    cache_dir: PathBuf,
    /// PBO extractor
    extractor: ExtractionManager,
}

impl GameDataScanner {
    /// Create a new game data scanner
    pub fn new(config: pbo_cache::ExtractionConfig) -> Result<Self> {
        let cache_dir = config.cache_dir.clone();
        let extractor = ExtractionManager::new(config)
            .map_err(|e| ToolError::CacheError(format!("Failed to create PBO extractor: {}", e)))?;
            
        Ok(Self { cache_dir, extractor })
    }
    
    /// Extract PBOs without scanning (preparation step)
    pub async fn extract_only(&mut self, dirs: Option<Vec<String>>) -> Result<Vec<PathBuf>> {
        info!("Extracting game data PBOs...");
        
        // Process game data PBOs
        let extracted = self.extractor.process_game_data(false).await
            .map_err(|e| ToolError::CacheError(format!("Failed to extract game data PBOs: {}", e)))?;
            
        info!("Extracted {} game data files", extracted.len());
        Ok(extracted)
    }
    
    /// Scan classes from previously extracted PBOs
    pub fn scan_only(&mut self, _dirs: Option<Vec<String>>) -> Result<GameDataClasses> {
        info!("Scanning extracted PBOs in {}", self.cache_dir.display());
        
        // For now return dummy data - we'll implement actual scanning later
        Ok(GameDataClasses {
            classes: vec![
                GameDataClassDefinition {
                    name: "DummyClass".to_string(),
                    parent: Some("BaseClass".to_string()),
                }
            ]
        })
    }
    
    /// Scan game data
    pub async fn scan(&mut self, dirs: Option<Vec<String>>) -> Result<GameDataClasses> {
        self.extract_only(dirs).await?;
        self.scan_only(None)
    }
} 