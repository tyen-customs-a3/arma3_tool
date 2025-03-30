use std::path::PathBuf;
use std::collections::HashMap;
use log::{info, warn};
use crate::error::{Result, ToolError};
use arma3_models::{GameDataClasses, GameDataClass, PropertyValue};
use gamedata_scanner::{
    GameClass, 
    PropertyValue as GameDataScannerPropertyValue,
    Scanner,
    ScannerConfig
};
use arma3_database::{DatabaseManager, repos::ClassRepository};

/// Scanner for game data PBOs
pub struct GameDataScanner {
    /// Cache directory
    cache_dir: PathBuf,
    /// Use advanced scanner for more accurate but slower parsing
    use_advanced_scanner: bool,
    /// Database manager, if database storage is enabled
    db_manager: Option<DatabaseManager>,
}

impl GameDataScanner {
    /// Helper function to create a new scanner with common initialization
    fn create_scanner(
        config: arma3_extractor::ExtractionConfig,
        db_manager: Option<DatabaseManager>
    ) -> Result<Self> {
        let cache_dir = config.cache_dir.clone();
            
        Ok(Self { 
            cache_dir, 
            use_advanced_scanner: true,
            db_manager,
        })
    }

    /// Create a new game data scanner
    pub fn new(config: arma3_extractor::ExtractionConfig) -> Result<Self> {
        Self::create_scanner(config, None)
    }
    
    /// Create a new game data scanner with provided database manager
    pub fn with_database(config: arma3_extractor::ExtractionConfig, db_manager: DatabaseManager) -> Result<Self> {
        Self::create_scanner(config, Some(db_manager))
    }
    
    /// Create a new game data scanner with database connection from the specified path
    pub fn with_database_path(config: arma3_extractor::ExtractionConfig, db_path: &PathBuf) -> Result<Self> {
        let db_config = arma3_database::models::CacheConfig::new(
            db_path.to_str().unwrap_or("arma3.db"), 
            config.cache_dir.to_str().unwrap_or("cache")
        );
        
        let db_manager = DatabaseManager::with_config(db_config)
            .map_err(|e| ToolError::DatabaseError(format!("Failed to create database manager: {}", e)))?;
            
        Self::create_scanner(config, Some(db_manager))
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
        
        Some(GameDataClass {
            name: class.name.clone(),
            parent: class.parent.clone(),
            container_class: class.container_class.clone(),
            properties,
            source_file_index: file_index,
        })
    }
    
    /// Scan classes from extracted PBOs
    pub async fn scan(&self) -> Result<GameDataClasses> {
        let gamedata_dir = self.cache_dir.join("gamedata");
        info!("Scanning extracted game data PBOs in {}", gamedata_dir.display());
        
        info!("Using {} scanner for game data", if self.use_advanced_scanner { "advanced" } else { "simple" });
        
        // Create scanner configuration
        let scanner_config = ScannerConfig::default();
        
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
