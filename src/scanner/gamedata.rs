use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use log::{info, warn, error};
use crate::error::{Result, ToolError};
use arma3_tool_pbo_cache::ExtractionManager;
use arma3_tool_shared_models::{GameDataClasses, GameDataClass, PropertyValue};
use walkdir::WalkDir;
use gamedata_scanner::{
    GameClass, 
    ClassProperty, 
    PropertyValue as GameDataScannerPropertyValue, 
    Scanner as ClassScanner, 
    ScanResult,
    scanner_factory,
    scanner::ScannerConfig,
    scanner::ScannerResult
};
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
    /// Use advanced scanner for more accurate but slower parsing
    use_advanced_scanner: bool,
}

impl GameDataScanner {
    /// Create a new game data scanner
    pub fn new(config: arma3_tool_pbo_cache::ExtractionConfig) -> Result<Self> {
        let cache_dir = config.cache_dir.clone();
        let output_dir = cache_dir.join("scan_results");
        let extractor = ExtractionManager::new(config)
            .map_err(|e| ToolError::CacheError(format!("Failed to create PBO extractor: {}", e)))?;
            
        // Ensure output directory exists
        fs::create_dir_all(&output_dir)
            .map_err(|e| ToolError::IoError(format!("Failed to create output directory: {}", e)))?;
            
        Ok(Self { 
            cache_dir, 
            extractor, 
            output_dir,
            use_advanced_scanner: false,
        })
    }
    
    /// Set whether to use the advanced scanner
    pub fn set_advanced_scanner(&mut self, use_advanced: bool) {
        self.use_advanced_scanner = use_advanced;
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
    
    /// Convert a GameDataScannerPropertyValue to PropertyValue
    fn convert_property_value(value: &GameDataScannerPropertyValue) -> PropertyValue {
        match value {
            GameDataScannerPropertyValue::String(s) => PropertyValue::String(s.clone()),
            GameDataScannerPropertyValue::Number(n) => PropertyValue::Number(*n as f64),
            GameDataScannerPropertyValue::Array(arr) => {
                // Convert each string in the array to PropertyValue::String
                PropertyValue::Array(arr.iter().map(|s| PropertyValue::String(s.clone())).collect())
            },
            GameDataScannerPropertyValue::Class(c) => PropertyValue::String(c.name.clone()),
        }
    }
    
    /// Convert a GameClass from the scanner interface to GameDataClass
    fn convert_game_class(class: &GameClass) -> GameDataClass {
        let mut properties = HashMap::new();
        
        // Convert properties
        for prop in &class.properties {
            properties.insert(prop.name.clone(), Self::convert_property_value(&prop.value));
        }
        
        let mut game_class = GameDataClass {
            name: class.name.clone(),
            parent: class.parent.clone(),
            properties,
            source_file_index: None,
            pbo_id: None,
            line_number: None,
        };
        
        game_class
    }
    
    /// Scan classes from previously extracted PBOs
    pub async fn scan_only(&self, diagnostic_mode: bool) -> Result<GameDataClasses> {
        let gamedata_dir = self.cache_dir.join("gamedata");
        info!("Scanning extracted game data PBOs in {}", gamedata_dir.display());
        
        let use_advanced = self.use_advanced_scanner;
        
        info!("Using {} scanner for game data", if self.use_advanced_scanner { "advanced" } else { "simple" });
        
        // Get appropriate scanner based on needs
        let scanner = scanner_factory::get_scanner(use_advanced);
        
        // Create scanner configuration
        let mut scanner_config = ScannerConfig::default();
        scanner_config.diagnostic_mode = diagnostic_mode;
        
        // Scan the directory
        let scan_result = scanner.scan_directory(&gamedata_dir)
            .map_err(|e| ToolError::GameDataScanError(format!("Failed to scan game data: {}", e)))?;
            
        info!("Found {} classes in {} files", 
            scan_result.classes_found,
            scan_result.files_scanned);
            
        if scan_result.files_with_errors > 0 {
            warn!("{} files had errors during scanning", scan_result.files_with_errors);
        }
        
        // Output diagnostic report if enabled
        if diagnostic_mode && scan_result.scan_time_ms.is_some() {
            let scan_time_ms = scan_result.scan_time_ms.unwrap();
            let report = format!(
                "=== Scan Report ===\n\
                Total scan time: {:.2} seconds\n\
                Files processed: {}\n\
                Classes found: {}\n\
                Files with errors: {}\n",
                scan_time_ms as f64 / 1000.0,
                scan_result.files_scanned,
                scan_result.classes_found,
                scan_result.files_with_errors
            );
            
            info!("Scan Report:\n{}", report);
            
            // Save diagnostic report to file
            let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
            let report_file = self.output_dir.join(format!("scan_report_{}.txt", timestamp));
            fs::write(&report_file, &report)
                .map_err(|e| ToolError::IoError(format!("Failed to write scan report: {}", e)))?;
            
            info!("Scan report saved to {}", report_file.display());
        }
        
        // Create game data classes container
        let mut classes = GameDataClasses::new();
        let mut file_sources: HashMap<String, usize> = HashMap::new();
        
        // Map from PBO path to ID for consistent PBO IDs
        let mut pbo_id_map: HashMap<String, usize> = HashMap::new();
        let mut next_pbo_id = 0;
        
        // Process all classes from the scan result
        for (_, class_list) in &scan_result.class_map {
            for class in class_list {
                // Get or create file source index
                let file_path_str = class.file_path.to_string_lossy().to_string();
                
                // Get or add file source index
                let file_index = if let Some(&idx) = file_sources.get(&file_path_str) {
                    idx
                } else {
                    let idx = classes.add_file_source(class.file_path.clone());
                    file_sources.insert(file_path_str, idx);
                    idx
                };
                
                // Determine PBO ID based on the PBO path
                let pbo_path = determine_pbo_path(&class.file_path);
                let pbo_path_str = pbo_path.to_string_lossy().to_string();
                
                let pbo_id = if let Some(&id) = pbo_id_map.get(&pbo_path_str) {
                    id
                } else {
                    let id = next_pbo_id;
                    pbo_id_map.insert(pbo_path_str, id);
                    next_pbo_id += 1;
                    id
                };
                
                // Convert the class
                let mut converted_class = Self::convert_game_class(class);
                converted_class.source_file_index = Some(file_index);
                converted_class.pbo_id = Some(pbo_id);
                
                // Add the class to our collection
                classes.add_class(converted_class);
            }
        }
        
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

/// Determine the PBO path from a file path
/// This assumes that the gamedata directory structure follows the PBO structure
fn determine_pbo_path(file_path: &Path) -> PathBuf {
    let mut path = file_path.to_path_buf();
    
    // We need to go up to the PBO root directory
    if path.pop() {
        // If we're in a subdirectory, keep going up until we find the PBO root
        while path.file_name().map_or(false, |name| 
            name != "addons" && name != "gamedata" && name.to_string_lossy() != "Addons"
        ) {
            if !path.pop() {
                break;
            }
        }
    }
    
    path
} 