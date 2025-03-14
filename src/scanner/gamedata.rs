use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;
use rayon::prelude::*;
use log::{debug, info, warn, error};
use crate::config::ToolConfig;
use crate::cache::CacheManager;
use crate::error::{Result, ToolError};
use extraction::extract_pbo_with_options;
use pbo_tools::extract::ExtractOptions;

/// Represents the result of scanning game data
#[derive(Debug, Clone)]
pub struct GameDataClasses {
    /// Map of class names to their definitions
    pub classes: Vec<ClassDefinition>,
    
    /// Source PBOs where classes were found
    pub sources: Vec<PboSource>,
}

/// Represents a class definition found in game data
#[derive(Debug, Clone)]
pub struct ClassDefinition {
    /// Name of the class
    pub name: String,
    
    /// Type of the class (e.g., "Vehicle", "Weapon", etc.)
    pub class_type: String,
    
    /// Parent class, if any
    pub parent: Option<String>,
    
    /// Source file where the class was defined
    pub source_file: PathBuf,
    
    /// Source PBO index
    pub source_pbo_index: usize,
}

/// Represents a PBO source
#[derive(Debug, Clone)]
pub struct PboSource {
    /// Path to the PBO file
    pub path: PathBuf,
    
    /// Name of the PBO
    pub name: String,
    
    /// Mod or addon name
    pub mod_name: String,
}

/// Scanner for game data PBOs
pub struct GameDataScanner {
    /// Tool configuration
    config: ToolConfig,
    
    /// Cache manager
    cache_manager: CacheManager,
}

impl GameDataScanner {
    /// Create a new game data scanner
    pub fn new(config: ToolConfig, cache_manager: CacheManager) -> Self {
        Self {
            config,
            cache_manager,
        }
    }
    
    /// Scan game data
    pub fn scan(&self, dirs: Option<Vec<String>>) -> Result<GameDataClasses> {
        // Discover PBOs
        let pbo_paths = self.discover_pbos(dirs)?;
        info!("Found {} PBO files", pbo_paths.len());
        
        // Extract PBOs
        let extracted_paths = self.extract_pbos(&pbo_paths)?;
        info!("Extracted {} PBO files", extracted_paths.len());
        
        // Scan classes
        let game_data = self.scan_classes(&extracted_paths)?;
        info!("Found {} classes from {} PBOs", game_data.classes.len(), game_data.sources.len());
        
        Ok(game_data)
    }
    
    /// Discover PBO files in game data directories
    fn discover_pbos(&self, dirs: Option<Vec<String>>) -> Result<Vec<PathBuf>> {
        let mut pbo_paths = Vec::new();
        
        // Use specified directories or all configured directories
        let directories = match dirs {
            Some(dir_strings) => dir_strings.iter()
                .map(|s| PathBuf::from(s))
                .collect::<Vec<_>>(),
            None => self.config.game_data_dirs.clone(),
        };
        
        // Walk through each directory and find PBO files
        for dir in directories {
            debug!("Scanning directory for PBOs: {}", dir.display());
            
            for entry in WalkDir::new(&dir)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                
                if path.is_file() && path.extension().map_or(false, |ext| ext == "pbo") {
                    debug!("Found PBO: {}", path.display());
                    pbo_paths.push(path.to_path_buf());
                }
            }
        }
        
        Ok(pbo_paths)
    }
    
    /// Extract PBO files to cache
    fn extract_pbos(&self, pbo_paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut extracted_paths = Vec::new();
        
        // Process each PBO in parallel
        let results: Vec<_> = pbo_paths.par_iter()
            .map(|pbo_path| {
                let cache_path = self.cache_manager.get_game_data_cache_path(pbo_path);
                
                // Check if already cached
                if self.cache_manager.is_cached(pbo_path, &cache_path) {
                    debug!("Using cached extraction for {}", pbo_path.display());
                    return Ok(cache_path);
                }
                
                // Create cache directory if it doesn't exist
                if !cache_path.exists() {
                    fs::create_dir_all(&cache_path)
                        .map_err(|e| ToolError::CacheError(
                            format!("Failed to create cache directory: {}", e)
                        ))?;
                }
                
                // Extract PBO
                debug!("Extracting PBO: {} to {}", pbo_path.display(), cache_path.display());
                
                // Create extraction options
                let mut options = ExtractOptions::default();
                
                // Set file filter if extensions are specified
                if !self.config.game_data_extensions.is_empty() {
                    let filter = self.config.game_data_extensions.iter()
                        .map(|ext| format!("*.{}", ext))
                        .collect::<Vec<_>>()
                        .join(",");
                    options.file_filter = Some(filter);
                }
                
                // Extract PBO
                match extract_pbo_with_options(pbo_path, &cache_path, options) {
                    Ok(_) => {},
                    Err(e) => return Err(ToolError::ExtractionError(e.to_string())),
                }
                
                // Update cache
                self.cache_manager.update_cache(pbo_path, &cache_path)?;
                
                Ok(cache_path)
            })
            .collect();
        
        // Collect results
        for result in results {
            match result {
                Ok(path) => extracted_paths.push(path),
                Err(e) => error!("Failed to extract PBO: {}", e),
            }
        }
        
        Ok(extracted_paths)
    }
    
    /// Scan extracted PBOs for class definitions
    fn scan_classes(&self, extracted_paths: &[PathBuf]) -> Result<GameDataClasses> {
        let mut game_data = GameDataClasses {
            classes: Vec::new(),
            sources: Vec::new(),
        };
        
        // Process each extracted PBO
        for (index, path) in extracted_paths.iter().enumerate() {
            let pbo_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let mod_name = self.determine_mod_name(path);
            
            // Add PBO source
            game_data.sources.push(PboSource {
                path: path.to_path_buf(),
                name: pbo_name,
                mod_name,
            });
            
            // Scan for class definitions
            self.scan_pbo_for_classes(path, index, &mut game_data)?;
        }
        
        Ok(game_data)
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
        // Find all files with configured extensions
        for entry in WalkDir::new(pbo_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }
            
            let extension = match path.extension() {
                Some(ext) => ext.to_string_lossy().to_lowercase(),
                None => continue,
            };
            
            if !self.config.game_data_extensions.iter().any(|ext| ext.to_lowercase() == extension) {
                continue;
            }
            
            // Parse file for class definitions
            self.parse_file_for_classes(path, pbo_index, game_data)?;
        }
        
        Ok(())
    }
    
    /// Parse a file for class definitions
    fn parse_file_for_classes(&self, file_path: &Path, pbo_index: usize, game_data: &mut GameDataClasses) -> Result<()> {
        // This is a placeholder for the actual parsing logic
        // In a real implementation, we would use the gamedata_scanner crate to parse the file
        
        // For now, just log that we're parsing the file
        debug!("Parsing file for classes: {}", file_path.display());
        
        // TODO: Implement actual parsing using gamedata_scanner
        // For example:
        // let classes = gamedata_scanner::parse_file(file_path)?;
        // for class in classes {
        //     game_data.classes.push(ClassDefinition {
        //         name: class.name,
        //         class_type: class.type,
        //         parent: class.parent,
        //         source_file: file_path.to_path_buf(),
        //         source_pbo_index: pbo_index,
        //     });
        // }
        
        Ok(())
    }
} 