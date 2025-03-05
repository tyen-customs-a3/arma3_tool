use std::path::PathBuf;
use pbo_tools::core::api::{PboApi, PboApiOps};
use pbo_tools::extract::ExtractOptions;

fn main() {
    let pbo_path = PathBuf::from("src/extraction/tests/data/ace_medical.pbo");
    
    let api = PboApi::builder()
        .with_timeout(30)
        .build();

    let options = ExtractOptions {
        no_pause: true,
        warnings_as_errors: false,
        verbose: true,
        ..Default::default()
    };

    match api.list_with_options(&pbo_path, options) {
        Ok(result) => {
            println!("Files in PBO:");
            for file in result.get_file_list() {
                println!("  {}", file);
            }
            if let Some(prefix) = result.get_prefix() {
                println!("\nPBO Prefix: {}", prefix);
            }
        }
        Err(e) => {
            eprintln!("Error listing PBO contents: {}", e);
            std::process::exit(1);
        }
    }
} 