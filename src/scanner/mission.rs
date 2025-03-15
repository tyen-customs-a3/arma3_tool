use std::path::PathBuf;
use log::{debug, info, error};
use crate::config::ToolConfig;
use crate::cache::CacheManager;
use crate::database::DatabaseManager;
use crate::error::{Result, ToolError};
use crate::scanner::prescan::PboPrescan;
use crate::scanner::models::{MissionData, MissionDefinition, MissionDependency};
use mission_scanner::{scan_mission, MissionScannerConfig, MissionResults};
use tokio::runtime::Runtime;

/// Scanner for mission PBOs
pub struct MissionScanner {
    /// Tool configuration
    config: ToolConfig,
    
    /// Cache manager
    cache_manager: CacheManager,

    /// Database manager
    database_manager: DatabaseManager,
}

impl MissionScanner {
    /// Create a new mission scanner
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
            None => self.config.mission_dirs.clone(),
        };
        
        // Create a PBO prescanner for missions
        let mut prescanner = PboPrescan::for_missions(self.cache_manager.clone());
        
        // Discover PBOs
        let pbo_paths = prescanner.discover_pbos(&directories)?;
        info!("Found {} mission PBO files", pbo_paths.len());
        
        // Prescan PBOs to determine which need extraction
        let prescanned_pbos = prescanner.prescan(&pbo_paths, &self.config.mission_extensions)?;
        
        // Extract PBOs
        let extracted_paths = prescanner.extract(&prescanned_pbos)?;
        info!("Extracted {} mission PBO files", extracted_paths.len());
        
        Ok(extracted_paths)
    }
    
    /// Scan missions from previously extracted PBOs
    pub fn scan_only(&mut self, dirs: Option<Vec<String>>) -> Result<MissionData> {
        // Use specified directories or all configured directories
        let directories = match dirs {
            Some(dir_strings) => dir_strings.iter()
                .map(|s| PathBuf::from(s))
                .collect::<Vec<_>>(),
            None => self.config.mission_dirs.clone(),
        };
        
        // Create a PBO prescanner for missions (for discovery only)
        let prescanner = PboPrescan::for_missions(self.cache_manager.clone());
        
        // Get cached extraction paths
        let extracted_paths = self.get_cached_extraction_paths(&directories)?;
        if extracted_paths.is_empty() {
            return Err(ToolError::MissionScanError(
                "No extracted mission PBOs found. Run extraction first.".to_string()
            ));
        }
        info!("Found {} extracted mission paths", extracted_paths.len());
        
        // Scan missions
        let mission_results = self.scan_missions(&extracted_paths)?;
        info!("Scanned {} missions", mission_results.len());
        
        // Convert to our internal model
        let mission_data = self.convert_to_mission_data(mission_results);
        info!("Converted {} missions to internal model", mission_data.missions.len());

        // Save to database
        self.database_manager.save_mission_data(mission_data.clone())?;
        info!("Saved mission data to database");
        
        Ok(mission_data)
    }
    
    /// Get paths of previously extracted mission PBOs
    fn get_cached_extraction_paths(&self, directories: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut extracted_paths = Vec::new();
        
        // For each mission directory, check for extracted PBOs
        for dir in directories {
            // Create a PBO prescanner for missions to use its discovery
            let prescanner = PboPrescan::for_missions(self.cache_manager.clone());
            
            // Discover PBOs
            let pbo_paths = prescanner.discover_pbos(&[dir.clone()])?;
            
            // For each PBO path, check if it has a cached extraction
            for pbo_path in &pbo_paths {
                let cache_path = self.cache_manager.get_mission_cache_path(pbo_path);
                if cache_path.exists() {
                    extracted_paths.push(cache_path);
                }
            }
        }
        
        Ok(extracted_paths)
    }
    
    /// Scan missions
    pub fn scan(&mut self, dirs: Option<Vec<String>>) -> Result<MissionData> {
        // Use specified directories or all configured directories
        let directories = match dirs {
            Some(dir_strings) => dir_strings.iter()
                .map(|s| PathBuf::from(s))
                .collect::<Vec<_>>(),
            None => self.config.mission_dirs.clone(),
        };
        
        // Create a PBO prescanner for missions
        let mut prescanner = PboPrescan::for_missions(self.cache_manager.clone());
        
        // Discover PBOs
        let pbo_paths = prescanner.discover_pbos(&directories)?;
        info!("Found {} mission PBO files", pbo_paths.len());
        
        // Prescan PBOs to determine which need extraction
        let prescanned_pbos = prescanner.prescan(&pbo_paths, &self.config.mission_extensions)?;
        
        // Extract PBOs
        let extracted_paths = prescanner.extract(&prescanned_pbos)?;
        info!("Extracted {} mission PBO files", extracted_paths.len());
        
        // Scan missions
        let mission_results = self.scan_missions(&extracted_paths)?;
        info!("Scanned {} missions", mission_results.len());
        
        // Convert to our internal model
        let mission_data = self.convert_to_mission_data(mission_results);
        info!("Converted {} missions to internal model", mission_data.missions.len());

        // Save to database
        self.database_manager.save_mission_data(mission_data.clone())?;
        info!("Saved mission data to database");
        
        Ok(mission_data)
    }
    
    /// Scan extracted missions
    fn scan_missions(&self, extracted_paths: &[PathBuf]) -> Result<Vec<MissionResults>> {
        let mut mission_results = Vec::new();
        
        // Create mission scanner config
        let scanner_config = MissionScannerConfig {
            max_threads: self.config.threads,
            file_extensions: self.config.mission_extensions.clone(),
        };
        
        // Create a Tokio runtime for async operations
        let runtime = Runtime::new()
            .map_err(|e| ToolError::MissionScanError(format!("Failed to create Tokio runtime: {}", e)))?;
        
        // Process each extracted mission
        for path in extracted_paths {
            debug!("Scanning mission: {}", path.display());
            
            // Use the runtime to block on the async scan_mission function
            let result = runtime.block_on(scan_mission(path, self.config.threads, &scanner_config));
            
            match result {
                Ok(result) => {
                    info!("Scanned mission: {} with {} dependencies", 
                        result.mission_name, 
                        result.class_dependencies.len()
                    );
                    mission_results.push(result);
                },
                Err(e) => {
                    error!("Failed to scan mission {}: {}", path.display(), e);
                }
            }
        }
        
        Ok(mission_results)
    }
    
    /// Convert mission_scanner::MissionResults to our internal MissionData model
    fn convert_to_mission_data(&self, mission_results: Vec<MissionResults>) -> MissionData {
        let missions = mission_results.into_iter()
            .map(|result| {
                // Convert dependencies
                let dependencies = result.class_dependencies.into_iter()
                    .map(|dep| MissionDependency {
                        class_name: dep.class_name,
                        context: dep.context,
                        reference_type: format!("{:?}", dep.reference_type),
                    })
                    .collect();
                
                // Create mission definition
                MissionDefinition {
                    name: result.mission_name,
                    path: result.mission_dir,
                    dependencies,
                    sqf_files: result.sqf_files.len(),
                    cpp_files: result.cpp_files.len(),
                }
            })
            .collect();
        
        MissionData { missions }
    }
} 