use eframe::egui;
use crate::ui::state::Arma3ToolState;

pub struct ClassDetailsPanel;

impl ClassDetailsPanel {
    pub fn show(ui: &mut egui::Ui, state: &Arma3ToolState) {
        if let Some(class) = &state.selected_class {
            ui.heading(&class.name);
            ui.add_space(4.0);
            
            // Parent class
            if let Some(parent) = &class.parent {
                ui.horizontal(|ui| {
                    ui.label("Parent:");
                    ui.strong(parent);
                });
            }
            
            // Source information
            if let Some(source_graph) = &state.source_graph {
                ui.add_space(8.0);
                ui.heading("Source Information");
                ui.add_space(4.0);
                
                // Current source file
                if let Some(source_pbo) = source_graph.get_source_pbo(&class.name) {
                    ui.horizontal(|ui| {
                        ui.label("Current file:");
                        ui.monospace(source_pbo.display().to_string());
                    });
                }
                
                // Original PBO
                if let Some(original_pbo) = source_graph.get_original_pbo(&class.name) {
                    ui.horizontal(|ui| {
                        ui.label("Original PBO:");
                        ui.monospace(original_pbo.display().to_string());
                    });
                }
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
} 