use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// Extract game data and missions from PBOs
    Extract {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
    },
    
    /// Process extracted game data and missions to create asset database
    Process {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override database file path
        #[arg(long)]
        db_path: Option<PathBuf>,
    },
    
    /// Generate dependency reports from asset database
    Report {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override database file path
        #[arg(long)]
        db_path: Option<PathBuf>,
        
        /// Override output directory from config
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    
    /// Run all operations (extract, process, and report)
    All {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override database file path
        #[arg(long)]
        db_path: Option<PathBuf>,
        
        /// Override output directory from config
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
} 