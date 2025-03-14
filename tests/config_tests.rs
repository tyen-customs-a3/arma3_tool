use std::path::PathBuf;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;
use arma3_tool::config::ToolConfig;

#[test]
fn test_load_valid_config() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, r#"{{
        "game_data_dirs": ["tests/fixtures/gamedata"],
        "game_data_extensions": ["hpp", "cpp"],
        "mission_dirs": ["tests/fixtures/missions"],
        "mission_extensions": ["hpp", "cpp", "sqf", "sqm"],
        "cache_dir": "tests/fixtures/cache",
        "report_dir": "tests/fixtures/reports",
        "threads": 4,
        "timeout": 30
    }}"#).unwrap();
    
    let config = ToolConfig::from_file(file.path()).unwrap();
    assert_eq!(config.game_data_dirs.len(), 1);
    assert_eq!(config.mission_dirs.len(), 1);
    assert_eq!(config.threads, 4);
    assert_eq!(config.timeout, 30);
    assert_eq!(config.process_base_game, false);
}

#[test]
fn test_default_values() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, r#"{{
        "game_data_dirs": ["tests/fixtures/gamedata"],
        "game_data_extensions": ["hpp", "cpp"],
        "mission_dirs": ["tests/fixtures/missions"],
        "mission_extensions": ["hpp", "cpp", "sqf", "sqm"],
        "cache_dir": "tests/fixtures/cache",
        "report_dir": "tests/fixtures/reports"
    }}"#).unwrap();
    
    let config = ToolConfig::from_file(file.path()).unwrap();
    assert_eq!(config.threads, num_cpus::get());
    assert_eq!(config.timeout, 60);
    assert_eq!(config.process_base_game, false);
}

#[test]
fn test_invalid_config_path() {
    let result = ToolConfig::from_file(&PathBuf::from("nonexistent_file.json"));
    assert!(result.is_err());
}

#[test]
fn test_invalid_config_json() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, r#"{{ invalid json }}"#).unwrap();
    
    let result = ToolConfig::from_file(file.path());
    assert!(result.is_err());
} 