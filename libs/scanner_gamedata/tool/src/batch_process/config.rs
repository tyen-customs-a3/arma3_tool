use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::fmt;
use std::str::FromStr;
use log::{LevelFilter, info};
use env_logger;

/// Parser type
#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum ParserType {
    /// The simple regex-based parser
    Simple,
    /// The advanced parser with full property parsing
    Advanced,
}

impl fmt::Display for ParserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserType::Simple => write!(f, "simple"),
            ParserType::Advanced => write!(f, "advanced"),
        }
    }
}

impl FromStr for ParserType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "simple" => Ok(ParserType::Simple),
            "advanced" => Ok(ParserType::Advanced),
            _ => Err(format!("Unknown parser type: {}", s)),
        }
    }
}

/// Batch parser configuration
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about = "Batch parser for gamedata files")]
pub struct Args {
    /// Input directory to scan for files
    #[clap(short, long, default_value = "./input")]
    pub input_dir: PathBuf,
    
    /// Output directory for reports and failed files
    #[clap(short, long, default_value = "./output")]
    pub output_dir: PathBuf,
    
    /// Path for the JSON report file
    #[clap(long, default_value = "./output/report.json")]
    pub report_path: PathBuf,
    
    /// Path for the diagnostic report file
    #[clap(long, default_value = "./output/diagnostics.log")]
    pub diagnostic_path: PathBuf,
    
    /// File extensions to process
    #[clap(long, value_delimiter = ',', default_values = &["hpp", "cpp", "h", "c"])]
    pub file_extensions: Vec<String>,
    
    /// Maximum number of files to process (0 = unlimited)
    #[clap(long, default_value = "0")]
    pub max_files: usize,
    
    /// Process files in parallel
    #[clap(long, default_value = "true")]
    pub parallel: bool,
    
    /// Copy failed files to output directory
    #[clap(long, default_value = "true")]
    pub copy_failed_files: bool,
    
    /// Maximum number of failures before stopping (0 = unlimited)
    #[clap(long, default_value = "0")]
    pub max_failures: usize,
    
    /// Processing timeout in seconds (0 = unlimited)
    #[clap(long, default_value = "60")]
    pub timeout_secs: u64,
    
    /// Which parser to use (simple or advanced)
    #[clap(long, default_value = "advanced")]
    pub parser_type: ParserType,
    
    /// Verbosity level
    /// (0=error, 1=warn, 2=info, 3=debug, 4=trace)
    #[clap(short, long, default_value = "2")]
    pub verbose: u8,
}

// Configuration constants
pub const CONTEXT_LINES: usize = 3;  // Number of lines before and after the error to show
pub const MAX_LINE_LENGTH: usize = 150;  // Maximum length of a line to show in context

impl Args {
    pub fn validate(&self) -> std::io::Result<()> {
        // Verify input directory exists
        let input_path = self.input_dir.clone();
        if !input_path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Input directory not found: {}", self.input_dir.display()),
            ));
        }
        Ok(())
    }

    /// Setup logging based on verbosity level
    pub fn setup_logging(&self) {
        let level = match self.verbose {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        };
        
        // Initialize env_logger
        env_logger::Builder::new()
            .filter_level(level)
            .init();
        
        info!("Log level set to: {}", level);
    }
} 