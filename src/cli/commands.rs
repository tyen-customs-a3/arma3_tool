use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand)]
pub enum Commands {
    /// Extract game data and missions from PBOs
    Extract {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override extractor database file path (stores PBO extraction state)
        #[arg(long)]
        extractor_db_path: Option<PathBuf>,
    },
    
    /// Process extracted game data and missions to create asset database
    Process {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override analysis database file path (stores class/dependency data)
        #[arg(long)]
        analysis_db_path: Option<PathBuf>,
    },
    
    /// Generate dependency reports from asset database
    Report {
        /// Override cache directory from config (used for default DB path if needed)
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override analysis database file path (stores class/dependency data)
        #[arg(long)]
        analysis_db_path: Option<PathBuf>,
        
        /// Override output directory for reports
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
    
    /// Run all operations (extract, process, and report)
    All {
        /// Override cache directory from config
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override extractor database file path (stores PBO extraction state)
        #[arg(long)]
        extractor_db_path: Option<PathBuf>,
        
        /// Override analysis database file path (stores class/dependency data)
        #[arg(long)]
        analysis_db_path: Option<PathBuf>,
        
        /// Override output directory for reports
        #[arg(long)]
        output_dir: Option<PathBuf>,
    },
} 