//! Implementation of parser-common traits for the SQF parser

use arma3_parser_common::{Parser, ParseError as CommonParseError};
use crate::{Error, ClassReference, parse_file as parse_sqf_file};
use std::path::Path;

/// Wrapper that implements the common Parser trait for the SQF parser
pub struct SqfParser;

impl SqfParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SqfParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<Vec<ClassReference>> for SqfParser {
    fn parse_str(&self, content: &str) -> arma3_parser_common::Result<Vec<ClassReference>> {
        // For string parsing, create a temporary file
        use std::io::Write;
        use tempfile::NamedTempFile;

        let mut temp_file = NamedTempFile::new()
            .map_err(|e| CommonParseError::new(format!("Failed to create temp file: {}", e)))?;
        
        temp_file.write_all(content.as_bytes())
            .map_err(|e| CommonParseError::new(format!("Failed to write temp file: {}", e)))?;

        let references = parse_sqf_file(temp_file.path())
            .map_err(|e| CommonParseError::new(format!("SQF parsing failed: {}", e)))?;

        Ok(references)
    }

    fn parse_file<P: AsRef<Path>>(&self, path: P) -> arma3_parser_common::Result<Vec<ClassReference>> {
        let references = parse_sqf_file(path.as_ref())
            .map_err(|e| CommonParseError::new(format!("SQF parsing failed: {}", e)))?;

        Ok(references)
    }

    fn supported_extensions(&self) -> &[&str] {
        &["sqf"]
    }
}

/// Convert SQF Error to CommonParseError
impl From<Error> for CommonParseError {
    fn from(err: Error) -> Self {
        CommonParseError::new(err.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "I/O error: {}", e),
            Error::ParserError(e) => write!(f, "Parser error: {}", e),
            Error::WorkspaceError(e) => write!(f, "Workspace error: {}", e),
            Error::UnparseableSyntax(e) => write!(f, "Unparseable syntax: {}", e),
            Error::SqfError(e) => write!(f, "SQF error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(e) => Some(e),
            Error::ParserError(_) => None, // ParserError doesn't implement std::error::Error
            Error::WorkspaceError(e) => Some(e),
            Error::UnparseableSyntax(_) => None,
            Error::SqfError(e) => Some(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_sqf_parser_trait() {
        let parser = SqfParser::new();
        
        let content = r#"
            player addItem "FirstAidKit";
            player addWeapon "arifle_MX_F";
            player addVest "V_PlateCarrier1_rgr";
        "#;

        let result = parser.parse_str(content);
        assert!(result.is_ok());
        
        let references = result.unwrap();
        // The specific results depend on the evaluator logic
        // This test just ensures the trait implementation works
    }

    #[test]
    fn test_sqf_parser_supported_extensions() {
        let parser = SqfParser::new();
        let extensions = parser.supported_extensions();
        assert!(extensions.contains(&"sqf"));
    }

    #[test]
    fn test_sqf_parser_can_parse() {
        let parser = SqfParser::new();
        assert!(parser.can_parse("script.sqf"));
        assert!(!parser.can_parse("config.hpp"));
        assert!(!parser.can_parse("mission.sqm"));
    }

    #[test]
    fn test_sqf_parser_file_parsing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.sqf");
        
        let content = r#"
            _unit = player;
            _unit addItem "FirstAidKit";
            _unit addWeapon "arifle_MX_F";
        "#;
        
        fs::write(&file_path, content).unwrap();
        
        let parser = SqfParser::new();
        let result = parser.parse_file(&file_path);
        
        assert!(result.is_ok());
        let references = result.unwrap();
        // The specific results depend on the evaluator implementation
    }

    #[test]
    fn test_error_conversion() {
        let io_error = Error::IoError(std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"));
        let common_error: CommonParseError = io_error.into();
        assert!(common_error.to_string().contains("I/O error"));
    }
}