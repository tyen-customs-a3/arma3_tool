use std::path::{Path, PathBuf};
use std::fs;
use log::{debug, info, warn};
use crate::config::ToolConfig;
use crate::cache::CacheManager;
use crate::database::DatabaseManager;
use crate::error::{Result, ToolError};
use crate::scanner::prescan::PboPrescan;
use crate::scanner::models::{GameDataClasses, GameDataClassDefinition};
use gamedata_scanner::{
    scan_directory,
    GameDataScannerConfig,
    ScanResult as GDScanResult,
};

/// Scanner for game data PBOs
pub struct GameDataScanner {
    /// Tool configuration
    config: ToolConfig,
    
    /// Cache manager
    cache_manager: CacheManager,

    /// Database manager
    database_manager: DatabaseManager,
}

impl GameDataScanner {
    /// Create a new game data scanner
    pub fn new(config: ToolConfig, cache_manager: CacheManager) -> Self {
        let database_manager = DatabaseManager::new(config.cache_dir.clone());
        Self {
            config,
            cache_manager,
            database_manager,
        }
    }
    
    /// Extract PBOs without scanning (preparation step)
    pub fn extract_only(&mut self, dirs: Option<Vec<String>>) -> Result<Vec<PathBuf>> {
        // Use specified directories or all configured directories
        let directories = match dirs {
            Some(dir_strings) => dir_strings.iter()
                .map(|s| PathBuf::from(s))
                .collect::<Vec<_>>(),
            None => self.config.game_data_dirs.clone(),
        };
        
        // Create a PBO prescanner for game data
        let mut prescanner = PboPrescan::for_game_data(self.cache_manager.clone());
        
        // Discover PBOs
        let pbo_paths = prescanner.discover_pbos(&directories)?;
        info!("Found {} PBO files", pbo_paths.len());
        
        // Prescan PBOs to determine which need extraction
        let prescanned_pbos = prescanner.prescan(&pbo_paths, &self.config.game_data_extensions)?;
        
        // Extract PBOs
        let extracted_paths = prescanner.extract(&prescanned_pbos)?;
        info!("Extracted {} PBO files", extracted_paths.len());
        
        Ok(extracted_paths)
    }
    
    /// Scan classes from previously extracted PBOs
    pub fn scan_only(&mut self, dirs: Option<Vec<String>>) -> Result<GameDataClasses> {
        // Use specified directories or all configured directories
        let directories = match dirs {
            Some(dir_strings) => dir_strings.iter()
                .map(|s| PathBuf::from(s))
                .collect::<Vec<_>>(),
            None => self.config.game_data_dirs.clone(),
        };
        
        // Create a PBO prescanner for game data
        let prescanner = PboPrescan::for_game_data(self.cache_manager.clone());
        
        // Discover PBOs
        let pbo_paths = prescanner.discover_pbos(&directories)?;
        info!("Found {} PBO files", pbo_paths.len());
        
        // Get cached extraction paths
        let extracted_paths = self.get_cached_extraction_paths()?;
        if extracted_paths.is_empty() {
            return Err(ToolError::GameDataScanError(
                "No extracted PBOs found. Run extraction first.".to_string()
            ));
        }
        info!("Found {} extracted PBO paths", extracted_paths.len());
        
        // Scan classes from extracted paths
        let game_data = self.scan_classes(&extracted_paths)?;
        info!("Found {} classes", game_data.classes.len());

        // Save to database
        self.database_manager.save_game_data(game_data.clone())?;
        info!("Saved game data to database");
        
        Ok(game_data)
    }
    
    /// Get paths of previously extracted game data PBOs
    fn get_cached_extraction_paths(&self) -> Result<Vec<PathBuf>> {
        // Get base cache directory
        let base_cache_dir = self.cache_manager.get_game_data_cache_path(&PathBuf::from("dummy"))
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .to_path_buf();
            
        if base_cache_dir.exists() {
            // For game data, we just need the base directory
            Ok(vec![base_cache_dir])
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Scan game data
    pub fn scan(&mut self, dirs: Option<Vec<String>>) -> Result<GameDataClasses> {
        // Use specified directories or all configured directories
        let directories = match dirs {
            Some(dir_strings) => dir_strings.iter()
                .map(|s| PathBuf::from(s))
                .collect::<Vec<_>>(),
            None => self.config.game_data_dirs.clone(),
        };
        
        // Create a PBO prescanner for game data
        let mut prescanner = PboPrescan::for_game_data(self.cache_manager.clone());
        
        // Discover PBOs
        let pbo_paths = prescanner.discover_pbos(&directories)?;
        info!("Found {} PBO files", pbo_paths.len());
        
        // Prescan PBOs to determine which need extraction
        let prescanned_pbos = prescanner.prescan(&pbo_paths, &self.config.game_data_extensions)?;
        
        // Extract PBOs
        let extracted_paths = prescanner.extract(&prescanned_pbos)?;
        info!("Extracted {} PBO files", extracted_paths.len());
        
        // Scan classes
        let game_data = self.scan_classes(&extracted_paths)?;
        info!("Found {} classes", game_data.classes.len());

        // Save to database
        self.database_manager.save_game_data(game_data.clone())?;
        info!("Saved game data to database");
        
        Ok(game_data)
    }
    
    /// Scan extracted PBOs for class definitions
    fn scan_classes(&self, extracted_paths: &[PathBuf]) -> Result<GameDataClasses> {
        let mut game_data = GameDataClasses {
            classes: Vec::new(),
        };
        
        // Process each extracted PBO
        for (index, path) in extracted_paths.iter().enumerate() {
            let pbo_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let mod_name = self.determine_mod_name(path);
            
            // Create scanner configuration for gamedata_scanner
            let scanner_config = GameDataScannerConfig {
                file_extensions: self.config.game_data_extensions.clone(),
                max_threads: self.config.threads,
                follow_symlinks: true,
                use_cache: true,
            };
            
            // Use gamedata_scanner to scan the PBO directory
            match tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(scan_directory(path, &scanner_config))
            {
                Ok(scan_result) => {
                    // Convert scan results to our format
                    self.convert_scan_results(&scan_result, index, &mut game_data);
                    debug!("Scanned {} with gamedata_scanner: found {} classes", 
                           path.display(), scan_result.classes_found);
                },
                Err(err) => {
                    warn!("Error scanning {}: {}", path.display(), err);
                }
            }
        }
        
        Ok(game_data)
    }
    
    /// Convert gamedata_scanner results to our format
    fn convert_scan_results(&self, scan_result: &GDScanResult, pbo_index: usize, game_data: &mut GameDataClasses) {
        for (class_name, instances) in &scan_result.class_map {
            for instance in instances {
                // Determine parent class if any
                let parent = instance.parent.clone();
                
                // Add class definition
                game_data.classes.push(GameDataClassDefinition {
                    name: class_name.clone(),
                    parent,
                });
            }
        }
    }
    
    /// Parse a file for class definitions
    fn parse_file_for_classes(&self, path: &Path, pbo_index: usize, game_data: &mut GameDataClasses) -> Result<()> {
        // This method is now handled by the gamedata_scanner crate
        // We keep it for compatibility but it's essentially a no-op
        Ok(())
    }
    
    /// Determine the mod name from a PBO path
    fn determine_mod_name(&self, pbo_path: &Path) -> String {
        // Try to find $PBOPREFIX$ file
        let pboprefix_path = pbo_path.join("$PBOPREFIX$.txt");
        if pboprefix_path.exists() {
            if let Ok(content) = fs::read_to_string(&pboprefix_path) {
                let prefix = content.trim();
                if !prefix.is_empty() {
                    // Extract mod name from prefix (usually the first part)
                    if let Some(first_part) = prefix.split('\\').next() {
                        return first_part.to_string();
                    }
                }
            }
        }
        
        // Fall back to parent directory name
        if let Some(parent) = pbo_path.parent() {
            if let Some(name) = parent.file_name() {
                return name.to_string_lossy().to_string();
            }
        }
        
        // Last resort: use PBO name
        pbo_path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }
    
    /// Scan a PBO for class definitions
    fn scan_pbo_for_classes(&self, pbo_path: &Path, pbo_index: usize, game_data: &mut GameDataClasses) -> Result<()> {
        // Create scanner configuration for gamedata_scanner
        let scanner_config = GameDataScannerConfig {
            file_extensions: self.config.game_data_extensions.clone(),
            max_threads: self.config.threads,
            follow_symlinks: true,
            use_cache: true,
        };
        
        // Use gamedata_scanner to scan the PBO directory
        match tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(scan_directory(pbo_path, &scanner_config))
        {
            Ok(scan_result) => {
                // Convert scan results to our format
                self.convert_scan_results(&scan_result, pbo_index, game_data);
                debug!("Scanned {} with gamedata_scanner: found {} classes", 
                       pbo_path.display(), scan_result.classes_found);
                Ok(())
            },
            Err(err) => {
                warn!("Error scanning {}: {}", pbo_path.display(), err);
                Err(ToolError::GameDataScanError(format!("Error scanning {}: {}", pbo_path.display(), err)))
            }
        }
    }
} 