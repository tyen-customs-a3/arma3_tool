//! Common utilities, helpers, and macros shared across the Arma3 Tool workspace
//! 
//! This crate provides shared functionality that doesn't belong to any specific
//! layer but is used throughout the application.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Common configuration and utility functions

/// Normalize a path for consistent handling across platforms
pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let mut components = Vec::new();
    
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                if !components.is_empty() && components.last() != Some(&"..") {
                    components.pop();
                } else {
                    components.push("..");
                }
            }
            std::path::Component::CurDir => {
                // Skip current directory components
            }
            _ => {
                components.push(component.as_os_str().to_string_lossy().as_ref());
            }
        }
    }
    
    components.iter().collect()
}

/// Create a relative path from one path to another (simplified implementation)
pub fn relative_path<P: AsRef<Path>, B: AsRef<Path>>(path: P, base: B) -> PathBuf {
    let path = path.as_ref();
    let base = base.as_ref();
    
    // Simplified implementation - in real use, consider using pathdiff crate
    if path.is_absolute() && base.is_absolute() {
        path.strip_prefix(base).unwrap_or(path).to_path_buf()
    } else {
        path.to_path_buf()
    }
}

/// File extension utilities
pub mod extensions {
    /// Check if a file has a specific extension (case-insensitive)
    pub fn has_extension<P: AsRef<std::path::Path>>(path: P, ext: &str) -> bool {
        path.as_ref()
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.eq_ignore_ascii_case(ext))
            .unwrap_or(false)
    }
    
    /// Get the file extension as a lowercase string
    pub fn get_extension<P: AsRef<std::path::Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
    }
    
    /// Check if a file is an Arma3 config file (hpp, sqf, sqm)
    pub fn is_arma3_config<P: AsRef<std::path::Path>>(path: P) -> bool {
        match get_extension(path).as_deref() {
            Some("hpp") | Some("sqf") | Some("sqm") | Some("ext") => true,
            _ => false,
        }
    }
    
    /// Check if a file is a PBO file
    pub fn is_pbo<P: AsRef<std::path::Path>>(path: P) -> bool {
        has_extension(path, "pbo")
    }
}

/// String utilities
pub mod strings {
    /// Convert a string to a valid Rust identifier
    pub fn to_identifier(s: &str) -> String {
        s.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect()
    }
    
    /// Sanitize a string for use in file names
    pub fn sanitize_filename(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                c if c.is_control() => '_',
                c => c,
            })
            .collect()
    }
    
    /// Compare strings case-insensitively
    pub fn case_insensitive_eq(a: &str, b: &str) -> bool {
        a.eq_ignore_ascii_case(b)
    }
}

/// Timing and performance utilities
pub mod timing {
    use std::time::{Duration, Instant};
    
    /// Simple timer for measuring operation duration
    #[derive(Debug)]
    pub struct Timer {
        start: Instant,
        name: String,
    }
    
    impl Timer {
        /// Start a new timer with a given name
        pub fn new<S: Into<String>>(name: S) -> Self {
            Self {
                start: Instant::now(),
                name: name.into(),
            }
        }
        
        /// Get the elapsed time since timer creation
        pub fn elapsed(&self) -> Duration {
            self.start.elapsed()
        }
        
        /// Stop the timer and log the elapsed time
        pub fn stop(self) -> Duration {
            let elapsed = self.elapsed();
            log::debug!("Timer '{}' completed in {:?}", self.name, elapsed);
            elapsed
        }
    }
}

/// Serialization helpers
pub mod serde_helpers {
    use serde::{Deserialize, Serialize};
    
    /// Helper for serializing/deserializing optional fields that might be empty strings
    pub mod optional_string {
        use serde::{Deserialize, Deserializer, Serializer};
        
        pub fn serialize<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match value {
                Some(s) if !s.is_empty() => serializer.serialize_some(s),
                _ => serializer.serialize_none(),
            }
        }
        
        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = Option::<String>::deserialize(deserializer)?;
            Ok(s.filter(|s| !s.is_empty()))
        }
    }
}

/// Simple macro for debug logging with timing
#[macro_export]
macro_rules! timed_debug {
    ($($arg:tt)*) => {
        let start = std::time::Instant::now();
        log::debug!($($arg)*);
        log::debug!("Operation took {:?}", start.elapsed());
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_normalize_path() {
        let path = PathBuf::from("./test/../file.txt");
        let normalized = normalize_path(path);
        assert_eq!(normalized, PathBuf::from("file.txt"));
    }
    
    #[test]
    fn test_extensions() {
        assert!(extensions::has_extension("test.hpp", "hpp"));
        assert!(extensions::has_extension("TEST.HPP", "hpp"));
        assert!(!extensions::has_extension("test.txt", "hpp"));
        
        assert!(extensions::is_arma3_config("config.hpp"));
        assert!(extensions::is_arma3_config("script.sqf"));
        assert!(extensions::is_arma3_config("mission.sqm"));
        assert!(!extensions::is_arma3_config("data.json"));
        
        assert!(extensions::is_pbo("addon.pbo"));
        assert!(!extensions::is_pbo("addon.txt"));
    }
    
    #[test]
    fn test_string_utils() {
        assert_eq!(strings::to_identifier("hello-world"), "hello_world");
        assert_eq!(strings::sanitize_filename("file:name?.txt"), "file_name_.txt");
        assert!(strings::case_insensitive_eq("Hello", "HELLO"));
    }
    
    #[test]
    fn test_timer() {
        let timer = timing::Timer::new("test");
        std::thread::sleep(std::time::Duration::from_millis(1));
        let elapsed = timer.stop();
        assert!(elapsed.as_millis() >= 1);
    }
}