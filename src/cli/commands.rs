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
    
    /// Export items (weapons, uniforms, vests, backpacks, etc.) to CSV
    Export {
        /// Override cache directory from config (used for default DB path if needed)
        #[arg(long)]
        cache_dir: Option<PathBuf>,
        
        /// Override analysis database file path (stores class/dependency data)
        #[arg(long)]
        analysis_db_path: Option<PathBuf>,
        
        /// Output CSV file path
        #[arg(short, long, default_value = "items_export.csv")]
        output: PathBuf,
        
        /// Item types to export (comma-separated): weapons, uniforms, vests, backpacks, vehicles, all
        #[arg(short, long, default_value = "all")]
        item_types: String,
        
        /// Custom parent class filters (comma-separated, overrides item_types if provided)
        #[arg(long)]
        filter_parents: Option<String>,
        
        /// Maximum number of items to export (0 for unlimited)
        #[arg(short, long, default_value = "0")]
        limit: usize,
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

    /// Generate a report of missing classes with fuzzy match suggestions
    FuzzyReport {
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
}
