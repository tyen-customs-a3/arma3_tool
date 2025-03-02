use std::path::Path;
use crate::reporting::{
    ClassReportWriter, 
    DependencyReportWriter, 
    MissionReportWriter, 
    ReportFormat, 
    ReportConfig
};

/// Examples of how to use the report configuration system

/// Example: Disable circular dependencies report
pub fn example_disable_circular_dependencies() {
    let output_dir = Path::new("./reports");
    
    // Create a new report configuration
    let mut config = ReportConfig::new();
    
    // Disable the circular dependencies report
    config.disable("circular_dependencies");
    
    // Create a class report writer with the configuration
    let writer = ClassReportWriter::with_config(output_dir, ReportFormat::Json, config);
    
    // Now when you call write_circular_dependencies_report, it will be skipped
    // writer.write_circular_dependencies_report(&circular_deps).unwrap();
}

/// Example: Disable multiple reports
pub fn example_disable_multiple_reports() {
    let output_dir = Path::new("./reports");
    
    // Create a new report configuration and disable multiple reports
    let mut config = ReportConfig::new();
    config.disable("circular_dependencies")
          .disable("class_hierarchy")
          .disable("class_mission_usage");
    
    // Create a class report writer with the configuration
    let writer = ClassReportWriter::with_config(output_dir, ReportFormat::Json, config);
    
    // All disabled reports will be skipped when called
}

/// Example: Disable all reports by default and enable only specific ones
pub fn example_enable_only_specific_reports() {
    let output_dir = Path::new("./reports");
    
    // Create a configuration with all reports disabled by default
    let mut config = ReportConfig::all_disabled();
    
    // Enable only specific reports
    config.enable("classes")
          .enable("class_stats");
    
    // Create a class report writer with the configuration
    let writer = ClassReportWriter::with_config(output_dir, ReportFormat::Json, config);
    
    // Only the enabled reports will be generated
}

/// Example: Configure dependency reports
pub fn example_configure_dependency_reports() {
    let output_dir = Path::new("./reports");
    
    // Create a configuration and disable specific dependency reports
    let mut config = ReportConfig::new();
    config.disable("missing_classes")
          .disable("class_usage_frequency")
          .disable("compatibility_diagnostics");
    
    // Create a dependency report writer with the configuration
    let writer = DependencyReportWriter::with_config(output_dir, ReportFormat::Json, config);
    
    // The disabled reports will be skipped
}

/// Example: Configure mission reports
pub fn example_configure_mission_reports() {
    let output_dir = Path::new("./reports");
    
    // Create a configuration and disable specific mission reports
    let mut config = ReportConfig::new();
    config.disable("mission_summary")
          .disable("equipment_items");
    
    // Disable reports for a specific mission
    config.disable("mission_mission1");
    
    // Create a mission report writer with the configuration
    let writer = MissionReportWriter::with_config(output_dir, ReportFormat::Json, config);
    
    // The disabled reports will be skipped
}

/// Example: Share configuration between different report writers
pub fn example_share_configuration() {
    let output_dir = Path::new("./reports");
    
    // Create a shared configuration
    let mut config = ReportConfig::new();
    config.disable("circular_dependencies")
          .disable("missing_classes")
          .disable("mission_summary");
    
    // Create different report writers with the same configuration
    let class_writer = ClassReportWriter::with_config(output_dir, ReportFormat::Json, config.clone());
    let dependency_writer = DependencyReportWriter::with_config(output_dir, ReportFormat::Json, config.clone());
    let mission_writer = MissionReportWriter::with_config(output_dir, ReportFormat::Json, config);
    
    // All writers will respect the shared configuration
}

/// Example: Modify configuration after writer creation
pub fn example_modify_configuration() {
    let output_dir = Path::new("./reports");
    
    // Create a writer with default configuration
    let mut writer = ClassReportWriter::new(output_dir);
    
    // Modify the configuration after creation
    writer.config_mut().disable("circular_dependencies")
                       .disable("class_hierarchy");
    
    // The writer will now respect the updated configuration
} 