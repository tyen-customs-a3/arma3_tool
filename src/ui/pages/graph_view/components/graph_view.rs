use eframe::egui;
use egui::Color32;
use egui_graphs::{
    GraphView, SettingsNavigation, SettingsInteraction,
    DefaultNodeShape, DefaultEdgeShape,
};
use petgraph::{Directed, stable_graph::DefaultIx};
use crate::ui::pages::graph_view::state::{GraphViewState, ViewMode, NodeStatus};

pub struct GraphViewComponent;

impl GraphViewComponent {
    pub fn show(ui: &mut egui::Ui, state: &mut GraphViewState) {
        if let Some(graph) = &mut state.graph {
            let response = ui.add(
                &mut GraphView::<String, (), Directed, DefaultIx, DefaultNodeShape, DefaultEdgeShape>::new(graph)
                    .with_navigations(
                        &SettingsNavigation::default()
                            .with_fit_to_screen_enabled(true)
                            .with_zoom_and_pan_enabled(true),
                    )
                    .with_interactions(
                        &SettingsInteraction::default()
                            .with_node_selection_enabled(true)
                            .with_dragging_enabled(true),
                    ),
            );
            
            // For now, just clear selection if background is clicked
            // The properties panel will try to determine which node is selected
            if response.clicked() {
                if !ui.input(|i| i.pointer.any_down()) {
                    // Clear selection if clicked on background
                    state.handle_node_selection(None);
                }
            }
            
            // Display impact legend if in impact analysis mode
            if state.view_mode == ViewMode::ImpactAnalysis {
                Self::show_impact_legend(ui, state);
            }
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("No graph data. Select view mode and settings to generate graph.");
            });
        }
        
        // Display performance metrics
        if state.metrics.analysis_time_ms > 0 {
            ui.separator();
            ui.label(format!("Analysis time: {} ms", state.metrics.analysis_time_ms));
        }
    }
    
    fn show_impact_legend(ui: &mut egui::Ui, state: &GraphViewState) {
        // Display legend
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Legend:");
            ui.label("▮ Normal");
            
            ui.label(egui::RichText::new("▮ Removed")
                .color(Color32::from_rgb(255, 0, 0)));
            
            ui.label(egui::RichText::new("▮ Orphaned")
                .color(Color32::from_rgb(255, 165, 0)));
            
            ui.label(egui::RichText::new("▮ Affected")
                .color(Color32::from_rgb(255, 255, 0)));
        });
        
        // Stats
        ui.separator();
        ui.label(format!("Classes to remove: {}", state.classes_to_remove.len()));
        ui.label(format!("Orphaned classes: {}", state.orphaned_classes.len()));
        ui.label(format!("Affected classes: {}", state.affected_classes.len()));
    }
} 