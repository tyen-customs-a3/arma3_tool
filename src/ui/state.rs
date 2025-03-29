use arma3_tool_shared_models::{GameDataClass, GameDataClasses, MissionData};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Sender, Receiver};
use tempfile::TempDir;
use std::collections::{HashMap, HashSet};
use crate::config::{ScanConfig, UiSettings};
use std::fs::File;
use std::io::{self, Write};
use walkdir::WalkDir;
use std::thread;
use arma3_db::{DatabaseManager, repos::ClassRepository};
use log::{info, warn, error};

/// UI state for the visualization page to separate UI concerns from logic
#[derive(Default)]
pub struct UIState {
    // Filters
    pub pbo_filter: String,
    pub class_filter: String,
    // Selection state
    pub selected_pbo: Option<PathBuf>,
    pub selected_classes: HashSet<String>,
    // View state
    pub show_graph: bool,
    pub layout_initialized: bool,
}

/// Model for class selection functionality
#[derive(Default)]
pub struct ClassSelectionModel {
    /// Custom classes to preserve (not related to mission dependencies)
    pub custom_preserved_classes: HashSet<String>,
    /// Classes to show in the panel (filtered list)
    pub filtered_classes: Vec<String>,
    /// All available classes in the game data
    pub all_classes: HashSet<String>,
}

impl ClassSelectionModel {
    /// Create a new class selection model
    pub fn new() -> Self {
        Self {
            custom_preserved_classes: HashSet::new(),
            filtered_classes: Vec::new(),
            all_classes: HashSet::new(),
        }
    }
    
    /// Add a class to preserve
    pub fn add_class(&mut self, class_name: String) {
        self.custom_preserved_classes.insert(class_name);
    }
    
    /// Remove a class from preservation
    pub fn remove_class(&mut self, class_name: &str) {
        self.custom_preserved_classes.remove(class_name);
    }
    
    /// Update the list of all available classes from game data
    pub fn update_all_classes(&mut self, game_data: &GameDataClasses) {
        self.all_classes.clear();
        for class in &game_data.classes {
            self.all_classes.insert(class.name.clone());
        }
    }
    
    /// Apply a filter to generate the filtered class list
    pub fn apply_filter(&mut self, filter: &str) {
        self.filtered_classes.clear();
        
        // First show custom preserved classes
        for class_name in &self.custom_preserved_classes {
            if filter.is_empty() || class_name.to_lowercase().contains(&filter.to_lowercase()) {
                self.filtered_classes.push(class_name.clone());
            }
        }
        
        // Then add filtered available classes
        if !filter.is_empty() {
            for class_name in &self.all_classes {
                if class_name.to_lowercase().contains(&filter.to_lowercase()) 
                    && !self.custom_preserved_classes.contains(class_name) {
                    self.filtered_classes.push(class_name.clone());
                }
            }
        }
        
        // Sort the list for better UX
        self.filtered_classes.sort();
    }
    
    /// Check if a class is in the custom preserved list
    pub fn is_class_preserved(&self, class_name: &str) -> bool {
        self.custom_preserved_classes.contains(class_name)
    }
    
    /// Get all custom preserved classes as a vector
    pub fn get_preserved_classes(&self) -> Vec<String> {
        self.custom_preserved_classes.iter().cloned().collect()
    }
    
    /// Import classes from a vector (e.g., from saved configuration)
    pub fn import_classes(&mut self, classes: &[String]) {
        self.custom_preserved_classes.clear();
        for class_name in classes {
            self.custom_preserved_classes.insert(class_name.clone());
        }
    }
}

pub enum LoadingMessage {
    Progress(String),
    Error(String),
    Complete,
}

/// Application state
pub struct Arma3ToolState {
    /// Game data classes
    pub game_data: Option<GameDataClasses>,
    
    /// Selected class
    pub selected_class: Option<GameDataClass>,
    
    /// Search text
    pub search_text: String,
    
    /// Search results
    pub search_results: Vec<(String, i64)>,
    
    pub error_message: Option<String>,
    pub config: Option<ScanConfig>,
    pub ui_settings: UiSettings,
    pub dark_mode: bool,
    pub config_path: PathBuf,
    pub export_path: PathBuf,
    pub ui_settings_path: PathBuf,
    // Folder parsing fields
    pub selected_folder: Option<PathBuf>,
    pub temp_dir: Option<TempDir>,
    pub scan_status: String,
    pub is_scanning: bool,
    pub scan_receiver: Option<Receiver<Result<String, String>>>,
    pub using_temp_cache: bool,
    // Loading state
    pub is_loading: bool,
    pub loading_progress: String,
    pub loading_receiver: Option<mpsc::Receiver<LoadingMessage>>,
    // Track discovered PBOs without loading
    pub discovered_pbos: Vec<PathBuf>,
    pub total_discovered_pbos: usize,
    // Database connection
    pub db_manager: Option<DatabaseManager>,
}

impl Arma3ToolState {
    /// Set the directory for future loading but don't load data yet
    pub fn prepare_directory(&mut self, path: PathBuf) {
        // Clear any previously discovered PBOs
        self.discovered_pbos.clear();
        self.total_discovered_pbos = 0;
        
        // Set the selected folder
        self.selected_folder = Some(path.clone());
        
        // Discover PBOs but don't load them yet
        let pbos: Vec<_> = WalkDir::new(&path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("pbo"))
            })
            .map(|e| e.path().to_path_buf())
            .collect();
            
        self.total_discovered_pbos = pbos.len();
        self.discovered_pbos = pbos;
        
        self.loading_progress = format!("Found {} PBO files. Ready to load.", self.total_discovered_pbos);
    }

    /// Load classes from a directory that was previously prepared
    pub fn load_prepared_directory(&mut self) {
        if self.discovered_pbos.is_empty() {
            self.error_message = Some("No PBOs have been discovered. Please select a directory first.".to_string());
            return;
        }
        
        // Clear current state
        self.game_data = None;
        self.selected_class = None;
        self.search_results.clear();
        self.search_text.clear();
        
        self.is_loading = true;
        self.loading_progress = "Loading discovered PBO files...".to_string();

        // Create new game data collection
        let mut game_data = GameDataClasses::new();
        
        let total_pbos = self.discovered_pbos.len();
        let mut processed_pbos = 0;
        
        // Process all discovered PBO files
        for pbo_path in &self.discovered_pbos {
            processed_pbos += 1;
            self.loading_progress = format!(
                "Processing PBO files... [{}/{}] - {}", 
                processed_pbos,
                total_pbos,
                pbo_path.file_name().unwrap_or_default().to_string_lossy()
            );
            
            // Add PBO to file sources
            let source_index = game_data.add_file_source(pbo_path.clone());
            
            // For now, just create a dummy class for each PBO
            // In a real implementation, you would parse the PBO contents
            let class_name = pbo_path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
                
            let mut class = GameDataClass::new(class_name, None);
            class.set_source_file_index(source_index);
            game_data.add_class(class);
        }
        
        // Update state
        self.game_data = Some(game_data);
        self.loading_progress = format!("Loaded {} PBO files. Loading complete.", total_pbos);
        self.is_loading = false;
    }
    
    /// Legacy method that now calls prepare_directory instead
    pub fn load_directory(&mut self, path: PathBuf) {
        self.prepare_directory(path);
    }

    /// Start loading game data in the background
    pub fn start_background_loading(&mut self) {
        if self.game_data.is_none() || self.is_loading {
            return;
        }
        
        // Since the data is already loaded, we don't need an actual background thread
        // Just mark the process as complete immediately
        self.loading_progress = "Game data ready for use".to_string();
        self.is_loading = false;
    }

    /// Process any pending loading messages
    pub fn process_loading_messages(&mut self) {
        if let Some(receiver) = &self.loading_receiver {
            while let Ok(message) = receiver.try_recv() {
                match message {
                    LoadingMessage::Progress(msg) => {
                        self.loading_progress = msg;
                    }
                    LoadingMessage::Error(msg) => {
                        self.error_message = Some(msg);
                        self.is_loading = false;
                        self.loading_receiver = None;
                        break;
                    }
                    LoadingMessage::Complete => {
                        self.is_loading = false;
                        self.loading_receiver = None;
                        break;
                    }
                }
            }
        }
    }

    /// Load game data from database
    pub fn load_from_database(&mut self, db_path: Option<PathBuf>) -> Result<(), String> {
        let db_path = db_path.unwrap_or_else(|| {
            if let Some(config) = &self.config {
                config.cache_dir.join("arma3.db")
            } else {
                PathBuf::from("arma3.db")
            }
        });
        
        info!("Loading game data from database: {}", db_path.display());
        self.loading_progress = format!("Loading game data from database: {}", db_path.display());
        self.is_loading = true;
        
        // Create database manager
        let db_config = arma3_db::models::DatabaseConfig::new(db_path.clone(), {
            if let Some(config) = &self.config {
                config.cache_dir.clone()
            } else {
                PathBuf::from("cache")
            }
        });
        
        let db_manager = match DatabaseManager::with_config(db_config) {
            Ok(manager) => manager,
            Err(e) => {
                let error_msg = format!("Failed to create database manager: {}", e);
                error!("{}", error_msg);
                self.error_message = Some(error_msg.clone());
                self.is_loading = false;
                return Err(error_msg);
            }
        };
        
        // Create class repository
        let class_repo = ClassRepository::new(&db_manager);
        
        // Get all classes from database
        let class_models = match class_repo.get_all() {
            Ok(models) => models,
            Err(e) => {
                let error_msg = format!("Failed to load classes from database: {}", e);
                error!("{}", error_msg);
                self.error_message = Some(error_msg.clone());
                self.is_loading = false;
                return Err(error_msg);
            }
        };
        
        // Convert class models to game data classes
        let mut game_data = GameDataClasses::new();
        for model in class_models {
            game_data.add_class(model.to_game_data_class());
        }
        
        info!("Loaded {} classes from database", game_data.classes.len());
        self.loading_progress = format!("Loaded {} classes from database", game_data.classes.len());
        
        // Update state
        self.game_data = Some(game_data);
        self.db_manager = Some(db_manager);
        self.is_loading = false;
        
        Ok(())
    }
    
    /// Refresh database connection and data
    pub fn refresh_database(&mut self) {
        // Get database path from config if available
        let db_path = if let Some(config) = &self.config {
            config.cache_dir.join("arma3.db")
        } else {
            PathBuf::from("arma3.db")
        };
        
        // Load data from database if it exists
        if db_path.exists() {
            match self.load_from_database(Some(db_path)) {
                Ok(_) => {
                    info!("Successfully refreshed database");
                },
                Err(e) => {
                    error!("Failed to refresh database: {}", e);
                    self.error_message = Some(format!("Failed to refresh database: {}", e));
                }
            }
        } else {
            info!("Database file not found: {}", db_path.display());
            self.error_message = Some(format!("Database file not found: {}", db_path.display()));
        }
    }

    pub fn save_ui_settings(&self) -> Result<(), String> {
        self.ui_settings.save(&self.ui_settings_path.to_string_lossy())
    }

    pub fn export_classes_to_file(&self) -> io::Result<()> {
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
}

impl Default for Arma3ToolState {
    fn default() -> Self {
        Self {
            game_data: None,
            selected_class: None,
            search_text: String::new(),
            search_results: Vec::new(),
            error_message: None,
            config: None,
            ui_settings: UiSettings::default(),
            dark_mode: false,
            config_path: PathBuf::from("scan_config.json"),
            export_path: PathBuf::from("export.json"),
            ui_settings_path: PathBuf::from("ui_settings.json"),
            selected_folder: None,
            temp_dir: None,
            scan_status: String::new(),
            is_scanning: false,
            scan_receiver: None,
            using_temp_cache: false,
            is_loading: false,
            loading_progress: String::new(),
            loading_receiver: None,
            discovered_pbos: Vec::new(),
            total_discovered_pbos: 0,
            db_manager: None,
        }
    }
} 