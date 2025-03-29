pub mod settings;
mod graph_view;

pub use settings::SettingsPage;
pub use graph_view::GraphViewPage;

/// Page identifier
#[derive(Debug, PartialEq)]
pub enum PageId {
    GraphView,
    Settings,
}

/// Trait for UI pages
pub trait Page {
    /// Get the page identifier
    fn id(&self) -> PageId;
    
    /// Get the page title
    fn title(&self) -> &'static str;
    
    /// Render the page
    fn show(&mut self, ui: &mut egui::Ui, state: &mut crate::ui::state::Arma3ToolState);
} 