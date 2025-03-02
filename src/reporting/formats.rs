/// Supported report formats
#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    /// JSON format (default)
    Json,
    /// YAML format
    Yaml,
    /// TOML format
    Toml,
}

impl Default for ReportFormat {
    fn default() -> Self {
        Self::Json
    }
}

impl ReportFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Yaml => "yaml",
            Self::Toml => "toml",
        }
    }
    
    /// Get the content type for this format
    pub fn content_type(&self) -> &'static str {
        match self {
            Self::Json => "application/json",
            Self::Yaml => "application/yaml",
            Self::Toml => "application/toml",
        }
    }
    
    /// Parse a format from a string
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(Self::Json),
            "yaml" | "yml" => Some(Self::Yaml),
            "toml" => Some(Self::Toml),
            _ => None,
        }
    }
} 