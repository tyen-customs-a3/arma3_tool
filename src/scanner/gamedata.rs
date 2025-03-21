use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use log::{info, warn, error};
use crate::error::{Result, ToolError};
use pbo_cache::ExtractionManager;
use arma3_tool_models::{GameDataClasses, GameDataClass, PropertyValue};
use walkdir::WalkDir;
use gamedata_scanner::{scan_directory, GameDataScannerConfig, FileResult, ScanResult};
use parser_code::{CodeClass, CodeValue};
use serde_json;
use chrono::Utc;

/// Scanner for game data PBOs
pub struct GameDataScanner {
    /// Cache directory
    cache_dir: PathBuf,
    /// PBO extractor
    extractor: ExtractionManager,
    /// Output directory for scan results
    output_dir: PathBuf,
}

impl GameDataScanner {
    /// Create a new game data scanner
    pub fn new(config: pbo_cache::ExtractionConfig) -> Result<Self> {
        let cache_dir = config.cache_dir.clone();
        let output_dir = cache_dir.join("scan_results");
        let extractor = ExtractionManager::new(config)
            .map_err(|e| ToolError::CacheError(format!("Failed to create PBO extractor: {}", e)))?;
            
        // Ensure output directory exists
        fs::create_dir_all(&output_dir)
            .map_err(|e| ToolError::IoError(format!("Failed to create output directory: {}", e)))?;
            
        Ok(Self { cache_dir, extractor, output_dir })
    }
    
    /// Extract PBOs without scanning (preparation step)
    pub async fn extract_only(&mut self, dirs: Option<Vec<String>>) -> Result<Vec<PathBuf>> {
        info!("Extracting game data PBOs...");
        
        // Process all game data PBOs
        let extraction_results = self.extractor.process_game_data(false).await
            .map_err(|e| ToolError::CacheError(format!("Failed to process game data: {}", e)))?;
            
        info!("Extracted {} game data files", extraction_results.len());
        Ok(extraction_results)
    }
    
    /// Convert a CodeValue to PropertyValue
    fn convert_value(value: &CodeValue) -> PropertyValue {
        match value {
            CodeValue::String(s) => PropertyValue::String(s.clone()),
            CodeValue::Number(n) => PropertyValue::Number(*n as f64),
            CodeValue::Array(arr) => PropertyValue::Array(arr.iter().map(|s| PropertyValue::String(s.clone())).collect()),
            CodeValue::Class(c) => PropertyValue::String(c.name.clone()),
        }
    }
    
    /// Convert a class from the scanner result to GameDataClass
    fn convert_class(class: &CodeClass, pbo_name: &Option<String>, source_file: PathBuf) -> GameDataClass {
        let mut properties = HashMap::new();
        
        // Convert properties
        for prop in &class.properties {
            properties.insert(prop.name.clone(), Self::convert_value(&prop.value));
        }
        
        GameDataClass {
            name: class.name.clone(),
            parent: class.parent.clone(),
            properties,
            mod_name: None, // TODO: Extract mod name from PBO path
            pbo_name: pbo_name.clone(),
            source_file: Some(source_file),
        }
    }
    
    /// Scan classes from previously extracted PBOs
    pub async fn scan_only(&self) -> Result<GameDataClasses> {
        let gamedata_dir = self.cache_dir.join("gamedata");
        info!("Scanning extracted game data PBOs in {}", gamedata_dir.display());
        
        // Create scanner config
        let config = GameDataScannerConfig::builder()
            .with_extensions(vec!["cpp".to_string(), "hpp".to_string()])
            .with_max_threads(num_cpus::get())
            .build();
            
        // Scan the directory
        let scan_result = scan_directory(&gamedata_dir, &config).await
            .map_err(|e| ToolError::GameDataScanError(format!("Failed to scan game data: {}", e)))?;
            
        info!("Found {} classes in {} files", scan_result.classes_found, scan_result.files_scanned);
        if scan_result.files_with_errors > 0 {
            warn!("{} files had errors during scanning", scan_result.files_with_errors);
        }
        
        // Convert scan results to GameDataClasses
        let mut classes = GameDataClasses::new();
        
        for file_result in scan_result.file_results {
            // Skip files with no classes
            if file_result.classes.is_empty() {
                continue;
            }
            
            // Try to determine PBO name from path
            let pbo_name = file_result.file_path
                .components()
                .find(|c| c.as_os_str().to_string_lossy().ends_with(".pbo"))
                .map(|c| c.as_os_str().to_string_lossy().into_owned());
                
            for class in file_result.classes {
                classes.add_class(Self::convert_class(
                    &class,
                    &pbo_name,
                    file_result.file_path.clone()
                ));
            }
        }
        
        // Save results
        self.save_results(&classes)?;
        
        Ok(classes)
    }
    
    fn save_results(&self, classes: &GameDataClasses) -> Result<()> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let output_file = self.output_dir.join(format!("gamedata_scan_{}.json", timestamp));
        
        info!("Saving scan results to {}", output_file.display());
        
        let json = serde_json::to_string_pretty(classes)
            .map_err(|e| ToolError::JsonError(format!("Failed to serialize scan results: {}", e)))?;
        fs::write(&output_file, json)
            .map_err(|e| ToolError::IoError(format!("Failed to write scan results: {}", e)))?;
        
        Ok(())
    }
    
    /// Scan game data
    pub async fn scan(&mut self, dirs: Option<Vec<String>>) -> Result<GameDataClasses> {
        let _extracted = self.extract_only(dirs).await?;
        self.scan_only().await
    }
} 