use eframe::egui;
use crate::ui::state::Arma3ToolState;
use crate::ui::pages::{Page, PageId};

pub struct ExtractPage;

impl Default for ExtractPage {
    fn default() -> Self {
        Self
    }
}

impl Page for ExtractPage {
    fn id(&self) -> PageId {
        PageId::Extract
    }

    fn title(&self) -> &'static str {
        "Extract Data"
    }

    fn show(&mut self, ui: &mut egui::Ui, state: &mut Arma3ToolState) {
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ui.heading("Extract Game Data");
            ui.add_space(8.0);
            
            ui.label("Extract game data and missions from PBO files.");
            // TODO: Implement extraction UI
        });
    }
} 