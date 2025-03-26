use eframe;
use arma3_tool::ui::Arma3ToolUI;

fn main() -> eframe::Result<()> {
    env_logger::init();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("Arma 3 Tool - Class Browser"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Arma 3 Tool - Class Browser",
        options,
        Box::new(|cc| Ok(Box::new(Arma3ToolUI::new(cc))))
    )
} 