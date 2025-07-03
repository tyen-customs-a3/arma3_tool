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
    
    /// Export items using inheritance-based categorization to CSV
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
        
        /// Item types to export from config (comma-separated): weapons, uniforms, vests, backpacks
        /// If not specified, exports all types defined in configuration
        #[arg(long)]
        types: Option<String>,
        
        /// Path to custom item filter configuration file
        #[arg(long, default_value = "item_filter_config.json")]
        item_config: PathBuf,
        
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
