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
    Scanner,
    ScannerConfig,
    ScannerResult
};
use gamedata_scanner::scanner::FileScanResult;
use serde_json;
use chrono::Utc;
use std::collections::HashSet;
use arma3_db::{DatabaseManager, repos::ClassRepository};

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
    /// Database manager, if database storage is enabled
    db_manager: Option<DatabaseManager>,
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
            use_advanced_scanner: true,
            db_manager: None,
        })
    }
    
    /// Create a new game data scanner with database support
    pub fn with_database(config: arma3_tool_pbo_cache::ExtractionConfig, db_path: PathBuf) -> Result<Self> {
        let cache_dir = config.cache_dir.clone();
        let output_dir = cache_dir.join("scan_results");
        let extractor = ExtractionManager::new(config.clone())
            .map_err(|e| ToolError::CacheError(format!("Failed to create PBO extractor: {}", e)))?;
            
        // Ensure output directory exists
        fs::create_dir_all(&output_dir)
            .map_err(|e| ToolError::IoError(format!("Failed to create output directory: {}", e)))?;
        
        // Create database config and manager
        let db_config = arma3_db::models::DatabaseConfig::new(db_path, cache_dir.clone());
        let db_manager = DatabaseManager::with_config(db_config)
            .map_err(|e| ToolError::DatabaseError(format!("Failed to create database manager: {}", e)))?;
            
        Ok(Self { 
            cache_dir, 
            extractor, 
            output_dir,
            use_advanced_scanner: true,
            db_manager: Some(db_manager),
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
                PropertyValue::Array(arr.iter().map(|s| PropertyValue::String(s.clone())).collect())
            },
            GameDataScannerPropertyValue::Class(c) => PropertyValue::String(c.name.clone()),
        }
    }
    
    /// Convert a GameClass from the scanner interface to GameDataClass
    fn convert_game_class(class: &GameClass, file_index: Option<usize>) -> Option<GameDataClass> {
        let mut properties = HashMap::new();
        for prop in &class.properties {
            properties.insert(prop.name.clone(), Self::convert_property_value(&prop.value));
        }
        
        let mut game_class = GameDataClass {
            name: class.name.clone(),
            parent: class.parent.clone(),
            container_class: class.container_class.clone(),
            properties,
            source_file_index: file_index,
        };
        
        Some(game_class)
    }
    
    /// Scan classes from previously extracted PBOs
    pub async fn scan_only(&self, diagnostic_mode: bool) -> Result<GameDataClasses> {
        let gamedata_dir = self.cache_dir.join("gamedata");
        info!("Scanning extracted game data PBOs in {}", gamedata_dir.display());
        
        info!("Using {} scanner for game data", if self.use_advanced_scanner { "advanced" } else { "simple" });
        
        // Create scanner configuration
        let mut scanner_config = ScannerConfig::default();
        scanner_config.diagnostic_mode = diagnostic_mode;
        
        // Create scanner with config
        let scanner = Scanner::new(scanner_config);
        
        // Scan the directory
        let scan_result = scanner.scan_directory(&gamedata_dir)
            .map_err(|e| ToolError::GameDataScanError(format!("Failed to scan game data: {}", e)))?;

        info!("Found {} classes in {} files", 
            scan_result.total_files,
            scan_result.successful_files);
            
        if scan_result.failed_files > 0 {
            warn!("{} files had errors during scanning", scan_result.failed_files);
        }
        
        // Output diagnostic report if enabled
        if diagnostic_mode {
            let report = if let Some(diagnostics) = scan_result.diagnostics {
                format!(
                    "=== Scan Report ===\n\
                    Total scan time: {:.2} seconds\n\
                    Files processed: {}\n\
                    Successful files: {}\n\
                    Failed files: {}\n\
                    Timeout count: {}\n",
                    diagnostics.total_scan_time_ms as f64 / 1000.0,
                    scan_result.total_files,
                    scan_result.successful_files,
                    scan_result.failed_files,
                    diagnostics.timeout_count
                )
            } else {
                format!(
                    "=== Scan Report ===\n\
                    Files processed: {}\n\
                    Successful files: {}\n\
                    Failed files: {}\n",
                    scan_result.total_files,
                    scan_result.successful_files,
                    scan_result.failed_files
                )
            };
            
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
        
        // Process all classes from the scan result
        for (file_path, scan_result) in scan_result.results {
            // Add file source
            let file_index = classes.add_file_source(file_path);
            
            // Convert and add each class
            for class in scan_result.classes {
                if let Some(converted_class) = Self::convert_game_class(&class, Some(file_index)) {
                    classes.add_class(converted_class);
                }
            }
        }
        
        Ok(classes)
    }
    
    /// Scan game data
    pub async fn scan(&mut self, dirs: Option<Vec<String>>, diagnostic_mode: bool) -> Result<GameDataClasses> {
        let _extracted = self.extract_only(dirs).await?;
        self.scan_only(diagnostic_mode).await
    }
    
    /// Save game data to database
    pub fn save_to_database(&self, game_data: &GameDataClasses) -> Result<()> {
        // Check if database is available
        let db_manager = match &self.db_manager {
            Some(manager) => manager,
            None => return Err(ToolError::DatabaseError("Database not initialized".to_string())),
        };
        
        info!("Saving game data to database ({} classes)...", game_data.classes.len());
        
        // Create class repository
        let class_repo = ClassRepository::new(db_manager);
        
        // Create PBO mapping
        let mut pbo_map = HashMap::new();
        for (idx, file_path) in game_data.file_sources.iter().enumerate() {
            // Extract PBO name from path
            let pbo_name = file_path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown.pbo")
                .to_string();
                
            pbo_map.insert(idx, pbo_name);
        }
        
        // Import game data
        class_repo.import_from_game_data(game_data)
            .map_err(|e| ToolError::DatabaseError(format!("Failed to import game data: {}", e)))?;
            
        info!("Saved {} classes to database", game_data.classes.len());
        
        Ok(())
    }
    
    /// Load game data from database
    pub fn load_from_database(&self) -> Result<GameDataClasses> {
        // Check if database is available
        let db_manager = match &self.db_manager {
            Some(manager) => manager,
            None => return Err(ToolError::DatabaseError("Database not initialized".to_string())),
        };
        
        info!("Loading game data from database...");
        
        // Create class repository
        let class_repo = ClassRepository::new(db_manager);
        
        // Get all classes from database
        let class_models = class_repo.get_all()
            .map_err(|e| ToolError::DatabaseError(format!("Failed to load classes: {}", e)))?;
            
        // Create game data classes container
        let mut classes = GameDataClasses::new();
        
        // Convert class models to game data classes
        for model in class_models {
            let game_class = model.to_game_data_class();
            classes.add_class(game_class);
        }
        
        info!("Loaded {} classes from database", classes.classes.len());
        
        Ok(classes)
    }
}
