use anyhow::Result;
use log::{debug, error, info, trace, warn, LevelFilter};
use std::path::Path;

mod helpers;

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
        
        // Define a pattern that includes the target context
        let pattern = "{d(%Y-%m-%d %H:%M:%S)} [{h({l}):<5}] [{t}] {m}{n}";
        
        // Create appenders
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
            .build();
        
        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(pattern)))
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

/// Macro for logging with a specific level and target
#[macro_export]
macro_rules! log_with_level {
    ($level:expr, $($arg:tt)+) => {
        match $level.to_lowercase().as_str() {
            "error" => log::error!($($arg)+),
            "warn" => log::warn!($($arg)+),
            "info" => log::info!($($arg)+),
            "debug" => log::debug!($($arg)+),
            "trace" => log::trace!($($arg)+),
            _ => log::info!($($arg)+),
        }
    };
} 