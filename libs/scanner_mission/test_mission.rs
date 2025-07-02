use std::path::PathBuf;
use anyhow::Result;
use mission_scanner::{
    scan_mission,
    MissionScannerConfig,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Scan the specific mission we copied
    let mission_dir = PathBuf::from("../arma3_tool/test_missions/co45_Cold_Night_In_Hell.Chernarus_Winter");
    let config = MissionScannerConfig::default();
    
    println!("Scanning mission: {}", mission_dir.display());
    
    match scan_mission(&mission_dir, num_cpus::get(), &config).await {
        Ok(result) => {
            println!("✅ Mission: {}", result.mission_name);
            println!("📁 Mission directory: {}", result.mission_dir.display());
            
            if let Some(sqm_file) = &result.sqm_file {
                println!("📄 SQM file: {}", sqm_file.display());
            }
            
            println!("📜 Found {} SQF files", result.sqf_files.len());
            println!("🔧 Found {} CPP/HPP files", result.cpp_files.len());
            println!("🔗 Found {} class dependencies", result.class_dependencies.len());
            
            // Group dependencies by type
            let mut ref_types = std::collections::HashMap::new();
            for dep in &result.class_dependencies {
                *ref_types.entry(&dep.reference_type).or_insert(0) += 1;
            }
            
            println!("\n📊 Dependencies by type:");
            for (ref_type, count) in &ref_types {
                println!("  {:?}: {}", ref_type, count);
            }
            
            // Show first 10 dependencies
            println!("\n🔍 Example dependencies:");
            for (i, dep) in result.class_dependencies.iter().take(10).enumerate() {
                println!("  {}. {} ({:?}) from {}", 
                    i + 1, 
                    dep.class_name, 
                    dep.reference_type,
                    dep.source_file.file_name().unwrap_or_default().to_string_lossy()
                );
            }
            
            if result.class_dependencies.len() > 10 {
                println!("  ... and {} more", result.class_dependencies.len() - 10);
            }
        }
        Err(e) => {
            println!("❌ Error scanning mission: {}", e);
            return Err(e);
        }
    }

    Ok(())
} 