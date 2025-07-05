use crate::FileParser;
use arma3_types::Class as GameClass;
use std::sync::Arc;
use std::path::Path;

/// Returns a file parser implementation based on the parser type
/// Note: use_advanced parameter is kept for API compatibility but ignored since the unified parser supports both modes
pub fn get_parser(project_root_dir: &Path, _use_advanced: bool) -> Result<Arc<dyn FileParser>, String> {
    match arma3_parser_hpp::AdvancedProjectParser::new(
        project_root_dir,
        None, // Auto-discover hemtt.toml in project_root_dir
    ) {
        Ok(project_parser) => {
            let wrapper = HppParserWrapper::new(Arc::new(project_parser), project_root_dir);
            Ok(Arc::new(wrapper))
        }
        Err(e) => {
            let err_msg = format!("Failed to create AdvancedProjectParser for root '{}': {:?}", project_root_dir.display(), e);
            log::error!("{}", err_msg);
            Err(err_msg)
        }
    }
}

/// Wrapper around the arma3-parser-hpp crate to implement our FileParser trait
struct HppParserWrapper {
    parser: Arc<arma3_parser_hpp::AdvancedProjectParser>,
    project_root_dir: std::path::PathBuf,
}

impl HppParserWrapper {
    fn new(parser: Arc<arma3_parser_hpp::AdvancedProjectParser>, project_root_dir: &Path) -> Self {
        Self { 
            parser,
            project_root_dir: project_root_dir.to_path_buf(),
        }
    }
}

impl FileParser for HppParserWrapper {
    fn parse_file(&self, file_path: &Path) -> Vec<GameClass> {
        // Convert absolute path to relative if needed
        let relative_path = if file_path.is_absolute() {
            match file_path.strip_prefix(&self.project_root_dir) {
                Ok(rel_path) => rel_path,
                Err(_) => {
                    log::warn!("File path {} is not within project root {}", file_path.display(), self.project_root_dir.display());
                    return Vec::new();
                }
            }
        } else {
            file_path
        };

        match self.parser.parse_file(relative_path) {
            Ok((classes, warnings)) => {
                // Log any warnings
                for warning in warnings {
                    log::warn!("Parse warning for {}: {} - {}", file_path.display(), warning.code, warning.message);
                }
                classes
            }
            Err(e) => {
                log::error!("Failed to parse file {}: {:?}", file_path.display(), e);
                Vec::new()
            }
        }
    }

    fn name(&self) -> &str {
        "Advanced HPP Parser"
    }
}
