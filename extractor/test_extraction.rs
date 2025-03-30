use std::path::Path;
use anyhow::Result;
use std::fs;
use std::time::Instant;
use arma3_extractor::{
    ExtractionConfig,
    ExtractionManager,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== PBO Cache Extraction Test ===");
    
    // Setup paths
    let current_dir = std::env::current_dir()?;
    let cache_dir = current_dir.join("target").join("cache_test");
    let game_data_cache_dir = cache_dir.join("gamedata");
    let mission_cache_dir = cache_dir.join("missions");
    
    // Define test locations
    let mod_path = current_dir.join("tests").join("fixtures").join("modfolder");
    let mission_path = current_dir.join("tests").join("fixtures").join("missions");
    
    println!("Cache directory: {}", cache_dir.display());
    println!("Mod path: {}", mod_path.display());
    println!("Mission path: {}", mission_path.display());
    
    // Cleanup any previous test data
    if cache_dir.exists() {
        println!("Cleaning up previous test cache...");
        fs::remove_dir_all(&cache_dir)?;
    }
    
    // Create a test config with both game data and mission directories
    let config = ExtractionConfig {
        cache_dir: cache_dir.clone(),
        game_data_cache_dir: game_data_cache_dir.clone(),
        mission_cache_dir: mission_cache_dir.clone(),
        game_data_dirs: vec![mod_path.clone()],
        game_data_extensions: vec!["p3d".to_string(), "paa".to_string(), "hpp".to_string(), "cpp".to_string(), "bin".to_string()],
        mission_dirs: vec![mission_path.clone()],
        mission_extensions: vec!["hpp".to_string(), "sqf".to_string(), "xml".to_string(), "paa".to_string()],
        threads: 4,
        timeout: 60,
        verbose: true,
        db_path: cache_dir.join("extractor.db"),
    };
    
    // Create the extraction manager
    let mut manager = ExtractionManager::new(config)?;
    
    // Extract game data
    println!("\n=== Processing Game Data ===");
    let start = Instant::now();
    let game_data_files = manager.process_game_data(false).await?;
    let duration = start.elapsed();
    println!("Processed game data in {:.2?}", duration);
    println!("Extracted {} game data files", game_data_files.len());
    
    // Extract missions
    println!("\n=== Processing Missions ===");
    let start = Instant::now();
    let mission_results = manager.process_all_missions(false).await?;
    let duration = start.elapsed();
    println!("Processed missions in {:.2?}", duration);
    println!("Processed {} mission PBOs", mission_results.len());
    
    let total_mission_files: usize = mission_results.values().map(|files| files.len()).sum();
    println!("Extracted {} mission files", total_mission_files);
    
    // Verify extraction
    verify_extraction(&game_data_cache_dir, &mission_cache_dir)?;
    
    println!("\n=== Test Completed Successfully ===");
    Ok(())
}

fn verify_extraction(game_data_dir: &Path, mission_dir: &Path) -> Result<()> {
    println!("\n=== Verifying Extraction Results ===");
    
    // Check game data
    let game_data_exists = game_data_dir.exists();
    println!("Game data directory exists: {}", game_data_exists);
    
    if game_data_exists {
        let game_data_files = count_files(game_data_dir)?;
        println!("Game data directory contains {} files", game_data_files);
        
        if game_data_files == 0 {
            println!("WARNING: No files found in game data directory!");
        }
    }
    
    // Check missions
    let mission_exists = mission_dir.exists();
    println!("Mission directory exists: {}", mission_exists);
    
    if mission_exists {
        let mission_files = count_files(mission_dir)?;
        println!("Mission directory contains {} files", mission_files);
        
        if mission_files == 0 {
            println!("WARNING: No files found in mission directory!");
        }
    }
    
    if !game_data_exists || !mission_exists {
        println!("ERROR: One or more extraction directories were not created!");
        return Err(anyhow::anyhow!("Extraction failed - directories not created"));
    }
    
    Ok(())
}

fn count_files(dir: &Path) -> Result<usize> {
    let mut count = 0;
    
    if !dir.exists() {
        return Ok(0);
    }
    
    for entry in walkdir::WalkDir::new(dir)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            count += 1;
            
            // Print first 5 files as examples
            if count <= 5 {
                println!("  - {}", entry.path().display());
            } else if count == 6 {
                println!("  - ... and more");
            }
        }
    }
    
    Ok(count)
} 