// This file defines the pages of the application
pub mod processing;
pub mod reporting;
pub mod settings;
pub mod browsing;

pub use processing::extract::ExtractPage;
pub use reporting::reports::ReportsPage;
pub use settings::SettingsPage;
pub use browsing::BrowserPage;

/// Page identifier
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PageId {
    Extract,
    Browser,
    Reports,
    Settings
}

/// Trait for UI pages
pub trait Page {
    /// Get the page identifier
    fn id(&self) -> PageId;
    
    /// Get the page title
    fn title(&self) -> &'static str;
    
    /// Render the page
    fn show(&mut self, ui: &mut eframe::egui::Ui, state: &mut crate::ui::state::Arma3ToolState);
} 