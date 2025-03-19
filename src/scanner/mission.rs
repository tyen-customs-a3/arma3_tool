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
        
        let mut mission_data = MissionData::new();
        
        // Create scanner config
        let config = MissionScannerConfig::default();
        
        // Find all mission directories in the missions directory
        for entry in fs::read_dir(&missions_dir)
            .map_err(|e| ToolError::IoError(format!("Failed to read missions directory: {}", e)))?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false)) {
                
            let mission_dir = entry.path();
            info!("Scanning mission: {}", mission_dir.file_name().unwrap_or_default().to_string_lossy());
            
            // Scan the mission
            match scan_mission(&mission_dir, num_cpus::get(), &config).await {
                Ok(scan_result) => {
                    // Create mission object
                    let mut mission = Mission::new(
                        scan_result.mission_name,
                        mission_dir.to_path_buf()
                    );
                    
                    // Add dependencies
                    for dep in scan_result.class_dependencies {
                        mission.add_dependency(DependencyRef::new(
                            dep.class_name,
                            Self::convert_reference_type(dep.reference_type),
                            PathBuf::from(dep.context)
                        ));
                    }
                    
                    // Add components from SQM file
                    if let Some(sqm_file) = scan_result.sqm_file {
                        // TODO: Implement SQM file parsing and component extraction
                        // For now, just add a placeholder component
                        let sqm_component = MissionComponent::new(
                            "mission.sqm".to_string(),
                            arma3_tool_models::MissionComponentType::Other("SQM".to_string()),
                            sqm_file
                        );
                        mission.add_component(sqm_component);
                    }
                    
                    // Save individual mission results
                    self.save_mission_results(&mission)?;
                    
                    // Add to collection
                    mission_data.add_mission(mission);
                },
                Err(e) => {
                    error!("Failed to scan mission {}: {}", mission_dir.display(), e);
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