//! Test to verify that hemtt.toml lint configuration is being loaded correctly

use std::path::Path;
use gamedata_scanner::{Scanner, ScannerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("Testing lint configuration loading...");
    
    // Create a scanner with the current directory as project root
    let project_root = std::env::current_dir()?;
    println!("Project root: {}", project_root.display());
    
    let mut config = ScannerConfig::default();
    config.use_advanced_parser = true;
    config.show_progress = false;
    
    let scanner = Scanner::new(&project_root, config)?;
    
    // Check if hemtt.toml exists
    let hemtt_toml_path = project_root.join("hemtt.toml");
    if hemtt_toml_path.exists() {
        println!("✓ hemtt.toml found at: {}", hemtt_toml_path.display());
    } else {
        println!("✗ hemtt.toml not found at: {}", hemtt_toml_path.display());
        return Ok(());
    }
    
    // Create a temporary test file with content that would normally trigger lints
    let test_content = r#"
        // This would normally trigger file_type lint (L-C11ME) if not disabled
        value = 123; // Missing class wrapper
        class TestClass {
            property = "test"    // Missing semicolon - would trigger missing_semicolon lint
        }
    "#;
    
    let temp_file = project_root.join("test_lint_file.hpp");
    std::fs::write(&temp_file, test_content)?;
    
    // Try to parse the file
    match scanner.scan_file(&temp_file) {
        Ok(result) => {
            println!("✓ File parsed successfully with {} classes", result.classes.len());
            println!("✓ Lint configuration appears to be working - no errors from disabled lints");
        }
        Err(e) => {
            println!("✗ File parsing failed: {}", e);
            println!("This might indicate that lint configuration is not being applied correctly");
        }
    }
    
    // Clean up
    let _ = std::fs::remove_file(&temp_file);
    
    Ok(())
}