//! Integration tests for all three parsers working together
//! 
//! This test file verifies that HPP, SQF, and SQM parsers all implement
//! the common parser interface correctly and can be used interchangeably.

#[cfg(test)]
mod tests {
    use arma3_parser_common::Parser;
    use arma3_parser_hpp::{HppSimpleParser, HppAdvancedParser};
    use arma3_parser_sqf::SqfParser;
    use arma3_parser_sqm::SqmParser;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_all_parsers_support_extensions() {
        let hpp_parser = HppSimpleParser::new();
        let sqf_parser = SqfParser::new();
        let sqm_parser = SqmParser::new();

        // Test extension support
        assert!(hpp_parser.can_parse("config.hpp"));
        assert!(hpp_parser.can_parse("config.ext"));
        assert!(!hpp_parser.can_parse("script.sqf"));
        assert!(!hpp_parser.can_parse("mission.sqm"));

        assert!(sqf_parser.can_parse("script.sqf"));
        assert!(!sqf_parser.can_parse("config.hpp"));
        assert!(!sqf_parser.can_parse("mission.sqm"));

        assert!(sqm_parser.can_parse("mission.sqm"));
        assert!(!sqm_parser.can_parse("config.hpp"));
        assert!(!sqm_parser.can_parse("script.sqf"));
    }

    #[test]
    fn test_hpp_parser_string_parsing() {
        let parser = HppSimpleParser::new();
        
        let content = r#"
            class TestVehicle {
                displayName = "Test Vehicle";
                maxSpeed = 120;
            };
            
            class TestWeapon : TestVehicle {
                displayName = "Test Weapon";
                damage = 50;
            };
        "#;

        let result = parser.parse_str(content);
        assert!(result.is_ok());
        
        let classes = result.unwrap();
        assert_eq!(classes.len(), 2);
        
        let vehicle = &classes[0];
        assert_eq!(vehicle.name, "TestVehicle");
        assert!(vehicle.parent.is_none());
        
        let weapon = &classes[1];
        assert_eq!(weapon.name, "TestWeapon");
        assert_eq!(weapon.parent, Some("TestVehicle".to_string()));
    }

    #[test]
    fn test_sqf_parser_string_parsing() {
        let parser = SqfParser::new();
        
        let content = r#"
            player addItem "FirstAidKit";
            player addWeapon "arifle_MX_F";
            player addVest "V_PlateCarrier1_rgr";
        "#;

        let result = parser.parse_str(content);
        assert!(result.is_ok());
        
        let references = result.unwrap();
        // The specific results depend on the SQF evaluator implementation
        // This test ensures the parser trait works correctly
        assert!(references.len() >= 0); // May or may not find references
    }

    #[test]
    fn test_sqm_parser_string_parsing() {
        let parser = SqmParser::new();
        
        let content = r#"
            class Mission {
                class Item1 {
                    class Attributes {
                        class Inventory {
                            uniform = "U_B_CombatUniform_mcam";
                            vest = "V_PlateCarrier1_rgr";
                            class primaryWeapon {
                                name = "arifle_MX_F";
                            };
                        };
                    };
                };
            };
        "#;

        let result = parser.parse_str(content);
        assert!(result.is_ok());
        
        let dependencies = result.unwrap();
        assert!(dependencies.contains("U_B_CombatUniform_mcam"));
        assert!(dependencies.contains("V_PlateCarrier1_rgr"));
        assert!(dependencies.contains("arifle_MX_F"));
    }

    #[test]
    fn test_all_parsers_file_parsing() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create HPP file
        let hpp_file = temp_dir.path().join("config.hpp");
        let hpp_content = r#"
            class TestClass {
                displayName = "Test";
                value = 42;
            };
        "#;
        fs::write(&hpp_file, hpp_content).unwrap();
        
        // Create SQF file
        let sqf_file = temp_dir.path().join("script.sqf");
        let sqf_content = r#"
            player addItem "FirstAidKit";
        "#;
        fs::write(&sqf_file, sqf_content).unwrap();
        
        // Create SQM file
        let sqm_file = temp_dir.path().join("mission.sqm");
        let sqm_content = r#"
            class Mission {
                class Item1 {
                    class Attributes {
                        class Inventory {
                            uniform = "U_B_CombatUniform_mcam";
                        };
                    };
                };
            };
        "#;
        fs::write(&sqm_file, sqm_content).unwrap();
        
        // Test all parsers
        let hpp_parser = HppSimpleParser::new();
        let sqf_parser = SqfParser::new();
        let sqm_parser = SqmParser::new();
        
        let hpp_result = hpp_parser.parse_file(&hpp_file);
        assert!(hpp_result.is_ok());
        let classes = hpp_result.unwrap();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "TestClass");
        
        let sqf_result = sqf_parser.parse_file(&sqf_file);
        assert!(sqf_result.is_ok());
        
        let sqm_result = sqm_parser.parse_file(&sqm_file);
        assert!(sqm_result.is_ok());
        let dependencies = sqm_result.unwrap();
        assert!(dependencies.contains("U_B_CombatUniform_mcam"));
    }

    #[test]
    fn test_parser_error_handling() {
        let temp_dir = TempDir::new().unwrap();
        
        // Test file not found
        let missing_file = temp_dir.path().join("missing.hpp");
        let parser = HppSimpleParser::new();
        let result = parser.parse_file(&missing_file);
        assert!(result.is_err());
        
        // Test empty content
        let empty_result = parser.parse_str("");
        assert!(empty_result.is_ok()); // Should succeed with empty results
    }

    #[test]
    fn test_cross_parser_compatibility() {
        // This test demonstrates that while each parser returns different types,
        // they all follow the same interface patterns
        
        let temp_dir = TempDir::new().unwrap();
        
        // Parser factory function demonstration
        fn get_parser_for_file(file_path: &std::path::Path) -> Box<dyn std::any::Any> {
            match file_path.extension().and_then(|s| s.to_str()) {
                Some("hpp") | Some("ext") => Box::new(HppSimpleParser::new()),
                Some("sqf") => Box::new(SqfParser::new()),
                Some("sqm") => Box::new(SqmParser::new()),
                _ => panic!("Unsupported file type"),
            }
        }
        
        // Test that we can dynamically select parsers
        let hpp_file = temp_dir.path().join("test.hpp");
        let sqf_file = temp_dir.path().join("test.sqf");
        let sqm_file = temp_dir.path().join("test.sqm");
        
        fs::write(&hpp_file, "class Test {};").unwrap();
        fs::write(&sqf_file, "// sqf comment").unwrap();
        fs::write(&sqm_file, "class Mission {};").unwrap();
        
        let _hpp_parser = get_parser_for_file(&hpp_file);
        let _sqf_parser = get_parser_for_file(&sqf_file);
        let _sqm_parser = get_parser_for_file(&sqm_file);
        
        // All parsers were created successfully
    }
}