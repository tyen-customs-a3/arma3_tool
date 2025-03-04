use log::{debug, error, info, warn};

/// Log a message related to PBO scanning
/// 
/// This function is provided for backward compatibility.
/// It's recommended to use the standard log macros with appropriate targets instead.
#[deprecated(
    since = "0.2.0",
    note = "Please use standard log macros with appropriate targets instead"
)]
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
/// 
/// This function is provided for backward compatibility.
/// It's recommended to use the standard log macros with appropriate targets instead.
#[deprecated(
    since = "0.2.0",
    note = "Please use standard log macros with appropriate targets instead"
)]
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
/// 
/// This function is provided for backward compatibility.
/// It's recommended to use the standard log macros with appropriate targets instead.
#[deprecated(
    since = "0.2.0",
    note = "Please use standard log macros with appropriate targets instead"
)]
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
/// 
/// This function is provided for backward compatibility.
/// It's recommended to use the standard log macros with appropriate targets instead.
#[deprecated(
    since = "0.2.0",
    note = "Please use standard log macros with appropriate targets instead"
)]
pub fn log_dependency_analysis(level: &str, message: &str) {
    match level.to_lowercase().as_str() {
        "error" => error!(target: "arma3_tool::manager", "{}", message),
        "warn" => warn!(target: "arma3_tool::manager", "{}", message),
        "info" => info!(target: "arma3_tool::manager", "{}", message),
        "debug" => debug!(target: "arma3_tool::manager", "{}", message),
        _ => info!(target: "arma3_tool::manager", "{}", message),
    }
} 