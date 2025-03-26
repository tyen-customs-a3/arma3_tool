use eframe::egui;
use crate::ui::{
    state::Arma3ToolState,
    pages::{Page, PageId},
};
use super::super::processing::components::ParsePanel;

pub struct DiagnosticPage;

impl Default for DiagnosticPage {
    fn default() -> Self {
        Self
    }
}

impl Page for DiagnosticPage {
    fn id(&self) -> PageId {
        PageId::Reports
    }

    fn title(&self) -> &'static str {
        "Diagnostic Scan"
    }

    fn show(&mut self, ui: &mut egui::Ui, state: &mut Arma3ToolState) {
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ParsePanel::show(ui, state);
        });
    }
} 