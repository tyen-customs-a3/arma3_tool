use eframe::egui;
use crate::ui::state::Arma3ToolState;
use super::{
    state::{GraphViewState, ViewMode, HierarchyMode},
    components::{PanelView, GraphViewComponent, PropertiesPanel},
};
use crate::ui::pages::Page;
use std::time::Instant;
use arma3_tool_shared_models::GameDataClasses;

pub struct GraphViewPage {
    state: GraphViewState,
    last_update: Option<Instant>,
}

impl Default for GraphViewPage {
    fn default() -> Self {
        Self {
            state: GraphViewState::default(),
            last_update: None,
        }
    }
}

impl Page for GraphViewPage {
    fn id(&self) -> super::super::PageId {
        super::super::PageId::GraphView
    }

    fn title(&self) -> &'static str {
        "Class Graph View"
    }

    fn show(&mut self, ui: &mut egui::Ui, app_state: &mut Arma3ToolState) {
        if app_state.game_data.is_none() {
            ui.heading("No game data loaded");
            ui.label("Please load game data first in the Extract Data or Settings page.");
            return;
        }

        let game_data = app_state.game_data.as_ref().unwrap();
        
        // Initialize dependency cache if needed
        self.state.ensure_dependency_cache(game_data);
        
        // Initialize filtered classes and PBOs if they're empty
        if self.state.filtered_classes.is_empty() {
            self.state.handle_class_filter_change(game_data);
        }
        
        if self.state.filtered_pbos.is_empty() {
            self.state.handle_pbo_filter_change(game_data);
        }
        
        // Initialize graph if it's None
        if self.state.graph.is_none() && self.state.view_mode == ViewMode::ClassHierarchy {
            self.state.update_graph(game_data);
        }

        // Use a three-column layout
        egui::SidePanel::left("graph_settings_panel")
            .resizable(true)
            .default_width(250.0)
            .min_width(200.0)
            .show_inside(ui, |ui| {
                // Left side - Settings and controls
                PanelView::show(ui, &mut self.state, game_data);
                
                // Update button
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Update Graph").clicked() {
                        self.handle_update_graph(game_data);
                    }
                    
                    if let Some(last_update) = self.last_update {
                        ui.label(format!("Last update: {:.2} sec ago", last_update.elapsed().as_secs_f32()));
                    }
                });
            });

        // Right panel - Properties display
        egui::SidePanel::right("graph_properties_panel")
            .resizable(true)
            .default_width(300.0)
            .min_width(200.0)
            .show_inside(ui, |ui| {
                PropertiesPanel::show(ui, &mut self.state, game_data);
            });
        
        // Center panel - Graph display
        egui::CentralPanel::default().show_inside(ui, |ui| {
            GraphViewComponent::show(ui, &mut self.state);
        });
    }
}

impl GraphViewPage {
    fn handle_update_graph(&mut self, game_data: &GameDataClasses) {
        let now = Instant::now();
        self.state.update_graph(game_data);
        self.last_update = Some(now);
    }
} 