use anyhow::Result;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sqf_analyzer::{
    analyze_sqf, extract_common_items, extract_function_calls, extract_items_direct, Args,
};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn generate_synthetic_sqf(size: usize, complexity: usize) -> String {
    let mut content = String::new();
    
    // Add file header
    content.push_str("/**\n * Synthetic SQF test file\n */\n\n");
    
    // Add variable definitions
    for i in 0..complexity {
        content.push_str(&format!(
            "private _weapon{} = \"weapon_test_{}\";\n", 
            i, i
        ));
        content.push_str(&format!(
            "private _magazine{} = \"magazine_test_{}\";\n", 
            i, i
        ));
        content.push_str(&format!(
            "private _item{} = \"item_test_{}\";\n", 
            i, i
        ));
    }
    
    content.push_str("\n");
    
    // Add weapon/equipment arrays
    content.push_str("private _weaponItems = [\n");
    for i in 0..size {
        if i > 0 {
            content.push_str(",\n");
        }
        content.push_str(&format!("    \"weapon_array_{}\"", i));
    }
    content.push_str("\n];\n\n");
    
    content.push_str("private _magazineItems = [\n");
    for i in 0..size {
        if i > 0 {
            content.push_str(",\n");
        }
        content.push_str(&format!("    \"magazine_array_{}\"", i));
    }
    content.push_str("\n];\n\n");
    
    // Add direct function calls
    for i in 0..(size/5) {
        content.push_str(&format!(
            "_unit addItemToUniform \"uniform_item_{}\";\n", 
            i
        ));
        content.push_str(&format!(
            "_unit addItemToVest \"vest_item_{}\";\n", 
            i
        ));
        content.push_str(&format!(
            "_unit addItemToBackpack \"backpack_item_{}\";\n", 
            i
        ));
        content.push_str(&format!(
            "_unit addWeapon \"weapon_direct_{}\";\n", 
            i
        ));
        content.push_str(&format!(
            "_unit addMagazine \"magazine_direct_{}\";\n", 
            i
        ));
    }
    
    content.push_str("\n");
    
    // Add for loops with function calls
    for i in 0..(size/10) {
        content.push_str(&format!(
            "for \"_i\" from 1 to (ceil (random [1, 2, 3])) do {{_unit addItemToUniform \"uniform_loop_{}\"}};\n", 
            i
        ));
        content.push_str(&format!(
            "for \"_i\" from 1 to (ceil (random [1, 2, 3])) do {{_unit addItemToVest \"vest_loop_{}\"}};\n", 
            i
        ));
        content.push_str(&format!(
            "for \"_i\" from 1 to (ceil (random [1, 2, 3])) do {{_unit addItemToBackpack \"backpack_loop_{}\"}};\n", 
            i
        ));
        content.push_str(&format!(
            "for \"_i\" from 1 to (ceil (random [1, 2, 3])) do {{_unit addMagazine \"magazine_loop_{}\"}};\n", 
            i
        ));
    }
    
    content.push_str("\n");
    
    // Add variable references
    for i in 0..complexity {
        content.push_str(&format!("_unit addWeapon _weapon{};\n", i % complexity));
        content.push_str(&format!("_unit addItemToVest _magazine{};\n", i % complexity));
        content.push_str(&format!("_unit addItemToBackpack _item{};\n", i % complexity));
    }
    
    content
}

fn write_synthetic_files() -> Result<Vec<PathBuf>> {
    let synthetic_dir = PathBuf::from("bench-files/synthetic");
    
    // Create directory if it doesn't exist
    fs::create_dir_all(&synthetic_dir)?;
    
    let sizes = [100, 500, 1000, 5000];
    let complexities = [10, 50, 100];
    
    let mut files = Vec::new();
    
    for &size in &sizes {
        for &complexity in &complexities {
            let filename = format!("synthetic_s{}_c{}.sqf", size, complexity);
            let path = synthetic_dir.join(&filename);
            
            let content = generate_synthetic_sqf(size, complexity);
            fs::write(&path, content)?;
            
            files.push(path);
        }
    }
    
    Ok(files)
}

fn benchmark_file(path: &Path, functions: Option<Vec<String>>) -> Result<BenchResult> {
    let content = fs::read_to_string(path)?;
    let file_size = content.len();
    
    // Prepare for benchmarking
    let mut items = HashSet::new();
    let func_refs = functions.as_ref().map(|f| f.as_ref());
    
    // Benchmark extract_function_calls
    let start = Instant::now();
    extract_function_calls(&content, &mut items, func_refs)?;
    let function_calls_time = start.elapsed();
    
    // Reset items for next benchmark
    items.clear();
    
    // Benchmark extract_common_items
    let start = Instant::now();
    items = extract_common_items(&content, func_refs)?;
    let common_items_time = start.elapsed();
    
    // Benchmark extract_items_direct
    let start = Instant::now();
    items = extract_items_direct(path, func_refs)?;
    let direct_time = start.elapsed();
    
    // Create Args for analyze_sqf
    let args = Args {
        path: path.to_path_buf(),
        output: "text".to_string(),
        full_paths: false,
        include_vars: false,
        functions: functions
            .as_ref()
            .map(|f| f.join(","))
    };
    
    // Benchmark analyze_sqf
    let start = Instant::now();
    let items = analyze_sqf(&args)?;
    let analyze_time = start.elapsed();
    
    Ok(BenchResult {
        file_path: path.to_string_lossy().to_string(),
        file_size,
        item_count: items.len(),
        function_calls_time,
        common_items_time,
        direct_time,
        analyze_time,
    })
}

struct BenchResult {
    file_path: String,
    file_size: usize,
    item_count: usize,
    function_calls_time: Duration,
    common_items_time: Duration,
    direct_time: Duration,
    analyze_time: Duration,
}

fn run_benchmarks() -> Result<()> {
    // Create synthetic files
    println!("Generating synthetic test files...");
    let synthetic_files = write_synthetic_files()?;
    
    // Find existing test files
    let test_dir = PathBuf::from("test-files");
    let mut test_files = Vec::new();
    
    if test_dir.exists() {
        for entry in fs::read_dir(test_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "sqf") {
                test_files.push(path);
            }
        }
    }
    
    println!("\nRunning benchmarks on {} synthetic files and {} test files...", 
             synthetic_files.len(), test_files.len());
    
    // Define function sets to test
    let function_sets: Vec<(String, Option<Vec<String>>)> = vec![
        ("No Filter".to_string(), None),
        (
            "Equipment Functions".to_string(),
            Some(vec![
                "addItemToUniform".to_string(),
                "addItemToVest".to_string(),
                "addItemToBackpack".to_string(),
                "addWeapon".to_string(),
                "addMagazine".to_string(),
            ]),
        ),
        (
            "Single Function".to_string(),
            Some(vec!["addItemToUniform".to_string()]),
        ),
    ];
    
    // Run benchmarks on synthetic files
    println!("\n==== SYNTHETIC FILES ====");
    for file in &synthetic_files {
        println!("\nFile: {}", file.file_name().unwrap().to_string_lossy());
        
        for (desc, functions) in &function_sets {
            let result = benchmark_file(file, functions.clone())?;
            
            println!("  {} - Items: {}", desc, result.item_count);
            println!("    extract_function_calls: {:?}", result.function_calls_time);
            println!("    extract_common_items:   {:?}", result.common_items_time);
            println!("    extract_items_direct:   {:?}", result.direct_time);
            println!("    analyze_sqf:            {:?}", result.analyze_time);
        }
    }
    
    // Run benchmarks on test files
    println!("\n==== TEST FILES ====");
    for file in &test_files {
        println!("\nFile: {}", file.file_name().unwrap().to_string_lossy());
        
        for (desc, functions) in &function_sets {
            let result = benchmark_file(file, functions.clone())?;
            
            println!("  {} - Items: {}", desc, result.item_count);
            println!("    extract_function_calls: {:?}", result.function_calls_time);
            println!("    extract_common_items:   {:?}", result.common_items_time);
            println!("    extract_items_direct:   {:?}", result.direct_time);
            println!("    analyze_sqf:            {:?}", result.analyze_time);
        }
    }
    
    Ok(())
}

fn criterion_benchmark(c: &mut Criterion) {
    let synthetic_content = generate_synthetic_sqf(500, 50);
    
    // Benchmark synthetic file with different functions
    let mut group = c.benchmark_group("Synthetic File");
    
    {
        let mut items = HashSet::new();
        group.bench_function("extract_function_calls/no_filter", |b| {
            b.iter(|| {
                items.clear();
                extract_function_calls(
                    black_box(&synthetic_content),
                    &mut items,
                    None,
                )
                .unwrap()
            })
        });
    }
    
    {
        let mut items = HashSet::new();
        let functions = vec![
            "addItemToUniform".to_string(),
            "addItemToVest".to_string(),
            "addItemToBackpack".to_string(),
        ];
        group.bench_function("extract_function_calls/with_filter", |b| {
            b.iter(|| {
                items.clear();
                extract_function_calls(
                    black_box(&synthetic_content),
                    &mut items,
                    Some(black_box(&functions)),
                )
                .unwrap()
            })
        });
    }
    
    group.bench_function("extract_common_items/no_filter", |b| {
        b.iter(|| {
            extract_common_items(
                black_box(&synthetic_content),
                None,
            )
            .unwrap()
        })
    });
    
    {
        let functions = vec![
            "addItemToUniform".to_string(),
            "addItemToVest".to_string(),
            "addItemToBackpack".to_string(),
        ];
        group.bench_function("extract_common_items/with_filter", |b| {
            b.iter(|| {
                extract_common_items(
                    black_box(&synthetic_content),
                    Some(black_box(&functions)),
                )
                .unwrap()
            })
        });
    }
    
    group.finish();
    
    // Benchmark real test files if available
    if let Ok(test_files) = fs::read_dir("test-files") {
        for entry in test_files {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "sqf") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let filename = path.file_name().unwrap().to_string_lossy();
                        let mut group = c.benchmark_group(format!("TestFile/{}", filename));
                        
                        group.bench_function("extract_common_items/no_filter", |b| {
                            b.iter(|| {
                                extract_common_items(
                                    black_box(&content),
                                    None,
                                )
                                .unwrap()
                            })
                        });
                        
                        {
                            let functions = vec![
                                "addItemToUniform".to_string(),
                                "addItemToVest".to_string(),
                                "addItemToBackpack".to_string(),
                            ];
                            group.bench_function("extract_common_items/with_filter", |b| {
                                b.iter(|| {
                                    extract_common_items(
                                        black_box(&content),
                                        Some(black_box(&functions)),
                                    )
                                    .unwrap()
                                })
                            });
                        }
                        
                        group.finish();
                    }
                }
            }
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); 