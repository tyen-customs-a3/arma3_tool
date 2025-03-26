use eframe::egui;
use crate::ui::state::Arma3ToolState;
use fuzzy_matcher::FuzzyMatcher;

pub struct SearchPanel;

impl SearchPanel {
    pub fn show(ui: &mut egui::Ui, state: &mut Arma3ToolState) {
        // Search box
        ui.horizontal(|ui| {
            ui.label("🔍");
            if ui.text_edit_singleline(&mut state.search_text).changed() {
                Self::search_classes(state);
            }
        });
        
        ui.add_space(4.0);
        
        if !state.search_results.is_empty() {
            ui.label(format!("Found {} matches", state.search_results.len()));
            ui.add_space(4.0);
        }
        
        // Results list
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (name, score) in &state.search_results {
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
                        if let Some(game_data) = &state.game_data {
                            state.selected_class = game_data.classes.iter()
                                .find(|c| &c.name == name)
                                .cloned();
                        }
                    }
                }
            });
    }
    
    fn search_classes(state: &mut Arma3ToolState) {
        // Clear previous results if search is empty
        if state.search_text.is_empty() {
            state.search_results.clear();
            return;
        }

        // Use source graph for fast search if available
        if let Some(source_graph) = &state.source_graph {
            let matches = source_graph.search_classes(&state.search_text);
            
            // Convert to scored results
            let mut scored_results: Vec<(String, i64)> = matches.into_iter()
                .filter_map(|name| {
                    state.fuzzy_matcher.fuzzy_match(&name, &state.search_text)
                        .map(|score| (name, score))
                })
                .collect();
            
            // Sort by score in descending order
            scored_results.sort_by(|a, b| b.1.cmp(&a.1));
            
            // Take top 50 results
            state.search_results = scored_results.into_iter().take(50).collect();
        } else if let Some(game_data) = &state.game_data {
            // Fall back to old search method if source graph not available
            let mut matches: Vec<(String, i64)> = game_data.classes.iter()
                .filter_map(|class| {
                    state.fuzzy_matcher.fuzzy_match(&class.name, &state.search_text)
                        .map(|score| (class.name.clone(), score))
                })
                .collect();
            
            matches.sort_by(|a, b| b.1.cmp(&a.1));
            state.search_results = matches.into_iter().take(50).collect();
        }
    }
} 