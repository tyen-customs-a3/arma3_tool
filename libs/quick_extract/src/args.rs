use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Source directory containing PBO files to process.
    #[arg(short, long, value_name = "SOURCE_DIR")]
    pub source_dir: PathBuf,

    /// Destination directory to create the HEMTT structure.
    #[arg(short, long, value_name = "DEST_DIR")]
    pub destination_dir: PathBuf,

    /// Timeout in seconds for PBO operations
    #[arg(short, long, default_value = "60")]
    pub timeout: u32,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
} 