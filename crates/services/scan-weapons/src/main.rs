use clap::Parser;
use anyhow::Result;
use std::path::{PathBuf, Path};

// Use the library crate instead of local modules
use arma3_scan_weapons::{
    WeaponMagazineScanner,
    Database,
    ReportGenerator,
    ConfigManager,
    config::CliArgsProvider,
    report::WeaponsByModExporter,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Target folder to scan for weapon and magazine data
    #[arg(short, long)]
    pub target: Option<PathBuf>,

    /// Output file for the report (overrides config file)
    #[arg(short, long, default_value = "weapons_magazines_report.json")]
    pub output: PathBuf,

    /// Database file for caching scan results (overrides config file)
    #[arg(short, long, default_value = "weapon_magazine_cache.db")]
    pub database: PathBuf,

    /// Force rescan even if files haven't changed
    #[arg(short, long)]
    pub force: bool,

    /// Number of threads to use for parallel scanning (overrides config file)
    #[arg(short = 'j', long, default_value_t = num_cpus::get())]
    pub threads: usize,

    /// Output format (json, yaml, csv, text) (overrides config file)
    #[arg(long, default_value = "json")]
    pub format: String,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Timeout per file in seconds (overrides config file)
    #[arg(long, default_value_t = 30)]
    pub timeout: u64,

    /// Project root directory for parser (overrides config file)
    /// This should be the root containing all config files to be scanned
    #[arg(long)]
    pub project_root: Option<PathBuf>,

    /// Path to configuration file (YAML, JSON, or TOML)
    /// If not specified, will look for config.yaml, config.json, etc. in current directory
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Generate a sample configuration file and exit
    #[arg(long)]
    pub generate_config: Option<PathBuf>,

    /// Output file for weapons grouped by mod (overrides config file)
    #[arg(long)]
    pub weapons_by_mod: Option<PathBuf>,
}

impl CliArgsProvider for Args {
    fn target(&self) -> Option<&PathBuf> {
        self.target.as_ref()
    }
    
    fn output(&self) -> &PathBuf {
        &self.output
    }
    
    fn database(&self) -> &PathBuf {
        &self.database
    }
    
    fn force(&self) -> bool {
        self.force
    }
    
    fn threads(&self) -> usize {
        self.threads
    }
    
    fn format(&self) -> &String {
        &self.format
    }
    
    fn verbose(&self) -> bool {
        self.verbose
    }
    
    fn timeout(&self) -> u64 {
        self.timeout
    }
    
    fn project_root(&self) -> &Option<PathBuf> {
        &self.project_root
    }

    fn weapons_by_mod(&self) -> &Option<PathBuf> {
        &self.weapons_by_mod
    }
}

/// Convert Windows UNC path to clean display format
fn clean_path_display(path: &Path) -> String {
    let path_str = path.to_string_lossy();
    if path_str.starts_with(r"\\?\") {
        path_str.strip_prefix(r"\\?\").unwrap_or(&path_str).to_string()
    } else {
        path_str.to_string()
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle config generation request
    if let Some(config_path) = &args.generate_config {
        let config_manager = ConfigManager::new();
        config_manager.generate_sample_config(config_path)?;
        println!("Sample configuration file generated: {}", config_path.display());
        println!("Edit this file to customize your settings, then run without --generate-config");
        return Ok(());
    }

    // Load configuration
    let config_manager = ConfigManager::new();
    let base_config = config_manager.load_config(args.config.as_deref())?;
    let config = config_manager.merge_with_args(base_config, &args);

    // Initialize logger
    if config.verbose {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    // Log configuration source
    match &args.config {
        Some(path) => log::info!("Using configuration file: {}", path.display()),
        None => log::info!("Using auto-discovered or default configuration"),
    }

    // Validate format
    if !ReportGenerator::is_valid_format(&config.format) {
        anyhow::bail!("Invalid format '{}'. Supported formats: {:?}",
                     config.format, ReportGenerator::available_formats());
    }

    // Validate target path
    if config.target.as_os_str().is_empty() {
        anyhow::bail!("Target directory must be specified in config file or via --target argument");
    }
    if !config.target.exists() {
        anyhow::bail!("Target directory does not exist: {}", config.target.display());
    }

    // Determine project root
    let project_root = match config.project_root {
        Some(specified_root) => {
            if !specified_root.exists() {
                anyhow::bail!("Specified project root does not exist: {}", specified_root.display());
            }
            specified_root
        },
        None => {
            // Auto-detect project root: use target directory or its closest parent that exists
            auto_detect_project_root(&config.target)?
        }
    };

    // Resolve canonical paths
    let canonical_target = config.target.canonicalize()
        .map_err(|e| anyhow::anyhow!("Failed to resolve target path: {}", e))?;
    let canonical_project_root = project_root.canonicalize()
        .map_err(|e| anyhow::anyhow!("Failed to resolve project root path: {}", e))?;

    // Validate that target is within project root
    if !canonical_target.starts_with(&canonical_project_root) {
        // Get a suggested project root - use common parent or target's parent
        let suggested_root = match find_common_parent(&canonical_target, &canonical_project_root)? {
            Some(common) => common,
            None => {
                match canonical_target.parent() {
                    Some(parent) => parent.to_path_buf(),
                    None => canonical_target.clone()
                }
            }
        };
            
        anyhow::bail!(
            "Target directory '{}' is not within project root '{}'.\n\
            Use project_root in config file or --project-root to specify a parent directory that contains the target.\n\
            For example: project_root: \"{}\"",
            clean_path_display(&canonical_target),
            clean_path_display(&canonical_project_root),
            clean_path_display(&suggested_root)
        );
    }

    // Initialize database with configured path
    let mut db = Database::new(&config.database)?;
    
    // Create scanner with proper project root
    let scanner = WeaponMagazineScanner::new(&canonical_project_root, config.threads, config.timeout)?;

    log::info!("Scanner initialized:");
    log::info!("  Project root: {}", clean_path_display(&canonical_project_root));
    log::info!("  Target directory: {}", clean_path_display(&canonical_target));
    log::info!("  Database: {}", config.database.display());
    log::info!("  Output: {}", config.output.display());
    log::info!("  Threads: {}", config.threads);
    log::info!("  Output format: {}", config.format);

    // Check if we need to rescan
    let should_scan = config.force || scanner.should_rescan(&canonical_target, &mut db)?;

    if should_scan {
        log::info!("Scanning target folder: {}", clean_path_display(&canonical_target));
        let scan_result = scanner.scan(&canonical_target)?;
        
        log::info!("Scan completed successfully:");
        log::info!("  Weapons found: {}", scan_result.weapons.len());
        log::info!("  Magazine wells found: {}", scan_result.magazine_wells.len());
        
        if config.verbose {
            log::info!("  Weapons:");
            for weapon in &scan_result.weapons {
                log::info!("    - {}: {} magazine wells, {} compatible magazines",
                          weapon.name, weapon.magazine_wells.len(), weapon.compatible_magazines.len());
            }
            
            log::info!("  Magazine wells:");
            for (name, well) in &scan_result.magazine_wells {
                let total_magazines: usize = well.magazines.values().map(|v| v.len()).sum();
                log::info!("    - {}: {} magazines", name, total_magazines);
            }
        }
        
        // Save to database using optimized binary format
        if let Err(e) = db.save_scan_result_binary(&scan_result) {
            log::warn!("Failed to save binary cache: {}, falling back to JSON", e);
            db.save_scan_result(&scan_result)?;
        }
        
        // Generate report
        let report_gen = ReportGenerator::new();
        report_gen.generate_with_project_root(&scan_result, &config.output, &config.format, &canonical_project_root)?;
        
        log::info!("Report generated: {}", config.output.display());

        // Generate weapons-by-mod report if requested
        if let Some(ref weapons_by_mod_path) = config.weapons_by_mod {
            let weapons_by_mod_exporter = WeaponsByModExporter::new();
            weapons_by_mod_exporter.export_weapons_by_mod(&scan_result, weapons_by_mod_path)?;
            
            // Log statistics about mod distribution
            let mod_stats = weapons_by_mod_exporter.get_mod_statistics(&scan_result.weapons);
            log::info!("Weapons by mod statistics:");
            log::info!("  Total mods with weapons: {}", mod_stats.total_mods);
            log::info!("  Total weapons grouped: {}", mod_stats.total_weapons);
            log::info!("  Average weapons per mod: {:.1}", mod_stats.avg_weapons_per_mod);
            if let Some((mod_name, count)) = &mod_stats.mod_with_most_weapons {
                log::info!("  Mod with most weapons: {} ({} weapons)", mod_name, count);
            }
        }
    } else {
        log::info!("No changes detected, loading from cache");
        
        // Load from database using optimized method
        let scan_result = db.load_scan_result()?;
        
        log::info!("Loaded from cache:");
        log::info!("  Weapons: {}", scan_result.weapons.len());
        log::info!("  Magazine wells: {}", scan_result.magazine_wells.len());
        
        // Generate report
        let report_gen = ReportGenerator::new();
        report_gen.generate_with_project_root(&scan_result, &config.output, &config.format, &canonical_project_root)?;
        
        log::info!("Report generated from cache: {}", config.output.display());

        // Generate weapons-by-mod report if requested
        if let Some(ref weapons_by_mod_path) = config.weapons_by_mod {
            let weapons_by_mod_exporter = WeaponsByModExporter::new();
            weapons_by_mod_exporter.export_weapons_by_mod(&scan_result, weapons_by_mod_path)?;
            
            // Log statistics about mod distribution
            let mod_stats = weapons_by_mod_exporter.get_mod_statistics(&scan_result.weapons);
            log::info!("Weapons by mod statistics:");
            log::info!("  Total mods with weapons: {}", mod_stats.total_mods);
            log::info!("  Total weapons grouped: {}", mod_stats.total_weapons);
            log::info!("  Average weapons per mod: {:.1}", mod_stats.avg_weapons_per_mod);
            if let Some((mod_name, count)) = &mod_stats.mod_with_most_weapons {
                log::info!("  Mod with most weapons: {} ({} weapons)", mod_name, count);
            }
        }
    }

    Ok(())
}

/// Auto-detect the best project root for scanning
fn auto_detect_project_root(target: &Path) -> Result<PathBuf> {
    // For external directories, use the target itself as the project root
    // This allows scanning any directory structure
    let canonical_target = target.canonicalize()
        .map_err(|e| anyhow::anyhow!("Failed to resolve target path: {}", e))?;
    
    // Try to find a reasonable project root by going up the directory tree
    // and looking for common project markers or just use the target itself
    let mut current = canonical_target.as_path();
    
    // Look for common Arma 3 project indicators
    loop {
        // Check for common project root indicators
        if current.join("config.cpp").exists() || 
           current.join("CfgPatches").exists() ||
           current.join("gamedata").exists() ||
           current.join("addons").exists() ||
           current.join("mod.cpp").exists() {
            log::info!("Auto-detected project root: {}", clean_path_display(current));
            return Ok(current.to_path_buf());
        }
        
        // Move up one directory
        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }
    
    // If no project markers found, use the target directory itself
    log::info!("No project markers found, using target as project root: {}", clean_path_display(&canonical_target));
    Ok(canonical_target)
}

/// Find a common parent directory between two paths
fn find_common_parent(path1: &PathBuf, path2: &PathBuf) -> Result<Option<PathBuf>> {
    let ancestors1: Vec<_> = path1.ancestors().collect();
    let ancestors2: Vec<_> = path2.ancestors().collect();
    
    for ancestor1 in &ancestors1 {
        for ancestor2 in &ancestors2 {
            if ancestor1 == ancestor2 {
                return Ok(Some(ancestor1.to_path_buf()));
            }
        }
    }
    
    Ok(None)
}
