use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use log::{info, warn, error};
use crate::error::{Result, ToolError};
use pbo_cache::ExtractionManager;
use arma3_tool_models::{GameDataClasses, GameDataClass, PropertyValue};
use walkdir::WalkDir;
use gamedata_scanner::{Scanner, ScannerConfig, ScannerResult};
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
    fn convert_class(class: &CodeClass) -> GameDataClass {
        let mut properties = HashMap::new();
        
        // Convert properties
        for prop in &class.properties {
            properties.insert(prop.name.clone(), Self::convert_value(&prop.value));
        }
        
        GameDataClass {
            name: class.name.clone(),
            parent: class.parent.clone(),
            properties,
            source_file_index: None,
        }
    }
    
    /// Scan classes from previously extracted PBOs
    pub async fn scan_only(&self, diagnostic_mode: bool) -> Result<GameDataClasses> {
        let gamedata_dir = self.cache_dir.join("gamedata");
        info!("Scanning extracted game data PBOs in {}", gamedata_dir.display());
        
        // Create scanner config
        let config = ScannerConfig {
            extensions: vec!["cpp".to_string(), "hpp".to_string()],
            max_files: None,
            show_progress: true,
            timeout: 30, // Default timeout of 30 seconds
            diagnostic_mode,
        };
            
        // Create scanner and scan the directory
        let scanner = Scanner::new(config);
        let scan_result = scanner.scan_directory(&gamedata_dir)
            .map_err(|e| ToolError::GameDataScanError(format!("Failed to scan game data: {}", e)))?;
            
        info!("Found {} classes in {} files", 
            scan_result.results.values().map(|r| r.classes.len()).sum::<usize>(),
            scan_result.total_files);
            
        if scan_result.failed_files > 0 {
            warn!("{} files had errors during scanning", scan_result.failed_files);
        }
        
        // Output diagnostic report if enabled
        if diagnostic_mode {
            if let Some(diagnostics) = &scan_result.diagnostics {
                let report = diagnostics.generate_report();
                info!("Diagnostic Report:\n{}", report);
                
                // Save diagnostic report to file
                let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
                let report_file = self.output_dir.join(format!("diagnostic_report_{}.txt", timestamp));
                fs::write(&report_file, &report)
                    .map_err(|e| ToolError::IoError(format!("Failed to write diagnostic report: {}", e)))?;
                
                info!("Diagnostic report saved to {}", report_file.display());
            }
        }
        
        // Create file sources index
        let mut classes = GameDataClasses::new();
        let mut file_path_to_index = HashMap::new();
        
        // First pass: Create file sources index from all results
        for (file_path, _) in &scan_result.results {
            let index = classes.add_file_source(file_path.clone());
            file_path_to_index.insert(file_path.clone(), index);
        }
        
        // Create a map to track which class came from which file
        let mut class_to_file_map: HashMap<String, usize> = HashMap::new();
        
        // Second pass: Build mapping of class names to file indices
        for (file_path, file_result) in &scan_result.results {
            let file_index = file_path_to_index.get(file_path).unwrap();
            
            for class in &file_result.classes {
                // We'll use the last file occurrence for each class
                // This should be sufficient as we're mainly interested in having some source
                class_to_file_map.insert(class.name.clone(), *file_index);
            }
        }
        
        // Third pass: Process all classes
        for (file_path, file_result) in &scan_result.results {
            for class in &file_result.classes {
                let mut converted_class = Self::convert_class(class);
                
                // Look up the source file index for this class
                if let Some(&file_index) = class_to_file_map.get(&class.name) {
                    converted_class.source_file_index = Some(file_index);
                }
                
                classes.add_class(converted_class);
            }
        }
        
        // Save results
        // self.save_results(&classes)?;
        
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
    pub async fn scan(&mut self, dirs: Option<Vec<String>>, diagnostic_mode: bool) -> Result<GameDataClasses> {
        let _extracted = self.extract_only(dirs).await?;
        self.scan_only(diagnostic_mode).await
    }
} 