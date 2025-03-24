use std::path::PathBuf;
use std::fs;
use log::{info, warn, error};
use crate::error::{Result, ToolError};
use arma3_tool_pbo_cache::ExtractionManager;
use arma3_tool_models::{MissionData, Mission, MissionComponent, DependencyRef, ReferenceType as ModelReferenceType};
use walkdir::WalkDir;
use mission_scanner::{scan_mission, MissionScannerConfig, ReferenceType as ScannerReferenceType};
use serde_json;
use chrono::Utc;
use num_cpus;
use tokio::sync::Semaphore;

/// Scanner for mission PBOs
pub struct MissionScanner {
    /// Cache directory
    cache_dir: PathBuf,
    /// PBO extractor
    extractor: ExtractionManager,
    /// Output directory for scan results
    output_dir: PathBuf,
}

impl MissionScanner {
    /// Create a new mission scanner
    pub fn new(config: arma3_tool_pbo_cache::ExtractionConfig) -> Result<Self> {
        let cache_dir = config.cache_dir.clone();
        let output_dir = cache_dir.join("scan_results");
        let extractor = ExtractionManager::new(config)
            .map_err(|e| ToolError::CacheError(format!("Failed to create PBO extractor: {}", e)))?;
            
        // Ensure output directory exists
        fs::create_dir_all(&output_dir)
            .map_err(|e| ToolError::IoError(format!("Failed to create output directory: {}", e)))?;
            
        Ok(Self { cache_dir, extractor, output_dir })
    }
    
    /// Convert scanner reference type to model reference type
    fn convert_reference_type(ref_type: ScannerReferenceType) -> ModelReferenceType {
        match ref_type {
            ScannerReferenceType::Direct => ModelReferenceType::DirectClass,
            ScannerReferenceType::Inheritance => ModelReferenceType::InheritedClass,
            ScannerReferenceType::Variable => ModelReferenceType::Other("Variable".to_string()),
        }
    }
    
    /// Extract PBOs without scanning (preparation step)
    pub async fn extract_only(&mut self, dirs: Option<Vec<String>>) -> Result<Vec<PathBuf>> {
        info!("Extracting mission PBOs...");
        
        // Process all mission PBOs
        let extraction_results = self.extractor.process_all_missions(false).await
            .map_err(|e| ToolError::CacheError(format!("Failed to process missions: {}", e)))?;
            
        // Flatten results into a single vector of paths
        let mut extracted = Vec::new();
        for (_, paths) in extraction_results {
            extracted.extend(paths);
        }
            
        info!("Extracted {} mission files", extracted.len());
        Ok(extracted)
    }
    
    /// Scan missions from previously extracted PBOs
    pub async fn scan_only(&self, _dirs: Option<Vec<String>>) -> Result<MissionData> {
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
        
        // Calculate optimal thread distribution
        let total_threads = num_cpus::get();
        let num_missions = mission_dirs.len();
        
        // Calculate concurrent missions based on available threads and limits
        let threads_per_mission = total_threads;
        
        // Create a semaphore to limit concurrent mission parsing
        let semaphore = std::sync::Arc::new(Semaphore::new(total_threads));
        
        // Process missions in parallel
        let mut tasks = Vec::new();
        
        for mission_dir in mission_dirs {
            let mission_name = mission_dir.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
                
            info!("Scheduling scan for mission: {}", mission_name);
            
            // Clone variables needed for the task
            let mission_dir_clone = mission_dir.clone();
            let config_clone = config.clone();
            let semaphore = semaphore.clone();
            
            // Spawn a task for each mission
            let task = tokio::spawn(async move {
                // Acquire semaphore permit
                let _permit = semaphore.acquire().await.unwrap();
                info!("Scanning mission: {} with {} threads", mission_name, threads_per_mission);
                
                match scan_mission(&mission_dir_clone, threads_per_mission, &config_clone).await {
                    Ok(scan_result) => {
                        // Create mission object
                        let mut mission = Mission::new(
                            scan_result.mission_name,
                            mission_dir_clone.to_path_buf()
                        );
                        
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
                                arma3_tool_models::MissionComponentType::Other("SQM".to_string()),
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
                                arma3_tool_models::MissionComponentType::Other("SQF".to_string()),
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
                                arma3_tool_models::MissionComponentType::Other("CPP".to_string()),
                                cpp_file
                            );
                            mission.add_component(cpp_component);
                        }
                        
                        Ok(mission)
                    },
                    Err(e) => {
                        error!("Failed to scan mission {}: {}", mission_dir_clone.display(), e);
                        Err(ToolError::MissionScanError(format!(
                            "Failed to scan mission {}: {}", 
                            mission_dir_clone.display(), e
                        )))
                    }
                }
            });
            
            tasks.push((mission_dir, task));
        }
        
        // Collect results
        let mut mission_data = MissionData::new();
        
        for (mission_dir, task) in tasks {
            match task.await {
                Ok(Ok(mission)) => {
                    // Save individual mission results
                    self.save_mission_results(&mission)?;
                    
                    // Add to collection
                    mission_data.add_mission(mission);
                },
                Ok(Err(e)) => {
                    error!("Task for mission {} returned an error: {}", mission_dir.display(), e);
                },
                Err(e) => {
                    error!("Task for mission {} panicked: {}", mission_dir.display(), e);
                }
            }
        }
        
        // Save complete scan results
        self.save_results(&mission_data)?;
        
        Ok(mission_data)
    }
    
    /// Save scan results for a single mission
    fn save_mission_results(&self, mission: &Mission) -> Result<()> {
        let mission_name = mission.name.replace(" ", "_").replace("/", "_");
        let output_file = self.output_dir.join(format!("mission_{}_{}.json", 
            mission_name,
            Utc::now().format("%Y%m%d_%H%M%S").to_string()
        ));
        
        info!("Saving mission scan results to {}", output_file.display());
        
        let json = serde_json::to_string_pretty(mission)
            .map_err(|e| ToolError::JsonError(format!("Failed to serialize mission results: {}", e)))?;
            
        fs::write(&output_file, json)
            .map_err(|e| ToolError::IoError(format!("Failed to write mission results: {}", e)))?;
            
        Ok(())
    }
    
    /// Save complete scan results
    fn save_results(&self, data: &MissionData) -> Result<()> {
        let output_file = self.output_dir.join(format!("mission_scan_{}.json",
            Utc::now().format("%Y%m%d_%H%M%S").to_string()
        ));
        
        info!("Saving complete scan results to {}", output_file.display());
        
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| ToolError::JsonError(format!("Failed to serialize scan results: {}", e)))?;
            
        fs::write(&output_file, json)
            .map_err(|e| ToolError::IoError(format!("Failed to write scan results: {}", e)))?;
            
        Ok(())
    }
    
    /// Scan missions
    pub async fn scan(&mut self, dirs: Option<Vec<String>>) -> Result<MissionData> {
        let _extracted = self.extract_only(dirs).await?;
        self.scan_only(None).await
    }
} 