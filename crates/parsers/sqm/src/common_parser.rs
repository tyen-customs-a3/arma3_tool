//! Implementation of parser-common traits for the SQM parser

use arma3_parser_common::{Parser, ParseError as CommonParseError};
use crate::extract_class_dependencies;
use std::path::Path;
use std::collections::HashSet;

/// Wrapper that implements the common Parser trait for the SQM parser
pub struct SqmParser;

impl SqmParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SqmParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser<HashSet<String>> for SqmParser {
    fn parse_str(&self, content: &str) -> arma3_parser_common::Result<HashSet<String>> {
        let dependencies = extract_class_dependencies(content);
        Ok(dependencies)
    }

    fn parse_file<P: AsRef<Path>>(&self, path: P) -> arma3_parser_common::Result<HashSet<String>> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| CommonParseError::new(format!("Failed to read SQM file: {}", e)))?;
        
        let dependencies = extract_class_dependencies(&content);
        Ok(dependencies)
    }

    fn supported_extensions(&self) -> &[&str] {
        &["sqm"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_sqm_parser_trait() {
        let parser = SqmParser::new();
        
        let content = r#"
            class Mission {
                class Item1 {
                    class Attributes {
                        class Inventory {
                            class primaryWeapon {
                                name = "arifle_MX_F";
                            };
                            uniform = "U_B_CombatUniform_mcam";
                            vest = "V_PlateCarrier1_rgr";
                        };
                    };
                };
            };
        "#;

        let result = parser.parse_str(content);
        assert!(result.is_ok());
        
        let dependencies = result.unwrap();
        assert!(dependencies.contains("U_B_CombatUniform_mcam"));
        assert!(dependencies.contains("arifle_MX_F"));
        assert!(dependencies.contains("V_PlateCarrier1_rgr"));
    }

    #[test]
    fn test_sqm_parser_supported_extensions() {
        let parser = SqmParser::new();
        let extensions = parser.supported_extensions();
        assert!(extensions.contains(&"sqm"));
    }

    #[test]
    fn test_sqm_parser_can_parse() {
        let parser = SqmParser::new();
        assert!(parser.can_parse("mission.sqm"));
        assert!(!parser.can_parse("config.hpp"));
        assert!(!parser.can_parse("script.sqf"));
    }

    #[test]
    fn test_sqm_parser_file_parsing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.sqm");
        
        let content = r#"
            class Mission {
                class Item1 {
                    class Attributes {
                        class Inventory {
                            uniform = "U_B_CombatUniform_mcam";
                            magazines[] = {"30Rnd_65x39_caseless_mag", "30Rnd_65x39_caseless_mag"};
                        };
                    };
                };
            };
        "#;
        
        fs::write(&file_path, content).unwrap();
        
        let parser = SqmParser::new();
        let result = parser.parse_file(&file_path);
        
        assert!(result.is_ok());
        let dependencies = result.unwrap();
        assert!(dependencies.contains("U_B_CombatUniform_mcam"));
    }

    #[test]
    fn test_empty_sqm() {
        let parser = SqmParser::new();
        
        let content = "";
        let result = parser.parse_str(content);
        assert!(result.is_ok());
        
        let dependencies = result.unwrap();
        assert!(dependencies.is_empty());
    }

    #[test]
    fn test_invalid_sqm() {
        let parser = SqmParser::new();
        
        let content = "invalid sqm content { }";
        let result = parser.parse_str(content);
        assert!(result.is_ok()); // Should not fail, just return empty set
        
        let dependencies = result.unwrap();
        assert!(dependencies.is_empty());
    }
}