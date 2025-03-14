use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};
use arma3_tool::cache::CacheManager;

#[test]
fn test_cache_path_generation() {
    let temp_dir = TempDir::new().unwrap();
    let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
    
    let pbo_path = Path::new("tests/fixtures/gamedata/@tc_headgear_pumpkin/addons/headgear_pumpkin.pbo");
    let cache_path = cache_manager.get_game_data_cache_path(pbo_path);
    
    assert!(cache_path.starts_with(cache_manager.game_data_cache_dir()));
    assert!(cache_path.to_string_lossy().contains("headgear_pumpkin_"));
}

#[test]
fn test_mission_cache_path_generation() {
    let temp_dir = TempDir::new().unwrap();
    let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
    
    let pbo_path = Path::new("tests/fixtures/missions/adv48_Joust.VR.pbo");
    let cache_path = cache_manager.get_mission_cache_path(pbo_path);
    
    assert!(cache_path.starts_with(cache_manager.mission_cache_dir()));
    assert!(cache_path.to_string_lossy().contains("adv48_Joust.VR_"));
}

#[test]
fn test_cache_update_and_check() {
    let temp_dir = TempDir::new().unwrap();
    let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
    
    // Create a test file
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "test content").unwrap();
    
    // Create cache directory
    let cache_path = temp_dir.path().join("cache_test");
    fs::create_dir_all(&cache_path).unwrap();
    
    // Update cache
    cache_manager.update_cache(file.path(), &cache_path).unwrap();
    
    // Check if cached
    assert!(cache_manager.is_cached(file.path(), &cache_path));
    
    // Modify file
    writeln!(file, "modified content").unwrap();
    
    // Should no longer be cached
    assert!(!cache_manager.is_cached(file.path(), &cache_path));
}

#[test]
fn test_clear_cache() {
    let temp_dir = TempDir::new().unwrap();
    let cache_manager = CacheManager::new(temp_dir.path().to_path_buf());
    
    // Create some test files in the cache
    let game_data_file = temp_dir.path().join("game_data").join("test_file.txt");
    let mission_file = temp_dir.path().join("missions").join("test_file.txt");
    
    fs::create_dir_all(temp_dir.path().join("game_data")).unwrap();
    fs::create_dir_all(temp_dir.path().join("missions")).unwrap();
    
    File::create(&game_data_file).unwrap().write_all(b"test").unwrap();
    File::create(&mission_file).unwrap().write_all(b"test").unwrap();
    
    // Clear cache
    cache_manager.clear_cache().unwrap();
    
    // Check that files are gone but directories exist
    assert!(!game_data_file.exists());
    assert!(!mission_file.exists());
    assert!(temp_dir.path().join("game_data").exists());
    assert!(temp_dir.path().join("missions").exists());
} 