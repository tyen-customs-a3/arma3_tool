use std::path::PathBuf;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Scan and extract files from PBO archives
    ScanPbos(ScanPboArgs),
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