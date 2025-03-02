pub mod commands;
pub mod scanning;
pub mod reporting;

// Re-export the main scanning functions for easier access
pub use scanning::pbo::scan_pbos;
pub use scanning::classes::scan_classes;
pub use scanning::missions::scan_missions;
pub use scanning::missions::analyze_mission_dependencies;
pub use scanning::full_analysis;

// Re-export the main reporting types for easier access
pub use reporting::{
    ReportFormat,
    ReportWriter,
    MissionReportWriter,
    ClassReportWriter,
    DependencyReportWriter,
    ReportConfig,
};

/// Create a report configuration from command-line arguments
pub fn create_report_config(disable_reports: Option<&str>, enable_reports: Option<&str>) -> ReportConfig {
    match (disable_reports, enable_reports) {
        // If enable_reports is specified, create a config with all reports disabled by default
        // and then enable only the specified reports
        (_, Some(enable_list)) => {
            let mut config = ReportConfig::all_disabled();
            for report_type in enable_list.split(',').map(|s| s.trim()) {
                if !report_type.is_empty() {
                    config.enable(report_type);
                }
            }
            config
        },
        // If only disable_reports is specified, create a config with all reports enabled by default
        // and then disable the specified reports
        (Some(disable_list), None) => {
            let mut config = ReportConfig::new();
            for report_type in disable_list.split(',').map(|s| s.trim()) {
                if !report_type.is_empty() {
                    config.disable(report_type);
                }
            }
            config
        },
        // If neither is specified, create a config with all reports enabled by default
        (None, None) => ReportConfig::new(),
    }
} 