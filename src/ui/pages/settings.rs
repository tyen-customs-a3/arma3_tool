use eframe::egui;
use crate::ui::state::Arma3ToolState;
use super::{Page, PageId};
use rfd::FileDialog;
use std::path::PathBuf;

pub struct SettingsPage {
    cache_dir_string: String,
    default_cache_dir_string: String,
}

impl Default for SettingsPage {
    fn default() -> Self {
        Self {
            cache_dir_string: String::new(),
            default_cache_dir_string: String::new(),
        }
    }
}

impl Page for SettingsPage {
    fn id(&self) -> PageId {
        PageId::Settings
    }

    fn title(&self) -> &'static str {
        "Settings"
    }

    fn show(&mut self, ui: &mut egui::Ui, state: &mut Arma3ToolState) {
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ui.heading("Settings");
            ui.add_space(8.0);
            
            ui.label("Configure application settings.");
            
            // Scan Configuration Section
            ui.group(|ui| {
                ui.heading("Scan Configuration");
                ui.add_space(4.0);
                
                if let Some(config) = &mut state.config {
                    ui.label("Game Data Directories:");
                    ui.add_space(4.0);
                    
                    for dir in &mut config.game_data_dirs {
                        ui.horizontal(|ui| {
                            ui.text_edit_singleline(dir);
                            if ui.button("Browse").clicked() {
                                if let Some(path) = FileDialog::new()
                                    .set_title("Select Game Data Directory")
                                    .pick_folder() 
                                {
                                    *dir = path.to_string_lossy().to_string();
                                }
                            }
                        });
                    }
                    
                    if ui.button("Add Directory").clicked() {
                        config.game_data_dirs.push(String::new());
                    }
                    
                    ui.add_space(8.0);
                    
                    ui.label("Cache Directory:");
                    ui.add_space(4.0);
                    
                    // Initialize cache_dir_string if empty
                    if self.cache_dir_string.is_empty() {
                        self.cache_dir_string = config.cache_dir.to_string_lossy().into_owned();
                    }
                    
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.cache_dir_string);
                        if ui.button("Browse").clicked() {
                            if let Some(path) = FileDialog::new()
                                .set_title("Select Cache Directory")
                                .pick_folder() 
                            {
                                self.cache_dir_string = path.to_string_lossy().to_string();
                            }
                        }
                    });
                    
                    // Update the PathBuf when the string changes
                    config.cache_dir = PathBuf::from(&self.cache_dir_string);
                    
                    // Add cache operations
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("Check Cache").clicked() {
                            // This just checks if cache exists without loading
                            state.refresh_database();
                        }
                        
                        // Show cache status
                        if !state.loading_progress.is_empty() {
                            ui.label(&state.loading_progress);
                        }
                    });
                }
            });
            
            ui.add_space(16.0);
            
            // UI Settings Section
            ui.group(|ui| {
                ui.heading("UI Settings");
                ui.add_space(4.0);
                
                // Dark mode setting
                ui.checkbox(&mut state.ui_settings.dark_mode, "Dark Mode");
                state.dark_mode = state.ui_settings.dark_mode;
                
                ui.add_space(8.0);
                
                // Default cache directory
                ui.label("Default Cache Directory:");
                ui.add_space(4.0);
                
                // Initialize default_cache_dir_string if empty and a default is set
                if self.default_cache_dir_string.is_empty() {
                    if let Some(dir) = &state.ui_settings.default_cache_dir {
                        self.default_cache_dir_string = dir.to_string_lossy().into_owned();
                    }
                }
                
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.default_cache_dir_string);
                    if ui.button("Browse").clicked() {
                        if let Some(path) = FileDialog::new()
                            .set_title("Select Default Cache Directory")
                            .pick_folder() 
                        {
                            self.default_cache_dir_string = path.to_string_lossy().to_string();
                        }
                    }
                    
                    if ui.button("Clear").clicked() {
                        self.default_cache_dir_string.clear();
                        state.ui_settings.default_cache_dir = None;
                    }
                });
                
                // Update the default_cache_dir if string is not empty
                if !self.default_cache_dir_string.is_empty() {
                    state.ui_settings.default_cache_dir = Some(PathBuf::from(&self.default_cache_dir_string));
                }
                
                // Use same as current cache dir
                if let Some(config) = &state.config {
                    if ui.button("Use Current Cache Directory").clicked() {
                        self.default_cache_dir_string = config.cache_dir.to_string_lossy().to_string();
                        state.ui_settings.default_cache_dir = Some(config.cache_dir.clone());
                    }
                }
            });
            
            ui.add_space(16.0);
            
            // Status and game data information
            if state.game_data.is_some() {
                ui.group(|ui| {
                    ui.heading("Game Data Status");
                    ui.add_space(4.0);
                    
                    if let Some(game_data) = &state.game_data {
                        ui.horizontal(|ui| {
                            ui.label("Classes loaded:");
                            ui.strong(format!("{}", game_data.classes.len()));
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Source files:");
                            ui.strong(format!("{}", game_data.file_sources.len()));
                        });
                        
                        if ui.button("Unload Data").clicked() {
                            state.game_data = None;
                            state.selected_class = None;
                            state.search_results.clear();
                            state.search_text.clear();
                            state.loading_progress = "Game data unloaded".to_string();
                        }
                    }
                });
                
                ui.add_space(16.0);
            }
            
            // Save settings button
            if ui.button("Save Settings").clicked() {
                // Save scan config
                if let Some(config) = &state.config {
                    if let Err(err) = config.save(&state.config_path.to_string_lossy()) {
                        state.error_message = Some(format!("Failed to save scan config: {}", err));
                    } else {
                        state.error_message = None;
                    }
                }
                
                // Save UI settings
                if let Err(err) = state.save_ui_settings() {
                    state.error_message = Some(format!("Failed to save UI settings: {}", err));
                } else if state.error_message.is_none() {
                    state.error_message = Some("Settings saved successfully.".to_string());
                }
            }
            
            // Show any error message
            if let Some(error) = &state.error_message {
                ui.add_space(16.0);
                ui.colored_label(egui::Color32::RED, error);
            }
        });
    }
} 