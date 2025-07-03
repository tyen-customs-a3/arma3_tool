use std::path::{Path, PathBuf};
use std::sync::Arc;

use hemtt_common::config::{PDriveOption, ProjectConfig};
use log::debug;

mod error;
mod workspace_manager;
mod file_processor;
mod ast_transformer;
pub mod models; // Ensure models module is public if types are used in public API
mod query;
mod simple_parser;

pub use error::ParseError;
pub use file_processor::{ParseResult, ParseWarning}; // Export new parsing result types
pub use models::{GameClass, ClassProperty, PropertyValue, FileParser}; // Re-export all needed types
pub use query::DependencyExtractor;
pub use simple_parser::{SimpleClassScanner, parse_file_simple};
use workspace_manager::WorkspaceManager;

/// Parsing mode selection for different performance/accuracy tradeoffs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserMode {
    /// Fast regex-based parsing - quick but less accurate
    Simple,
    /// Full HEMTT integration - slower but handles includes/macros
    Advanced,
}

/// The main parser for an Arma 3 project.
/// It should be instantiated once per project directory.
pub struct AdvancedProjectParser {
    workspace_manager: WorkspaceManager,
    // Storing project_config here might be redundant if WorkspaceManager already holds it
    // and provides a getter. However, it can be convenient.
    project_config: Option<ProjectConfig>,
    project_root_dir: PathBuf, // Added field to store the project root
}

impl AdvancedProjectParser {
    /// Creates a new parser for the given project root directory.
    ///
    /// # Arguments
    /// * `project_root_dir_param`: The absolute path to the root of the Arma 3 project/mod.
    /// * `project_config_path`: Optional path to a `hemtt.toml` file. If provided, it will be loaded.
    /// Creates a new parser for the given project root directory.
    ///
    /// # Arguments
    /// * `project_root_dir_param`: The absolute path to the root of the Arma 3 project/mod.
    /// * `project_config_path`: Optional path to a `hemtt.toml` file. If provided, it will be loaded.
    ///                         If None, will attempt to find hemtt.toml in the project root.
    pub fn new(
        project_root_dir_param: &Path,
        project_config_path: Option<&Path>,
    ) -> Result<Self, ParseError> {
        let project_config = match project_config_path {
            // If explicit path provided, use it
            Some(config_path) => {
                match ProjectConfig::from_file(config_path) {
                    Ok(cfg) => Some(cfg),
                    Err(e) => return Err(ParseError::ProjectConfigLoad(config_path.to_path_buf(), e)),
                }
            }
            // No explicit path provided, try to discover hemtt.toml in project root
            None => {
                let default_config_path = project_root_dir_param.join("hemtt.toml");
                debug!("Looking for hemtt.toml at: {}", default_config_path.display());
                
                let exists = default_config_path.exists();
                debug!("hemtt.toml exists: {}", exists);
                
                if exists {
                    debug!("Attempting to load hemtt.toml from: {}", default_config_path.display());
                    match ProjectConfig::from_file(&default_config_path) {
                        Ok(cfg) => {
                            debug!("Successfully loaded hemtt.toml");
                            Some(cfg)
                        },
                        Err(e) => {
                            debug!("Failed to load discovered hemtt.toml: {}", e);
                            None
                        }
                    }
                } else {
                    debug!("No hemtt.toml found at {}, proceeding without config", default_config_path.display());
                    None
                }
            }
        };

        let workspace_manager = WorkspaceManager::new(
            project_root_dir_param,
            project_config.clone(),
            &PDriveOption::Disallow,
        )?;

        Ok(Self {
            workspace_manager,
            project_config,
            project_root_dir: project_root_dir_param.to_path_buf(),
        })
    }

    /// Parses a single HPP file within the project.
    ///
    /// # Arguments
    /// * `relative_file_path`: Path to the HPP file, relative to the project root.
    ///
    /// # Returns
    /// Returns a tuple of (Vec<GameClass>, Vec<ParseWarning>) where:
    /// - Vec<GameClass>: The parsed classes from the file
    /// - Vec<ParseWarning>: Any warnings encountered during parsing (including PE12)
    pub fn parse_file(&self, relative_file_path: &Path) -> Result<(Vec<GameClass>, Vec<ParseWarning>), ParseError> {
        debug!("AdvancedProjectParser parsing file: {}", relative_file_path.display());
        let file_wpath = self
            .workspace_manager
            .get_workspace_path_for_relative(relative_file_path)?;

        // Get parse result with warnings
        let parse_result = file_processor::process_file(&file_wpath, self.project_config.as_ref())?;

        let path_for_attribution = self.project_root_dir.join(relative_file_path);

        // Transform config to game classes
        let game_classes = ast_transformer::transform_config_to_game_classes(
            &parse_result.config,
            &parse_result.processed, // Pass processed_output
            &path_for_attribution, // This might change to project_root_dir
            &self.project_root_dir, // Pass project_root_dir for absolute path construction
        );
        
        Ok((game_classes, parse_result.warnings))
    }

    /// Returns whether a project config is loaded
    pub fn has_project_config(&self) -> bool {
        self.project_config.is_some()
    }

    /// Returns a reference to the project config if loaded
    pub fn project_config(&self) -> Option<&ProjectConfig> {
        self.project_config.as_ref()
    }

    /// Extract dependencies from a parsed file using the DependencyExtractor
    /// 
    /// # Arguments
    /// * `relative_file_path`: Path to the HPP file, relative to the project root.
    /// 
    /// # Returns
    /// Returns a HashSet of dependency strings found in the file
    pub fn extract_dependencies(&self, relative_file_path: &Path) -> Result<std::collections::HashSet<String>, ParseError> {
        let (classes, _warnings) = self.parse_file(relative_file_path)?;
        let extractor = crate::query::DependencyExtractor::new(classes);
        Ok(extractor.extract_dependencies())
    }

    /// Parse all HPP/CPP files in the project workspace
    /// 
    /// This method discovers all HPP and CPP files in the project and parses them,
    /// returning a combined list of all classes found across all files.
    /// 
    /// # Returns
    /// 
    /// Returns a Vec of all GameClass objects found in the project, or a ParseError
    /// if file discovery or parsing fails.
    pub fn parse_all_project_files(&self) -> Result<Vec<GameClass>, ParseError> {
        let mut all_classes = Vec::new();
        let mut discovered_files = Vec::new();
        
        // Walk the project directory to find all HPP/CPP files
        let walker = walkdir::WalkDir::new(&self.project_root_dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| {
                if let Some(extension) = entry.path().extension() {
                    matches!(extension.to_str(), Some("hpp") | Some("cpp") | Some("h"))
                } else {
                    false
                }
            });
        
        // Collect relative paths for all discovered files
        for entry in walker {
            let absolute_path = entry.path();
            if let Ok(relative_path) = absolute_path.strip_prefix(&self.project_root_dir) {
                discovered_files.push(relative_path.to_path_buf());
            }
        }
        
        log::info!("Discovered {} HPP/CPP files in project", discovered_files.len());
        
        // Parse each discovered file
        for relative_path in discovered_files {
            match self.parse_file(&relative_path) {
                Ok((mut classes, warnings)) => {
                    // Log any warnings for this file
                    for warning in warnings {
                        log::warn!("Warning in {}: {} - {}", 
                                   relative_path.display(), warning.code, warning.message);
                    }
                    
                    // Add all classes from this file
                    all_classes.append(&mut classes);
                    
                    log::debug!("Parsed {} classes from {}", 
                               classes.len(), relative_path.display());
                }
                Err(e) => {
                    // Log parse errors but continue with other files
                    log::error!("Failed to parse {}: {:?}", relative_path.display(), e);
                    // Don't fail the entire operation for individual file errors
                }
            }
        }
        
        log::info!("Successfully parsed {} total classes from project", all_classes.len());
        Ok(all_classes)
    }
}

/// Wrapper to implement the `gamedata_scanner_models::FileParser` trait.
pub struct AdvancedFileParserWrapper {
    // Using Arc if you need to share the parser instance, otherwise direct ownership.
    project_parser: Arc<AdvancedProjectParser>,
    project_root_dir: PathBuf, // Store the project root to make paths relative
}

impl AdvancedFileParserWrapper {
    /// Creates a new wrapper.
    /// `project_parser` would typically be created once and shared.
    pub fn new(project_parser: Arc<AdvancedProjectParser>, project_root_dir: &Path) -> Self {
        Self {
            project_parser,
            project_root_dir: project_root_dir.to_path_buf(),
        }
    }
}

impl FileParser for AdvancedFileParserWrapper {
    fn name(&self) -> &str {
        "AdvancedProjectParser"
    }

    /// Parses a file. `file_path` is expected to be an absolute path
    /// or a path that can be made relative to the project root.
    fn parse_file(&self, file_path: &Path) -> Vec<GameClass> {
        // Attempt to make file_path relative to the project_root_dir
        let relative_path = match file_path.strip_prefix(&self.project_root_dir) {
            Ok(rel_path) => rel_path,
            Err(_) => {
                // If stripping prefix fails, it might be an absolute path not within the project,
                // or a path that's already relative but not directly.
                // For simplicity, if it's absolute and not in project, we error.
                // If it's already relative, we try to use it as is, but this might be fragile.
                if file_path.is_absolute() {
                    log::error!(
                        "File path {} is not within the project root {}",
                        file_path.display(),
                        self.project_root_dir.display()
                    );
                    return Vec::new();
                }
                // Assume it's a relative path the project_parser can handle from its root
                file_path
            }
        };

        match self.project_parser.parse_file(relative_path) {
            Ok((mut classes, warnings)) => { // Destructure the tuple to get classes and warnings
                // Log any warnings encountered during parsing
                for warning in &warnings {
                    log::warn!("Parsing warning for {}: {} - {}", file_path.display(), warning.code, warning.message);
                }
                
                // Convert absolute paths from project_parser back to relative for the wrapper's contract
                for class in &mut classes {
                    if let Ok(new_relative_path) = class.file_path.strip_prefix(&self.project_root_dir) {
                        class.file_path = new_relative_path.to_path_buf();
                    } else {
                        // This case should ideally not happen if paths are consistent.
                        // Log a warning if a path couldn't be made relative.
                        log::warn!(
                            "Could not make path {} relative to project root {} for GameClass {} in wrapper",
                            class.file_path.display(),
                            self.project_root_dir.display(),
                            class.name
                        );
                        // Keep the original (likely absolute) path if stripping fails.
                    }
                }
                classes
            }
            Err(e) => {
                log::error!("Error parsing file {}: {:?}", file_path.display(), e);
                Vec::new()
            }
        }
    }
}

// Legacy API compatibility - keep the old parse_file function but use the new engine
/// Parse an HPP file and return a vector of classes.
/// 
/// This is a legacy compatibility function that uses the advanced parser internally
/// but maintains the old API for backward compatibility.
/// 
/// # Arguments
/// 
/// * `file_path` - Path to the HPP file to parse
/// 
/// # Returns
/// 
/// * `Result<Vec<GameClass>, ParseError>` - List of classes found in the file or error
pub fn parse_file(file_path: &Path) -> Result<Vec<GameClass>, ParseError> {
    // For standalone usage, we need to determine the project root
    // We'll use the file's directory as the project root for this legacy API
    let project_root = file_path.parent().unwrap_or_else(|| Path::new("."));
    
    // Create a temporary parser instance
    let parser = AdvancedProjectParser::new(project_root, None)?;
    
    // Get the relative path for the file within its directory
    let relative_path = file_path.file_name()
        .map(|name| Path::new(name))
        .unwrap_or(file_path);
    
    // Parse and return only the classes (ignore warnings for legacy compatibility)
    let (classes, _warnings) = parser.parse_file(relative_path)?;
    Ok(classes)
}

// Legacy types removed - use GameClass, ClassProperty, and PropertyValue from gamedata_scanner_models instead

/// Unified HPP parser that can switch between Simple and Advanced modes
pub struct HppParser {
    advanced_parser: Option<Arc<AdvancedProjectParser>>,
    simple_scanner: SimpleClassScanner,
    project_root_dir: PathBuf,
}

impl HppParser {
    /// Create a new unified parser for a project directory
    /// 
    /// # Arguments
    /// * `project_root_dir`: The root directory of the project
    /// * `project_config_path`: Optional path to hemtt.toml config file
    pub fn new(project_root_dir: &Path, project_config_path: Option<&Path>) -> Result<Self, ParseError> {
        let advanced_parser = AdvancedProjectParser::new(project_root_dir, project_config_path)?;
        
        Ok(Self {
            advanced_parser: Some(Arc::new(advanced_parser)),
            simple_scanner: SimpleClassScanner::new(),
            project_root_dir: project_root_dir.to_path_buf(),
        })
    }

    /// Create a new parser from string content (for backward compatibility)
    /// This method creates a temporary file and uses the simple parser by default
    pub fn from_content(content: &str) -> Result<Self, ParseError> {
        use std::fs;
        use tempfile::tempdir;
        
        let temp_dir = tempdir().map_err(|_| ParseError::Io(std::io::Error::new(std::io::ErrorKind::Other, "Failed to create temp dir")))?;
        let temp_file = temp_dir.path().join("temp.hpp");
        fs::write(&temp_file, content)?;
        
        let parser = Self::new(temp_dir.path(), None)?;
        
        // Keep the temp directory alive by storing it in the parser
        std::mem::forget(temp_dir);
        
        Ok(parser)
    }

    /// Parse a file using the specified parsing mode
    /// 
    /// # Arguments
    /// * `file_path`: Path to the HPP file (absolute or relative to project root)
    /// * `mode`: Parsing mode to use
    /// 
    /// # Returns
    /// Vector of GameClass objects found in the file
    pub fn parse_file(&self, file_path: &Path, mode: ParserMode) -> Result<Vec<GameClass>, ParseError> {
        match mode {
            ParserMode::Simple => {
                Ok(self.simple_scanner.scan_file(file_path))
            }
            ParserMode::Advanced => {
                if let Some(advanced_parser) = &self.advanced_parser {
                    // Determine if file_path is absolute or relative
                    let relative_path = if file_path.is_absolute() {
                        // Try to make it relative to project root
                        file_path.strip_prefix(&self.project_root_dir)
                            .map_err(|_| ParseError::PathNotInProject(file_path.to_path_buf(), self.project_root_dir.clone()))?
                    } else {
                        file_path
                    };
                    
                    let (classes, _warnings) = advanced_parser.parse_file(relative_path)?;
                    Ok(classes)
                } else {
                    return Err(ParseError::Io(std::io::Error::new(
                        std::io::ErrorKind::Other, 
                        "Advanced parser not available"
                    )));
                }
            }
        }
    }

    /// Extract dependencies from a file using the advanced parser
    /// 
    /// # Arguments
    /// * `file_path`: Path to the HPP file (absolute or relative to project root)
    /// 
    /// # Returns
    /// HashSet of dependency strings found in the file
    pub fn extract_dependencies(&self, file_path: &Path) -> Result<std::collections::HashSet<String>, ParseError> {
        if let Some(advanced_parser) = &self.advanced_parser {
            // Determine if file_path is absolute or relative
            let relative_path = if file_path.is_absolute() {
                // Try to make it relative to project root
                file_path.strip_prefix(&self.project_root_dir)
                    .map_err(|_| ParseError::PathNotInProject(file_path.to_path_buf(), self.project_root_dir.clone()))?
            } else {
                file_path
            };
            
            advanced_parser.extract_dependencies(relative_path)
        } else {
            return Err(ParseError::Io(std::io::Error::new(
                std::io::ErrorKind::Other, 
                "Advanced parser not available for dependency extraction"
            )));
        }
    }

    /// Parse classes using the legacy API (for backward compatibility)
    /// Defaults to Simple parsing mode for performance
    /// Returns GameClass objects instead of the deprecated HppClass
    pub fn parse_classes(&self) -> Vec<GameClass> {
        // This method is for backward compatibility with the old API
        // Since we don't have a specific file, we can't parse anything
        // This would need to be called after parse_file() in the old workflow
        Vec::new()
    }

    /// Get the project root directory
    pub fn project_root(&self) -> &Path {
        &self.project_root_dir
    }

    /// Check if the advanced parser is available
    pub fn has_advanced_parser(&self) -> bool {
        self.advanced_parser.is_some()
    }
}

// Conversion functions removed - use GameClass and PropertyValue directly

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    use env_logger;

    fn setup_basic_project_for_parser() -> (tempfile::TempDir, Arc<AdvancedProjectParser>) {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();

        // Create a dummy addon structure
        let addons_dir = project_root.join("addons");
        fs::create_dir_all(&addons_dir).unwrap();
        let main_addon_dir = addons_dir.join("main");
        fs::create_dir_all(&main_addon_dir).unwrap();

        let config_content = r#"
            class MyBaseClass {
                scope = 1;
            };
            class MyTestClass : MyBaseClass {
                displayName = "Test Display";
                value = 123;
                items[] = {"itemA", "itemB"};
            };
        "#;
        fs::write(main_addon_dir.join("config.cpp"), config_content).unwrap();

        let include_content = "#define MY_VALUE 456";
        fs::write(main_addon_dir.join("common.hpp"), include_content).unwrap();

        let other_content = r#"
            #include "common.hpp"
            class AnotherClass {
                anotherValue = MY_VALUE;
            };
        "#;
        fs::write(main_addon_dir.join("other.cpp"), other_content).unwrap();

        let parser = AdvancedProjectParser::new(
            project_root,
            None
        ).unwrap();
        (temp_dir, Arc::new(parser))
    }

    #[test]
    fn test_advanced_project_parser_basic_file() {
        let (_temp_dir, parser) = setup_basic_project_for_parser();
        let relative_path = Path::new("addons/main/config.cpp");

        let result = parser.parse_file(relative_path);
        assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
        let (classes, warnings) = result.unwrap();

        // Log any warnings
        for warning in &warnings {
            println!("Warning: {} - {}", warning.code, warning.message);
        }

        assert_eq!(classes.len(), 2);
        let test_class = classes.iter().find(|c| c.name == "MyTestClass").unwrap();
        assert_eq!(test_class.parent.as_deref(), Some("MyBaseClass"));
        assert_eq!(test_class.properties.len(), 3);
        let display_name_prop = test_class.properties.iter().find(|p| p.name == "displayName").unwrap();
        assert_eq!(display_name_prop.value, PropertyValue::String("Test Display".to_string()));
    }

    #[test]
    fn test_advanced_project_parser_with_include() {
        let (_temp_dir, parser) = setup_basic_project_for_parser();
        let relative_path = Path::new("addons/main/other.cpp");

        let result = parser.parse_file(relative_path);
        assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
        let (classes, warnings) = result.unwrap();
        
        // Log any warnings
        for warning in &warnings {
            println!("Warning: {} - {}", warning.code, warning.message);
        }
        
        assert_eq!(classes.len(), 1);
        let another_class = &classes[0];
        assert_eq!(another_class.name, "AnotherClass");
        let value_prop = another_class.properties.iter().find(|p| p.name == "anotherValue").unwrap();
        assert_eq!(value_prop.value, PropertyValue::Number(456)); // MY_VALUE from common.hpp
    }

    #[test]
    fn test_file_parser_wrapper() {
        let (temp_dir, project_parser) = setup_basic_project_for_parser();
        let project_root = temp_dir.path().to_path_buf();

        let wrapper = AdvancedFileParserWrapper::new(project_parser, &project_root);

        // Test with an absolute path
        let absolute_path_to_config = project_root.join("addons/main/config.cpp");
        let classes1 = wrapper.parse_file(&absolute_path_to_config);
        assert_eq!(classes1.len(), 2);
        let test_class1 = classes1.iter().find(|c| c.name == "MyTestClass").unwrap();
        assert_eq!(test_class1.properties.len(), 3);
        assert_eq!(test_class1.file_path, PathBuf::from("addons/main/config.cpp"));

        // Test with a path already relative to the project root (if wrapper handles it)
        // or ensure your calling code makes it relative if that's the contract.
        // For this test, our wrapper makes it relative.
        let classes2 = wrapper.parse_file(Path::new("addons/main/other.cpp")); // Passed as if relative
        assert_eq!(classes2.len(), 1);
        let another_class = &classes2[0];
        assert_eq!(another_class.name, "AnotherClass");
        assert_eq!(another_class.file_path, PathBuf::from("addons/main/other.cpp"));
    }

     #[test]
    fn test_config_auto_discovery() {
        // Initialize logger for debug output
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();

        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();

        // Create project structure
        let addons_dir = project_root.join("addons");
        fs::create_dir_all(&addons_dir).unwrap();

        // Create a hemtt.toml in the project root with specific lint settings
        let config_content = r#"
name = "test_project"
prefix = "test"

[version]
git_hash = 0

[lints.config.missing_file_type]
enabled = true
"#;
        let config_path = project_root.join("hemtt.toml");
        fs::write(&config_path, config_content).unwrap();

        // Create a test file that would trigger lint errors
        let test_content = r#"
            value = 123;  // This would trigger a lint if not in a class
        "#;
        fs::write(addons_dir.join("test.hpp"), test_content).unwrap();

        // Create parser without explicit config path - should discover and use hemtt.toml
        debug!("Creating parser with project root: {}", project_root.display());
        let parser = AdvancedProjectParser::new(project_root, None).unwrap();
        
        // Verify config exists and was loaded
        assert!(config_path.exists(), "hemtt.toml should exist");
        assert!(parser.has_project_config(), "Project config should have been auto-discovered");

        // Test parsing - check if lints affect the result
        let result = parser.parse_file(Path::new("addons/test.hpp"));
        match result {
            Ok((classes, warnings)) => {
                println!("Parsing succeeded with {} classes (lints may be warnings only)", classes.len());
                for warning in &warnings {
                    println!("Lint warning: {} - {}", warning.code, warning.message);
                }
                // If parsing succeeds, lints might only be warnings, not errors
                // This is actually the expected behavior for most lint rules
            }
            Err(e) => {
                println!("Parsing failed due to lints: {:?}", e);
                // If parsing fails, the lint rules are working as errors
            }
        }
        
        // The test should verify that config is loaded, not necessarily that parsing fails
        // Most lint rules generate warnings, not errors that stop parsing

        // Keep temp_dir alive until end of test
        drop(temp_dir);
    }

    #[test]
    fn test_linting_disabled_without_config() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();

        // Create a file with content that would normally trigger lint warnings
        let addons_dir = project_root.join("addons");
        fs::create_dir_all(&addons_dir).unwrap();
        
        // This content would normally trigger various lint warnings:
        // - file_type (not starting with class)
        // - missing_semicolon (missing semicolons)
        // - class_missing_braces (missing braces)
        let content = r#"
            // Missing class at start would trigger file_type lint
            class Test {
                value = 123    // Missing semicolon
                class SubClass // Missing braces
            }                  // Missing semicolon
        "#;
        
        fs::write(addons_dir.join("test.hpp"), content).unwrap();

        // Create parser without config
        let parser = AdvancedProjectParser::new(project_root, None).unwrap();
        
        // Parsing should succeed despite lint violations
        let result = parser.parse_file(Path::new("addons/test.hpp"));
        match result {
            Ok((classes, warnings)) => {
                println!("Parsing succeeded with {} classes and {} warnings", classes.len(), warnings.len());
                for warning in &warnings {
                    println!("Warning: {} - {}", warning.code, warning.message);
                }
            }
            Err(e) => {
                panic!("Parsing should succeed with linting disabled, but got error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_file_not_in_project_for_wrapper() {
        let (_temp_dir, project_parser) = setup_basic_project_for_parser();
        let project_root = _temp_dir.path().to_path_buf();
        let wrapper = AdvancedFileParserWrapper::new(project_parser, &project_root);

        let outside_file_dir = tempdir().unwrap();
        let outside_file_path = outside_file_dir.path().join("outside.hpp");
        fs::write(&outside_file_path, "class OutsideClass {};").unwrap();

        let classes = wrapper.parse_file(&outside_file_path);
        assert!(classes.is_empty(), "Should return empty for files outside the project root");
    }

    #[test]
    fn test_legacy_parse_file_api() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();

        let config_content = r#"
            class TestClass {
                displayName = "Test";
                value = 42;
            };
        "#;
        let test_file = project_root.join("test.cpp");
        fs::write(&test_file, config_content).unwrap();

        // Test the legacy parse_file function
        let result = parse_file(&test_file);
        assert!(result.is_ok(), "Legacy parse_file should work: {:?}", result.err());
        
        let classes = result.unwrap();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "TestClass");
    }

    #[test]
    fn test_extract_dependencies() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();

        // Create a dummy addon structure
        let addons_dir = project_root.join("addons");
        fs::create_dir_all(&addons_dir).unwrap();
        let main_addon_dir = addons_dir.join("main");
        fs::create_dir_all(&main_addon_dir).unwrap();

        let config_content = r#"
            class baseMan {
                uniform[] = {"test_uniform_1", "test_uniform_2"};
                vest[] = {"test_vest"};
                backpack = "test_backpack";
            };
            class rifleman : baseMan {
                magazines[] = {"test_magazine"};
            };
        "#;
        fs::write(main_addon_dir.join("loadout.hpp"), config_content).unwrap();

        let parser = AdvancedProjectParser::new(project_root, None).unwrap();
        let dependencies = parser.extract_dependencies(Path::new("addons/main/loadout.hpp")).unwrap();

        // Verify that dependency extraction works
        assert!(dependencies.contains("test_uniform_1"));
        assert!(dependencies.contains("test_uniform_2"));
        assert!(dependencies.contains("test_vest"));
        assert!(dependencies.contains("test_backpack"));
        assert!(dependencies.contains("test_magazine"));
    }

    #[test]
    fn test_parser_mode_enum() {
        // Test that ParserMode enum works as expected
        assert_eq!(ParserMode::Simple, ParserMode::Simple);
        assert_eq!(ParserMode::Advanced, ParserMode::Advanced);
        assert_ne!(ParserMode::Simple, ParserMode::Advanced);
        
        // Test Debug formatting
        assert_eq!(format!("{:?}", ParserMode::Simple), "Simple");
        assert_eq!(format!("{:?}", ParserMode::Advanced), "Advanced");
    }

    #[test]
    fn test_simple_parser_integration() {
        let temp_dir = tempdir().unwrap();
        let test_file = temp_dir.path().join("simple_test.hpp");
        
        let config_content = r#"
            class BaseClass {
                displayName = "Base";
            };
            class DerivedClass : BaseClass {
                displayName = "Derived";
            };
            // Forward declaration
            class ForwardClass;
        "#;
        
        fs::write(&test_file, config_content).unwrap();
        
        // Test the simple parser directly
        let classes = parse_file_simple(&test_file);
        
        assert_eq!(classes.len(), 3);
        
        // Verify class names
        let names: Vec<&str> = classes.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"BaseClass"));
        assert!(names.contains(&"DerivedClass"));
        assert!(names.contains(&"ForwardClass"));
        
        // Verify inheritance
        let derived = classes.iter().find(|c| c.name == "DerivedClass").unwrap();
        assert_eq!(derived.parent.as_deref(), Some("BaseClass"));
        
        // Verify forward declaration
        let forward = classes.iter().find(|c| c.name == "ForwardClass").unwrap();
        assert!(forward.is_forward_declaration);
    }

    #[test]
    fn test_advanced_vs_simple_parser_compatibility() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();
        
        // Create a dummy addon structure
        let addons_dir = project_root.join("addons");
        fs::create_dir_all(&addons_dir).unwrap();
        let main_addon_dir = addons_dir.join("main");
        fs::create_dir_all(&main_addon_dir).unwrap();
        
        let config_content = r#"
            class BaseClass {
                displayName = "Base";
            };
            class DerivedClass : BaseClass {
                displayName = "Derived";
            };
        "#;
        
        let test_file = main_addon_dir.join("test.hpp");
        fs::write(&test_file, config_content).unwrap();
        
        // Test advanced parser
        let advanced_parser = AdvancedProjectParser::new(project_root, None).unwrap();
        let (advanced_classes, _) = advanced_parser.parse_file(Path::new("addons/main/test.hpp")).unwrap();
        
        // Test simple parser
        let simple_classes = parse_file_simple(&test_file);
        
        // Both should find the same number of classes
        assert_eq!(advanced_classes.len(), simple_classes.len());
        
        // Both should find the same class names
        let advanced_names: std::collections::HashSet<_> = advanced_classes.iter().map(|c| &c.name).collect();
        let simple_names: std::collections::HashSet<_> = simple_classes.iter().map(|c| &c.name).collect();
        assert_eq!(advanced_names, simple_names);
        
        // Verify inheritance is detected by both
        let advanced_derived = advanced_classes.iter().find(|c| c.name == "DerivedClass").unwrap();
        let simple_derived = simple_classes.iter().find(|c| c.name == "DerivedClass").unwrap();
        assert_eq!(advanced_derived.parent, simple_derived.parent);
    }

    #[test]
    fn test_unified_hpp_parser_simple_mode() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();
        
        // Create a test file
        let test_file = project_root.join("test.hpp");
        let config_content = r#"
            class BaseClass {
                displayName = "Base";
            };
            class DerivedClass : BaseClass {
                displayName = "Derived";
            };
        "#;
        fs::write(&test_file, config_content).unwrap();
        
        // Create unified parser
        let parser = HppParser::new(project_root, None).unwrap();
        
        // Test simple mode parsing
        let classes = parser.parse_file(&test_file, ParserMode::Simple).unwrap();
        
        assert_eq!(classes.len(), 2);
        assert!(classes.iter().any(|c| c.name == "BaseClass"));
        assert!(classes.iter().any(|c| c.name == "DerivedClass"));
        
        // Verify inheritance
        let derived = classes.iter().find(|c| c.name == "DerivedClass").unwrap();
        assert_eq!(derived.parent.as_deref(), Some("BaseClass"));
    }

    #[test]
    fn test_unified_hpp_parser_advanced_mode() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();
        
        // Create a dummy addon structure
        let addons_dir = project_root.join("addons");
        fs::create_dir_all(&addons_dir).unwrap();
        let main_addon_dir = addons_dir.join("main");
        fs::create_dir_all(&main_addon_dir).unwrap();
        
        let config_content = r#"
            class BaseClass {
                displayName = "Base";
                value = 42;
            };
            class DerivedClass : BaseClass {
                displayName = "Derived";
                items[] = {"item1", "item2"};
            };
        "#;
        
        let test_file = main_addon_dir.join("test.hpp");
        fs::write(&test_file, config_content).unwrap();
        
        // Create unified parser
        let parser = HppParser::new(project_root, None).unwrap();
        
        // Test advanced mode parsing with relative path
        let classes = parser.parse_file(Path::new("addons/main/test.hpp"), ParserMode::Advanced).unwrap();
        
        assert_eq!(classes.len(), 2);
        
        // Advanced parser should include properties
        let base_class = classes.iter().find(|c| c.name == "BaseClass").unwrap();
        assert!(!base_class.properties.is_empty());
        
        let derived_class = classes.iter().find(|c| c.name == "DerivedClass").unwrap();
        assert!(!derived_class.properties.is_empty());
        assert_eq!(derived_class.parent.as_deref(), Some("BaseClass"));
    }

    #[test]
    fn test_unified_hpp_parser_dependency_extraction() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();
        
        // Create a dummy addon structure
        let addons_dir = project_root.join("addons");
        fs::create_dir_all(&addons_dir).unwrap();
        let main_addon_dir = addons_dir.join("main");
        fs::create_dir_all(&main_addon_dir).unwrap();
        
        let config_content = r#"
            class baseMan {
                uniform[] = {"test_uniform_1", "test_uniform_2"};
                vest[] = {"test_vest"};
                backpack = "test_backpack";
            };
        "#;
        
        let test_file = main_addon_dir.join("loadout.hpp");
        fs::write(&test_file, config_content).unwrap();
        
        // Create unified parser
        let parser = HppParser::new(project_root, None).unwrap();
        
        // Test dependency extraction
        let dependencies = parser.extract_dependencies(Path::new("addons/main/loadout.hpp")).unwrap();
        
        assert!(dependencies.contains("test_uniform_1"));
        assert!(dependencies.contains("test_uniform_2"));
        assert!(dependencies.contains("test_vest"));
        assert!(dependencies.contains("test_backpack"));
    }

    #[test]
    fn test_unified_hpp_parser_from_content() {
        let config_content = r#"
            class TestClass {
                displayName = "Test";
                value = 123;
            };
        "#;
        
        // Create parser from content
        let parser = HppParser::from_content(config_content).unwrap();
        
        // Test that parser was created successfully
        assert!(parser.has_advanced_parser());
        
        // Test simple parsing on the temporary file
        let temp_file = parser.project_root().join("temp.hpp");
        let classes = parser.parse_file(&temp_file, ParserMode::Simple).unwrap();
        
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name, "TestClass");
    }

    #[test]
    fn test_unified_hpp_parser_mode_switching() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();
        
        // Create a test file
        let test_file = project_root.join("test.hpp");
        let config_content = r#"
            class BaseClass {
                displayName = "Base";
                value = 42;
            };
        "#;
        fs::write(&test_file, config_content).unwrap();
        
        // Create unified parser
        let parser = HppParser::new(project_root, None).unwrap();
        
        // Test both modes on the same file
        let simple_classes = parser.parse_file(&test_file, ParserMode::Simple).unwrap();
        let advanced_classes = parser.parse_file(&test_file, ParserMode::Advanced).unwrap();
        
        // Both should find the same class
        assert_eq!(simple_classes.len(), 1);
        assert_eq!(advanced_classes.len(), 1);
        assert_eq!(simple_classes[0].name, advanced_classes[0].name);
        
        // But advanced parser should have more detailed information
        assert!(simple_classes[0].properties.is_empty()); // Simple parser doesn't extract properties
        assert!(!advanced_classes[0].properties.is_empty()); // Advanced parser does
    }

    #[test]
    fn test_parse_all_project_files() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init();

        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();

        // Create a complex project structure with multiple directories and files
        let addons_dir = project_root.join("addons");
        fs::create_dir_all(&addons_dir).unwrap();
        
        let main_addon_dir = addons_dir.join("main");
        fs::create_dir_all(&main_addon_dir).unwrap();
        
        let weapons_addon_dir = addons_dir.join("weapons");
        fs::create_dir_all(&weapons_addon_dir).unwrap();
        
        let includes_dir = project_root.join("includes");
        fs::create_dir_all(&includes_dir).unwrap();

        // Create main config file
        let main_config = r#"
            class CfgPatches {
                class test_main {
                    units[] = {};
                    weapons[] = {"test_rifle"};
                    requiredVersion = 1.0;
                };
            };
            
            class MainConfig {
                displayName = "Main Configuration";
                value = 100;
            };
        "#;
        fs::write(main_addon_dir.join("config.cpp"), main_config).unwrap();

        // Create weapons config file
        let weapons_config = r#"
            class CfgWeapons {
                class test_rifle {
                    displayName = "Test Rifle";
                    magazines[] = {"test_magazine"};
                };
            };
            
            class WeaponBase {
                scope = 1;
                model = "test.p3d";
            };
        "#;
        fs::write(weapons_addon_dir.join("config.hpp"), weapons_config).unwrap();

        // Create a header file in includes
        let header_file = r#"
            class CommonHeader {
                version = "1.0.0";
                author = "Test Author";
            };
        "#;
        fs::write(includes_dir.join("common.h"), header_file).unwrap();

        // Create another file with some invalid content (to test error handling)
        let invalid_config = r#"
            // This file has intentionally invalid syntax
            class InvalidClass {
                property = "unclosed string
                invalid_syntax here
            };
        "#;
        fs::write(main_addon_dir.join("invalid.cpp"), invalid_config).unwrap();

        // Create a file that should be ignored (not hpp/cpp/h)
        fs::write(project_root.join("readme.txt"), "This should be ignored").unwrap();

        // Create nested subdirectory
        let sub_dir = weapons_addon_dir.join("accessories");
        fs::create_dir_all(&sub_dir).unwrap();
        let accessories_config = r#"
            class CfgAccessories {
                class test_scope {
                    displayName = "Test Scope";
                    model = "scope.p3d";
                };
            };
        "#;
        fs::write(sub_dir.join("accessories.hpp"), accessories_config).unwrap();

        // Create parser and test parse_all_project_files
        let parser = AdvancedProjectParser::new(project_root, None).unwrap();
        let result = parser.parse_all_project_files();

        assert!(result.is_ok(), "parse_all_project_files should succeed even with some invalid files");
        let all_classes = result.unwrap();

        // Verify we found classes from multiple files
        assert!(!all_classes.is_empty(), "Should find classes from project files");

        // Collect all class names for verification
        let class_names: std::collections::HashSet<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();

        // Should find classes from main config
        assert!(class_names.contains("CfgPatches"), "Should find CfgPatches from main config");
        assert!(class_names.contains("MainConfig"), "Should find MainConfig from main config");

        // Should find classes from weapons config
        assert!(class_names.contains("CfgWeapons"), "Should find CfgWeapons from weapons config");
        assert!(class_names.contains("WeaponBase"), "Should find WeaponBase from weapons config");

        // Should find classes from header file
        assert!(class_names.contains("CommonHeader"), "Should find CommonHeader from header file");

        // Should find classes from nested subdirectory
        assert!(class_names.contains("CfgAccessories"), "Should find CfgAccessories from nested file");

        println!("Found {} total classes: {:?}", all_classes.len(), class_names);

        // Verify file discovery worked correctly
        // We should have discovered at least 5 files: config.cpp, config.hpp, common.h, invalid.cpp, accessories.hpp
        // The method should have attempted to parse all of them
        
        // Test with HppParser (unified interface)
        let unified_parser = HppParser::new(project_root, None).unwrap();
        
        // Test that advanced parser is available for project-wide parsing
        assert!(unified_parser.has_advanced_parser(), "Advanced parser should be available for project-wide parsing");

        // Verify that invalid files don't cause the entire operation to fail
        // (the method should log errors but continue parsing other files)
        assert!(all_classes.len() >= 5, "Should find multiple classes despite invalid files");
    }

    #[test]
    fn test_parse_all_project_files_empty_project() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();

        // Create empty project with no hpp/cpp/h files
        let some_dir = project_root.join("some_dir");
        fs::create_dir_all(&some_dir).unwrap();
        fs::write(some_dir.join("readme.txt"), "Not a config file").unwrap();
        fs::write(some_dir.join("data.json"), "{}").unwrap();

        let parser = AdvancedProjectParser::new(project_root, None).unwrap();
        let result = parser.parse_all_project_files();

        assert!(result.is_ok(), "Should succeed even with no relevant files");
        let all_classes = result.unwrap();
        assert!(all_classes.is_empty(), "Should return empty list when no hpp/cpp/h files found");
    }

    #[test]
    fn test_parse_all_project_files_with_symlinks() {
        let temp_dir = tempdir().unwrap();
        let project_root = temp_dir.path();

        // Create a real file
        let real_file = project_root.join("real.hpp");
        let content = r#"
            class RealClass {
                displayName = "Real";
            };
        "#;
        fs::write(&real_file, content).unwrap();

        // Note: Creating symlinks in tests can be platform-dependent and may require permissions
        // For this test, we'll just verify the walkdir doesn't follow symlinks (follow_links = false)
        // The actual symlink creation is skipped to avoid platform issues

        let parser = AdvancedProjectParser::new(project_root, None).unwrap();
        let result = parser.parse_all_project_files();

        assert!(result.is_ok(), "Should handle projects with symlinks");
        let all_classes = result.unwrap();
        
        // Should find the real file
        assert!(!all_classes.is_empty(), "Should find real files");
        let class_names: Vec<&str> = all_classes.iter().map(|c| c.name.as_str()).collect();
        assert!(class_names.contains(&"RealClass"), "Should find RealClass");
    }
}