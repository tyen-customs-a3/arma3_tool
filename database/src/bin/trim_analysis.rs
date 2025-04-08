use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use arma3_database::{DatabaseManager, GraphQueryEngine};
use clap::Parser;
use serde_json;
use notify::{Watcher, RecursiveMode};
use std::sync::mpsc::channel;
use fancy_regex::Regex;
use arma3_database::repos::ClassRepository;

// Import the shared implementation
use arma3_database::analysis::trim_analysis::analyze_pbo_impact;

/// TrimAnalysis for Arma 3 Database
/// 
/// Analyzes the impact of removing classes from the Arma 3 game hierarchy.
/// Can identify orphaned classes and empty PBOs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the database file
    #[arg(short, long)]
    database: String,

    /// Input file with classes to trim (one per line)
    #[arg(short, long)]
    input: String,
    
    /// Output file for analysis results
    #[arg(short, long, default_value = "trim_analysis.md")]
    output: String,
    
    /// Watch input file for changes and rerun analysis
    #[arg(short, long)]
    watch: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();
    
    // Parse command line arguments
    let args = Args::parse();
    
    // Connect to the database
    let db_path = PathBuf::from(&args.database);
    if !db_path.exists() {
        return Err(format!("Database file not found: {}", args.database).into());
    }
    
    println!("Connecting to database: {}", args.database);
    let db = DatabaseManager::new(&db_path)?;
    
    if args.watch {
        // Run in watch mode
        watch_and_analyze(&args.input, &args.output, &db)?;
    } else {
        // Run once
        analyze_trim_file(&args.input, &args.output, &db)?;
    }
    
    Ok(())
}

/// Read classes from file, handling comments, empty lines, and regex patterns
fn read_classes_from_file(file_path: &str, db: &DatabaseManager) -> Result<(Vec<String>, Vec<String>), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut classes: Vec<String> = Vec::new();
    let mut protected_classes: Vec<String> = Vec::new();
    let mut regex_patterns: Vec<String> = Vec::new();
    let mut protected_regex_patterns: Vec<String> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Check if line starts with "+" (protected class)
        if line.starts_with('+') {
            let class_name = line[1..].trim().to_string();
            
            // Check if it's a regex pattern by looking for special characters
            if class_name.contains('*') || class_name.contains('?') || 
               class_name.contains('[') || class_name.contains(']') ||
               class_name.contains('(') || class_name.contains(')') ||
               class_name.contains('|') || class_name.contains('.') ||
               class_name.contains('$') || class_name.contains('^') ||
               class_name.contains('\\') || class_name.contains('+') ||
               class_name.contains("?<") {
                protected_regex_patterns.push(class_name);
            } else {
                protected_classes.push(class_name);
            }
        } else {
            // Regular class to remove
            // Check if it's a regex pattern
            if line.contains('*') || line.contains('?') || 
               line.contains('[') || line.contains(']') ||
               line.contains('(') || line.contains(')') ||
               line.contains('|') || line.contains('.') ||
               line.contains('$') || line.contains('^') ||
               line.contains('\\') || line.contains('+') ||
               line.contains("?<") {
                regex_patterns.push(line.to_string());
            } else {
                classes.push(line.to_string());
            }
        }
    }
    
    // If we have regex patterns, compile them and get matching classes from database
    if !regex_patterns.is_empty() || !protected_regex_patterns.is_empty() {
        // Get all class names from database
        let class_repo = ClassRepository::new(db);
        let all_classes = class_repo.get_all()?;
        let all_class_names: Vec<String> = all_classes.iter().map(|c| c.id.clone()).collect();
        
        println!("Found {} classes in database", all_class_names.len());
        
        // Process regular regex patterns
        for pattern in regex_patterns {
            println!("Processing regex pattern: {}", pattern);
            match Regex::new(&pattern) {
                Ok(regex) => {
                    let mut match_count = 0;
                    for class_name in &all_class_names {
                        match regex.is_match(class_name) {
                            Ok(is_match) => {
                                if is_match {
                                    classes.push(class_name.clone());
                                    match_count += 1;
                                }
                            },
                            Err(e) => {
                                println!("Error matching {}: {}", class_name, e);
                            }
                        }
                    }
                    println!("  - Found {} matches for pattern: {}", match_count, pattern);
                },
                Err(e) => {
                    println!("Error compiling regex '{}': {}", pattern, e);
                }
            }
        }
        
        // Process protected regex patterns
        for pattern in protected_regex_patterns {
            println!("Processing protected regex pattern: {}", pattern);
            match Regex::new(&pattern) {
                Ok(regex) => {
                    let mut match_count = 0;
                    for class_name in &all_class_names {
                        match regex.is_match(class_name) {
                            Ok(is_match) => {
                                if is_match {
                                    protected_classes.push(class_name.clone());
                                    match_count += 1;
                                }
                            },
                            Err(e) => {
                                println!("Error matching {}: {}", class_name, e);
                            }
                        }
                    }
                    println!("  - Found {} matches for protected pattern: {}", match_count, pattern);
                },
                Err(e) => {
                    println!("Error compiling regex '{}': {}", pattern, e);
                }
            }
        }
    }
    
    // Remove duplicates
    classes.sort();
    classes.dedup();
    
    protected_classes.sort();
    protected_classes.dedup();
    
    // Remove any classes that are also in the protected list
    classes.retain(|class| !protected_classes.contains(class));
    
    Ok((classes, protected_classes))
}

/// Write analysis results to file in Markdown format
fn write_analysis_results(
    output_file: &str,
    impact: &arma3_database::queries::graph_query_engine::ImpactAnalysisResult,
    empty_pbos: &[String],
    at_risk_protected_classes: &[String],
    protected_classes: &[String],
    db: &DatabaseManager,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::{HashMap, HashSet};
    
    // Build a class hierarchy map to visualize orphan chains
    let mut class_children: HashMap<String, Vec<String>> = HashMap::new();
    let mut orphan_roots: HashSet<String> = HashSet::new();
    
    // Gather all direct relationships from orphaned classes and their children
    for orphan in &impact.orphaned_classes {
        // Check if this orphan has a parent in the removed classes
        // If so, it's a root orphan that we'll start our tree from
        orphan_roots.insert(orphan.clone());
    }
    
    // Create parent-child relationships for hierarchy visualization
    let class_repo = ClassRepository::new(db);
    
    // Get all classes to build the hierarchy
    let all_classes = class_repo.get_all()?;
    
    // Build parent-child relationships
    for class in &all_classes {
        if let Some(parent_id) = &class.parent_id {
            class_children
                .entry(parent_id.clone())
                .or_default()
                .push(class.id.clone());
        }
    }
    
    // Generate the Markdown content
    let mut content = String::new();
    
    // Title
    content.push_str("# Trim Analysis Results\n\n");
    
    // Summary
    content.push_str("## Summary\n\n");
    content.push_str(&format!("- **Classes to Remove**: {}\n", impact.removed_classes.len()));
    content.push_str(&format!("- **Orphaned Classes**: {}\n", impact.orphaned_classes.len()));
    content.push_str(&format!("- **Empty PBOs**: {}\n", empty_pbos.len()));
    content.push_str(&format!("- **Protected Classes**: {}\n", protected_classes.len()));
    content.push_str(&format!("- **At-Risk Protected Classes**: {}\n\n", at_risk_protected_classes.len()));
    
    // Classes to Remove
    content.push_str("## Classes to Remove\n\n");
    for class in &impact.removed_classes {
        content.push_str(&format!("{}\n", class));
    }
    content.push_str("\n");
    
    // Orphan Class Chains
    content.push_str("## Orphaned Class Chains\n\n");
    content.push_str("Classes that would become orphaned and their children:\n\n");
    
    // Helper function to recursively print the class hierarchy
    fn print_hierarchy(
        class_id: &str, 
        class_children: &HashMap<String, Vec<String>>,
        orphaned_classes: &HashSet<String>,
        indent: usize,
        content: &mut String
    ) {
        // Print this class with appropriate indentation
        let indent_str = "    ".repeat(indent);
        let orphan_marker = if orphaned_classes.contains(class_id) { " (orphan)" } else { "" };
        
        content.push_str(&format!("{}{}{}\n", indent_str, class_id, orphan_marker));
        
        // Print children recursively
        if let Some(children) = class_children.get(class_id) {
            for child in children {
                print_hierarchy(child, class_children, orphaned_classes, indent + 1, content);
            }
        }
    }
    
    // Convert orphaned_classes to a HashSet for faster lookups
    let orphaned_set: HashSet<String> = impact.orphaned_classes.iter().cloned().collect();
    
    // Print each orphan tree
    for orphan_root in &orphan_roots {
        print_hierarchy(orphan_root, &class_children, &orphaned_set, 0, &mut content);
        content.push('\n');
    }
    
    // Empty PBOs
    content.push_str("## Empty PBOs\n\n");
    content.push_str("PBOs that would be empty after removing these classes:\n\n");
    for pbo in empty_pbos {
        content.push_str(&format!("{}\n", pbo));
    }
    content.push_str("\n");
    
    // Protected Classes
    content.push_str("## Protected Classes\n\n");
    for class in protected_classes {
        content.push_str(&format!("{}\n", class));
    }
    content.push_str("\n");
    
    // At-Risk Protected Classes
    if !at_risk_protected_classes.is_empty() {
        content.push_str("## At-Risk Protected Classes\n\n");
        content.push_str("These protected classes would become orphans and may need their parent classes protected:\n\n");
        for class in at_risk_protected_classes {
            content.push_str(&format!("{}\n", class));
        }
        content.push_str("\n");
    }
    
    // Write to file
    let mut file = File::create(output_file)?;
    file.write_all(content.as_bytes())?;
    
    Ok(())
}

/// Analyze the impact of trimming classes from a file
fn analyze_trim_file(
    input_file: &str, 
    output_file: &str,
    db: &DatabaseManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading classes from: {}", input_file);
    
    // Read classes to trim and protected classes - pass the database connection
    let (classes_to_trim, protected_classes) = read_classes_from_file(input_file, db)?;
    println!("Found {} classes to analyze and {} protected classes", classes_to_trim.len(), protected_classes.len());
    
    if classes_to_trim.is_empty() {
        println!("No classes to analyze, exiting.");
        return Ok(());
    }
    
    // Run analysis with batching for large result sets
    let engine = GraphQueryEngine::new(db);
    
    // Get class impact - process in batches to avoid SQLite parameter limits
    println!("Analyzing class hierarchy impact...");
    
    // SQLite has a limit of about 32,766 parameters per query
    // To be safe, process in batches of 1000
    const BATCH_SIZE: usize = 1000;
    
    let mut all_removed_classes = Vec::new();
    let mut all_orphaned_classes = Vec::new();
    let mut all_affected_classes = Vec::new();
    let mut all_graph_data = None;
    
    // Process in batches
    for chunk in classes_to_trim.chunks(BATCH_SIZE) {
        println!("Processing batch of {} classes", chunk.len());
        let impact = engine.impact_analysis(chunk)?;
        
        all_removed_classes.extend(impact.removed_classes);
        all_orphaned_classes.extend(impact.orphaned_classes);
        all_affected_classes.extend(impact.affected_classes);
        
        // Keep the graph data from the first batch (or merge if needed)
        if all_graph_data.is_none() {
            all_graph_data = Some(impact.graph_data);
        }
    }
    
    // Deduplicate results
    all_removed_classes.sort();
    all_removed_classes.dedup();
    
    all_orphaned_classes.sort();
    all_orphaned_classes.dedup();
    
    all_affected_classes.sort();
    all_affected_classes.dedup();
    
    // Create a combined impact result
    let mut impact = arma3_database::queries::graph_query_engine::ImpactAnalysisResult {
        removed_classes: all_removed_classes,
        orphaned_classes: all_orphaned_classes,
        affected_classes: all_affected_classes,
        graph_data: all_graph_data.unwrap_or_default(),
    };
    
    // Filter out directly removed classes from the orphaned classes list
    // This prevents reporting classes as orphaned when they would be directly removed
    impact.orphaned_classes.retain(|class| !impact.removed_classes.contains(class));
    
    // Check if any protected classes would become orphans
    let mut at_risk_protected_classes = Vec::new();
    for protected_class in &protected_classes {
        if impact.orphaned_classes.contains(protected_class) {
            at_risk_protected_classes.push(protected_class.clone());
        }
    }
    
    // Get PBO impact (analyze which PBOs would be empty)
    println!("Analyzing PBO impact...");
    let empty_pbos = analyze_pbo_impact(db, &impact.removed_classes, &impact.orphaned_classes)?;
    
    // Format and write results
    println!("Writing results to: {}", output_file);
    write_analysis_results(output_file, &impact, &empty_pbos, &at_risk_protected_classes, &protected_classes, db)?;
    
    // Print summary
    println!("\nSummary:");
    println!("  - Classes to remove: {}", impact.removed_classes.len());
    println!("  - Orphaned classes: {}", impact.orphaned_classes.len());
    println!("  - Empty PBOs: {}", empty_pbos.len());
    
    if !at_risk_protected_classes.is_empty() {
        println!("\n⚠️ WARNING: {} protected classes would become orphans:", at_risk_protected_classes.len());
        for class in &at_risk_protected_classes {
            println!("  - {}", class);
        }
        println!("\nYou may need to protect their parent classes as well.");
    }
    
    Ok(())
}

/// Watch input file for changes and rerun analysis
fn watch_and_analyze(
    input_file: &str,
    output_file: &str,
    db: &DatabaseManager,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Watching file: {} (press Ctrl+C to exit)", input_file);
    
    // Create a channel to receive events
    let (tx, rx) = channel();
    
    // Create a watcher
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            tx.send(event).unwrap();
        }
    })?;
    
    // Watch the input file
    watcher.watch(input_file.as_ref(), RecursiveMode::NonRecursive)?;
    
    // Initial analysis
    analyze_trim_file(input_file, output_file, db)?;
    
    // Watch for changes
    loop {
        match rx.recv() {
            Ok(event) => {
                if let notify::EventKind::Modify(_) = event.kind {
                    println!("\nFile changed, rerunning analysis...");
                    match analyze_trim_file(input_file, output_file, db) {
                        Ok(_) => println!("Analysis complete"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                break;
            }
        }
    }
    
    Ok(())
} 