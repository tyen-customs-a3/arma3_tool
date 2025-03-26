use eframe::egui;
use crate::ui::state::Arma3ToolState;
use crate::ui::pages::{Page, PageId};
use arma3_tool_report_writer::{
    DependencyReportWriter, ComparisonReportWriter, FuzzySearchReportWriter,
    ensure_dir_exists, MissionDependencyBuilder
};
use arma3_tool_cache_storage::StorageManager;
use rfd::FileDialog;
use std::path::PathBuf;
use log::{info, warn};
use arma3_tool_shared_models::MissionData;
use once_cell::sync::Lazy;

static EMPTY_MISSION_DATA: Lazy<MissionData> = Lazy::new(|| MissionData::new());

#[derive(PartialEq)]
enum ReportType {
    Dependency,
    Comparison,
    FuzzySearch,
}

impl ReportType {
    fn name(&self) -> &'static str {
        match self {
            ReportType::Dependency => "Dependency Report",
            ReportType::Comparison => "Comparison Report",
            ReportType::FuzzySearch => "Fuzzy Search Report",
        }
    }
}

pub struct ReportsPage {
    output_dir: Option<PathBuf>,
    cache_dir_a: Option<PathBuf>,
    cache_dir_b: Option<PathBuf>,
    fuzzy_threshold: f64,
    is_generating: bool,
    status_message: String,
    selected_report: ReportType,
}

impl Default for ReportsPage {
    fn default() -> Self {
        Self {
            output_dir: None,
            cache_dir_a: None,
            cache_dir_b: None,
            fuzzy_threshold: 0.8,
            is_generating: false,
            status_message: String::new(),
            selected_report: ReportType::Dependency,
        }
    }
}

impl Page for ReportsPage {
    fn id(&self) -> PageId {
        PageId::Reports
    }

    fn title(&self) -> &'static str {
        "Reports"
    }

    fn show(&mut self, ui: &mut egui::Ui, state: &mut Arma3ToolState) {
        egui::CentralPanel::default().show(ui.ctx(), |ui| {
            ui.heading("Generate Reports");
            ui.add_space(8.0);

            // Report type selection
            ui.horizontal(|ui| {
                ui.label("Report Type:");
                ui.selectable_value(&mut self.selected_report, ReportType::Dependency, "Dependency");
                ui.selectable_value(&mut self.selected_report, ReportType::Comparison, "Comparison");
                ui.selectable_value(&mut self.selected_report, ReportType::FuzzySearch, "Fuzzy Search");
            });

            ui.add_space(16.0);

            // Directory inputs based on report type
            match self.selected_report {
                ReportType::Dependency => {
                    self.show_single_cache_dir_input(ui);
                }
                ReportType::Comparison => {
                    self.show_comparison_dir_inputs(ui);
                }
                ReportType::FuzzySearch => {
                    self.show_single_cache_dir_input(ui);
                    
                    // Fuzzy search threshold
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label("Fuzzy Search Threshold:");
                        ui.add(egui::Slider::new(&mut self.fuzzy_threshold, 0.0..=1.0)
                            .text("Threshold"));
                    });
                }
            }

            // Output directory selection
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.label("Output Directory:");
                if ui.button("Browse").clicked() {
                    if let Some(path) = FileDialog::new()
                        .set_title("Select Output Directory")
                        .pick_folder() 
                    {
                        self.output_dir = Some(path);
                    }
                }
                if let Some(dir) = &self.output_dir {
                    ui.label(dir.display().to_string());
                } else {
                    ui.weak("No directory selected");
                }
            });

            ui.add_space(16.0);

            // Generate button
            if !self.is_generating {
                if ui.button(format!("Generate {}", self.selected_report.name())).clicked() {
                    match self.selected_report {
                        ReportType::Dependency => self.generate_dependency_report(state),
                        ReportType::Comparison => self.generate_comparison_report(),
                        ReportType::FuzzySearch => self.generate_fuzzy_report(state),
                    }
                }
            }

            // Status message
            if !self.status_message.is_empty() {
                ui.add_space(8.0);
                ui.label(&self.status_message);
            }
        });
    }
}

impl ReportsPage {
    fn show_single_cache_dir_input(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Cache Directory:");
            if ui.button("Browse").clicked() {
                if let Some(path) = FileDialog::new()
                    .set_title("Select Cache Directory")
                    .pick_folder() 
                {
                    self.cache_dir_a = Some(path);
                }
            }
            if let Some(dir) = &self.cache_dir_a {
                ui.label(dir.display().to_string());
            } else {
                ui.weak("No directory selected");
            }
        });
    }

    fn show_comparison_dir_inputs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Reference Cache Directory (A):");
            if ui.button("Browse##a").clicked() {
                if let Some(path) = FileDialog::new()
                    .set_title("Select Reference Cache Directory")
                    .pick_folder() 
                {
                    self.cache_dir_a = Some(path);
                }
            }
            if let Some(dir) = &self.cache_dir_a {
                ui.label(dir.display().to_string());
            } else {
                ui.weak("No directory selected");
            }
        });

        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Comparison Cache Directory (B):");
            if ui.button("Browse##b").clicked() {
                if let Some(path) = FileDialog::new()
                    .set_title("Select Comparison Cache Directory")
                    .pick_folder() 
                {
                    self.cache_dir_b = Some(path);
                }
            }
            if let Some(dir) = &self.cache_dir_b {
                ui.label(dir.display().to_string());
            } else {
                ui.weak("No directory selected");
            }
        });
    }

    fn generate_dependency_report(&mut self, state: &mut Arma3ToolState) {
        if let Some(output_dir) = &self.output_dir {
            if let Some(cache_dir) = &self.cache_dir_a {
                self.is_generating = true;
                self.status_message = "Generating dependency report...".to_string();

                // Create storage and load data
                let storage = StorageManager::new(cache_dir);

                // Load cache data
                match storage.load() {
                    Ok(cache_data) => {
                        // Create scanner
                        let scanner = MissionDependencyBuilder::new(&cache_data.game_data);

                        // Scan missions
                        let scan_report = scanner.scan_missions(&cache_data.mission_data);

                        // Write reports
                        match DependencyReportWriter::new(&scan_report).write_report(output_dir) {
                            Ok(_) => {
                                if !scan_report.missing.is_empty() {
                                    self.status_message = format!(
                                        "Found {} missing dependencies across {} missions. Report saved to {}",
                                        scan_report.missing.len(),
                                        scan_report.total_missions_scanned,
                                        output_dir.display()
                                    );
                                    warn!("{}", self.status_message);
                                } else {
                                    self.status_message = format!(
                                        "No missing dependencies found across {} missions. Report saved to {}",
                                        scan_report.total_missions_scanned,
                                        output_dir.display()
                                    );
                                    info!("{}", self.status_message);
                                }
                            }
                            Err(e) => {
                                self.status_message = format!("Failed to write report: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        self.status_message = format!("Failed to load cache data: {}", e);
                    }
                }

                self.is_generating = false;
            } else {
                self.status_message = "Please select a cache directory first.".to_string();
            }
        } else {
            self.status_message = "Please select an output directory first.".to_string();
        }
    }

    fn generate_comparison_report(&mut self) {
        if let Some(output_dir) = &self.output_dir {
            if let Some(cache_dir_a) = &self.cache_dir_a {
                if let Some(cache_dir_b) = &self.cache_dir_b {
                    self.is_generating = true;
                    self.status_message = "Generating comparison report...".to_string();

                    // Create storage and load data for both sets
                    let storage_a = StorageManager::new(cache_dir_a);
                    let storage_b = StorageManager::new(cache_dir_b);

                    // Load cache data
                    match (storage_a.load(), storage_b.load()) {
                        (Ok(cache_data_a), Ok(cache_data_b)) => {
                            // Create scanners
                            let scanner_a = MissionDependencyBuilder::new(&cache_data_a.game_data);
                            let scanner_b = MissionDependencyBuilder::new(&cache_data_b.game_data);

                            // Scan missions for both sets
                            let scan_report_a = scanner_a.scan_missions(&cache_data_a.mission_data);
                            let scan_report_b = scanner_b.scan_missions(&cache_data_b.mission_data);

                            // Write comparison report
                            match ComparisonReportWriter::new(&scan_report_a, &scan_report_b, &cache_data_a.game_data)
                                .write_report(output_dir) {
                                Ok(comparison) => {
                                    if !comparison.missing_in_b.is_empty() {
                                        self.status_message = format!(
                                            "Found {} dependencies in set B that were previously defined in set A. Report saved to {}",
                                            comparison.missing_in_b.len(),
                                            output_dir.display()
                                        );
                                        warn!("{}", self.status_message);
                                    } else {
                                        self.status_message = format!(
                                            "No missing dependencies found in set B. Report saved to {}",
                                            output_dir.display()
                                        );
                                        info!("{}", self.status_message);
                                    }
                                }
                                Err(e) => {
                                    self.status_message = format!("Failed to write comparison report: {}", e);
                                }
                            }
                        }
                        (Err(e), _) | (_, Err(e)) => {
                            self.status_message = format!("Failed to load cache data: {}", e);
                        }
                    }

                    self.is_generating = false;
                } else {
                    self.status_message = "Please select comparison cache directory (B) first.".to_string();
                }
            } else {
                self.status_message = "Please select reference cache directory (A) first.".to_string();
            }
        } else {
            self.status_message = "Please select an output directory first.".to_string();
        }
    }

    fn generate_fuzzy_report(&mut self, state: &mut Arma3ToolState) {
        if let Some(output_dir) = &self.output_dir {
            if let Some(cache_dir) = &self.cache_dir_a {
                self.is_generating = true;
                self.status_message = "Generating fuzzy search report...".to_string();

                // Create storage and load data
                let storage = StorageManager::new(cache_dir);

                // Load cache data
                match storage.load() {
                    Ok(cache_data) => {
                        // Create scanner
                        let scanner = MissionDependencyBuilder::new(&cache_data.game_data);

                        // Scan missions
                        let scan_report = scanner.scan_missions(&cache_data.mission_data);

                        if scan_report.missing.is_empty() {
                            self.status_message = "No missing dependencies found to analyze.".to_string();
                            self.is_generating = false;
                            return;
                        }

                        // Create and write fuzzy search report
                        match FuzzySearchReportWriter::new(&scan_report, &cache_data.game_data)
                            .write_report(output_dir) {
                            Ok(fuzzy_report) => {
                                self.status_message = format!(
                                    "Analyzed {} missing classes and found similar matches. Report saved to {}",
                                    fuzzy_report.missing_classes.len(),
                                    output_dir.display()
                                );
                                info!("{}", self.status_message);
                            }
                            Err(e) => {
                                self.status_message = format!("Failed to write fuzzy search report: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        self.status_message = format!("Failed to load cache data: {}", e);
                    }
                }

                self.is_generating = false;
            } else {
                self.status_message = "Please select a cache directory first.".to_string();
            }
        } else {
            self.status_message = "Please select an output directory first.".to_string();
        }
    }
} 