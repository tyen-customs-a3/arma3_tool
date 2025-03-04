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
use crate::reporting::missing_classes_report::MissingClassesReportWriter;
use crate::logging::log_dependency_analysis;

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
        self.register_report("detailed_missing_classes", vec![ReportDependency::MissionScan, ReportDependency::ClassScan]);
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
    pub async fn process_class_validation_for_missions(&mut self, 
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

    /// Process class validation
    pub async fn process_class_validation(&mut self, args: &crate::commands::ValidateClassesArgs) -> Result<()> {
        // Configure reports with default set for class validation and any exclusions
        let default_validation_reports = ["class_validation"];
        self.configure_reports_with_exclusions(&default_validation_reports, args.exclude_reports.as_deref());
        
        info!("Processing class validation for specified classes");
        
        // Process class files first
        let class_args = crate::commands::ScanClassesArgs {
            input_dir: args.input_dir.clone(),
            output_dir: args.output_dir.clone(),
            max_files: None,
            verbose_errors: false,
            exclude_reports: None,
        };
        
        let classes = self.process_class_files(&class_args).await?;
        
        if classes.is_empty() {
            return Err(anyhow::anyhow!("No classes were found in the input directory"));
        }
        
        // Split the comma-separated class names into a vector
        let class_names: Vec<String> = args.class_names
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if class_names.is_empty() {
            return Err(anyhow::anyhow!("No class names provided for validation"));
        }
        
        // Create a class report manager and validate the classes
        let mut class_report_manager = crate::reporting::class_reports::ClassReportManager::with_config(
            &args.output_dir,
            self.report_config.clone(),
        );
        
        class_report_manager.validate_class_names(&classes, &class_names)?;
        
        info!("Class validation completed successfully");
        Ok(())
    }
    
    /// Run a full analysis pipeline
    pub async fn run_full_analysis(&mut self, args: &crate::commands::FullAnalysisArgs) -> Result<()> {
        info!("Running full analysis pipeline");
        
        // Create necessary directories
        std::fs::create_dir_all(&self.output_dir)?;
        std::fs::create_dir_all(&self.cache_dir)?;
        
        // Create cache directories for base game and mods
        let arma3_dir = self.cache_dir.join("arma3");
        let mods_dir = self.cache_dir.join("mods");
        let missions_dir = self.cache_dir.join("missions");
        
        std::fs::create_dir_all(&arma3_dir)?;
        std::fs::create_dir_all(&mods_dir)?;
        std::fs::create_dir_all(&missions_dir)?;
        
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
            cache_dir: arma3_dir.clone(),
            extensions: "hpp,cpp,sqf,sqm".to_string(),
            threads: args.threads,
            exclude_reports: None,
        };
        
        info!("Processing base game PBOs from {}", args.arma3_dir.display());
        self.process_pbo_files(&base_game_pbo_args).await?;
        
        // Process mods PBOs
        let mods_pbo_args = crate::commands::ScanPboArgs {
            input_dir: args.mods_dir.clone(),
            cache_dir: mods_dir.clone(),
            extensions: "hpp,cpp,sqf,sqm".to_string(),
            threads: args.threads,
            exclude_reports: None,
        };
        
        info!("Processing mods PBOs from {}", args.mods_dir.display());
        self.process_pbo_files(&mods_pbo_args).await?;
        
        // Process base game classes
        let base_game_class_args = crate::commands::ScanClassesArgs {
            input_dir: arma3_dir.clone(),
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
            input_dir: mods_dir.clone(),
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
            cache_dir: missions_dir,
            output_dir: self.output_dir.join("mission_reports"),
            threads: args.threads,
            exclude_reports: None,
        };
        
        info!("Processing mission files");
        let mission_results = self.process_mission_files(&mission_args).await?;
        
        // Process class validation
        if !all_processed_classes.is_empty() && !mission_results.is_empty() {
            self.process_class_validation_for_missions(&all_processed_classes, &mission_results).await?;
        }
        
        info!("Full analysis completed successfully");
        Ok(())
    }
    
    /// Process mission dependency analysis
    pub async fn process_mission_dependency_analysis(&mut self, args: &crate::commands::MissionDependencyAnalysisArgs) -> Result<()> {
        log_dependency_analysis("info", &format!("Starting mission dependency analysis from {}", args.mission_dir.display()));
        
        // Configure reports with default set for dependency analysis and any exclusions
        let default_dependency_reports = [
            "mission_dependencies", 
            "missing_dependencies", 
            "dependency_summary"
        ];
        self.configure_reports_with_exclusions(&default_dependency_reports, args.exclude_reports.as_deref());
        
        // Create a class scanner to process class files from Arma 3 and mods
        let mut class_files = Vec::new();
        
        // Create cache subdirectories
        let arma3_cache_dir = args.cache_dir.join("arma3");
        let mods_cache_dir = args.cache_dir.join("mods");
        
        // If arma3_dir is provided, scan the cache directory for class files
        if let Some(arma3_dir) = &args.arma3_dir {
            log_dependency_analysis("info", &format!("Scanning Arma 3 base game cache directory: {}", arma3_cache_dir.display()));
            
            // Create the cache directory if it doesn't exist
            if !arma3_cache_dir.exists() {
                std::fs::create_dir_all(&arma3_cache_dir)
                    .context("Failed to create Arma 3 cache directory")?;
                
                // Extract PBO files from Arma 3 directory to cache
                log_dependency_analysis("info", &format!("Extracting PBO files from Arma 3 directory: {}", arma3_dir.display()));
                let pbo_args = crate::commands::ScanPboArgs {
                    input_dir: arma3_dir.clone(),
                    cache_dir: arma3_cache_dir.clone(),
                    extensions: "hpp,cpp,sqf,sqm".to_string(),
                    threads: args.threads,
                    exclude_reports: None,
                };
                
                // Scan PBOs from Arma 3 directory
                crate::scanning::pbo::scan_pbos(pbo_args).await?;
            }
            
            let arma3_class_args = crate::commands::ScanClassesArgs {
                input_dir: arma3_cache_dir,
                output_dir: PathBuf::from("arma3_classes"),
                max_files: None,
                verbose_errors: false,
                exclude_reports: None,
            };
            
            // Process class files from Arma 3 base game cache
            let arma3_classes = crate::scanning::classes::scan_classes_only(&arma3_class_args).await?;
            log_dependency_analysis("info", &format!("Found {} class definitions in Arma 3 base game", arma3_classes.len()));
            class_files.extend(arma3_classes);
        }
        
        // If mods_dir is provided, scan the cache directory for class files
        if let Some(mods_dir) = &args.mods_dir {
            log_dependency_analysis("info", &format!("Scanning mods cache directory: {}", mods_cache_dir.display()));
            
            // Create the cache directory if it doesn't exist
            if !mods_cache_dir.exists() {
                std::fs::create_dir_all(&mods_cache_dir)
                    .context("Failed to create mods cache directory")?;
                
                // Extract PBO files from mods directory to cache
                log_dependency_analysis("info", &format!("Extracting PBO files from mods directory: {}", mods_dir.display()));
                let pbo_args = crate::commands::ScanPboArgs {
                    input_dir: mods_dir.clone(),
                    cache_dir: mods_cache_dir.clone(),
                    extensions: "hpp,cpp,sqf,sqm".to_string(),
                    threads: args.threads,
                    exclude_reports: None,
                };
                
                // Scan PBOs from mods directory
                crate::scanning::pbo::scan_pbos(pbo_args).await?;
            }
            
            let mods_class_args = crate::commands::ScanClassesArgs {
                input_dir: mods_cache_dir,
                output_dir: PathBuf::from("mods_classes"),
                max_files: None,
                verbose_errors: false,
                exclude_reports: None,
            };
            
            // Process class files from mods cache
            let mod_classes = crate::scanning::classes::scan_classes_only(&mods_class_args).await?;
            log_dependency_analysis("info", &format!("Found {} class definitions in mods", mod_classes.len()));
            class_files.extend(mod_classes);
        }
        
        // Scan mission files
        log_dependency_analysis("info", &format!("Scanning mission files from {}", args.mission_dir.display()));
        let mission_args = crate::commands::ScanMissionsArgs {
            input_dir: args.mission_dir.clone(),
            cache_dir: args.cache_dir.clone(),
            output_dir: PathBuf::from("mission_scan"),
            threads: args.threads,
            exclude_reports: None,
        };
        
        // Process mission files
        let mission_results = crate::scanning::missions::scan_missions_only(&mission_args).await?;
        log_dependency_analysis("info", &format!("Found {} mission files", mission_results.len()));
        
        // Generate dependency reports
        let dependency_report_dir = self.output_dir.join("dependency_reports");
        let mut dependency_report_manager = DependencyReportManager::with_config(
            &dependency_report_dir, 
            self.report_config.clone()
        );
        
        // Set available classes for dependency checking
        dependency_report_manager.set_available_classes(class_files);
        
        // Write dependency reports
        dependency_report_manager.write_dependency_report(&mission_results)?;
        log_dependency_analysis("info", &format!("Dependency reports generated in {}", dependency_report_dir.display()));
        
        Ok(())
    }
    
    /// Process missing classes report
    pub async fn process_missing_classes_report(&mut self, args: &crate::commands::MissingClassesReportArgs) -> Result<()> {
        info!("Generating detailed missing classes report");
        
        // Configure reports with default set for missing classes report and any exclusions
        let default_reports = ["detailed_missing_classes"];
        self.configure_reports_with_exclusions(&default_reports, args.exclude_reports.as_deref());
        
        // Step 1: Extract PBO files from Arma 3 and mods directories to cache
        let mut pbo_dirs = Vec::new();
        
        if let Some(arma3_dir) = &args.arma3_dir {
            info!("Extracting PBO files from Arma 3 directory: {}", arma3_dir.display());
            
            // Create PBO scan args for Arma 3 directory
            let arma3_pbo_args = crate::commands::ScanPboArgs {
                input_dir: arma3_dir.clone(),
                cache_dir: self.cache_dir.join("arma3"),
                extensions: "hpp,cpp".to_string(),
                threads: args.threads,
                exclude_reports: None,
            };
            
            // Extract PBO files from Arma 3 directory
            let coordinator = crate::scanning::pbo::coordinator::ScanCoordinator::new(arma3_pbo_args.clone())?;
            coordinator.run().await?;
            
            // Add the cache directory to the list of directories to scan for class files
            pbo_dirs.push(arma3_pbo_args.cache_dir);
        }
        
        if let Some(mods_dir) = &args.mods_dir {
            info!("Extracting PBO files from mods directory: {}", mods_dir.display());
            
            // Create PBO scan args for mods directory
            let mods_pbo_args = crate::commands::ScanPboArgs {
                input_dir: mods_dir.clone(),
                cache_dir: self.cache_dir.join("mods"),
                extensions: "hpp,cpp".to_string(),
                threads: args.threads,
                exclude_reports: None,
            };
            
            // Extract PBO files from mods directory
            let coordinator = crate::scanning::pbo::coordinator::ScanCoordinator::new(mods_pbo_args.clone())?;
            coordinator.run().await?;
            
            // Add the cache directory to the list of directories to scan for class files
            pbo_dirs.push(mods_pbo_args.cache_dir);
        }
        
        if pbo_dirs.is_empty() {
            warn!("No Arma 3 or mods directories provided. Please provide valid Arma 3 and/or mods directories.");
            return Ok(());
        }
        
        // Step 2: Collect class files from extracted PBO directories
        info!("Collecting class files from extracted PBO directories");
        let mut class_files = Vec::new();
        
        for dir in &pbo_dirs {
            info!("Scanning class files from directory: {}", dir.display());
            let dir_class_files = self.collect_class_files(dir)?;
            info!("Found {} class files in directory {}", dir_class_files.len(), dir.display());
            class_files.extend(dir_class_files);
        }
        
        if class_files.is_empty() {
            warn!("No class files found in extracted PBO directories. Please check the Arma 3 and mods directories.");
            return Ok(());
        }
        
        // Step 3: Process class files to get available classes
        info!("Processing {} class files", class_files.len());
        let class_scan_args = crate::commands::ScanClassesArgs {
            input_dir: PathBuf::from("."), // Dummy value, not used
            output_dir: self.output_dir.join("class_reports"),
            max_files: None,
            verbose_errors: false,
            exclude_reports: None,
        };
        
        // Use the scanner and processor directly
        let (classes, _) = crate::scanning::classes::processor::process_classes(
            &class_files,
            class_scan_args.verbose_errors,
            &self.cache_dir
        )?;
        info!("Processed {} classes", classes.len());
        
        // Step 4: Scan mission files
        info!("Scanning mission files from {}", args.mission_dir.display());
        let mission_scan_args = crate::commands::ScanMissionsArgs {
            input_dir: args.mission_dir.clone(),
            cache_dir: args.cache_dir.clone(),
            output_dir: self.output_dir.join("mission_reports"),
            threads: args.threads,
            exclude_reports: None,
        };
        
        let mission_results = crate::scanning::missions::scan_missions_only(&mission_scan_args).await?;
        info!("Scanned {} missions", mission_results.len());
        
        // Step 5: Generate the detailed missing classes report
        info!("Generating detailed missing classes report");
        let report_writer = MissingClassesReportWriter::with_config(
            &self.output_dir,
            ReportFormat::Json,
            self.report_config.clone(),
        );
        
        report_writer.write_detailed_missing_classes_report(&mission_results, &classes)?;
        info!("Detailed missing classes report generated in {}", self.output_dir.display());
        
        Ok(())
    }
    
    /// Collect class files from a directory
    fn collect_class_files(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut class_files = Vec::new();
        
        for entry in walkdir::WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }
            
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "cpp" || ext == "hpp" {
                    class_files.push(path.to_owned());
                }
            }
        }
        
        Ok(class_files)
    }
} 