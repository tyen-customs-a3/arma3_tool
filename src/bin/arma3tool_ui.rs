use eframe::egui;
use arma3_tool::config::ScanConfig;
use arma3_tool_cache_storage::{StorageManager, CacheData};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::path::PathBuf;
use arma3_tool_models::{GameDataClass, MissionData};
use std::fs::File;
use std::io::{self, Write};
use std::sync::mpsc::{self, Receiver};
use arma3_tool::scanner::gamedata::GameDataScanner;
use arma3_tool_pbo_cache::ExtractionConfig;
use tempfile::TempDir;
use std::thread;
use num_cpus;
use std::collections::HashMap;
use arma3_tool::scanner::gamedata as gamedata_scanner;
use gamedata_scanner_models::{
    GameClass,
    ClassProperty,
    PropertyValue as GameDataScannerPropertyValue
};
use walkdir::WalkDir;

struct Arma3ToolUI {
    search_text: String,
    search_results: Vec<(String, i64)>, // Store scores for highlighting
    selected_class: Option<GameDataClass>,
    error_message: Option<String>,
    game_data: Option<arma3_tool_models::GameDataClasses>,
    fuzzy_matcher: SkimMatcherV2,
    dark_mode: bool,
    config_path: PathBuf,
    export_path: PathBuf,
    // New fields for folder parsing
    selected_folder: Option<PathBuf>,
    temp_dir: Option<TempDir>,
    scan_status: String,
    is_scanning: bool,
    scan_receiver: Option<Receiver<Result<String, String>>>,
    using_temp_cache: bool,
}

impl Default for Arma3ToolUI {
    fn default() -> Self {
        Self {
            search_text: String::new(),
            search_results: Vec::new(),
            selected_class: None,
            error_message: None,
            game_data: None,
            fuzzy_matcher: SkimMatcherV2::default(),
            dark_mode: true,
            config_path: PathBuf::from("scan_config.json"),
            export_path: PathBuf::from("arma3_classes_export.txt"),
            // Initialize new fields
            selected_folder: None,
            temp_dir: None,
            scan_status: String::from("Ready"),
            is_scanning: false,
            scan_receiver: None,
            using_temp_cache: false,
        }
    }
}

impl Arma3ToolUI {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        
        // Set up custom fonts and style if needed
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        cc.egui_ctx.set_style(style);
        
        // Load initial data
        app.refresh_database();
        
        app
    }
    
    fn refresh_database(&mut self) {
        // Clear current state
        self.game_data = None;
        self.error_message = None;
        self.selected_class = None;
        self.search_results.clear();
        self.search_text.clear();
        
        if self.using_temp_cache && self.temp_dir.is_some() {
            // Load from temp cache
            let temp_dir_path = self.temp_dir.as_ref().unwrap().path().to_path_buf();
            let storage = StorageManager::new(&temp_dir_path);
            match storage.load() {
                Ok(cache_data) => {
                    self.game_data = Some(cache_data.game_data);
                }
                Err(err) => {
                    self.error_message = Some(format!("Failed to load temp cache: {}", err));
                }
            }
        } else {
            // Load from normal cache
            // Load the scan configuration
            match ScanConfig::load(&self.config_path.to_string_lossy()) {
                Ok(config) => {
                    // Create storage and try to load cache
                    let storage = StorageManager::new(&config.cache_dir);
                    match storage.load() {
                        Ok(cache_data) => {
                            self.game_data = Some(cache_data.game_data);
                        }
                        Err(err) => {
                            self.error_message = Some(format!("Failed to load cache: {}", err));
                        }
                    }
                }
                Err(err) => {
                    self.error_message = Some(format!("Failed to load config: {}", err));
                }
            }
        }
    }
    
    fn search_classes(&mut self) {
        if let Some(game_data) = &self.game_data {
            // Clear previous results
            self.search_results.clear();
            
            if self.search_text.is_empty() {
                return;
            }
            
            // Collect all classes and their fuzzy match scores
            let mut matches: Vec<(String, i64)> = game_data.classes.iter()
                .filter_map(|class| {
                    self.fuzzy_matcher.fuzzy_match(&class.name, &self.search_text)
                        .map(|score| (class.name.clone(), score))
                })
                .collect();
            
            // Sort by score in descending order
            matches.sort_by(|a, b| b.1.cmp(&a.1));
            
            // Take top 50 results
            self.search_results = matches.into_iter().take(50).collect();
        }
    }
    
    fn render_class_details(&mut self, ui: &mut egui::Ui) {
        if let Some(class) = &self.selected_class {
            ui.heading(&class.name);
            ui.add_space(4.0);
            
            // Parent class
            if let Some(parent) = &class.parent {
                ui.horizontal(|ui| {
                    ui.label("Parent:");
                    ui.strong(parent);
                });
            }
            
            // Properties
            if !class.properties.is_empty() {
                ui.add_space(8.0);
                ui.heading("Properties");
                ui.add_space(4.0);
                
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        egui::Grid::new("properties_grid")
                            .striped(true)
                            .spacing([8.0, 4.0])
                            .show(ui, |ui| {
                                for (key, value) in &class.properties {
                                    ui.monospace(key);
                                    ui.label(format!("{:?}", value));
                                    ui.end_row();
                                }
                            });
                    });
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.weak("Select a class to view details");
            });
        }
    }
    
    fn export_classes_to_file(&mut self) -> io::Result<()> {
        if let Some(game_data) = &self.game_data {
            let mut file = File::create(&self.export_path)?;
            
            // Sort classes by name for better readability
            let mut classes = game_data.classes.clone();
            classes.sort_by(|a, b| a.name.cmp(&b.name));
            
            // Write only class names, one per line
            for class in classes {
                writeln!(file, "{}", class.name)?;
            }
            
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No game data loaded"))
        }
    }

    fn parse_folder(&mut self) {
        if let Some(folder_path) = &self.selected_folder {
            // Clear previous scan state
            self.is_scanning = true;
            self.scan_status = "Starting scan...".to_string();
            self.error_message = None;
            
            // Create a temporary directory for the cache
            match TempDir::new() {
                Ok(temp_dir) => {
                    self.temp_dir = Some(temp_dir);
                    let temp_dir_path = self.temp_dir.as_ref().unwrap().path().to_path_buf();
                    
                    // Create a clone of the path for the thread
                    let folder_path_clone = folder_path.clone();
                    
                    // Create a channel to receive scan results and progress updates
                    let (sender, receiver) = mpsc::channel();
                    self.scan_receiver = Some(receiver);
                    
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
                                for entry in walkdir::WalkDir::new(&folder_path_clone)
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
                                        let mission_data = MissionData { missions: Vec::new() };
                                        
                                        // Create cache data
                                        let cache_data = CacheData::new(
                                            game_data,
                                            mission_data,
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
                    self.error_message = Some(format!("Failed to create temporary directory: {}", err));
                    self.is_scanning = false;
                }
            }
        } else {
            self.error_message = Some("No folder selected".to_string());
        }
    }

    fn select_folder_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Select Folder to Parse")
            .pick_folder() 
        {
            self.selected_folder = Some(path);
        }
    }

    fn check_scan_progress(&mut self) {
        if let Some(receiver) = &self.scan_receiver {
            if let Ok(result) = receiver.try_recv() {
                match result {
                    Ok(message) => {
                        if message == "DONE" {
                            // Scan is complete
                            self.scan_status = "Scan completed successfully".to_string();
                            self.is_scanning = false;
                            self.using_temp_cache = true;
                            self.refresh_database();
                        } else {
                            // Update progress message
                            self.scan_status = message;
                        }
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                        self.scan_status = "Scan failed".to_string();
                        self.is_scanning = false;
                    }
                }
            }
        }
    }

    fn render_parse_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Parse Game Data");
        ui.add_space(4.0);
        
        // Folder selection
        ui.horizontal(|ui| {
            if ui.button("Select Folder").clicked() {
                self.select_folder_dialog();
            }
            
            if let Some(path) = &self.selected_folder {
                ui.label(path.to_string_lossy().to_string());
            } else {
                ui.weak("No folder selected");
            }
        });
        
        ui.add_space(8.0);
        
        // Scan buttons and status
        ui.horizontal(|ui| {
            if ui.add_enabled(!self.is_scanning, egui::Button::new("Parse Folder")).clicked() {
                self.parse_folder();
            }
            
            if ui.add_enabled(!self.is_scanning, egui::Button::new("Diagnostic Scan")).clicked() && self.selected_folder.is_some() {
                self.parse_folder_diagnostic();
            }
            
            ui.label(&self.scan_status);
            
            if self.is_scanning {
                ui.spinner();
            }
        });
        
        if self.using_temp_cache {
            ui.horizontal(|ui| {
                ui.label("Currently using temporary cache");
                if ui.button("Switch to Normal Cache").clicked() {
                    self.using_temp_cache = false;
                    self.refresh_database();
                }
            });
        }
        
        // Check scan progress
        if self.is_scanning {
            self.check_scan_progress();
        }
    }

    /// Convert a GameDataScannerPropertyValue to PropertyValue
    fn convert_property_value(value: &GameDataScannerPropertyValue) -> arma3_tool_models::PropertyValue {
        match value {
            GameDataScannerPropertyValue::String(s) => arma3_tool_models::PropertyValue::String(s.clone()),
            GameDataScannerPropertyValue::Number(n) => arma3_tool_models::PropertyValue::Number(*n as f64),
            GameDataScannerPropertyValue::Array(arr) => {
                // Convert each string to PropertyValue::String
                let string_props: Vec<arma3_tool_models::PropertyValue> = arr.iter()
                    .map(|s| arma3_tool_models::PropertyValue::String(s.clone()))
                    .collect();
                arma3_tool_models::PropertyValue::Array(string_props)
            },
            GameDataScannerPropertyValue::Class(c) => arma3_tool_models::PropertyValue::String(c.name.clone()),
        }
    }
    
    /// Convert a class from the scanner result to GameDataClass
    fn convert_class(class: &GameClass) -> arma3_tool_models::GameDataClass {
        let mut properties = std::collections::HashMap::new();
        
        // Convert properties
        for prop in &class.properties {
            properties.insert(prop.name.clone(), Self::convert_property_value(&prop.value));
        }
        
        arma3_tool_models::GameDataClass {
            name: class.name.clone(),
            parent: class.parent.clone(),
            properties,
            source_file_index: None,
        }
    }

    /// Parse a folder with diagnostic mode enabled
    fn parse_folder_diagnostic(&mut self) {
        if let Some(folder_path) = &self.selected_folder {
            self.is_scanning = true;
            self.scan_status = "Starting diagnostic scan...".to_string();
            
            // Create a channel for the worker thread to send status updates
            let (sender, receiver) = mpsc::channel();
            self.scan_receiver = Some(receiver);
            
            // Clone the path for the worker thread
            let folder_path = folder_path.clone();
            
            // Create a temporary directory for the cache
            let temp_dir = TempDir::new().unwrap();
            let temp_dir_path = temp_dir.path().to_path_buf();
            self.temp_dir = Some(temp_dir);
            self.using_temp_cache = true;
            
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
                                let mut classes = HashMap::new();
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
}

impl eframe::App for Arma3ToolUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set theme
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Arma 3 Tool - Class Browser");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(if self.dark_mode { "☀" } else { "🌙" }).clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                    if ui.button("🔄").on_hover_text("Refresh database").clicked() {
                        self.refresh_database();
                    }
                    if ui.button("📄 Export").on_hover_text("Export all classes to file").clicked() {
                        match self.export_classes_to_file() {
                            Ok(_) => {
                                self.error_message = None;
                            }
                            Err(err) => {
                                self.error_message = Some(format!("Failed to export classes: {}", err));
                            }
                        }
                    }
                });
            });
        });
        
        if let Some(error) = &self.error_message {
            egui::TopBottomPanel::top("error_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::RED, "⚠");
                    ui.colored_label(egui::Color32::RED, error);
                });
            });
        }
        
        // Add a bottom panel for folder parsing
        egui::TopBottomPanel::bottom("parse_panel")
            .min_height(120.0)
            .show(ctx, |ui| {
                self.render_parse_panel(ui);
            });
        
        egui::SidePanel::left("search_panel")
            .resizable(true)
            .min_width(300.0)
            .show(ctx, |ui| {
                // Search box
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    if ui.text_edit_singleline(&mut self.search_text).changed() {
                        self.search_classes();
                    }
                });
                
                ui.add_space(4.0);
                
                if !self.search_results.is_empty() {
                    ui.label(format!("Found {} matches", self.search_results.len()));
                    ui.add_space(4.0);
                }
                
                // Results list
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (name, score) in &self.search_results {
                            let button = egui::Button::new(
                                egui::RichText::new(name)
                                    .monospace()
                                    .color(if score > &100 { // Adjust threshold as needed
                                        ui.style().visuals.strong_text_color()
                                    } else {
                                        ui.style().visuals.text_color()
                                    })
                            );
                            
                            if ui.add_sized([ui.available_width(), 0.0], button).clicked() {
                                if let Some(game_data) = &self.game_data {
                                    self.selected_class = game_data.classes.iter()
                                        .find(|c| &c.name == name)
                                        .cloned();
                                }
                            }
                        }
                    });
            });
            
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_class_details(ui);
        });
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("Arma 3 Tool - Class Browser"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Arma 3 Tool - Class Browser",
        options,
        Box::new(|cc| Ok(Box::new(Arma3ToolUI::new(cc))))
    )
} 