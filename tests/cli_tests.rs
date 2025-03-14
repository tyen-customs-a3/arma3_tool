use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use std::fs;
use std::io::Write;

#[test]
#[ignore] // Ignore by default as it requires the binary to be built
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Arma 3 Tool"));
    assert!(stdout.contains("USAGE:"));
    assert!(stdout.contains("OPTIONS:"));
    assert!(stdout.contains("SUBCOMMANDS:"));
}

#[test]
#[ignore] // Ignore by default as it requires the binary to be built
fn test_cli_scan_gamedata() {
    // Create temp directory for cache and reports
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("cache");
    let report_dir = temp_dir.path().join("reports");
    
    // Create directories
    fs::create_dir_all(&cache_dir).unwrap();
    fs::create_dir_all(&report_dir).unwrap();
    
    // Create config file
    let config_path = temp_dir.path().join("test_config.json");
    let mut config_file = fs::File::create(&config_path).unwrap();
    writeln!(config_file, r#"{{
        "game_data_dirs": ["tests/fixtures/gamedata"],
        "game_data_extensions": ["hpp", "cpp"],
        "mission_dirs": ["tests/fixtures/missions"],
        "mission_extensions": ["hpp", "cpp", "sqf", "sqm"],
        "cache_dir": "{}",
        "report_dir": "{}",
        "threads": 2,
        "timeout": 30,
        "process_base_game": false
    }}"#, 
        cache_dir.to_string_lossy(),
        report_dir.to_string_lossy()
    ).unwrap();
    
    // Run the CLI command
    let output = Command::new("cargo")
        .args(&[
            "run", 
            "--", 
            "--config", 
            config_path.to_string_lossy().as_ref(),
            "ScanGameData"
        ])
        .output()
        .expect("Failed to execute command");
    
    // Check if command succeeded
    assert!(output.status.success());
    
    // Check output
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Game data scanning completed successfully"));
    
    // Check that cache was created
    let cache_entries = fs::read_dir(&cache_dir.join("game_data"))
        .unwrap()
        .filter_map(Result::ok)
        .count();
    
    assert!(cache_entries >= 2);
} 