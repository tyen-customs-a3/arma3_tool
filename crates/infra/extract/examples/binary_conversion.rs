use arma3_extract::{PboExtractor, HemttPboExtractor};
use std::path::Path;
use tempfile::tempdir;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    println!("Binary Config Conversion Example");
    println!("================================");
    
    // Create a temporary directory for the example
    let temp_dir = tempdir()?;
    let output_dir = temp_dir.path();
    
    // Create a mock config.bin file with raP signature
    let config_bin_path = output_dir.join("some_mod/config.bin");
    fs::create_dir_all(config_bin_path.parent().unwrap())?;
    
    // Write a binary config file with raP signature
    let binary_data = b"\0raP\x00\x00\x00\x00This is mock binary config data";
    fs::write(&config_bin_path, binary_data)?;
    
    println!("Created mock binary config at: {}", config_bin_path.display());
    
    // Create the PBO extractor
    let extractor = HemttPboExtractor::new(30);
    
    // Simulate extracted files list
    let extracted_files = vec![
        Path::new("some_mod/config.bin").to_path_buf(),
        Path::new("some_mod/script.sqf").to_path_buf(),
    ];
    
    println!("\nFiles before conversion:");
    for file in &extracted_files {
        println!("  - {}", file.display());
    }
    
    // Attempt binary config conversion
    println!("\nAttempting binary config conversion...");
    match extractor.convert_binary_configs(&extracted_files, output_dir).await {
        Ok(conversions) => {
            if conversions.is_empty() {
                println!("No files were converted (debinarize tool not available)");
            } else {
                println!("Converted {} files:", conversions.len());
                for (original, converted) in conversions {
                    println!("  {} -> {}", original.display(), converted.display());
                }
            }
        }
        Err(e) => {
            eprintln!("Error during conversion: {}", e);
        }
    }
    
    // Check what files exist after conversion attempt
    println!("\nFiles after conversion attempt:");
    for entry in walkdir::WalkDir::new(output_dir) {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                if let Ok(rel_path) = entry.path().strip_prefix(output_dir) {
                    println!("  - {}", rel_path.display());
                }
            }
        }
    }
    
    Ok(())
}