use std::path::PathBuf;
use std::io::{Error as IoError, ErrorKind};
use arma3_tool::error::{ToolError, from_anyhow};
use anyhow::anyhow;

#[test]
fn test_error_conversion() {
    // Test IO error conversion
    let io_error = IoError::new(ErrorKind::NotFound, "file not found");
    let tool_error = ToolError::from(io_error);
    
    match tool_error {
        ToolError::IoError(msg) => {
            assert!(msg.contains("file not found"));
        },
        _ => panic!("Expected IoError variant"),
    }
    
    // Test JSON error conversion
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let tool_error = ToolError::from(json_error);
    
    match tool_error {
        ToolError::JsonError(msg) => {
            assert!(msg.contains("expected value"));
        },
        _ => panic!("Expected JsonError variant"),
    }
}

#[test]
fn test_anyhow_conversion() {
    // Test anyhow error with ToolError inside
    let original = ToolError::ConfigError("test config error".to_string());
    let anyhow_err = anyhow!(original.clone());
    let converted = from_anyhow(anyhow_err);
    
    match converted {
        ToolError::ConfigError(msg) => {
            assert_eq!(msg, "test config error");
        },
        _ => panic!("Expected ConfigError variant"),
    }
    
    // Test anyhow error with IO error inside
    let io_error = IoError::new(ErrorKind::PermissionDenied, "permission denied");
    let anyhow_err = anyhow!(io_error);
    let converted = from_anyhow(anyhow_err);
    
    match converted {
        ToolError::IoError(msg) => {
            assert!(msg.contains("permission denied"));
        },
        _ => panic!("Expected IoError variant"),
    }
    
    // Test generic anyhow error
    let anyhow_err = anyhow!("generic error");
    let converted = from_anyhow(anyhow_err);
    
    match converted {
        ToolError::ConfigError(msg) => {
            assert!(msg.contains("generic error"));
        },
        _ => panic!("Expected ConfigError variant for generic error"),
    }
}

#[test]
fn test_error_clone() {
    let original = ToolError::CacheError("cache error".to_string());
    let cloned = original.clone();
    
    match cloned {
        ToolError::CacheError(msg) => {
            assert_eq!(msg, "cache error");
        },
        _ => panic!("Expected CacheError variant"),
    }
} 