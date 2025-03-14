use std::path::PathBuf;
use arma3_tool::config::ToolConfig;

#[test]
fn test_load_real_config() {
    // Test loading the actual scan_config.json file
    let config_path = PathBuf::from("scan_config.json");
    let config = ToolConfig::from_file(&config_path).unwrap();
    
    // Verify the config values
    assert_eq!(config.game_data_dirs.len(), 2);
    assert!(config.game_data_dirs.contains(&PathBuf::from("C:/Program Files (x86)/Steam/steamapps/common/Arma 3")));
    assert!(config.game_data_dirs.contains(&PathBuf::from("E:/pca_quick")));
    
    assert_eq!(config.mission_dirs.len(), 1);
    assert!(config.mission_dirs.contains(&PathBuf::from("E:/pca_missions_quick")));
    
    assert_eq!(config.game_data_extensions, vec!["hpp", "cpp"]);
    assert_eq!(config.mission_extensions, vec!["hpp", "cpp", "sqf", "xml", "sqm"]);
    
    assert_eq!(config.cache_dir, PathBuf::from("./cache"));
    assert_eq!(config.report_dir, PathBuf::from("./reports"));
    
    assert_eq!(config.threads, 16);
    assert_eq!(config.timeout, 60);
    assert_eq!(config.process_base_game, false);
} 