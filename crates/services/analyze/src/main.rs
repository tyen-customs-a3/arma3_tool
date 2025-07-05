use anyhow::Result;
use sqf_analyzer::{Args, analyze_sqf};
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Print the mode being used
    if let Some(funcs) = &args.functions {
        println!("MODE: Extracting only classes sent to functions: {}", funcs);
    } else {
        println!("MODE: Extracting all function call parameters");
    }
    
    println!("Using SQFvm for preprocessing");

    // Analyze the SQF files
    let paths_to_process = args.path.clone();
    let start_time = std::time::Instant::now();
    let all_items = analyze_sqf(&args)?;
    let elapsed = start_time.elapsed();
    
    // Print summary
    println!("\nSUMMARY:");
    println!("Path processed: {}", paths_to_process.display());
    println!("Items found: {}", all_items.len());
    println!("Time taken: {:.2?}", elapsed);
    println!("");

    // Output results
    match args.output.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&all_items.into_iter().collect::<Vec<_>>())?;
            println!("{}", json);
        }
        _ => {
            let mut sorted_items: Vec<_> = all_items.into_iter().collect();
            sorted_items.sort();
            for item in sorted_items {
                println!("{}", item);
            }
        }
    }

    Ok(())
} 