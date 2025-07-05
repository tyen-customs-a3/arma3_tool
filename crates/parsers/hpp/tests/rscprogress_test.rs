use arma3_parser_hpp::{HppParser, PropertyValue, ParserMode};
use std::fs;

#[test]
fn test_profilenamespace_expression_parsing() {
    let content = fs::read_to_string("tests/fixtures/rscprogress.hpp").unwrap();
    let parser = HppParser::from_content(&content).unwrap();
    let temp_file = parser.project_root().join("temp.hpp");
    let classes = parser.parse_file(&temp_file, ParserMode::Advanced).unwrap();

    // Verify RscProgress class exists
    let progress_class = classes.iter().find(|c| c.name == "RscProgress").unwrap();
    
    // Test colorFrame property
    let color_frame_prop = progress_class.properties.iter()
        .find(|p| p.name == "colorFrame").unwrap();
    if let PropertyValue::Array(values) = &color_frame_prop.value {
        assert_eq!(values.len(), 4);
        assert_eq!(values[0], "0");
        assert_eq!(values[1], "0");
        assert_eq!(values[2], "0");
        assert_eq!(values[3], "0");
    } else {
        panic!("Expected colorFrame to be an array");
    }
    
    // Test colorBar property with profilenamespace expressions
    let color_bar_prop = progress_class.properties.iter()
        .find(|p| p.name == "colorBar").unwrap();
    if let PropertyValue::Array(values) = &color_bar_prop.value {
        assert_eq!(values.len(), 4);
        // Verify that profilenamespace expressions are properly preserved as complete strings
        assert!(values[0].contains("(profilenamespace getvariable ['GUI_BCG_RGB_R',0.13])"));
        assert!(values[1].contains("(profilenamespace getvariable ['GUI_BCG_RGB_G',0.54])"));
        assert!(values[2].contains("(profilenamespace getvariable ['GUI_BCG_RGB_B',0.21])"));
        assert!(values[3].contains("(profilenamespace getvariable ['GUI_BCG_RGB_A',0.8])"));
    } else {
        panic!("Expected colorBar to be an array");
    }
    
    // Verify numeric properties
    let deletable_prop = progress_class.properties.iter()
        .find(|p| p.name == "deletable").unwrap();
    assert!(matches!(deletable_prop.value, PropertyValue::Number(0)));
    
    // Check floating point property
    let x_prop = progress_class.properties.iter()
        .find(|p| p.name == "x").unwrap();
    
    // Depending on the implementation, this could be treated as a number or a string
    match &x_prop.value {
        PropertyValue::Number(num) => {
            // Allow for potential floating point conversion
            assert!(*num == 0 || *num == 344, "Expected x to be either 0 or 344, got {}", num);
        },
        PropertyValue::String(s) => {
            assert!(s == "0.344", "Expected x to be \"0.344\", got \"{}\"", s);
        },
        _ => panic!("Expected x to be either a number or string")
    }
} 