use std::collections::{HashSet, HashMap};
use std::path::PathBuf;
use anyhow::{Result, Context};
use clap::Parser;
use csv::Writer;
use log::{info};
use rayon::prelude::*;
use strsim::levenshtein;

use arma3_database::{
    DatabaseManager,
    queries::class_repository::ClassRepository,
    queries::mission_repository::MissionRepository,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the database file
    #[arg(short, long)]
    database: PathBuf,

    /// Path to the output CSV file
    #[arg(short, long)]
    output: PathBuf,

    /// Minimum similarity score (0-1) for fuzzy matching
    #[arg(short, long, default_value_t = 0.7)]
    similarity: f64,

    /// Number of similar classes to include in output
    #[arg(short, long, default_value_t = 4)]
    top_n: usize,

    /// Number of chunks to split the work into for parallel processing
    #[arg(short, long, default_value_t = 16)]
    chunks: usize,
}

/// Find similar classes for a given class name
fn find_similar_classes(
    class_name: &str,
    all_classes: &[String],
    min_similarity: f64,
    top_n: usize,
) -> Vec<String> {
    let mut similarities: Vec<(String, f64)> = all_classes
        .iter()
        .map(|class| {
            // Use case-insensitive comparison for similarity
            let score = levenshtein(&class_name.to_lowercase(), &class.to_lowercase()) as f64;
            let max_len = class_name.len().max(class.len()) as f64;
            let similarity = 1.0 - (score / max_len);
            (class.clone(), similarity)
        })
        .filter(|(_, similarity)| *similarity >= min_similarity)
        .collect();

    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    let mut result = similarities
        .into_iter()
        .take(top_n)
        .map(|(class, _)| class)
        .collect::<Vec<_>>();

    // Pad with empty strings if needed
    while result.len() < top_n {
        result.push(String::new());
    }

    result
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    // Initialize database
    let db = DatabaseManager::new(&args.database).context("Failed to initialize database")?;
    let class_repo = ClassRepository::new(&db);
    let mission_repo = MissionRepository::new(&db);

    // Get all mission dependencies and existing classes in parallel
    let (dependencies, existing_classes) = rayon::join(
        || mission_repo.get_all_dependencies().context("Failed to get mission dependencies"),
        || class_repo.get_all().context("Failed to get existing classes")
    );
    let dependencies = dependencies?;
    let existing_classes = existing_classes?;

    info!("Found {} total dependencies", dependencies.len());

    // Convert to HashSet and Vec for efficient lookup and iteration
    let existing_class_set: HashSet<String> = existing_classes
        .iter()
        .map(|c| c.id.to_lowercase())
        .collect();
    let all_classes: Vec<String> = existing_classes
        .iter()
        .map(|c| c.id.clone())
        .collect();

    info!("Found {} existing classes", existing_class_set.len());

    // Find missing classes (case-insensitive)
    let missing_classes: HashSet<String> = dependencies
        .into_par_iter()
        .map(|d| d.class_name)
        .filter(|class| !existing_class_set.contains(&class.to_lowercase()))
        .collect();

    info!("Found {} missing classes", missing_classes.len());

    // Process missing classes in parallel and collect results
    let results: HashMap<String, Vec<String>> = missing_classes
        .par_iter()
        .map(|missing_class| {
            let similar = find_similar_classes(
                missing_class,
                &all_classes,
                args.similarity,
                args.top_n
            );
            (missing_class.clone(), similar)
        })
        .collect();

    // Write results to CSV
    let mut writer = Writer::from_path(&args.output).context("Failed to create CSV writer")?;
    writer.write_record(&["missing_class", "similar1", "similar2", "similar3", "similar4"])
        .context("Failed to write CSV header")?;

    // Sort missing classes for consistent output
    let mut sorted_missing: Vec<_> = missing_classes.into_iter().collect();
    sorted_missing.sort();

    for missing_class in sorted_missing {
        let similar_classes = &results[&missing_class];
        let mut record = vec![missing_class];
        record.extend(similar_classes.iter().cloned());
        writer.write_record(&record).context("Failed to write CSV record")?;
    }

    writer.flush().context("Failed to flush CSV writer")?;
    info!("Results written to {}", args.output.display());

    Ok(())
} 