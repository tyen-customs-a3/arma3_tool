use std::path::PathBuf;
use arma3_tool_database::{StorageManager, DatabaseError};
use arma3_tool_models::{
    GameDataClasses, GameDataClass, MissionData, Mission,
    ProcessingSummary, ReportFormat, PropertyValue,
};

#[test]
fn test_storage_manager_creation() {
    let temp = assert_fs::TempDir::new().unwrap();
    let cache_dir = temp.path().to_path_buf();
    
    let _manager = StorageManager::new(
        cache_dir.clone(),
        None,
        None,
        None,
    ).unwrap();
    
    // Verify directories were created
    assert!(cache_dir.exists());
    assert!(cache_dir.join("extracted").exists());
    assert!(cache_dir.join("database").exists());
    assert!(cache_dir.join("reports").exists());
}

#[test]
fn test_game_data_storage() {
    let temp = assert_fs::TempDir::new().unwrap();
    let manager = StorageManager::new(temp.path().to_path_buf(), None, None, None).unwrap();
    
    // Create test game data
    let mut game_data = GameDataClasses::new();
    let mut class = GameDataClass::new("TestVehicle".to_string(), Some("Car".to_string()));
    class.add_property("maxSpeed".to_string(), PropertyValue::Number(100.0));
    game_data.add_class(class);
    
    // Save game data
    manager.save_game_data(&game_data).unwrap();
    
    // Load game data
    let loaded_data = manager.load_game_data().unwrap();
    
    // Verify data
    assert_eq!(loaded_data.classes.len(), 1);
    let loaded_class = loaded_data.get_class("TestVehicle").unwrap();
    assert_eq!(loaded_class.parent.as_deref(), Some("Car"));
    assert!(matches!(
        loaded_class.properties.get("maxSpeed"),
        Some(PropertyValue::Number(100.0))
    ));
}

#[test]
fn test_mission_data_storage() {
    let temp = assert_fs::TempDir::new().unwrap();
    let manager = StorageManager::new(temp.path().to_path_buf(), None, None, None).unwrap();
    
    // Create test mission data
    let mut mission_data = MissionData::new();
    let mission = Mission::new(
        "TestMission".to_string(),
        PathBuf::from("missions/test.sqm"),
    );
    mission_data.add_mission(mission);
    
    // Save mission data
    manager.save_mission_data(&mission_data).unwrap();
    
    // Load mission data
    let loaded_data = manager.load_mission_data().unwrap();
    
    // Verify data
    assert_eq!(loaded_data.missions.len(), 1);
    let loaded_mission = loaded_data.get_mission("TestMission").unwrap();
    assert_eq!(loaded_mission.path, PathBuf::from("missions/test.sqm"));
}

#[test]
fn test_extraction_paths_cache() {
    let temp = assert_fs::TempDir::new().unwrap();
    let mut manager = StorageManager::new(temp.path().to_path_buf(), None, None, None).unwrap();
    
    let paths = vec![
        PathBuf::from("test1.pbo"),
        PathBuf::from("test2.pbo"),
    ];
    
    // Save paths
    manager.save_extraction_paths("test_category", &paths).unwrap();
    
    // Load paths
    let loaded_paths = manager.load_extraction_paths("test_category").unwrap();
    
    // Verify paths
    assert_eq!(loaded_paths, paths);
}

#[test]
fn test_processing_summary_storage() {
    let temp = assert_fs::TempDir::new().unwrap();
    let mut manager = StorageManager::new(temp.path().to_path_buf(), None, None, None).unwrap();
    
    let summary = ProcessingSummary {
        processed_pbos: 10,
        files_processed: 100,
        entries_found: 500,
        elapsed_time: std::time::Duration::from_secs(60),
        errors: vec!["Test error".to_string()],
    };
    
    // Save summary
    manager.save_processing_summary("test_category", &summary).unwrap();
    
    // Load summary
    let loaded_summary = manager.load_processing_summary("test_category").unwrap().unwrap();
    
    // Verify summary
    assert_eq!(loaded_summary.processed_pbos, 10);
    assert_eq!(loaded_summary.files_processed, 100);
    assert_eq!(loaded_summary.entries_found, 500);
    assert_eq!(loaded_summary.elapsed_time.as_secs(), 60);
    assert_eq!(loaded_summary.errors, vec!["Test error"]);
}

#[test]
fn test_report_storage() {
    let temp = assert_fs::TempDir::new().unwrap();
    let manager = StorageManager::new(temp.path().to_path_buf(), None, None, None).unwrap();
    
    let content = b"Test report content";
    let report_name = "test_report";
    
    // Save report
    let report_info = manager.save_report(
        report_name,
        ReportFormat::PlainText,
        content,
    ).unwrap();
    
    println!("Saved report info: {:?}", report_info);
    
    // Verify report info
    assert_eq!(report_info.name, report_name);
    assert_eq!(report_info.format, ReportFormat::PlainText);
    assert_eq!(report_info.size_bytes, content.len() as u64);
    
    // List reports
    let reports = manager.list_reports().unwrap();
    println!("Listed reports: {:?}", reports);
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].name, report_name);
}

#[test]
fn test_nonexistent_data() {
    let temp = assert_fs::TempDir::new().unwrap();
    let mut manager = StorageManager::new(temp.path().to_path_buf(), None, None, None).unwrap();
    
    // Test loading non-existent data
    let game_data = manager.load_game_data().unwrap();
    assert_eq!(game_data.classes.len(), 0);
    
    let mission_data = manager.load_mission_data().unwrap();
    assert_eq!(mission_data.missions.len(), 0);
    
    let extraction_paths = manager.load_extraction_paths("nonexistent").unwrap();
    assert!(extraction_paths.is_empty());
    
    let processing_summary = manager.load_processing_summary("nonexistent").unwrap();
    assert!(processing_summary.is_none());
}

#[test]
fn test_invalid_data() {
    let temp = assert_fs::TempDir::new().unwrap();
    let manager = StorageManager::new(temp.path().to_path_buf(), None, None, None).unwrap();
    
    // Write invalid JSON to game data file
    std::fs::write(
        manager.get_database_dir().join("game_data.json"),
        "invalid json",
    ).unwrap();
    
    // Attempt to load invalid data
    let result = manager.load_game_data();
    assert!(matches!(result, Err(DatabaseError::Serialization(_))));
} 