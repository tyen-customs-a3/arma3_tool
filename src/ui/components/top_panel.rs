use eframe::egui;
use crate::ui::state::Arma3ToolState;

pub struct TopPanel;

impl TopPanel {
    pub fn show(ctx: &egui::Context, state: &mut Arma3ToolState) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Arma 3 Tool - Class Browser");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(if state.dark_mode { "☀" } else { "🌙" }).clicked() {
                        state.dark_mode = !state.dark_mode;
                    }
                    if ui.button("🔄").on_hover_text("Refresh database").clicked() {
                        state.refresh_database();
                    }
                    if ui.button("📄 Export").on_hover_text("Export all classes to file").clicked() {
                        match state.export_classes_to_file() {
                            Ok(_) => {
                                state.error_message = None;
                            }
                            Err(err) => {
                                state.error_message = Some(format!("Failed to export classes: {}", err));
                            }
                        }
                    }
                });
            });
        });
    }
} 