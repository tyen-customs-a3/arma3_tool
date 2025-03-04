use std::path::PathBuf;
use clap::{Args, Subcommand};

#[derive(Subcommand, Debug, Clone)]
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
    /// Analyze mission dependencies against class definitions
    #[command(name = "mission-dependency-analysis")]
    MissionDependencyAnalysis(MissionDependencyAnalysisArgs),
    /// Run a complete analysis pipeline for Arma 3 base game, mods, and missions
    #[command(name = "full-analysis")]
    FullAnalysis(FullAnalysisArgs),
    /// Validate specific class names exist in class definitions
    #[command(name = "validate-classes")]
    ValidateClasses(ValidateClassesArgs),
    /// Generate a detailed missing classes report for missions
    #[command(name = "missing-classes-report")]
    MissingClassesReport(MissingClassesReportArgs),
}

#[derive(Args, Debug, Clone)]
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
    
    /// Reports to exclude (comma-separated)
    #[arg(long)]
    pub exclude_reports: Option<String>,
}

#[derive(Args, Debug, Clone)]
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
    
    /// Reports to exclude (comma-separated)
    #[arg(long)]
    pub exclude_reports: Option<String>,
}

#[derive(Args, Debug, Clone)]
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
    
    /// Reports to exclude (comma-separated)
    #[arg(long)]
    pub exclude_reports: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct MissionDependencyAnalysisArgs {

    /// Arma 3 base game directory (optional if class-dir is provided)
    #[arg(long)]
    pub arma3_dir: Option<PathBuf>,

    /// Mods directory (optional if class-dir is provided)
    #[arg(long)]
    pub mods_dir: Option<PathBuf>,

    /// Input directory containing mission files
    #[arg(short, long)]
    pub mission_dir: PathBuf,

    /// Output directory for dependency reports
    #[arg(short, long, default_value = "dependency_reports")]
    pub output_dir: PathBuf,

    /// Cache directory for extracted files
    #[arg(short, long, default_value = "./dependency_cache")]
    pub cache_dir: PathBuf,

    /// Number of parallel processing threads
    #[arg(short, long, default_value = "4")]
    pub threads: usize,
    
    /// Reports to exclude (comma-separated)
    #[arg(long)]
    pub exclude_reports: Option<String>,
}

#[derive(Args, Debug, Clone)]
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
    
    /// Reports to exclude (comma-separated)
    #[arg(long)]
    pub exclude_reports: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct ValidateClassesArgs {
    /// Input directory containing cpp files to scan
    #[arg(short, long)]
    pub input_dir: PathBuf,

    /// Output directory for the validation report
    #[arg(short, long, default_value = "validation_reports")]
    pub output_dir: PathBuf,

    /// Comma-separated list of class names to validate
    #[arg(short, long)]
    pub class_names: String,
    
    /// Reports to exclude (comma-separated)
    #[arg(long)]
    pub exclude_reports: Option<String>,
}

#[derive(Args, Debug, Clone)]
pub struct MissingClassesReportArgs {
    /// Arma 3 base game directory (optional if class-dir is provided)
    #[arg(long)]
    pub arma3_dir: Option<PathBuf>,

    /// Mods directory (optional if class-dir is provided)
    #[arg(long)]
    pub mods_dir: Option<PathBuf>,

    /// Input directory containing mission files
    #[arg(short, long)]
    pub mission_dir: PathBuf,

    /// Output directory for the detailed missing classes report
    #[arg(short, long, default_value = "missing_classes_report")]
    pub output_dir: PathBuf,

    /// Cache directory for extracted files
    #[arg(short, long, default_value = "./missing_classes_cache")]
    pub cache_dir: PathBuf,

    /// Number of parallel processing threads
    #[arg(short, long, default_value = "4")]
    pub threads: usize,
    
    /// Reports to exclude (comma-separated)
    #[arg(long)]
    pub exclude_reports: Option<String>,
}