use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use crate::ui::pages::graph_view::state::GraphViewState;

pub struct ImpactAnalysis;

impl ImpactAnalysis {
    pub fn show(ui: &mut egui::Ui, state: &mut GraphViewState) {
        ui.heading("Impact Analysis");
        
        // File selection
        ui.horizontal(|ui| {
            if ui.button("Load Classes to Remove...").clicked() {
                state.handle_load_classes_to_remove_dialog();
            }
            
            if let Some(path) = &state.impact_file_path {
                ui.label(format!("Loaded: {}", path.display()));
            } else {
                ui.label("No file loaded");
            }
        });
        
        ui.separator();
        
        // Analysis options
        ui.horizontal(|ui| {
            // Various analysis options could go here
            ui.label("Analysis options:");
            ui.checkbox(&mut false, "Include children");
        });
        
        // Display counts and metrics
        ui.separator();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(format!("Classes to remove: {}", state.classes_to_remove.len()));
                ui.label(format!("Orphaned classes: {}", state.orphaned_classes.len()));
                ui.label(format!("Affected classes: {}", state.affected_classes.len()));
            });
            
            if state.classes_to_remove.len() > 0 {
                if ui.button("Analyze Impact").clicked() {
                    state.handle_analyze_impact();
                }
                
                if ui.button("Clear").clicked() {
                    state.handle_clear_impact_analysis();
                }
            }
        });
        
        // Display file contents
        if !state.classes_to_remove.is_empty() {
            ui.separator();
            ui.collapsing("Classes to remove", |ui| {
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for class in &state.classes_to_remove {
                        ui.label(class);
                    }
                });
            });
        }
        
        // Display orphaned classes
        if !state.orphaned_classes.is_empty() {
            ui.collapsing("Orphaned classes", |ui| {
                egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                    for class in &state.orphaned_classes {
                        ui.label(class);
                    }
                });
            });
        }
    }
} 