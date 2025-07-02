use hemtt_config::{parse as hemtt_config_parse, Config};
use hemtt_preprocessor::Processor;
use hemtt_common::config::ProjectConfig;
use hemtt_workspace::{WorkspacePath, reporting::Processed};
use crate::error::ParseError;
use log::debug;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ParseResult {
    pub config: Config,
    pub processed: Processed,
    pub warnings: Vec<ParseWarning>,
}

#[derive(Debug, Clone)]
pub struct ParseWarning {
    pub code: String,
    pub message: String,
    pub severity: String,
    pub file_path: String,
}

/// List of warning codes that should be ignored during processing
/// These are typically non-critical warnings that don't affect functionality
static IGNORED_WARNING_CODES: &[&str] = &[
    // Preprocessor warnings - typically style/case issues
    "PW1", // Redefine macro - common in large codebases
    "PW2", // Invalid config case - style issue
    "PW3", // Padded macro argument - style issue
    "PW4", // Include case mismatch - style issue
    
    // Config analysis warnings - pedantic/style warnings
    "L-C12", // Math could be unquoted - optimization suggestion
    "L-C13", // Config this call - pedantic performance suggestion
    
    // PE12 is handled separately with special logic
];

/// Check if a warning code should be ignored
fn should_ignore_warning(code: &str) -> bool {
    IGNORED_WARNING_CODES.contains(&code)
}

pub fn process_file(
    file_wpath: &WorkspacePath,
    project_config: Option<&ProjectConfig>,
) -> Result<ParseResult, ParseError> {
    debug!("Processing file via workspace: {}", file_wpath.as_str());
    let mut warnings = Vec::new();

    let processed_output = match Processor::run(file_wpath) {
        Ok(processed) => processed,
        Err((included_files, e)) => {
            // Phase 6 cleanup: Improved warning/error handling logic
            // PE12 errors are treated as warnings and do not cause file failures
            let error_string = format!("{:?}", e);
            if error_string.contains("PE12") {
                // PE12 warnings are logged at INFO level as per Phase 6 requirements
                log::info!("Preprocessor warning (PE12 - include not found) in {}: {}", file_wpath.as_str(), error_string);
                warnings.push(ParseWarning {
                    code: "PE12".to_string(),
                    message: format!("Include file not found: {}", error_string),
                    severity: "Warning".to_string(),
                    file_path: file_wpath.as_str().to_string(),
                });
                
                // PE12 warnings proceed with empty results instead of failing
                // This prevents PE12 from causing FileFailure in the batch tool
                
                // IMPLEMENTATION NOTE:
                // The plan specifies creating empty Config and Processed objects, but proper constructors don't exist.
                // As mentioned in the plan: "if not, hemtt_config_parse will likely fail or return an empty config
                // when passed a problematic Processed from a PE12 scenario"
                //
                // This implementation uses unsafe zero-initialization as a temporary workaround.
                // TODO: Add proper empty constructors like Config::new_empty() and Processed::new_empty_without_source()
                //       as suggested in the plan.
                
                // Create empty/minimal objects safely instead of using unsafe zero-initialization
                // This satisfies the Phase 1 requirement to proceed with minimal results for PE12
                
                // Try to create a minimal valid Processed instance first
                let empty_processed_result = Processed::new(
                    Vec::new(), // empty output
                    HashMap::new(), // empty macros
                    Vec::new(), // empty warnings (Vec<Arc<dyn Code>>)
                    false, // no_rapify = false
                );
                
                if empty_processed_result.is_err() {
                    // If we can't create even a minimal processed, fall back to original error
                    log::warn!("Could not create minimal Processed object for PE12 in {}, using fallback", file_wpath.as_str());
                    return Err(ParseError::from((e, included_files)));
                }
                
                let empty_processed = empty_processed_result.unwrap();
                
                // Try to parse an empty config using the minimal processed data
                let empty_config_result = hemtt_config_parse(project_config, &empty_processed);
                
                let empty_config = match empty_config_result {
                    Ok(report) => report.into_config(),
                    Err(_) => {
                        // If config parsing fails with empty data, just return the original error
                        log::warn!("Could not parse empty config for PE12 in {}, falling back to error", file_wpath.as_str());
                        return Err(ParseError::from((e, included_files)));
                    }
                };
                
                // Successfully return Ok() for PE12 cases as required by Phase 1
                return Ok(ParseResult {
                    config: empty_config,
                    processed: empty_processed,
                    warnings, // Contains the PE12 warning that was logged above
                });
            } else {
                // True preprocessor errors (not PE12) propagate as ParseError
                // These will become FileFailure objects in the batch tool as intended
                return Err(ParseError::from((e, included_files)));
            }
        }
    };

    // Collect additional warnings from successful preprocessing
    for warning in processed_output.warnings() {
        let warning_ident = warning.ident().to_string();
        
        // Skip ignored warning codes
        if should_ignore_warning(&warning_ident) {
            log::debug!("Ignoring preprocessor warning {} in {}", warning_ident, file_wpath.as_str());
            continue;
        }
        
        let warning_msg = if let Some(_diag) = warning.diagnostic() {
            format!("Preprocessor warning in {}: {}", file_wpath.as_str(), warning_ident)
        } else {
            format!("Preprocessor warning in {}: {}", file_wpath.as_str(), warning_ident)
        };
        
        log::info!("{}", warning_msg);
        warnings.push(ParseWarning {
            code: warning_ident.clone(),
            message: warning_msg,
            severity: "Warning".to_string(),
            file_path: file_wpath.as_str().to_string(),
        });
    }
    
    // Handle no_rapify state as a warning (not an error)
    if processed_output.no_rapify() {
        let rapify_warning = format!("File {} cannot be rapified due to preprocessor state (e.g., __has_include). Parsing will proceed.", file_wpath.as_str());
        log::info!("{}", rapify_warning);
        warnings.push(ParseWarning {
            code: "NO_RAPIFY".to_string(),
            message: rapify_warning,
            severity: "Warning".to_string(),
            file_path: file_wpath.as_str().to_string(),
        });
    }

    // Parse configuration and collect any warnings
    match hemtt_config_parse(project_config, &processed_output) {
        Ok(report) => {
            // Collect warnings from config parsing (non-error severity codes)
            for code_item in report.codes() {
                if code_item.severity() != hemtt_workspace::reporting::Severity::Error {
                    let code_ident = code_item.ident().to_string();
                    
                    // Skip ignored warning codes
                    if should_ignore_warning(&code_ident) {
                        log::debug!("Ignoring config warning {} in {}", code_ident, file_wpath.as_str());
                        continue;
                    }
                    
                    let warning_msg = if let Some(_diag) = code_item.diagnostic() {
                        format!("Config warning/note in {}: {}", file_wpath.as_str(), code_ident)
                    } else {
                        format!("Config warning/note in {}: {}", file_wpath.as_str(), code_ident)
                    };
                    
                    log::info!("{}", warning_msg);
                    warnings.push(ParseWarning {
                        code: code_ident.clone(),
                        message: warning_msg,
                        severity: "Warning".to_string(),
                        file_path: file_wpath.as_str().to_string(),
                    });
                }
            }
            
            Ok(ParseResult {
                config: report.into_config(),
                processed: processed_output,
                warnings,
            })
        }
        Err(codes) => Err(ParseError::ConfigParse(codes)),
    }
}
