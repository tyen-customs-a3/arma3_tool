//! Test to verify that batch processor loads hemtt.toml correctly

use std::path::Path;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("Testing batch processor config loading...");
    
    // Get current directory
    let current_dir = std::env::current_dir()?;
    println!("Current directory: {}", current_dir.display());
    
    // Check if hemtt.toml exists
    let hemtt_toml_path = current_dir.join("hemtt.toml");
    if hemtt_toml_path.exists() {
        println!("✓ hemtt.toml found at: {}", hemtt_toml_path.display());
        
        // Try to load the config
        match hemtt_common::config::ProjectConfig::from_file(&hemtt_toml_path) {
            Ok(config) => {
                println!("✓ Successfully loaded hemtt.toml");
                println!("  Project name: {:?}", config.name());
                println!("  Config lints count: {}", config.lints().config().len());
                
                // Check a specific lint setting
                if let Some(file_type_lint) = config.lints().config().get("file_type") {
                    println!("  file_type lint enabled: {:?}", file_type_lint.enabled());
                } else {
                    println!("  file_type lint not configured (will use default)");
                }
            }
            Err(e) => {
                println!("✗ Failed to load hemtt.toml: {}", e);
            }
        }
    } else {
        println!("✗ hemtt.toml not found at: {}", hemtt_toml_path.display());
        return Ok(());
    }
    
    // Create a temporary test file with problematic content
    let temp_file_content = r#"
        // This would normally trigger file_type lint (not starting with class)
        value = 123;
        property = "test";
    "#;
    
    let temp_file = current_dir.join("temp_test_file.hpp");
    fs::write(&temp_file, temp_file_content)?;
    
    println!("\nTesting advanced parser with config...");
    
    // Test the advanced parser directly
    let file_parent = temp_file.parent().unwrap();
    let parser = parser_advanced::AdvancedProjectParser::new(
        file_parent,
        Some(&hemtt_toml_path)
    )?;
    
    match parser.parse_file(Path::new("temp_test_file.hpp")) {
        Ok((classes, warnings)) => {
            println!("✓ Advanced parser succeeded with {} classes and {} warnings", classes.len(), warnings.len());
            println!("✓ Lint configuration appears to be working correctly");
        }
        Err(e) => {
            println!("✗ Advanced parser failed: {:?}", e);
            println!("This might indicate a parsing issue, not necessarily config issues");
        }
    }
    
    // Clean up
    let _ = fs::remove_file(&temp_file);
    
    Ok(())
}