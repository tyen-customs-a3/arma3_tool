use eframe::egui;
use crate::
    ui::{
        components::TopPanel,
        state::Arma3ToolState,
        pages::{
            PageId,
            Page,
            SettingsPage,
            GraphViewPage
        },
    }
;
use std::path::PathBuf;

pub struct Arma3ToolUI {
    state: Arma3ToolState,
    current_page: Box<dyn Page>,
    settings_page: SettingsPage,
    graph_view_page: GraphViewPage,
}

impl Arma3ToolUI {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Create the initial state
        let mut state = Arma3ToolState::default();
        
        // Set up custom fonts and style if needed
        let mut style = (*cc.egui_ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        cc.egui_ctx.set_style(style);
        
        // Apply the dark mode setting from UI settings
        let dark_mode = state.ui_settings.dark_mode;
        state.dark_mode = dark_mode;
        
        // Apply the default cache dir if set
        if let Some(default_cache_dir) = &state.ui_settings.default_cache_dir {
            if state.config.is_none() {
                // Create a new config with the default cache dir
                let mut config_path = PathBuf::from("scan_config.json");
                if let Ok(config) = crate::config::ScanConfig::load(&config_path.to_string_lossy()) {
                    state.config = Some(config);
                }
            }
        }
        
        // Create the application with default state
        let mut app = Self {
            state,
            current_page: Box::new(GraphViewPage::default()),
            settings_page: SettingsPage::default(),
            graph_view_page: GraphViewPage::default(),
        };
        
        // Load initial data
        app.refresh_database();
        
        app
    }
    
    pub fn refresh_database(&mut self) {
        self.state.refresh_database();
    }
    
    pub fn save_settings(&mut self) {
        // Save UI settings
        let _ = self.state.save_ui_settings();
    }
}

impl eframe::App for Arma3ToolUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set theme
        if self.state.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
        
        // Show top panel
        TopPanel::show(ctx, &mut self.state);
        
        // Show error panel if needed
        if let Some(error) = &self.state.error_message {
            egui::TopBottomPanel::top("error_panel").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.colored_label(egui::Color32::RED, "⚠");
                    ui.colored_label(egui::Color32::RED, error);
                });
            });
        }
        
        // Show navigation sidebar
        egui::SidePanel::left("nav_panel")
            .resizable(false)
            .min_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Navigation");
                ui.add_space(8.0);
                
                if ui.selectable_label(
                    matches!(self.current_page.id(), PageId::GraphView),
                    "Class Graph"
                ).clicked() {
                    self.current_page = Box::new(GraphViewPage::default());
                }
                
                if ui.selectable_label(
                    matches!(self.current_page.id(), PageId::Settings),
                    "Settings"
                ).clicked() {
                    self.current_page = Box::new(SettingsPage::default());
                }
            });
        
        // Show current page
        egui::CentralPanel::default().show(ctx, |ui| {
            self.current_page.show(ui, &mut self.state);
        });
        
        // Save settings when app updates
        self.save_settings();
    }
} 