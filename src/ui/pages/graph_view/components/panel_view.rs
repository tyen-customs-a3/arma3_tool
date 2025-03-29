use eframe::egui;
use arma3_tool_shared_models::GameDataClasses;
use crate::ui::pages::graph_view::state::{GraphViewState, ViewMode, HierarchyMode};
use super::{
    settings_panel::SettingsPanel,
    class_selector::ClassSelector,
    impact_analysis::ImpactAnalysis,
};

pub struct PanelView;

impl PanelView {
    pub fn show(ui: &mut egui::Ui, state: &mut GraphViewState, game_data: &GameDataClasses) {
        // Settings panel is always visible
        SettingsPanel::show(ui, state);
        
        ui.add_space(8.0);
        
        // Show content based on view mode
        match state.view_mode {
            ViewMode::ClassHierarchy => {
                if state.hierarchy_mode == HierarchyMode::FromClass {
                    ClassSelector::show(ui, state, game_data);
                }
            }
            ViewMode::PboAnalysis => {
                ClassSelector::show(ui, state, game_data);
            }
            ViewMode::ImpactAnalysis => {
                ImpactAnalysis::show(ui, state);
            }
            ViewMode::Custom => {
                // Custom view doesn't need a selector yet
                ui.label("Custom analysis options will be added here");
                
                if let Some(graph) = &state.analysis_graph {
                    // Show some basic stats about the graph
                    ui.label(format!("Nodes: {}", graph.node_count()));
                    ui.label(format!("Edges: {}", graph.edge_count()));
                }
            }
        }
        
        // Display view-specific metrics
        Self::show_metrics(ui, state);
    }
    
    fn show_metrics(ui: &mut egui::Ui, state: &GraphViewState) {
        if state.view_mode == ViewMode::PboAnalysis {
            ui.separator();
            ui.heading("PBO Analysis Metrics");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Total Size on Disk:");
                    ui.label(format!("{:.2} MB", state.metrics.size_on_disk as f64 / (1024.0 * 1024.0)));
                });
                
                ui.separator();
                
                ui.vertical(|ui| {
                    ui.label("Total Classes:");
                    ui.label(format!("{}", state.metrics.num_classes));
                });
                
                ui.separator();
                
                ui.vertical(|ui| {
                    ui.label("Dependencies:");
                    ui.label(format!("{}", state.metrics.num_dependencies));
                });
            });
        }
    }
} 