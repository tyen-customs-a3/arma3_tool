use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use anyhow::{Result, Context};
use log::{info, debug, warn};
use serde::Serialize;
use walkdir;

use crate::scanning::pbo::coordinator::ScanCoordinator;
use crate::scanning::classes::processor::{ProcessedClass, ProcessingStats};
use crate::scanning::missions::MissionDependencyResult;
use crate::scanning::missions::validator::ClassExistenceReport;
use crate::reporting::{ReportConfig, ReportFormat};
use crate::reporting::class_reports::ClassReportManager;
use crate::reporting::mission_reports::{MissionReportManager, DependencyReportManager};

/// Report dependency type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReportDependency {
    PboScan,
    ClassScan,
    MissionScan,
    ClassValidation,
}

/// Report type with dependencies
#[derive(Debug, Clone)]
pub struct ReportType {
    pub name: String,
    pub dependencies: Vec<ReportDependency>,
    pub enabled: bool,
}

impl ReportType {
    pub fn new(name: &str, dependencies: Vec<ReportDependency>) -> Self {
        Self {
            name: name.to_string(),
            dependencies,
            enabled: true,
        }
    }
}

/// Processing and reporting manager
pub struct ProcessingManager {
    output_dir: PathBuf,
    cache_dir: PathBuf,
    report_config: ReportConfig,
    report_types: HashMap<String, ReportType>,
}

impl ProcessingManager {
    pub fn new(output_dir: &Path, cache_dir: &Path) -> Self {
        let mut manager = Self {
            output_dir: output_dir.to_owned(),
            cache_dir: cache_dir.to_owned(),
            report_config: ReportConfig::new(),
            report_types: HashMap::new(),
        };
        
        // Register all available report types with their dependencies
        manager.register_report_types();
        
        manager
    }
    
    /// Register all available report types with their dependencies
    fn register_report_types(&mut self) {
        // Class reports
        self.register_report("classes", vec![ReportDependency::ClassScan]);
        self.register_report("class_stats", vec![ReportDependency::ClassScan]);
        self.register_report("classes_by_category", vec![ReportDependency::ClassScan]);
        self.register_report("class_mission_usage", vec![ReportDependency::ClassScan, ReportDependency::MissionScan]);
        self.register_report("class_hierarchy", vec![ReportDependency::ClassScan]);
        self.register_report("circular_dependencies", vec![ReportDependency::ClassScan]);
        
        // Mission reports
        self.register_report("mission_summary", vec![ReportDependency::MissionScan]);
        self.register_report("mission_info", vec![ReportDependency::MissionScan]);
        self.register_report("equipment_items", vec![ReportDependency::MissionScan]);
        self.register_report("dependencies", vec![ReportDependency::MissionScan]);
        
        // Dependency reports
        self.register_report("dependency_report", vec![ReportDependency::PboScan, ReportDependency::MissionScan, ReportDependency::ClassScan, ReportDependency::ClassValidation]);
        self.register_report("missing_classes", vec![ReportDependency::MissionScan, ReportDependency::ClassScan]);
        self.register_report("class_usage_frequency", vec![ReportDependency::MissionScan, ReportDependency::ClassScan]);
        self.register_report("mission_compatibility", vec![ReportDependency::MissionScan, ReportDependency::ClassScan]);
        self.register_report("category_needs", vec![ReportDependency::MissionScan, ReportDependency::ClassScan]);
        self.register_report("class_inheritance", vec![ReportDependency::ClassScan]);
        self.register_report("compatibility_diagnostics", vec![ReportDependency::MissionScan, ReportDependency::ClassScan]);
        self.register_report("class_existence_validation", vec![ReportDependency::MissionScan, ReportDependency::ClassScan, ReportDependency::ClassValidation]);
    }
    
    /// Register a report type with its dependencies
    fn register_report(&mut self, name: &str, dependencies: Vec<ReportDependency>) {
        self.report_types.insert(name.to_string(), ReportType::new(name, dependencies));
    }
    
    /// Configure reports based on exclude list
    pub fn configure_reports_with_exclusions(&mut self, default_reports: &[&str], exclude_reports: Option<&str>) {
        // First, disable all reports
        for report_type in self.report_types.values_mut() {
            report_type.enabled = false;
        }
        
        // Enable the default reports for this command
        for report_name in default_reports {
            if let Some(report_type) = self.report_types.get_mut(*report_name) {
                report_type.enabled = true;
            }
        }
        
        // If exclude_reports is provided, disable the specified reports
        if let Some(exclude_list) = exclude_reports {
            for report_name in exclude_list.split(',').map(|s| s.trim()) {
                if !report_name.is_empty() {
                    if let Some(report_type) = self.report_types.get_mut(report_name) {
                        report_type.enabled = false;
                    } else if report_name.contains('*') {
                        // Handle wildcard patterns
                        let pattern = report_name.replace('*', "");
                        for (name, report_type) in self.report_types.iter_mut() {
                            if name.contains(&pattern) {
                                report_type.enabled = false;
                            }
                        }
                    } else {
                        warn!("Unknown report type to exclude: {}", report_name);
                    }
                }
            }
        }
        
        // Update the report config based on enabled reports
        self.report_config = ReportConfig::all_disabled();
        for (name, report_type) in &self.report_types {
            if report_type.enabled {
                self.report_config.enable(name);
            }
        }
    }
    
    /// Get required dependencies for enabled reports
    pub fn get_required_dependencies(&self) -> HashSet<ReportDependency> {
        let mut dependencies = HashSet::new();
        
        for (_, report_type) in &self.report_types {
            if report_type.enabled {
                for dep in &report_type.dependencies {
                    dependencies.insert(*dep);
                }
            }
        }
        
        dependencies
    }
    
    /// Process PBO files and generate reports
    pub async fn process_pbo_files(&mut self, args: &crate::commands::ScanPboArgs) -> Result<()> {
        // Configure reports with default set for PBO scanning and any exclusions
        let default_pbo_reports = ["pbo_extraction_stats"];
        self.configure_reports_with_exclusions(&default_pbo_reports, args.exclude_reports.as_deref());
        
        info!("Processing PBO files from {}", args.input_dir.display());
        
        // Create the scan coordinator and run it
        let coordinator = ScanCoordinator::new(args.clone())?;
        coordinator.run().await?;
        
        info!("PBO files processed successfully");
        Ok(())
    }
    
    /// Process class files and generate reports
    pub async fn process_class_files(&mut self, args: &crate::commands::ScanClassesArgs) -> Result<Vec<ProcessedClass>> {
        // Configure reports with default set for class scanning and any exclusions
        let default_class_reports = [
            "classes", 
            "class_stats", 
            "classes_by_category", 
            "class_hierarchy", 
            "circular_dependencies"
        ];
        self.configure_reports_with_exclusions(&default_class_reports, args.exclude_reports.as_deref());
        
        info!("Processing class files from {}", args.input_dir.display());
        
        // Scan classes without generating reports directly
        let classes = crate::scanning::classes::scan_classes_only(args).await?;
        
        // Generate class reports
        let class_report_dir = self.output_dir.join("class_reports");
        let class_report_manager = ClassReportManager::with_config(&class_report_dir, self.report_config.clone());
        class_report_manager.write_all_reports(&classes)?;
        info!("Class reports generated in {}", class_report_dir.display());
        
        Ok(classes)
    }
    
    /// Process mission files and generate reports
    pub async fn process_mission_files(&mut self, args: &crate::commands::ScanMissionsArgs) -> Result<Vec<MissionDependencyResult>> {
        // Configure reports with default set for mission scanning and any exclusions
        let default_mission_reports = [
            "mission_summary", 
            "mission_info", 
            "equipment_items", 
            "dependencies"
        ];
        self.configure_reports_with_exclusions(&default_mission_reports, args.exclude_reports.as_deref());
        
        info!("Processing mission files from {}", args.input_dir.display());
        
        // Scan missions without generating reports directly
        let mission_results = crate::scanning::missions::scan_missions_only(args).await?;
        
        // Generate mission reports
        let mission_report_dir = self.output_dir.join("mission_reports");
        let mission_report_manager = MissionReportManager::with_config(&mission_report_dir, self.report_config.clone());
        mission_report_manager.write_reports(&mission_results)?;
        info!("Mission reports generated in {}", mission_report_dir.display());
        
        Ok(mission_results)
    }
    
    /// Check if there are any enabled reports with the specified dependency
    fn has_enabled_reports_with_dependency(&self, dependency: ReportDependency) -> bool {
        for (_, report_type) in &self.report_types {
            if report_type.enabled && report_type.dependencies.contains(&dependency) {
                return true;
            }
        }
        false
    }
    
    /// Process class validation and generate reports
    pub async fn process_class_validation(&mut self, 
        classes: &[ProcessedClass], 
        mission_results: &[MissionDependencyResult]
    ) -> Result<()> {
        info!("Validating classes against mission dependencies");
        
        // Create a class validator and load the class database from memory
        let mut validator = crate::scanning::missions::validator::ClassExistenceValidator::new();
        validator.load_class_database_from_memory(classes)?;
        
        // Validate mission classes
        let class_existence_report = validator.validate_mission_classes(mission_results)?;
        
        // Generate validation reports
        let validation_report_dir = self.output_dir.join("validation_reports");
        std::fs::create_dir_all(&validation_report_dir)?;
        
        let dependency_report_manager = DependencyReportManager::with_config(&validation_report_dir, self.report_config.clone());
        dependency_report_manager.write_dependency_report(mission_results)?;
        dependency_report_manager.write_class_existence_report(&class_existence_report)?;
        
        info!("Class validation reports generated in {}", validation_report_dir.display());
        Ok(())
    }
    
    /// Run a full analysis pipeline
    pub async fn run_full_analysis(&mut self, args: &crate::commands::FullAnalysisArgs) -> Result<()> {
        info!("Running full analysis pipeline");
        
        // Create necessary directories
        std::fs::create_dir_all(&self.output_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        
        // Create cache directories for base game and mods
        let base_game_cache_dir = self.cache_dir.join("base_game_cache");
        let mods_cache_dir = self.cache_dir.join("mods_cache");
        let mission_cache_dir = self.cache_dir.join("mission_cache");
        
        std::fs::create_dir_all(&base_game_cache_dir)?;
        std::fs::create_dir_all(&mods_cache_dir)?;
        std::fs::create_dir_all(&mission_cache_dir)?;
        
        // Configure reports with default set for full analysis and any exclusions
        let default_full_analysis_reports = [
            "classes",
            "class_stats",
            "classes_by_category",
            "class_hierarchy",
            "mission_summary",
            "mission_info",
            "equipment_items",
            "dependency_report",
            "class_existence_validation",
            "missing_classes",
            "class_usage_frequency",
            "mission_compatibility"
        ];
        self.configure_reports_with_exclusions(&default_full_analysis_reports, args.exclude_reports.as_deref());
        
        // Gather required dependencies up front
        let required_dependencies = self.get_required_dependencies();
        info!("Required dependencies for selected reports: {:?}", required_dependencies);
        
        // Initialize containers for processed data
        let mut all_processed_classes = Vec::new();
        
        // Process base game PBOs
        let base_game_pbo_args = crate::commands::ScanPboArgs {
            input_dir: args.arma3_dir.clone(),
            cache_dir: base_game_cache_dir.clone(),
            extensions: "hpp,cpp,sqf,sqm".to_string(),
            threads: args.threads,
            exclude_reports: None,
        };
        
        info!("Processing base game PBOs from {}", args.arma3_dir.display());
        self.process_pbo_files(&base_game_pbo_args).await?;
        
        // Process mods PBOs
        let mods_pbo_args = crate::commands::ScanPboArgs {
            input_dir: args.mods_dir.clone(),
            cache_dir: mods_cache_dir.clone(),
            extensions: "hpp,cpp,sqf,sqm".to_string(),
            threads: args.threads,
            exclude_reports: None,
        };
        
        info!("Processing mods PBOs from {}", args.mods_dir.display());
        self.process_pbo_files(&mods_pbo_args).await?;
        
        // Process base game classes
        let base_game_class_args = crate::commands::ScanClassesArgs {
            input_dir: base_game_cache_dir.clone(),
            output_dir: self.output_dir.join("class_reports"),
            max_files: None,
            verbose_errors: false,
            exclude_reports: None,
        };
        
        info!("Processing class files from base game cache");
        let base_game_classes = self.process_class_files(&base_game_class_args).await?;
        all_processed_classes.extend(base_game_classes);
        
        // Process mod classes
        let mods_class_args = crate::commands::ScanClassesArgs {
            input_dir: mods_cache_dir.clone(),
            output_dir: self.output_dir.join("class_reports"),
            max_files: None,
            verbose_errors: false,
            exclude_reports: None,
        };
        
        info!("Processing class files from mods cache");
        let mod_classes = self.process_class_files(&mods_class_args).await?;
        all_processed_classes.extend(mod_classes);
        
        // Process mission files
        let mission_args = crate::commands::ScanMissionsArgs {
            input_dir: args.missions_dir.clone(),
            cache_dir: mission_cache_dir,
            output_dir: self.output_dir.join("mission_reports"),
            threads: args.threads,
            exclude_reports: None,
        };
        
        info!("Processing mission files");
        let mission_results = self.process_mission_files(&mission_args).await?;
        
        // Process class validation
        if !all_processed_classes.is_empty() && !mission_results.is_empty() {
            self.process_class_validation(&all_processed_classes, &mission_results).await?;
        }
        
        info!("Full analysis completed successfully");
        Ok(())
    }
    
    /// Process mission dependency analysis
    pub async fn process_mission_dependency_analysis(&mut self, args: &crate::commands::MissionDependencyAnalysisArgs) -> Result<()> {
        info!("Running mission dependency analysis");
        
        // Create necessary directories
        std::fs::create_dir_all(&self.output_dir)?;
        std::fs::create_dir_all(&args.cache_dir)?;
        
        // Configure reports with default set for dependency analysis and any exclusions
        let default_dependency_reports = [
            "dependency_report",
            "class_existence_validation",
            "missing_classes",
            "class_usage_frequency",
            "mission_compatibility"
        ];
        self.configure_reports_with_exclusions(&default_dependency_reports, args.exclude_reports.as_deref());
        
        // Initialize container for processed classes
        let mut all_processed_classes = Vec::new();
        
        // Process base game PBOs if arma3_dir is provided
        if let Some(arma3_dir) = &args.arma3_dir {
            info!("Processing base game PBOs from {}", arma3_dir.display());
            let base_game_cache_dir = args.cache_dir.join("base_game_cache");
            std::fs::create_dir_all(&base_game_cache_dir)?;
            
            let base_game_pbo_args = crate::commands::ScanPboArgs {
                input_dir: arma3_dir.clone(),
                cache_dir: base_game_cache_dir.clone(),
                extensions: "hpp,cpp,sqf,sqm".to_string(),
                threads: args.threads,
                exclude_reports: None,
            };
            
            self.process_pbo_files(&base_game_pbo_args).await?;
            
            // Scan classes from base game cache
            info!("Scanning classes from base game cache");
            let base_game_class_args = crate::commands::ScanClassesArgs {
                input_dir: base_game_cache_dir,
                output_dir: self.output_dir.join("class_reports").join("base_game"),
                max_files: None,
                verbose_errors: false,
                exclude_reports: None,
            };
            
            let base_game_classes = self.process_class_files(&base_game_class_args).await?;
            all_processed_classes.extend(base_game_classes);
        }
        
        // Process mod PBOs if mods_dir is provided
        if let Some(mods_dir) = &args.mods_dir {
            info!("Processing mod PBOs from {}", mods_dir.display());
            let mods_cache_dir = args.cache_dir.join("mods_cache");
            std::fs::create_dir_all(&mods_cache_dir)?;
            
            let mods_pbo_args = crate::commands::ScanPboArgs {
                input_dir: mods_dir.clone(),
                cache_dir: mods_cache_dir.clone(),
                extensions: "hpp,cpp,sqf,sqm".to_string(),
                threads: args.threads,
                exclude_reports: None,
            };
            
            self.process_pbo_files(&mods_pbo_args).await?;
            
            // Scan classes from mods cache
            info!("Scanning classes from mods cache");
            let mods_class_args = crate::commands::ScanClassesArgs {
                input_dir: mods_cache_dir,
                output_dir: self.output_dir.join("class_reports").join("mods"),
                max_files: None,
                verbose_errors: false,
                exclude_reports: None,
            };
            
            let mod_classes = self.process_class_files(&mods_class_args).await?;
            all_processed_classes.extend(mod_classes);
        }
        
        if all_processed_classes.is_empty() {
            return Err(anyhow::anyhow!("No classes were processed. Either provide a valid class-dir or both arma3-dir and mods-dir"));
        }
        
        info!("Total processed classes: {}", all_processed_classes.len());
        
        // Process mission files
        let mission_args = crate::commands::ScanMissionsArgs {
            input_dir: args.mission_dir.clone(),
            cache_dir: args.cache_dir.clone(),
            output_dir: self.output_dir.join("mission_reports"),
            threads: args.threads,
            exclude_reports: None,
        };
        
        info!("Processing mission files from {}", args.mission_dir.display());
        let mission_results = self.process_mission_files(&mission_args).await?;
        
        if mission_results.is_empty() {
            warn!("No mission files were processed");
            return Ok(());
        }
        
        // Process class validation
        info!("Validating mission dependencies against {} class definitions", all_processed_classes.len());
        self.process_class_validation(&all_processed_classes, &mission_results).await?;
        
        info!("Mission dependency analysis completed successfully");
        Ok(())
    }
} 