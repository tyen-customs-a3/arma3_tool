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
    pub fn scan(&mut self, dirs: Option<Vec<String>>) -> Result<GameDataClasses> {
        // Discover PBOs
        let pbo_paths = self.discover_pbos(dirs)?;
        info!("Found {} PBO files", pbo_paths.len());
        
        // Extract PBOs
        let extracted_paths = self.extract_pbos(&pbo_paths)?;
        info!("Extracted {} PBO files", extracted_paths.len());
        
        // Scan classes
        let game_data = self.scan_classes(&extracted_paths)?;
        info!("Found {} classes from {} PBOs", game_data.classes.len(), game_data.sources.len());
        
        // Log class statistics
        let stats = self.get_class_statistics(&game_data);
        debug!("{}", stats);
        
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
    fn extract_pbos(&mut self, pbo_paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut extracted_paths = Vec::new();
        
        // Define a custom result type for the parallel extraction
        type ExtractionResult = std::result::Result<(PathBuf, Vec<String>, Option<String>), (String, Vec<String>, Option<String>)>;
        
        // Process each PBO in parallel
        let results: Vec<ExtractionResult> = pbo_paths.par_iter()
            .map(|pbo_path| {
                let cache_path = self.cache_manager.get_game_data_cache_path(pbo_path);
                
                // Create file filter if extensions are specified
                let extensions = self.config.game_data_extensions.clone();
                let file_filter = if !extensions.is_empty() {
                    let filter = extensions.iter()
                        .map(|ext| format!("*.{}", ext))
                        .collect::<Vec<_>>()
                        .join(",");
                    Some(filter)
                } else {
                    None
                };
                
                // Check if PBO has a permanent error (like bad SHA)
                if self.cache_manager.has_game_data_permanent_error(pbo_path) {
                    debug!("Skipping PBO with permanent error: {}", pbo_path.display());
                    return Err(("Skipping PBO with permanent error".to_string(), extensions, file_filter));
                }
                
                // Check if PBO has previously had an error but patterns have changed
                if self.cache_manager.has_game_data_error(pbo_path) && 
                   self.cache_manager.has_game_data_pattern_changed(pbo_path, &extensions, file_filter.as_deref()) {
                    debug!("Extraction patterns changed for PBO with previous error, retrying: {}", pbo_path.display());
                    // Continue with extraction since patterns changed
                } else if self.cache_manager.has_game_data_error(pbo_path) {
                    // Skip if it had an error and patterns haven't changed
                    let error_msg = self.cache_manager.get_game_data_error_message(pbo_path)
                        .unwrap_or_else(|| "Unknown error".to_string());
                    warn!("Skipping PBO that previously had an error: {} - {}", pbo_path.display(), error_msg);
                    return Err((format!("Skipping PBO that previously had an error: {}", error_msg), extensions, file_filter));
                }
                
                // Check if already cached and patterns haven't changed
                if self.cache_manager.is_game_data_cached(pbo_path, &cache_path) && 
                   !self.cache_manager.has_game_data_pattern_changed(pbo_path, &extensions, file_filter.as_deref()) {
                    debug!("Using cached extraction for {}", pbo_path.display());
                    return Ok((cache_path, extensions, file_filter));
                }
                
                // Create cache directory if it doesn't exist
                if !cache_path.exists() {
                    fs::create_dir_all(&cache_path)
                        .map_err(|e| (format!("Failed to create cache directory: {}", e), extensions.clone(), file_filter.clone()))?;
                } else if self.cache_manager.has_game_data_pattern_changed(pbo_path, &extensions, file_filter.as_deref()) {
                    // If patterns changed, clear the cache directory and recreate it
                    debug!("Extraction patterns changed, clearing cache for: {}", pbo_path.display());
                    fs::remove_dir_all(&cache_path)
                        .map_err(|e| (format!("Failed to clear cache directory: {}", e), extensions.clone(), file_filter.clone()))?;
                    fs::create_dir_all(&cache_path)
                        .map_err(|e| (format!("Failed to recreate cache directory: {}", e), extensions.clone(), file_filter.clone()))?;
                }
                
                // Extract PBO
                debug!("Extracting PBO: {} to {}", pbo_path.display(), cache_path.display());
                
                // Create extraction options
                let mut options = ExtractOptions::default();
                
                // Set file filter if extensions are specified
                if let Some(filter) = &file_filter {
                    options.file_filter = Some(filter.clone());
                }
                
                // Extract PBO
                match extract_pbo_with_options(pbo_path, &cache_path, options) {
                    Ok(_) => {
                        // Note: We can't update the cache here because we're in a parallel context
                        // and cache_manager is borrowed immutably. We'll handle this after collecting results.
                        Ok((cache_path, extensions, file_filter))
                    },
                    Err(e) => {
                        let error_msg = e.to_string();
                        
                        // Check if this is error code 11 (no files to extract)
                        if error_msg.contains("return code 11") || 
                           error_msg.contains("no file(s) to extract") {
                            debug!("No files to extract (error code 11), treating as success: {}", pbo_path.display());
                            return Ok((cache_path, extensions, file_filter));
                        }
                        
                        error!("Failed to extract PBO {}: {}", pbo_path.display(), error_msg);
                        Err((error_msg, extensions, file_filter))
                    },
                }
            })
            .collect();
        
        // Process results and update cache
        for (i, result) in results.into_iter().enumerate() {
            if let Some(pbo_path) = pbo_paths.get(i) {
                match result {
                    Ok((path, extensions, file_filter)) => {
                        // Update cache for successful extractions
                        if let Err(e) = self.cache_manager.update_game_data_cache(
                            pbo_path, 
                            &path, 
                            &extensions, 
                            file_filter.as_deref()
                        ) {
                            warn!("Failed to update cache for {}: {}", pbo_path.display(), e);
                        }
                        extracted_paths.push(path);
                    },
                    Err((error_msg, extensions, file_filter)) => {
                        // Check if this is a bad SHA error
                        if self.cache_manager.is_bad_sha_error(&error_msg) {
                            warn!("PBO has bad SHA, marking as permanent error: {}", pbo_path.display());
                            if let Err(e) = self.cache_manager.mark_game_data_permanent_error(pbo_path, &error_msg) {
                                warn!("Failed to mark permanent error in cache for {}: {}", pbo_path.display(), e);
                            }
                        } else {
                            // Mark regular error in cache
                            if let Err(cache_err) = self.cache_manager.mark_game_data_error(
                                pbo_path, 
                                &error_msg, 
                                &extensions, 
                                file_filter.as_deref()
                            ) {
                                warn!("Failed to mark error in cache for {}: {}", pbo_path.display(), cache_err);
                            }
                        }
                        error!("Failed to extract PBO: {}", error_msg);
                    },
                }
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
        // Read the file content
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                warn!("Failed to read file {}: {}", file_path.display(), e);
                return Ok(());
            }
        };
        
        debug!("Parsing file for classes: {}", file_path.display());
        
        // Simple parser for class definitions
        // This is a basic implementation - in a real-world scenario, you would use a more robust parser
        
        // Track current class context
        let mut current_class = String::new();
        let mut current_parent = Option::<String>::None;
        let mut in_class_def = false;
        let mut brace_level = 0;
        let mut classes_found = 0;
        
        // Process each line
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            
            // Skip comments and empty lines
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            
            // Look for class definitions
            if let Some(class_def) = trimmed.strip_prefix("class ") {
                // Extract class name and parent
                let parts: Vec<&str> = class_def.split(':').collect();
                
                if parts.len() >= 1 {
                    let class_name = parts[0].trim().trim_end_matches(['{', ' ']);
                    
                    // If we're already in a class definition, this is a nested class
                    let full_class_name = if !current_class.is_empty() && brace_level > 0 {
                        format!("{}_{}", current_class, class_name)
                    } else {
                        class_name.to_string()
                    };
                    
                    // Extract parent class if present
                    let parent = if parts.len() >= 2 {
                        let parent_part = parts[1].trim();
                        if parent_part.starts_with("public") || parent_part.starts_with("private") {
                            let parent_parts: Vec<&str> = parent_part.split_whitespace().collect();
                            if parent_parts.len() >= 2 {
                                Some(parent_parts[1].trim_end_matches(['{', ' ']).to_string())
                            } else {
                                None
                            }
                        } else {
                            Some(parent_part.trim_end_matches(['{', ' ']).to_string())
                        }
                    } else {
                        None
                    };
                    
                    // Check if this line contains an opening brace
                    if trimmed.contains('{') {
                        in_class_def = true;
                        brace_level += 1;
                        
                        // Save current class context
                        current_class = full_class_name.clone();
                        current_parent = parent.clone();
                    }
                    
                    // Add class definition to game data
                    if !full_class_name.is_empty() {
                        // Check if we already have this class (case-insensitive)
                        let lowercase_name = full_class_name.to_lowercase();
                        let duplicate = game_data.classes.iter().any(|c| 
                            c.name.to_lowercase() == lowercase_name
                        );
                        
                        if !duplicate {
                            game_data.classes.push(ClassDefinition {
                                name: full_class_name,
                                class_type: "Unknown".to_string(), // We would need more parsing to determine this
                                parent,
                                source_file: file_path.to_path_buf(),
                                source_pbo_index: pbo_index,
                            });
                            classes_found += 1;
                        } else {
                            debug!("Skipping duplicate class: {} in {}", full_class_name, file_path.display());
                        }
                    }
                }
            } else if in_class_def {
                // Track brace level to know when we exit a class
                if trimmed.contains('{') {
                    brace_level += 1;
                }
                
                if trimmed.contains('}') {
                    brace_level -= 1;
                    
                    // If we've exited the current class
                    if brace_level == 0 {
                        in_class_def = false;
                        current_class = String::new();
                        current_parent = None;
                    }
                }
            }
        }
        
        debug!("Found {} classes in {}", classes_found, file_path.display());
        
        Ok(())
    }
    
    /// Get statistics about the scanned classes
    pub fn get_class_statistics(&self, game_data: &GameDataClasses) -> String {
        let total_classes = game_data.classes.len();
        let total_sources = game_data.sources.len();
        
        // Count unique class names (case-insensitive)
        let mut unique_classes = std::collections::HashSet::new();
        for class in &game_data.classes {
            unique_classes.insert(class.name.to_lowercase());
        }
        
        // Count classes by mod
        let mut classes_by_mod = std::collections::HashMap::new();
        for class in &game_data.classes {
            if let Some(source) = game_data.sources.get(class.source_pbo_index) {
                *classes_by_mod.entry(source.mod_name.clone()).or_insert(0) += 1;
            }
        }
        
        // Format statistics
        let mut stats = format!("Class Statistics:\n");
        stats.push_str(&format!("  Total classes: {}\n", total_classes));
        stats.push_str(&format!("  Unique classes (case-insensitive): {}\n", unique_classes.len()));
        stats.push_str(&format!("  Total PBO sources: {}\n", total_sources));
        stats.push_str("  Classes by mod:\n");
        
        // Sort mods by class count
        let mut mods: Vec<_> = classes_by_mod.iter().collect();
        mods.sort_by(|a, b| b.1.cmp(a.1));
        
        for (mod_name, count) in mods {
            stats.push_str(&format!("    {}: {}\n", mod_name, count));
        }
        
        stats
    }
} 