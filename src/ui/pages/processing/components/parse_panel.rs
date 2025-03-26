use eframe::egui;
use std::path::PathBuf;
use crate::scanner::{gamedata::GameDataScanner, mission::MissionScanner};
use crate::ui::state::Arma3ToolState;
use arma3_tool_pbo_cache::ExtractionConfig;
use rfd::FileDialog;
use tokio::runtime::Runtime;
use std::sync::mpsc;
use log::info;
use std::time::Duration;
use num_cpus;
use std::thread;
use tempfile;
use walkdir::WalkDir;
use arma3_tool_cache_storage::{StorageManager, PboCache};
use std::collections::HashMap;
use arma3_tool_shared_models::{GameDataClasses, MissionData};

pub struct ParsePanel;

impl ParsePanel {
    pub fn show(ui: &mut egui::Ui, state: &mut Arma3ToolState) {
        ui.horizontal(|ui| {
            ui.heading("Parse Game Data");
            
            if ui.button("Select Folder").clicked() {
                if let Some(path) = FileDialog::new()
                    .set_title("Select Game Data Folder")
                    .pick_folder() 
                {
                    state.selected_folder = Some(path);
                    state.scan_status = "Folder selected".to_string();
                }
            }
            
            if let Some(folder) = &state.selected_folder {
                ui.label(format!("Selected: {}", folder.display()));
                
                if !state.is_scanning {
                    if ui.button("Start Scan").clicked() {
                        Self::start_scan(state);
                    }
                }
            }
        });
        
        // Show scan status
        ui.label(&state.scan_status);
        
        // Check for scan updates
        if let Some(receiver) = &state.scan_receiver {
            if let Ok(result) = receiver.try_recv() {
                match result {
                    Ok(status) => {
                        let status_clone = status.clone();
                        state.scan_status = status;
                        if status_clone == "Scan complete" {
                            state.is_scanning = false;
                            state.scan_receiver = None;
                            state.refresh_database();
                        }
                    }
                    Err(err) => {
                        state.error_message = Some(err);
                        state.is_scanning = false;
                        state.scan_receiver = None;
                    }
                }
            }
        }
    }
    
    fn start_scan(state: &mut Arma3ToolState) {
        if let Some(folder) = &state.selected_folder {
            // Create temp directory for cache
            if let Ok(temp_dir) = tempfile::tempdir() {
                state.temp_dir = Some(temp_dir);
                state.using_temp_cache = true;
                
                let cache_dir = state.temp_dir.as_ref().unwrap().path().to_path_buf();
                
                // Set up scan configuration
                let config = ExtractionConfig {
                    cache_dir: cache_dir.clone(),
                    game_data_cache_dir: cache_dir.join("gamedata"),
                    mission_cache_dir: cache_dir.join("missions"),
                    game_data_dirs: vec![folder.to_path_buf()],
                    game_data_extensions: vec!["pbo".to_string()],
                    mission_dirs: Vec::new(),
                    mission_extensions: vec!["pbo".to_string()],
                    threads: num_cpus::get(),
                    timeout: 300, // 5 minutes in seconds
                    verbose: false,
                };
                
                // Create channel for progress updates
                let (tx, rx) = mpsc::channel();
                state.scan_receiver = Some(rx);
                state.is_scanning = true;
                state.scan_status = "Starting scan...".to_string();
                
                // Create runtime for async operations
                if let Ok(rt) = Runtime::new() {
                    let tx_clone = tx.clone();
                    
                    // Spawn scanning task
                    thread::spawn(move || {
                        rt.block_on(async {
                            // Create scanner
                            if let Ok(mut scanner) = GameDataScanner::new(config) {
                                // Send progress update
                                let _ = tx_clone.send(Ok("Extracting PBOs...".to_string()));
                                
                                // Extract and scan
                                match scanner.scan(None, false).await {
                                    Ok(_) => {
                                        let _ = tx_clone.send(Ok("Scan complete".to_string()));
                                    }
                                    Err(e) => {
                                        let _ = tx_clone.send(Err(format!("Scan failed: {}", e)));
                                    }
                                }
                            } else {
                                let _ = tx_clone.send(Err("Failed to create scanner".to_string()));
                            }
                        });
                    });
                } else {
                    state.error_message = Some("Failed to create async runtime".to_string());
                    state.is_scanning = false;
                    state.scan_receiver = None;
                }
            } else {
                state.error_message = Some("Failed to create temporary directory".to_string());
            }
        }
    }
    
    fn parse_folder(state: &mut Arma3ToolState) {
        if let Some(folder_path) = &state.selected_folder {
            // Clear previous scan state
            state.is_scanning = true;
            state.scan_status = "Starting scan...".to_string();
            state.error_message = None;
            
            // Create a temporary directory for the cache
            match tempfile::TempDir::new() {
                Ok(temp_dir) => {
                    state.temp_dir = Some(temp_dir);
                    let temp_dir_path = state.temp_dir.as_ref().unwrap().path().to_path_buf();
                    
                    // Create a clone of the path for the thread
                    let folder_path_clone = folder_path.clone();
                    
                    // Create a channel to receive scan results and progress updates
                    let (sender, receiver) = std::sync::mpsc::channel();
                    state.scan_receiver = Some(receiver);
                    
                    // Start scanning in a separate thread
                    thread::spawn(move || {
                        // Create runtime for async operations
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        
                        // Send status update
                        sender.send(Ok("Preparing to scan folder...".to_string())).unwrap();
                        
                        // Create a temporary extraction config that points to our target folder
                        let extraction_config = ExtractionConfig::new(temp_dir_path.clone());
                        
                        // Create scanner 
                        match GameDataScanner::new(extraction_config) {
                            Ok(mut scanner) => {
                                // We'll scan the folder directly by doing a simple file copy operation first
                                sender.send(Ok("Copying files to temp location...".to_string())).unwrap();
                                
                                // Create gamedata directory in the temp folder
                                let gamedata_dir = temp_dir_path.join("gamedata");
                                std::fs::create_dir_all(&gamedata_dir).unwrap();
                                
                                // Copy all .cpp and .hpp files from source folder to temp gamedata directory
                                let mut file_count = 0;
                                for entry in WalkDir::new(&folder_path_clone)
                                    .into_iter()
                                    .filter_map(|e| e.ok())
                                    .filter(|e| e.path().is_file()) 
                                {
                                    let path = entry.path();
                                    if let Some(ext) = path.extension() {
                                        let ext_str = ext.to_string_lossy().to_lowercase();
                                        if ext_str == "cpp" || ext_str == "hpp" {
                                            // Get relative path from the source folder
                                            let rel_path = path.strip_prefix(&folder_path_clone).unwrap();
                                            let dest_path = gamedata_dir.join(rel_path);
                                            
                                            // Create parent directories
                                            if let Some(parent) = dest_path.parent() {
                                                std::fs::create_dir_all(parent).unwrap();
                                            }
                                            
                                            // Copy the file
                                            std::fs::copy(path, &dest_path).unwrap();
                                            file_count += 1;
                                        }
                                    }
                                }
                                
                                sender.send(Ok(format!("Copied {} files. Now scanning...", file_count))).unwrap();
                                
                                // Now run the scan
                                let scan_result = rt.block_on(async {
                                    scanner.scan_only(false).await
                                });
                                
                                match scan_result {
                                    Ok(game_data) => {
                                        // Send status update
                                        sender.send(Ok(format!("Scan complete. Found {} classes", game_data.classes.len()))).unwrap();
                                        
                                        // Create storage manager
                                        let storage = StorageManager::new(&temp_dir_path);
                                        
                                        // Create empty mission data (we're only scanning game data)
                                        let mission_data = arma3_tool_shared_models::MissionData { missions: Vec::new() };
                                        
                                        // Create cache data
                                        let game_data = GameDataClasses::new();
                                        let mission_data = MissionData::new();
                                        let pbo_cache = PboCache { game_data: HashMap::new() };
                                        let cache_data = arma3_tool_cache_storage::CacheData::new(
                                            game_data,
                                            mission_data,
                                            pbo_cache
                                        );
                                        
                                        // Save to temporary cache
                                        match storage.save(&cache_data) {
                                            Ok(_) => sender.send(Ok("DONE".to_string())).unwrap(),
                                            Err(err) => sender.send(Err(format!("Failed to save cache: {}", err))).unwrap(),
                                        }
                                    }
                                    Err(err) => {
                                        sender.send(Err(format!("Scan failed: {}", err))).unwrap();
                                    }
                                }
                            }
                            Err(err) => {
                                sender.send(Err(format!("Failed to create scanner: {}", err))).unwrap();
                            }
                        }
                    });
                }
                Err(err) => {
                    state.error_message = Some(format!("Failed to create temporary directory: {}", err));
                    state.is_scanning = false;
                }
            }
        } else {
            state.error_message = Some("No folder selected".to_string());
        }
    }
    
    fn parse_folder_diagnostic(state: &mut Arma3ToolState) {
        if let Some(folder_path) = &state.selected_folder {
            state.is_scanning = true;
            state.scan_status = "Starting diagnostic scan...".to_string();
            
            // Create a temporary directory for the cache
            let temp_dir = tempfile::TempDir::new().unwrap();
            let temp_dir_path = temp_dir.path().to_path_buf();
            state.temp_dir = Some(temp_dir);
            state.using_temp_cache = true;
            
            // Create a channel for the worker thread to send status updates
            let (sender, receiver) = std::sync::mpsc::channel();
            state.scan_receiver = Some(receiver);
            
            // Clone the path for the worker thread
            let folder_path = folder_path.clone();
            
            // Spawn a worker thread to do the scanning
            thread::spawn(move || {
                // Create a tokio runtime for async operations
                let rt = tokio::runtime::Runtime::new().unwrap();
                
                // Configure extraction
                let extraction_config = ExtractionConfig {
                    cache_dir: temp_dir_path.clone(),
                    game_data_cache_dir: temp_dir_path.join("gamedata"),
                    mission_cache_dir: temp_dir_path.join("missions"),
                    game_data_dirs: vec![folder_path.clone()],
                    game_data_extensions: vec!["hpp".into(), "cpp".into()],
                    mission_dirs: vec![],
                    mission_extensions: vec![],
                    threads: num_cpus::get(),
                    timeout: 30,
                    verbose: false,
                };
                
                // Create scanner
                match GameDataScanner::new(extraction_config) {
                    Ok(mut scanner) => {
                        // We'll scan the folder directly by doing a simple file copy operation first
                        sender.send(Ok("Copying files to temp location...".to_string())).unwrap();
                        
                        // Create gamedata directory in the temp folder
                        let gamedata_dir = temp_dir_path.join("gamedata");
                        std::fs::create_dir_all(&gamedata_dir).unwrap();
                        
                        // Copy all .cpp and .hpp files from source folder to temp gamedata directory
                        let mut file_count = 0;
                        for entry in WalkDir::new(&folder_path).into_iter().filter_map(|e| e.ok()) {
                            if entry.file_type().is_file() {
                                if let Some(ext) = entry.path().extension() {
                                    let ext_str = ext.to_string_lossy().to_lowercase();
                                    if ext_str == "cpp" || ext_str == "hpp" {
                                        // Create relative path
                                        let rel_path = entry.path().strip_prefix(&folder_path).unwrap();
                                        let dest_path = gamedata_dir.join(rel_path);
                                        
                                        // Create parent directories
                                        if let Some(parent) = dest_path.parent() {
                                            std::fs::create_dir_all(parent).unwrap();
                                        }
                                        
                                        // Copy file
                                        std::fs::copy(entry.path(), &dest_path).unwrap();
                                        file_count += 1;
                                        
                                        if file_count % 100 == 0 {
                                            sender.send(Ok(format!("Copied {} files...", file_count))).unwrap();
                                        }
                                    }
                                }
                            }
                        }
                        
                        sender.send(Ok(format!("Copied {} files. Starting diagnostic scan...", file_count))).unwrap();
                        
                        // Now run the diagnostic scan
                        let scan_result = rt.block_on(async {
                            scanner.scan_only(true).await
                        });
                        
                        match scan_result {
                            Ok(game_data) => {
                                // Send status update
                                sender.send(Ok(format!("Diagnostic scan complete. Found {} classes", game_data.classes.len()))).unwrap();
                                
                                // Convert the game data to our UI model
                                let mut classes = std::collections::HashMap::new();
                                for class in &game_data.classes {
                                    classes.insert(class.name.clone(), class.clone());
                                }
                            },
                            Err(e) => {
                                sender.send(Err(format!("Error scanning files: {}", e))).unwrap();
                            }
                        }
                    },
                    Err(e) => {
                        sender.send(Err(format!("Error creating scanner: {}", e))).unwrap();
                    }
                }
            });
        }
    }
    
    fn check_scan_progress(state: &mut Arma3ToolState) {
        if let Some(receiver) = &state.scan_receiver {
            if let Ok(result) = receiver.try_recv() {
                match result {
                    Ok(message) => {
                        if message == "DONE" {
                            // Scan is complete
                            state.scan_status = "Scan completed successfully".to_string();
                            state.is_scanning = false;
                            state.using_temp_cache = true;
                            state.refresh_database();
                        } else {
                            // Update progress message
                            state.scan_status = message;
                        }
                    }
                    Err(err) => {
                        state.error_message = Some(err);
                        state.scan_status = "Scan failed".to_string();
                        state.is_scanning = false;
                    }
                }
            }
        }
    }
} 