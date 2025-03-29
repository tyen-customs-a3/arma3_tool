use eframe::egui;
use crate::ui::pages::graph_view::state::{GraphViewState, ViewMode, HierarchyMode};

pub struct SettingsPanel;

impl SettingsPanel {
    pub fn show(ui: &mut egui::Ui, state: &mut GraphViewState) {
        ui.horizontal(|ui| {
            ui.label("View Mode:");
            
            let old_view_mode = state.view_mode.clone();
            
            if ui.radio_value(&mut state.view_mode, ViewMode::ClassHierarchy, "Class Hierarchy").clicked() {
                state.handle_view_mode_change(&old_view_mode);
            }
            if ui.radio_value(&mut state.view_mode, ViewMode::PboAnalysis, "PBO Analysis").clicked() {
                state.handle_view_mode_change(&old_view_mode);
            }
            if ui.radio_value(&mut state.view_mode, ViewMode::Custom, "Custom Analysis").clicked() {
                state.handle_view_mode_change(&old_view_mode);
            }
            if ui.radio_value(&mut state.view_mode, ViewMode::ImpactAnalysis, "Impact Analysis").clicked() {
                state.handle_view_mode_change(&old_view_mode);
            }
        });

        ui.add_space(8.0);

        match state.view_mode {
            ViewMode::ClassHierarchy => {
                // Class hierarchy specific settings
                ui.horizontal(|ui| {
                    ui.label("Max Depth:");
                    
                    let old_depth = state.max_depth;
                    if ui.add(egui::Slider::new(&mut state.max_depth, 1..=10)).changed() {
                        state.handle_max_depth_change(old_depth);
                    }
                    
                    ui.separator();
                    
                    let old_hierarchy_mode = state.hierarchy_mode.clone();
                    if ui.radio_value(&mut state.hierarchy_mode, HierarchyMode::FromRoot, "From Root").clicked() {
                        state.handle_hierarchy_mode_change(&old_hierarchy_mode);
                    }
                    if ui.radio_value(&mut state.hierarchy_mode, HierarchyMode::FromClass, "From Class").clicked() {
                        state.handle_hierarchy_mode_change(&old_hierarchy_mode);
                    }
                });
            }
            ViewMode::PboAnalysis => {
                // PBO analysis metrics
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
            ViewMode::ImpactAnalysis => {
                // Impact analysis metrics are shown directly in the ImpactAnalysis component
            }
            ViewMode::Custom => {
                ui.label("Custom analysis options will be added here");
            }
        }
        
        // Display performance metrics for all modes
        if state.metrics.analysis_time_ms > 0 {
            ui.horizontal(|ui| {
                ui.label(format!("Analysis time: {} ms", state.metrics.analysis_time_ms));
            });
        }
    }
} 