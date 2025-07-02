use std::path::{Path, PathBuf};
use std::sync::Arc;

use hemtt_common::config::{PDriveOption, ProjectConfig};
use log::debug;

mod error;
mod workspace_manager;
mod file_processor;
mod ast_transformer;
pub mod models; // Ensure models module is public if types are used in public API

pub use error::ParseError;
pub use file_processor::{ParseResult, ParseWarning}; // Export new parsing result types
pub use models::{GameClass, ClassProperty, PropertyValue, FileParser}; // Re-export all needed types
use workspace_manager::WorkspaceManager;

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

    // Optional:
    // pub fn parse_all_project_files(&self) -> Result<Vec<GameClass>, ParseError> {
    //     // 1. Discover HPP files in the workspace (e.g., walk "addons/")
    //     //    let root_vfs_path = self.workspace_manager.workspace_root().vfs();
    //     //    let addons_path_str = "addons"; // Or get from config
    //     //    let addons_vfs = root_vfs_path.join(addons_path_str)?;
    //     //    ... walk addons_vfs ...
    //     // 2. For each, determine its relative path.
    //     // 3. Call self.parse_file(relative_path).
    //     // 4. Aggregate results.
    //     unimplemented!("parse_all_project_files is not yet implemented");
    // }
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
}
