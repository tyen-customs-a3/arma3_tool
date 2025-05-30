use std::path::{Path, PathBuf};
use std::fs;
use log::{info, error};
use crate::error::{Result, ToolError};
use arma3_models::{MissionData, Mission, MissionComponent, DependencyRef, ReferenceType as ModelReferenceType};
use mission_scanner::{scan_mission, MissionScannerConfig, ReferenceType as ScannerReferenceType};
use num_cpus;
use arma3_database::MissionRepository;

/// Scanner for mission PBOs
pub struct MissionScanner {
    /// Cache directory
    cache_dir: PathBuf,
    /// Database manager, if database storage is enabled
    db_manager: Option<arma3_database::DatabaseManager>,
}

impl MissionScanner {
    /// Helper function to create a new scanner with common initialization
    fn create_scanner(
        config: arma3_extractor::ExtractionConfig, 
        db_manager: Option<arma3_database::DatabaseManager>
    ) -> Result<Self> {
        let cache_dir = config.cache_dir.clone();
        let output_dir = cache_dir.join("scan_results");
        
        // Ensure output directory exists
        fs::create_dir_all(&output_dir)
            .map_err(|e| ToolError::IoError(format!("Failed to create output directory: {}", e)))?;
            
        Ok(Self { 
            cache_dir, 
            db_manager,
        })
    }

    /// Create a new mission scanner
    pub fn new(config: arma3_extractor::ExtractionConfig) -> Result<Self> {
        Self::create_scanner(config, None)
    }
    
    /// Create a new mission scanner with a database manager
    pub fn with_database(config: arma3_extractor::ExtractionConfig, db_manager: arma3_database::DatabaseManager) -> Result<Self> {
        Self::create_scanner(config, Some(db_manager))
    }
    
    /// Create a new mission scanner with database connection from the specified path
    pub fn with_database_path(config: arma3_extractor::ExtractionConfig, db_path: &Path) -> Result<Self> {
        let db_config = arma3_database::models::CacheConfig::new(
            db_path.to_str().unwrap_or("arma3.db"), 
            config.cache_dir.to_str().unwrap_or("cache")
        );
        
        let db_manager = arma3_database::DatabaseManager::with_config(db_config)
            .map_err(|e| ToolError::DatabaseError(format!("Failed to create database manager: {}", e)))?;
            
        Self::create_scanner(config, Some(db_manager))
    }
    
    /// Convert scanner reference type to model reference type
    fn convert_reference_type(ref_type: ScannerReferenceType) -> ModelReferenceType {
        match ref_type {
            ScannerReferenceType::Direct => ModelReferenceType::DirectClass,
            ScannerReferenceType::Inheritance => ModelReferenceType::InheritedClass,
            ScannerReferenceType::Variable => ModelReferenceType::Other("Variable".to_string()),
        }
    }
    
    /// Scan missions from extracted PBOs
    pub async fn scan(&self) -> Result<MissionData> {
        let missions_dir = self.cache_dir.join("missions");
        info!("Scanning extracted mission PBOs in {}", missions_dir.display());
        
        // Create scanner config with default file extensions
        let config = MissionScannerConfig {
            file_extensions: vec!["sqf".to_string(), "cpp".to_string(), "hpp".to_string(), "ext".to_string()],
            ..Default::default()
        };
        
        // Find all mission directories in the missions directory
        let mission_dirs: Vec<_> = fs::read_dir(&missions_dir)
            .map_err(|e| ToolError::IoError(format!("Failed to read missions directory: {}", e)))?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
            .filter(|e| e.path().join("mission.sqm").exists())
            .map(|e| e.path())
            .collect();
            
        info!("Found {} mission directories to scan", mission_dirs.len());
        
        if mission_dirs.is_empty() {
            return Ok(MissionData::new());
        }
        
        let total_threads = num_cpus::get();
        
        // Process missions sequentially but use multithreading within each mission
        let mut mission_data = MissionData::new();
        
        for (index, mission_dir) in mission_dirs.iter().enumerate() {
            let mission_name = mission_dir.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
                
            info!("Scanning mission {}/{}: {} with {} threads",
                  index + 1, mission_dirs.len(), mission_name, total_threads);
            
            match scan_mission(mission_dir, total_threads, &config).await {
                Ok(scan_result) => {
                    // Create mission object
                    let mut mission = Mission::new(
                        scan_result.mission_name,
                        mission_dir.to_path_buf()
                    );
                    
                    // Set source PBO name
                    let pbo_name = mission_dir.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    // Only set source_pbo if we're scanning from a PBO
                    if mission_dir.parent()
                        .and_then(|p| p.file_name())
                        .and_then(|n| n.to_str())
                        .map(|n| n.ends_with(".pbo"))
                        .unwrap_or(false) {
                        mission.source_pbo = Some(pbo_name);
                    }
                    
                    // Add dependencies from all sources
                    for dep in scan_result.class_dependencies {
                        mission.add_dependency(DependencyRef::new(
                            dep.class_name,
                            Self::convert_reference_type(dep.reference_type),
                            dep.source_file
                        ));
                    }
                    
                    // Add mission.sqm as a component if present
                    if let Some(sqm_file) = scan_result.sqm_file {
                        let sqm_component = MissionComponent::new(
                            "mission.sqm".to_string(),
                            arma3_models::MissionComponentType::Other("SQM".to_string()),
                            sqm_file
                        );
                        mission.add_component(sqm_component);
                    }
                    
                    // Add SQF files as components
                    for sqf_file in scan_result.sqf_files {
                        let sqf_component = MissionComponent::new(
                            sqf_file.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                            arma3_models::MissionComponentType::Other("SQF".to_string()),
                            sqf_file
                        );
                        mission.add_component(sqf_component);
                    }
                    
                    // Add CPP/HPP files as components
                    for cpp_file in scan_result.cpp_files {
                        let cpp_component = MissionComponent::new(
                            cpp_file.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                            arma3_models::MissionComponentType::Other("CPP".to_string()),
                            cpp_file
                        );
                        mission.add_component(cpp_component);
                    }
                    
                    // Save individual mission results to database if available
                    if let Err(e) = self.save_mission_to_db(&mission) {
                        error!("Failed to save mission {} to database: {}", mission.name, e);
                    }
                    
                    // Add to collection
                    mission_data.add_mission(mission);
                },
                Err(e) => {
                    error!("Failed to scan mission {}: {}", mission_dir.display(), e);
                    // Continue with next mission rather than failing completely
                }
            }
        }

        Ok(mission_data)
    }
    
    /// Save mission data to database
    fn save_mission_to_db(&self, mission: &Mission) -> Result<()> {
        // Check if database is available
        let db_manager = match &self.db_manager {
            Some(manager) => manager,
            // If DB not configured, silently skip saving
            None => return Ok(()),
        };
        
        info!("Saving mission {} to database...", mission.name);
        
        // Create MissionRepository
        let mission_repo = MissionRepository::new(db_manager);
        
        // Import mission data using the shared model
        mission_repo.import_from_shared_model(mission)
            .map_err(|e| ToolError::DatabaseError(format!("Failed to save mission {}: {}", mission.name, e)))?;
            
        info!("Saved mission {} to database", mission.name);
        
        Ok(())
    }
}