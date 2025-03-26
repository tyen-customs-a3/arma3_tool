use arma3_tool_shared_models::{GameDataClass, GameDataClasses, MissionData};
use fuzzy_matcher::skim::SkimMatcherV2;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Sender, Receiver};
use tempfile::TempDir;
use std::collections::{HashMap, HashSet};
use crate::config::{ScanConfig, UiSettings};
use arma3_tool_cache_storage::{StorageManager, CacheData, PboCache};
use std::fs::File;
use std::io::{self, Write};
use crate::ui::pages::browsing::sourcegraph::SourceGraph;
use walkdir::WalkDir;
use std::thread;

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
    pub fn update_all_classes<G>(&mut self, graph: &G) 
    where 
        G: PboGraphTrait,
    {
        self.all_classes.clear();
        for class_name in graph.get_class_keys() {
            self.all_classes.insert(class_name.clone());
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

/// Trait for PboGraph to allow access to class keys
pub trait PboGraphTrait {
    fn get_class_keys(&self) -> impl Iterator<Item = &String>;
}

#[derive(Debug)]
pub enum LoadingMessage {
    Progress(String),
    GameDataLoaded(GameDataClasses),
    PboCacheLoaded(PboCache),
    SourceGraphReady(SourceGraph),
    Error(String),
    Complete,
}

/// Application state
pub struct Arma3ToolState {
    /// Game data classes
    pub game_data: Option<GameDataClasses>,
    
    /// Currently selected class
    pub selected_class: Option<GameDataClass>,
    
    /// Search text
    pub search_text: String,
    
    /// Search results (class name and match score)
    pub search_results: Vec<(String, i64)>,
    
    /// Fuzzy matcher for search
    pub fuzzy_matcher: SkimMatcherV2,
    
    /// Source graph for mapping classes to PBOs
    pub source_graph: Option<SourceGraph>,
    
    /// PBO cache index
    pub pbo_cache: Option<PboCache>,
    
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
    pub loading_receiver: Option<Receiver<LoadingMessage>>,
}

impl Arma3ToolState {
    /// Load classes from a directory
    pub fn load_directory(&mut self, path: PathBuf) {
        // Clear current state
        self.game_data = None;
        self.selected_class = None;
        self.search_results.clear();
        self.search_text.clear();
        self.source_graph = None;
        
        self.is_loading = true;
        self.loading_progress = "Scanning directory for PBO files...".to_string();

        // Create new game data collection
        let mut game_data = GameDataClasses::new();
        
        // First count total PBOs for progress tracking
        let total_pbos = WalkDir::new(&path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("pbo"))
            })
            .count();
            
        let mut processed_pbos = 0;
        
        // Walk directory and find all PBO files
        for entry in WalkDir::new(&path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("pbo"))
            }) 
        {
            processed_pbos += 1;
            self.loading_progress = format!(
                "Processing PBO files... [{}/{}] - {}", 
                processed_pbos,
                total_pbos,
                entry.path().file_name().unwrap_or_default().to_string_lossy()
            );
            
            // Add PBO to file sources
            let pbo_path = entry.path().to_path_buf();
            let source_index = game_data.add_file_source(pbo_path);
            
            // For now, just create a dummy class for each PBO
            // In a real implementation, you would parse the PBO contents
            let class_name = entry.path()
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
        self.loading_progress = format!("Found {} PBO files. Initializing source graph...", total_pbos);
    }

    /// Start loading game data in the background
    pub fn start_background_loading(&mut self) {
        if self.is_loading {
            return;
        }

        self.is_loading = true;
        self.loading_progress = "Starting initialization...".to_string();
        
        // Create channel for progress updates
        let (tx, rx) = mpsc::channel();
        self.loading_receiver = Some(rx);
        
        // Clone necessary data for the background thread
        let config_path = self.config_path.clone();
        let cache_dir = self.config.as_ref()
            .map(|c| c.cache_dir.clone())
            .unwrap_or_else(|| PathBuf::from("cache"));

        // Get game data for background processing
        let game_data = if let Some(data) = self.game_data.clone() {
            data
        } else {
            let _ = tx.send(LoadingMessage::Error("No game data available".to_string()));
            return;
        };

        // Spawn background thread
        thread::spawn(move || {
            let tx = tx.clone();
            
            // Load PBO cache
            let _ = tx.send(LoadingMessage::Progress("Loading PBO cache...".to_string()));
            let storage = StorageManager::new(&cache_dir);
            match storage.load() {
                Ok(cache_data) => {
                    let _ = tx.send(LoadingMessage::PboCacheLoaded(cache_data.pbo_cache.clone()));
                    
                    // Count total classes for progress tracking
                    let total_classes = game_data.classes.len();
                    let _ = tx.send(LoadingMessage::Progress(format!(
                        "Building source graph for {} classes...", 
                        total_classes
                    )));
                    
                    // Build source graph with progress updates
                    let mut processed = 0;
                    let chunk_size = (total_classes / 10).max(1); // Update every 10% or each item if small
                    
                    let source_graph = SourceGraph::from_classes(
                        &game_data.classes,
                        &game_data.file_sources,
                        &cache_data.pbo_cache
                    );
                    
                    let _ = tx.send(LoadingMessage::Progress("Finalizing source graph...".to_string()));
                    let _ = tx.send(LoadingMessage::GameDataLoaded(game_data));
                    let _ = tx.send(LoadingMessage::SourceGraphReady(source_graph));
                    let _ = tx.send(LoadingMessage::Complete);
                }
                Err(err) => {
                    let _ = tx.send(LoadingMessage::Error(format!("Failed to load cache: {}", err)));
                }
            }
        });
    }

    /// Process any pending loading messages
    pub fn process_loading_messages(&mut self) {
        let mut should_clear = false;
        let mut error_msg = None;
        
        if let Some(rx) = &self.loading_receiver {
            while let Ok(message) = rx.try_recv() {
                match message {
                    LoadingMessage::Progress(msg) => {
                        self.loading_progress = msg;
                    }
                    LoadingMessage::GameDataLoaded(game_data) => {
                        self.game_data = Some(game_data);
                    }
                    LoadingMessage::PboCacheLoaded(pbo_cache) => {
                        self.pbo_cache = Some(pbo_cache);
                    }
                    LoadingMessage::SourceGraphReady(source_graph) => {
                        self.source_graph = Some(source_graph);
                    }
                    LoadingMessage::Error(error) => {
                        error_msg = Some(error);
                        should_clear = true;
                    }
                    LoadingMessage::Complete => {
                        should_clear = true;
                    }
                }
            }
        }
        
        if should_clear {
            self.is_loading = false;
            self.loading_receiver = None;
            if let Some(error) = error_msg {
                self.error_message = Some(error);
            }
        }
    }

    pub fn refresh_database(&mut self) {
        // Clear current state
        self.game_data = None;
        self.config = None;
        self.error_message = None;
        self.selected_class = None;
        self.search_results.clear();
        self.search_text.clear();
        
        // Load the scan configuration first
        match ScanConfig::load(&self.config_path.to_string_lossy()) {
            Ok(config) => {
                self.config = Some(config.clone());
                
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
            }
            Err(err) => {
                self.error_message = Some(format!("Failed to load config: {}", err));
            }
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
        // Load UI settings
        let ui_settings_path = PathBuf::from("ui_settings.json");
        let ui_settings = UiSettings::load(&ui_settings_path.to_string_lossy());
        
        Self {
            game_data: None,
            selected_class: None,
            search_text: String::new(),
            search_results: Vec::new(),
            fuzzy_matcher: SkimMatcherV2::default(),
            source_graph: None,
            pbo_cache: None,
            error_message: None,
            config: None,
            ui_settings,
            dark_mode: true,
            config_path: PathBuf::from("scan_config.json"),
            export_path: PathBuf::from("arma3_classes_export.txt"),
            ui_settings_path,
            selected_folder: None,
            temp_dir: None,
            scan_status: String::from("Ready"),
            is_scanning: false,
            scan_receiver: None,
            using_temp_cache: false,
            is_loading: false,
            loading_progress: String::new(),
            loading_receiver: None,
        }
    }
} 