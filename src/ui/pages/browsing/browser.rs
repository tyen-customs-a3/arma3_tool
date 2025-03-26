use eframe::egui;
use std::path::PathBuf;
use rfd::FileDialog;
use crate::ui::{
    pages::browsing::components::{ClassDetailsPanel, SearchPanel},
    pages::browsing::sourcegraph::SourceGraph,
    state::Arma3ToolState,
    pages::{Page, PageId},
};

pub struct BrowserPage {
    last_loaded_dir: Option<PathBuf>,
    selected_tab: BrowserTab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BrowserTab {
    Details,
    Graph,
}

impl Default for BrowserPage {
    fn default() -> Self {
        Self {
            last_loaded_dir: None,
            selected_tab: BrowserTab::Details,
        }
    }
}

impl Page for BrowserPage {
    fn id(&self) -> PageId {
        PageId::Browser
    }

    fn title(&self) -> &'static str {
        "Browser"
    }

    fn show(&mut self, ui: &mut egui::Ui, state: &mut Arma3ToolState) {
        // Process any pending loading messages
        state.process_loading_messages();

        // Top panel for controls
        egui::TopBottomPanel::top("browser_controls").show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                let button_enabled = !state.is_loading;
                if ui.add_enabled(button_enabled, egui::Button::new("📂 Load Directory")).clicked() {
                    if let Some(path) = FileDialog::new()
                        .set_directory(self.last_loaded_dir.as_ref().unwrap_or(&std::env::current_dir().unwrap_or_default()))
                        .pick_folder() 
                    {
                        self.last_loaded_dir = Some(path.clone());
                        state.load_directory(path);
                    }
                }
                
                if let Some(dir) = &self.last_loaded_dir {
                    ui.label(format!("Current: {}", dir.display()));
                }
            });
        });

        // Start background loading if needed
        if state.game_data.is_some() && state.source_graph.is_none() && !state.is_loading {
            state.start_background_loading();
        }
        
        if state.is_loading {
            // Show loading indicator with progress
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    
                    // Extract progress percentage if available
                    if let Some((current, total)) = state.loading_progress.split_once('[')
                        .and_then(|(_, rest)| rest.split_once('/'))
                        .and_then(|(current, rest)| {
                            let current = current.trim().parse::<f32>().ok()?;
                            rest.split_once(']')
                                .and_then(|(total, _)| total.trim().parse::<f32>().ok())
                                .map(|total| (current, total))
                        }) 
                    {
                        // Show progress bar
                        let progress = current / total;
                        ui.add(egui::ProgressBar::new(progress)
                            .show_percentage()
                            .desired_width(300.0));
                    } else {
                        // Show indeterminate progress bar for operations without percentage
                        ui.add(egui::ProgressBar::new(0.0)
                            .animate(true)
                            .desired_width(300.0));
                    }
                    
                    ui.add_space(20.0);
                    ui.label(&state.loading_progress);
                });
            });
            return;
        }
        
        // Show search panel on left if data is loaded
        if state.source_graph.is_some() {
            egui::SidePanel::left("search_panel")
                .resizable(true)
                .min_width(300.0)
                .show(ui.ctx(), |ui| {
                    SearchPanel::show(ui, state);
                });
                
            // Show class details in central panel
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                ClassDetailsPanel::show(ui, state);
            });
        } else {
            // Show message if no data is loaded
            egui::CentralPanel::default().show(ui.ctx(), |ui| {
                ui.centered_and_justified(|ui| {
                    ui.heading("No data loaded");
                    ui.label("Please load a directory using the button above");
                });
            });
        }
    }
} 