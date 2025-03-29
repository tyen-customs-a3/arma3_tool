use eframe::egui;
use std::path::PathBuf;
use arma3_tool_shared_models::GameDataClasses;
use crate::ui::pages::graph_view::state::{GraphViewState, ViewMode};

pub struct PropertiesPanel;

impl PropertiesPanel {
    pub fn show(ui: &mut egui::Ui, state: &mut GraphViewState, game_data: &GameDataClasses) {
        ui.heading("Properties");
        
        // Handle showing properties or selectors
        if let Some(node_name) = &state.selected_node.clone() {
            // Display properties for the selected node
            ui.label(format!("Selected: {}", node_name));
            ui.separator();
            
            match state.view_mode {
                ViewMode::PboAnalysis => {
                    Self::show_pbo_properties(ui, node_name, state, game_data);
                },
                ViewMode::ClassHierarchy | ViewMode::ImpactAnalysis => {
                    Self::show_class_properties(ui, node_name, state, game_data);
                },
                ViewMode::Custom => {
                    ui.label("Custom node properties will be shown here");
                }
            }
            
            // Add button to clear selection
            if ui.button("Clear Selection").clicked() {
                state.handle_node_selection(None);
            }
        } else {
            // No node selected, show selector
            ui.label("No node selected");
            ui.label("Click on a node in the graph to view its properties");
            ui.label("or select from list below:");
            
            // Show node selector based on view mode
            match state.view_mode {
                ViewMode::ClassHierarchy | ViewMode::ImpactAnalysis => {
                    Self::show_class_selector(ui, state);
                },
                ViewMode::PboAnalysis => {
                    Self::show_pbo_selector(ui, state);
                },
                _ => {}
            }
        }
    }
    
    fn show_class_selector(ui: &mut egui::Ui, state: &mut GraphViewState) {
        // Show class selection interface
        ui.separator();
        ui.label("Select a class:");
        ui.text_edit_singleline(&mut state.class_filter);
        
        let mut selected = None;
        
        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            for class_name in &state.filtered_classes {
                if state.class_filter.is_empty() || class_name.contains(&state.class_filter) {
                    if ui.selectable_label(false, class_name).clicked() {
                        selected = Some(class_name.clone());
                    }
                }
            }
        });
        
        // Update selection outside the loop to avoid borrow issues
        if let Some(class_name) = selected {
            state.handle_node_selection(Some(class_name));
        }
    }
    
    fn show_pbo_selector(ui: &mut egui::Ui, state: &mut GraphViewState) {
        // Show PBO selection interface
        ui.separator();
        ui.label("Select a PBO:");
        ui.text_edit_singleline(&mut state.pbo_filter);
        
        let mut selected = None;
        
        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            for pbo_path in &state.filtered_pbos {
                let pbo_name = pbo_path.file_name()
                    .map(|name| name.to_string_lossy().to_string())
                    .unwrap_or_default();
                
                if state.pbo_filter.is_empty() || pbo_name.contains(&state.pbo_filter) {
                    if ui.selectable_label(false, &pbo_name).clicked() {
                        selected = Some(pbo_name);
                    }
                }
            }
        });
        
        // Update selection outside the loop to avoid borrow issues
        if let Some(pbo_name) = selected {
            state.handle_node_selection(Some(pbo_name));
        }
    }
    
    fn show_pbo_properties(ui: &mut egui::Ui, pbo_name: &str, state: &GraphViewState, game_data: &GameDataClasses) {
        // Find PBO path
        let pbo_path = state.filtered_pbos.iter()
            .find(|path| path.file_name().map_or(false, |name| name.to_string_lossy().to_string() == pbo_name));
        
        if let Some(pbo_path) = pbo_path {
            // Show PBO information
            if let Ok(metadata) = std::fs::metadata(pbo_path) {
                ui.label(format!("Size: {:.2} MB", metadata.len() as f64 / (1024.0 * 1024.0)));
            }
            
            ui.label(format!("Path: {}", pbo_path.display()));
            ui.separator();
            
            // Count classes in this PBO
            let classes_in_pbo: Vec<&String> = state.dependency_cache.class_to_pbo.iter()
                .filter(|(_, path)| path.as_path() == pbo_path.as_path())
                .map(|(class, _)| class)
                .collect();
            
            ui.label(format!("Contains {} classes", classes_in_pbo.len()));
            
            // Show classes in a collapsible
            ui.collapsing("Classes", |ui| {
                egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                    for class_name in &classes_in_pbo {
                        ui.label(&class_name.to_string());
                    }
                });
            });
            
            // Show dependencies
            ui.separator();
            ui.label("Dependencies");
            
            // This would require analyzing which other PBOs have classes that inherit from this PBO's classes
            // Placeholder for now
            ui.label("Dependency analysis not implemented yet");
        } else {
            ui.label(format!("PBO '{}' information not available", pbo_name));
        }
    }
    
    fn show_class_properties(ui: &mut egui::Ui, class_name: &str, state: &GraphViewState, game_data: &GameDataClasses) {
        // Find the class
        if let Some(class) = game_data.classes.iter().find(|c| &c.name == class_name) {
            // Show basic class information
            ui.horizontal(|ui| {
                ui.label("Parent:");
                if let Some(parent) = &class.parent {
                    ui.label(parent);
                } else {
                    ui.label("None (Root class)");
                }
            });
            
            // Show class source
            if let Some(idx) = class.source_file_index {
                if let Some(source_file) = game_data.file_sources.get(idx) {
                    ui.horizontal(|ui| {
                        ui.label("Source:");
                        ui.label(source_file.file_name().unwrap_or_default().to_string_lossy());
                    });
                }
            }
            
            // Show children
            ui.separator();
            if let Some(children) = state.dependency_cache.children.get(class_name) {
                ui.label(format!("Children: {}", children.len()));
                
                ui.collapsing("Direct Children", |ui| {
                    egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                        for child in children {
                            ui.label(child);
                        }
                    });
                });
            } else {
                ui.label("No children");
            }
            
            // Show class properties
            ui.separator();
            ui.label("Class Properties");
            
            // Display the properties if available
            if !class.properties.is_empty() {
                egui::ScrollArea::vertical().max_height(250.0).show(ui, |ui| {
                    for (key, value) in &class.properties {
                        ui.horizontal(|ui| {
                            ui.label(key);
                            ui.label(":");
                            ui.label(format!("{:?}", value));
                        });
                    }
                });
            } else {
                ui.label("No properties defined");
            }
        } else {
            ui.label(format!("Class '{}' not found in game data", class_name));
        }
    }
} 