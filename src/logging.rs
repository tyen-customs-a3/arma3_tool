use anyhow::Result;
use log::{debug, error, info, warn, LevelFilter};
use std::path::Path;

/// Initialize the logger with log4rs
pub fn initialize_logger(config_path: &Path, log_level: LevelFilter) -> Result<()> {
    // Check if the config file exists
    if config_path.exists() {
        // Use the YAML configuration file
        log4rs::init_file(config_path, Default::default())?;
        info!("Initialized logging from configuration file: {}", config_path.display());
    } else {
        // Create a basic configuration programmatically
        use log4rs::append::console::ConsoleAppender;
        use log4rs::append::file::FileAppender;
        use log4rs::encode::pattern::PatternEncoder;
        use log4rs::config::{Appender, Config, Root};
        
        // Create the logs directory if it doesn't exist
        std::fs::create_dir_all("logs")?;
        
        // Create appenders
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{h({l}):<5}] {m}{n}")))
            .build();
        
        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{h({l}):<5}] {m}{n}")))
            .build("logs/arma3_tool.log")?;
        
        // Build the configuration
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("file", Box::new(file)))
            .build(Root::builder()
                .appender("stdout")
                .appender("file")
                .build(log_level))?;
        
        // Initialize the logger
        log4rs::init_config(config)?;
        info!("Initialized logging with default configuration");
        info!("Log level set to: {:?}", log_level);
    }
    
    Ok(())
}

/// Log a message related to PBO scanning
pub fn log_pbo_scan(level: &str, message: &str) {
    match level.to_lowercase().as_str() {
        "error" => error!(target: "arma3_tool::scanning::pbo", "{}", message),
        "warn" => warn!(target: "arma3_tool::scanning::pbo", "{}", message),
        "info" => info!(target: "arma3_tool::scanning::pbo", "{}", message),
        "debug" => debug!(target: "arma3_tool::scanning::pbo", "{}", message),
        _ => info!(target: "arma3_tool::scanning::pbo", "{}", message),
    }
}

/// Log a message related to class scanning
pub fn log_class_scan(level: &str, message: &str) {
    match level.to_lowercase().as_str() {
        "error" => error!(target: "arma3_tool::scanning::classes", "{}", message),
        "warn" => warn!(target: "arma3_tool::scanning::classes", "{}", message),
        "info" => info!(target: "arma3_tool::scanning::classes", "{}", message),
        "debug" => debug!(target: "arma3_tool::scanning::classes", "{}", message),
        _ => info!(target: "arma3_tool::scanning::classes", "{}", message),
    }
}

/// Log a message related to mission scanning
pub fn log_mission_scan(level: &str, message: &str) {
    match level.to_lowercase().as_str() {
        "error" => error!(target: "arma3_tool::scanning::missions", "{}", message),
        "warn" => warn!(target: "arma3_tool::scanning::missions", "{}", message),
        "info" => info!(target: "arma3_tool::scanning::missions", "{}", message),
        "debug" => debug!(target: "arma3_tool::scanning::missions", "{}", message),
        _ => info!(target: "arma3_tool::scanning::missions", "{}", message),
    }
}

/// Log a message related to dependency analysis
pub fn log_dependency_analysis(level: &str, message: &str) {
    match level.to_lowercase().as_str() {
        "error" => error!(target: "arma3_tool::manager", "{}", message),
        "warn" => warn!(target: "arma3_tool::manager", "{}", message),
        "info" => info!(target: "arma3_tool::manager", "{}", message),
        "debug" => debug!(target: "arma3_tool::manager", "{}", message),
        _ => info!(target: "arma3_tool::manager", "{}", message),
    }
}

/// Create a macro for logging with a specific target
#[macro_export]
macro_rules! log_with_target {
    ($target:expr, $level:expr, $($arg:tt)+) => {
        match $level {
            "error" => log::error!(target: $target, $($arg)+),
            "warn" => log::warn!(target: $target, $($arg)+),
            "info" => log::info!(target: $target, $($arg)+),
            "debug" => log::debug!(target: $target, $($arg)+),
            "trace" => log::trace!(target: $target, $($arg)+),
            _ => log::info!(target: $target, $($arg)+),
        }
    };
} 