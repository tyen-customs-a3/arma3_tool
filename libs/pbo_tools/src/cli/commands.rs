use std::path::PathBuf;
use crate::ops::PboOperationResult;
use crate::core::api::PboApiOps;

/// List contents of a PBO file
pub async fn list_contents(api: &dyn PboApiOps, pbo_path: &PathBuf, verbose: bool) -> PboOperationResult<()> {
    let files = api.list_contents(pbo_path).await?;
    
    if verbose {
        println!("Found {} files in PBO:", files.len());
        for file in files {
            println!("  {:<40} {:>10} bytes  {} ({})", 
                     file.file_path, 
                     file.size, 
                     file.mime_type,
                     file.timestamp);
        }
    } else {
        for file in files {
            println!("{}", file.file_path);
        }
    }
    
    Ok(())
}

/// Extract contents from a PBO file
pub async fn extract_contents(
    api: &dyn PboApiOps,
    pbo_path: &PathBuf,
    output_dir: &PathBuf,
    filter: Option<String>,
    verbose: bool,
) -> PboOperationResult<()> {
    if verbose {
        println!("Extracting from {} to {}", pbo_path.display(), output_dir.display());
        if let Some(ref f) = filter {
            println!("Using filter: {}", f);
        }
    }

    match filter {
        Some(filter_pattern) => {
            api.extract_filtered(pbo_path, output_dir, &filter_pattern).await?;
        }
        None => {
            api.extract_all(pbo_path, output_dir).await?;
        }
    }
    
    if verbose {
        println!("Extraction completed successfully");
    }
    
    Ok(())
}

/// Show PBO properties and metadata
pub async fn show_properties(api: &dyn PboApiOps, pbo_path: &PathBuf) -> PboOperationResult<()> {
    let properties = api.get_properties(pbo_path).await?;
    
    println!("PBO Properties for: {}", pbo_path.display());
    println!("  File Count: {}", properties.file_count);
    println!("  Total Size: {} bytes", properties.total_size);
    println!("  Compression Ratio: {:.1}%", properties.compression_ratio() * 100.0);
    
    if let Some(version) = &properties.version {
        println!("  Version: {}", version);
    }
    
    if let Some(author) = &properties.author {
        println!("  Author: {}", author);
    }
    
    if let Some(prefix) = &properties.prefix {
        println!("  Prefix: {}", prefix);
    }
    
    if let Some(checksum) = &properties.checksum {
        println!("  Checksum: {}", checksum);
    }
    
    if !properties.custom_properties.is_empty() {
        println!("  Custom Properties:");
        for (key, value) in &properties.custom_properties {
            if key != "version" && key != "author" && key != "prefix" {
                println!("    {}: {}", key, value);
            }
        }
    }
    
    Ok(())
}

/// Validate a PBO file
pub async fn validate_pbo(api: &dyn PboApiOps, pbo_path: &PathBuf, verbose: bool) -> PboOperationResult<()> {
    let validation = api.validate_pbo(pbo_path).await?;
    
    if validation.is_valid {
        println!("✓ PBO file is valid: {}", pbo_path.display());
    } else {
        println!("✗ PBO file has issues: {}", pbo_path.display());
    }
    
    if verbose || !validation.is_valid {
        println!("Validation Results:");
        println!("  Files sorted: {}", if validation.files_sorted { "✓" } else { "✗" });
        
        if let Some(checksum_valid) = validation.checksum_valid {
            println!("  Checksum valid: {}", if checksum_valid { "✓" } else { "✗" });
        }
        
        if !validation.errors.is_empty() {
            println!("  Errors ({}):", validation.errors.len());
            for error in &validation.errors {
                println!("    ✗ {}", error.message);
                if let Some(file_path) = &error.file_path {
                    println!("      File: {}", file_path);
                }
            }
        }
        
        if !validation.warnings.is_empty() {
            println!("  Warnings ({}):", validation.warnings.len());
            for warning in &validation.warnings {
                println!("    ⚠ {}", warning.message);
                if let Some(file_path) = &warning.file_path {
                    println!("      File: {}", file_path);
                }
            }
        }
    }
    
    Ok(())
}
