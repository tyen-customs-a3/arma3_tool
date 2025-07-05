//! Implementation of parser-common traits for the HPP parser

use arma3_parser_common::{Parser, ParseError as CommonParseError};
use arma3_types::Class;
use crate::{AdvancedProjectParser, ParseError, SimpleClassScanner};
use std::path::Path;

/// Wrapper that implements the common Parser trait for the Advanced HPP parser
pub struct HppAdvancedParser {
    inner: AdvancedProjectParser,
}

impl HppAdvancedParser {
    pub fn new(project_root: &Path, config_path: Option<&Path>) -> Result<Self, CommonParseError> {
        let inner = AdvancedProjectParser::new(project_root, config_path)
            .map_err(|e| CommonParseError::new(format!("Failed to initialize HPP parser: {}", e)))?;
        Ok(Self { inner })
    }
}

impl Parser<Vec<Class>> for HppAdvancedParser {
    fn parse_str(&self, _content: &str) -> arma3_parser_common::Result<Vec<Class>> {
        Err(arma3_parser_common::Error::parse("",
            "AdvancedProjectParser requires file-based parsing with project context"
        ))
    }

    fn parse_file<P: AsRef<Path>>(&self, path: P) -> arma3_parser_common::Result<Vec<Class>> {
        let path = path.as_ref();
        let relative_path = if path.is_absolute() {
            // Try to make it relative to the project root
            path.strip_prefix(&self.inner.project_root_dir)
                .unwrap_or(path)
        } else {
            path
        };

        let (classes, warnings) = self.inner.parse_file(relative_path)
            .map_err(|e| CommonParseError::new(format!("HPP parsing failed: {}", e)))?;

        // Log warnings
        for warning in warnings {
            log::warn!("HPP parser warning: {:?}", warning);
        }

        Ok(classes)
    }

    fn supported_extensions(&self) -> &[&str] {
        &["hpp", "ext"]
    }
}

/// Wrapper that implements the common Parser trait for the Simple HPP scanner
pub struct HppSimpleParser {
    inner: SimpleClassScanner,
}

impl HppSimpleParser {
    pub fn new() -> Self {
        Self {
            inner: SimpleClassScanner::new(),
        }
    }
}

impl Default for HppSimpleParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<Vec<Class>> for HppSimpleParser {
    fn parse_str(&self, content: &str) -> arma3_parser_common::Result<Vec<Class>> {
        // For string parsing, we can't get file path information
        // Create a temporary file approach or parse without file context
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut temp_file = NamedTempFile::new()
            .map_err(|e| CommonParseError::new(format!("Failed to create temp file: {}", e)))?;
        
        temp_file.write_all(content.as_bytes())
            .map_err(|e| CommonParseError::new(format!("Failed to write temp file: {}", e)))?;

        let classes = self.inner.scan_file(temp_file.path());
        Ok(classes)
    }

    fn parse_file<P: AsRef<Path>>(&self, path: P) -> arma3_parser_common::Result<Vec<Class>> {
        let classes = self.inner.scan_file(path.as_ref());
        Ok(classes)
    }

    fn supported_extensions(&self) -> &[&str] {
        &["hpp", "ext"]
    }
}

/// Convert ParseError to CommonParseError
impl From<ParseError> for CommonParseError {
    fn from(err: ParseError) -> Self {
        CommonParseError::new(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_simple_parser_trait() {
        let parser = HppSimpleParser::new();
        
        let content = r#"
            class TestClass {
                displayName = "Test";
                armor = 100;
            };
            
            class ChildClass : TestClass {
                displayName = "Child";
            };
        "#;

        let result = parser.parse_str(content);
        assert!(result.is_ok());
        
        let classes = result.unwrap();
        assert_eq!(classes.len(), 2);
        assert_eq!(classes[0].name, "TestClass");
        assert_eq!(classes[1].name, "ChildClass");
        assert_eq!(classes[1].parent, Some("TestClass".to_string()));
    }

    #[test]
    fn test_simple_parser_supported_extensions() {
        let parser = HppSimpleParser::new();
        let extensions = parser.supported_extensions();
        assert!(extensions.contains(&"hpp"));
        assert!(extensions.contains(&"ext"));
    }

    #[test]
    fn test_simple_parser_can_parse() {
        let parser = HppSimpleParser::new();
        assert!(parser.can_parse("test.hpp"));
        assert!(parser.can_parse("config.ext"));
        assert!(!parser.can_parse("script.sqf"));
        assert!(!parser.can_parse("mission.sqm"));
    }

    #[test]
    fn test_simple_parser_file_parsing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.hpp");
        
        let content = r#"
            class Vehicle {
                displayName = "Test Vehicle";
                maxSpeed = 120;
            };
        "#;
        
        fs::write(&file_path, content).unwrap();
        
        let parser = HppSimpleParser::new();
        let result = parser.parse_file(&file_path);
        
        assert!(result.is_ok());
        let classes = result.unwrap();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "Vehicle");
    }
}