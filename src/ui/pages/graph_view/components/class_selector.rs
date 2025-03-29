use eframe::egui;
use crate::ui::pages::graph_view::state::{GraphViewState, ViewMode};
use arma3_tool_shared_models::GameDataClasses;
use std::path::PathBuf;

pub struct ClassSelector;

impl ClassSelector {
    pub fn show(ui: &mut egui::Ui, state: &mut GraphViewState, game_data: &GameDataClasses) {
        match state.view_mode {
            ViewMode::ClassHierarchy => {
                // Class selection UI
                ui.horizontal(|ui| {
                    ui.label("Class Filter:");
                    let filter_changed = ui.text_edit_singleline(&mut state.class_filter).changed();
                    if filter_changed {
                        state.handle_class_filter_change(game_data);
                    }
                });

                // Show filtered class list in a scrollable area
                let mut selected_class = None;
                egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                    for class_name in &state.filtered_classes {
                        let class_name_clone = class_name.clone();
                        if ui.selectable_label(state.root_class == class_name_clone, &class_name_clone).clicked() {
                            selected_class = Some(class_name_clone);
                        }
                    }
                });
                
                // Process selected class after the iterator completes
                if let Some(class_name) = selected_class {
                    state.handle_root_class_selection(class_name);
                }

                ui.label(format!("Selected Class: {}", state.root_class));
            }
            ViewMode::PboAnalysis => {
                // PBO selection UI
                ui.horizontal(|ui| {
                    ui.label("PBO Filter:");
                    let filter_changed = ui.text_edit_singleline(&mut state.pbo_filter).changed();
                    if filter_changed {
                        state.handle_pbo_filter_change(game_data);
                    }
                });

                // Show filtered PBO list in a scrollable area
                let mut selected_pbo = None;
                egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                    for pbo_path in &state.filtered_pbos {
                        let pbo_path_clone = pbo_path.clone();
                        let pbo_name = pbo_path.file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        
                        if ui.selectable_label(false, &pbo_name).clicked() {
                            selected_pbo = Some(pbo_path_clone);
                        }
                    }
                });
                
                // Process selected PBO after the iterator completes
                if let Some(pbo_path) = selected_pbo {
                    state.handle_pbo_selection(&pbo_path);
                }
            }
            ViewMode::Custom => {
                ui.label("Custom selection options will be added here");
            }
            ViewMode::ImpactAnalysis => {
                // No class selection needed for impact analysis
                // handled in impact_analysis.rs
            }
        }
    }
} 