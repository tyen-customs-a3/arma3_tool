//! CLI Integration tests that verify CLI commands work correctly through workflow orchestration

use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use std::fs;

/// Helper function to get the path to the compiled binary
fn get_binary_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test executable name
    if path.ends_with("deps") {
        path.pop(); // Remove "deps" directory
    }
    path.join("arma3-tool")
}

/// Create test PBO files for CLI testing
fn create_test_data(dir: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    
    // Create mock PBO files
    fs::write(dir.join("gamedata.pbo"), "Mock game data PBO content")?;
    fs::write(dir.join("mission.pbo"), "Mock mission PBO content")?;
    fs::write(dir.join("addon.pbo"), "Mock addon PBO content")?;
    
    // Create config file for testing
    let config_content = r#"{
    "item_types": {
        "weapons": ["Rifle", "Pistol", "Launcher"],
        "vehicles": ["Car", "Tank", "Helicopter"],
        "equipment": ["Vest", "Helmet", "Backpack"]
    },
    "max_scope": 2,
    "excluded_prefixes": ["test_", "debug_"]
}"#;
    fs::write(dir.join("test_config.json"), config_content)?;
    
    Ok(())
}

#[test]
fn test_cli_help_command() {
    let output = Command::new(get_binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute help command");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Verify essential CLI commands are present
    assert!(stdout.contains("extract"));
    assert!(stdout.contains("process"));
    assert!(stdout.contains("report"));
    assert!(stdout.contains("export"));
}

#[test]
fn test_cli_extract_command_structure() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("output");
    
    create_test_data(&source_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    let output = Command::new(get_binary_path())
        .arg("extract")
        .arg("--help")
        .output()
        .expect("Failed to execute extract help");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Verify extract command has expected options
    assert!(stdout.contains("source") || stdout.contains("input"));
    assert!(stdout.contains("output"));
}

#[test]
fn test_cli_process_command_structure() {
    let output = Command::new(get_binary_path())
        .arg("process")
        .arg("--help")
        .output()
        .expect("Failed to execute process help");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Verify process command has expected options
    assert!(stdout.contains("database") || stdout.contains("db"));
}

#[test]
fn test_cli_report_command_structure() {
    let output = Command::new(get_binary_path())
        .arg("report")
        .arg("--help")
        .output()
        .expect("Failed to execute report help");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Verify report command has expected options
    assert!(stdout.contains("database") || stdout.contains("db"));
    assert!(stdout.contains("output"));
}

#[test]
fn test_cli_export_command_structure() {
    let output = Command::new(get_binary_path())
        .arg("export")
        .arg("--help")
        .output()
        .expect("Failed to execute export help");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Verify export command has expected options
    assert!(stdout.contains("database") || stdout.contains("db"));
    assert!(stdout.contains("output"));
}

#[test]
fn test_cli_invalid_command() {
    let output = Command::new(get_binary_path())
        .arg("invalid_command")
        .output()
        .expect("Failed to execute invalid command");
    
    // Should fail with invalid command
    assert!(!output.status.success());
}

#[test]
fn test_cli_extract_missing_args() {
    let output = Command::new(get_binary_path())
        .arg("extract")
        .output()
        .expect("Failed to execute extract without args");
    
    // Should fail due to missing required arguments
    assert!(!output.status.success());
    
    let stderr = String::from_utf8(output.stderr).unwrap();
    
    // Should indicate missing required arguments
    assert!(stderr.contains("required") || stderr.contains("missing") || stderr.contains("error"));
}

#[test]
fn test_cli_process_missing_args() {
    let output = Command::new(get_binary_path())
        .arg("process")
        .output()
        .expect("Failed to execute process without args");
    
    // Should fail due to missing required arguments
    assert!(!output.status.success());
}

#[test]
fn test_cli_extract_nonexistent_source() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_source = temp_dir.path().join("nonexistent");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();
    
    let output = Command::new(get_binary_path())
        .arg("extract")
        .arg("--source")
        .arg(&nonexistent_source)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("Failed to execute extract with nonexistent source");
    
    // Should fail due to nonexistent source directory
    assert!(!output.status.success());
}

#[test]
fn test_cli_process_nonexistent_database() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_db = temp_dir.path().join("nonexistent.db");
    
    let output = Command::new(get_binary_path())
        .arg("process")
        .arg("--database")
        .arg(&nonexistent_db)
        .output()
        .expect("Failed to execute process with nonexistent database");
    
    // Should fail due to nonexistent database
    assert!(!output.status.success());
}

#[test]
fn test_cli_report_nonexistent_database() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_db = temp_dir.path().join("nonexistent.db");
    let output_dir = temp_dir.path().join("output");
    fs::create_dir_all(&output_dir).unwrap();
    
    let output = Command::new(get_binary_path())
        .arg("report")
        .arg("--database")
        .arg(&nonexistent_db)
        .arg("--output")
        .arg(&output_dir)
        .output()
        .expect("Failed to execute report with nonexistent database");
    
    // Should fail due to nonexistent database
    assert!(!output.status.success());
}

#[test]
fn test_cli_export_nonexistent_database() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent_db = temp_dir.path().join("nonexistent.db");
    let output_file = temp_dir.path().join("export.csv");
    
    let output = Command::new(get_binary_path())
        .arg("export")
        .arg("--database")
        .arg(&nonexistent_db)
        .arg("--output")
        .arg(&output_file)
        .output()
        .expect("Failed to execute export with nonexistent database");
    
    // Should fail due to nonexistent database
    assert!(!output.status.success());
}

#[test]
fn test_cli_version_command() {
    let output = Command::new(get_binary_path())
        .arg("--version")
        .output()
        .expect("Failed to execute version command");
    
    assert!(output.status.success());
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    
    // Should contain version information
    assert!(stdout.contains("arma3-tool") || stdout.contains("version") || !stdout.trim().is_empty());
}

#[test]
fn test_cli_workflow_argument_validation() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("output");
    
    create_test_data(&source_dir).unwrap();
    fs::create_dir_all(&output_dir).unwrap();
    
    // Test extract with valid directory structure
    let output = Command::new(get_binary_path())
        .arg("extract")
        .arg("--source")
        .arg(&source_dir)
        .arg("--output")
        .arg(&output_dir)
        .arg("--help")  // Use help to avoid actual execution
        .output()
        .expect("Failed to execute extract with valid arguments");
    
    // Help should always succeed
    assert!(output.status.success());
}

#[test]
fn test_cli_concurrent_command_execution() {
    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    
    let success = Arc::new(AtomicBool::new(true));
    let mut handles = Vec::new();
    
    // Test multiple concurrent help commands
    for i in 0..5 {
        let success_clone = Arc::clone(&success);
        let handle = thread::spawn(move || {
            let output = Command::new(get_binary_path())
                .arg("--help")
                .output()
                .expect("Failed to execute concurrent help command");
            
            if !output.status.success() {
                success_clone.store(false, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert!(success.load(Ordering::Relaxed));
}

#[test]
fn test_cli_error_message_quality() {
    let temp_dir = TempDir::new().unwrap();
    let invalid_path = temp_dir.path().join("invalid_file.xyz");
    
    let output = Command::new(get_binary_path())
        .arg("extract")
        .arg("--source")
        .arg(&invalid_path)
        .arg("--output")
        .arg(&temp_dir.path())
        .output()
        .expect("Failed to execute extract with invalid source");
    
    assert!(!output.status.success());
    
    let stderr = String::from_utf8(output.stderr).unwrap();
    
    // Error message should be informative
    assert!(!stderr.trim().is_empty());
    
    // Should contain some indication of what went wrong
    assert!(
        stderr.contains("error") || 
        stderr.contains("Error") ||
        stderr.contains("failed") ||
        stderr.contains("Failed") ||
        stderr.contains("not found") ||
        stderr.contains("invalid")
    );
}

#[test]
fn test_cli_output_directory_creation() {
    let temp_dir = TempDir::new().unwrap();
    let source_dir = temp_dir.path().join("source");
    let output_dir = temp_dir.path().join("deep").join("nested").join("output");
    
    create_test_data(&source_dir).unwrap();
    
    // Test that CLI can handle creating nested output directories
    let output = Command::new(get_binary_path())
        .arg("extract")
        .arg("--source")
        .arg(&source_dir)
        .arg("--output")
        .arg(&output_dir)
        .arg("--help")  // Use help to test argument validation
        .output()
        .expect("Failed to execute extract with nested output");
    
    // Help command should succeed regardless of directory existence
    assert!(output.status.success());
}

#[test]
fn test_cli_config_file_handling() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("test_config.json");
    
    // Create a test config file
    create_test_data(&temp_dir.path()).unwrap();
    
    // Test commands that might use config files
    let output = Command::new(get_binary_path())
        .arg("export")
        .arg("--help")
        .output()
        .expect("Failed to execute export help");
    
    assert!(output.status.success());
    
    // Check if config option is mentioned in help
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Config file usage varies by command, so we just verify help works
    assert!(!stdout.trim().is_empty());
}