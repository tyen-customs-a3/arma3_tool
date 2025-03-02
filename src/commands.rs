use std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan and extract files from PBO archives
    #[command(name = "scan-pbos")]
    ScanPbos(ScanPboArgs),
    /// Scan and analyze class definitions in cpp files
    #[command(name = "scan-classes")]
    ScanClasses(ScanClassesArgs),
    /// Scan mission files, extract PBOs, and analyze equipment dependencies
    #[command(name = "scan-missions")]
    ScanMissions(ScanMissionsArgs),
    /// Run a complete analysis pipeline: extract PBOs, scan missions, and verify class dependencies
    /// 
    /// This command extracts mission PBOs, analyzes their class dependencies, and can validate
    /// if the classes used in missions exist in a provided class database.
    #[command(name = "analyze-mission-dependencies")]
    AnalyzeMissionDependencies(AnalyzeMissionDependenciesArgs),
    /// Run a complete analysis pipeline for Arma 3 base game, mods, and missions
    #[command(name = "full-analysis")]
    FullAnalysis(FullAnalysisArgs),
}

#[derive(clap::Args, Debug)]
pub struct ScanPboArgs {
    /// Input directory containing PBO files (recursive search)
    #[arg(short, long)]
    pub input_dir: PathBuf,

    /// Cache directory for extracted files
    #[arg(short, long, default_value = "./cache")]
    pub cache_dir: PathBuf,

    /// File extensions to extract (comma-separated)
    #[arg(short, long, default_value = "hpp,cpp,sqf,sqm")]
    pub extensions: String,

    /// Number of parallel extraction threads
    #[arg(short, long, default_value = "4")]
    pub threads: usize,
}

#[derive(clap::Args, Debug)]
pub struct ScanClassesArgs {
    /// Input directory containing cpp files to scan
    #[arg(short, long)]
    pub input_dir: PathBuf,

    /// Output directory for the class reports
    #[arg(short, long, default_value = "class_reports")]
    pub output_dir: PathBuf,
    
    /// Maximum number of files to process (useful for debugging)
    #[arg(long)]
    pub max_files: Option<usize>,
    
    /// Enable verbose error reporting for parse errors
    #[arg(short, long)]
    pub verbose_errors: bool,
    
    /// Disable specific reports (comma-separated list)
    /// 
    /// Available report types:
    /// - classes: Main class list report
    /// - class_stats: Class statistics report
    /// - classes_by_category: Classes categorized by type
    /// - class_mission_usage: Classes used in missions
    /// - class_hierarchy: Class inheritance hierarchy
    /// - circular_dependencies: Circular dependencies in class hierarchy
    /// - category_*: Individual category reports
    #[arg(long)]
    pub disable_reports: Option<String>,
    
    /// Enable only specific reports (comma-separated list)
    /// 
    /// If specified, only the listed reports will be generated.
    /// This overrides --disable-reports if both are specified.
    #[arg(long)]
    pub enable_reports: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct ScanMissionsArgs {
    /// Input directory containing mission PBO files (recursive search)
    #[arg(short, long)]
    pub input_dir: PathBuf,

    /// Cache directory for extracted mission files
    #[arg(short, long, default_value = "./mission_cache")]
    pub cache_dir: PathBuf,

    /// Output directory for mission equipment reports
    #[arg(short, long, default_value = "mission_reports")]
    pub output_dir: PathBuf,

    /// Number of parallel extraction threads
    #[arg(short, long, default_value = "4")]
    pub threads: usize,
    
    /// Disable specific reports (comma-separated list)
    /// 
    /// Available report types:
    /// - mission_summary: Summary of all missions
    /// - mission_info: Basic mission information
    /// - equipment_items: Equipment items used in missions
    /// - dependencies: Missing dependencies in missions
    /// - mission_*: Reports for specific missions
    #[arg(long)]
    pub disable_reports: Option<String>,
    
    /// Enable only specific reports (comma-separated list)
    /// 
    /// If specified, only the listed reports will be generated.
    /// This overrides --disable-reports if both are specified.
    #[arg(long)]
    pub enable_reports: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct AnalyzeMissionDependenciesArgs {
    /// Input directory containing mission PBO files
    #[arg(short, long)]
    pub mission_dir: PathBuf,

    /// Input directory containing addon PBO files
    #[arg(short, long)]
    pub addon_dir: PathBuf,

    /// Cache directory for extracted files
    #[arg(short, long, default_value = "./analysis_cache")]
    pub cache_dir: PathBuf,

    /// Output directory for analysis reports
    #[arg(short, long, default_value = "analysis_reports")]
    pub output_dir: PathBuf,

    /// Number of parallel extraction threads
    #[arg(short, long, default_value = "4")]
    pub threads: usize,
    
    /// Directory containing class database files for validation
    #[arg(long)]
    pub class_db_dir: Option<PathBuf>,
    
    /// Disable specific reports (comma-separated list)
    /// 
    /// Available report types:
    /// - dependency_report: Summary of dependencies
    /// - missing_classes: Missing classes across all missions
    /// - class_usage_frequency: Class usage frequency
    /// - mission_compatibility: Mission compatibility with available classes
    /// - category_needs: Class categories needed by missions
    /// - class_inheritance: Class inheritance relationships
    /// - compatibility_diagnostics: Detailed compatibility diagnostics
    /// - class_existence_validation: Class existence validation
    #[arg(long)]
    pub disable_reports: Option<String>,
    
    /// Enable only specific reports (comma-separated list)
    /// 
    /// If specified, only the listed reports will be generated.
    /// This overrides --disable-reports if both are specified.
    #[arg(long)]
    pub enable_reports: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct FullAnalysisArgs {
    /// Arma 3 base game directory
    #[arg(long, default_value = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Arma 3")]
    pub arma3_dir: PathBuf,

    /// Mods directory
    #[arg(long)]
    pub mods_dir: PathBuf,

    /// Missions directory
    #[arg(long)]
    pub missions_dir: PathBuf,

    /// Cache directory for extracted files
    #[arg(short, long, default_value = "./cache")]
    pub cache_dir: PathBuf,

    /// Output directory for analysis reports
    #[arg(short, long, default_value = "./reports")]
    pub output_dir: PathBuf,

    /// Number of parallel extraction threads
    #[arg(short, long, default_value = "4")]
    pub threads: usize,
    
    /// Disable specific reports (comma-separated list)
    /// 
    /// This can include any report type from scan-classes, scan-missions,
    /// and analyze-mission-dependencies commands.
    #[arg(long)]
    pub disable_reports: Option<String>,
    
    /// Enable only specific reports (comma-separated list)
    /// 
    /// If specified, only the listed reports will be generated.
    /// This overrides --disable-reports if both are specified.
    #[arg(long)]
    pub enable_reports: Option<String>,
}