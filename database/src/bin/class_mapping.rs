use std::path::PathBuf;
use clap::Parser;
use log::info;

use arma3_database::DatabaseManager;
use arma3_database::analysis::class_mapping_analysis::ClassMappingAnalysis;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the SQLite database file
    #[arg(short, long)]
    database: PathBuf,

    /// Path to the CSV file containing class mappings
    #[arg(short, long)]
    input: PathBuf,

    /// Path to write the analysis results
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Watch the mappings file for changes
    #[arg(short, long)]
    watch: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Parse command line arguments
    let args = Args::parse();

    // Create database manager
    let db = DatabaseManager::new(&args.database)?;
    info!("Connected to database: {}", args.database.display());

    // Create analysis tool
    let mut analysis = ClassMappingAnalysis::new(db);

    // Set output file if specified
    if let Some(output_path) = args.output {
        analysis.set_output_file(output_path.to_str().unwrap());
    }

    if args.watch {
        // Watch the mappings file and analyze on changes
        analysis.watch_and_analyze(&args.input)?;
    } else {
        // Just run the analysis once
        analysis.load_mappings(&args.input)?;
        analysis.analyze_missions()?;
    }

    Ok(())
} 