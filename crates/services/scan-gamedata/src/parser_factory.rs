use gamedata_scanner_models::FileParser;
use std::sync::Arc;
use std::path::Path;

/// Returns a file parser implementation based on the parser type
/// Note: use_advanced parameter is kept for API compatibility but ignored since the unified parser supports both modes
pub fn get_parser(project_root_dir: &Path, _use_advanced: bool) -> Result<Arc<dyn FileParser>, String> {
    match ::parser_hpp::AdvancedProjectParser::new(
        project_root_dir,
        None, // Auto-discover hemtt.toml in project_root_dir
    ) {
        Ok(project_parser) => {
            let wrapper = ::parser_hpp::AdvancedFileParserWrapper::new(
                Arc::new(project_parser),
                project_root_dir,
            );
            Ok(Arc::new(wrapper))
        }
        Err(e) => {
            let err_msg = format!("Failed to create AdvancedProjectParser for root '{}': {:?}", project_root_dir.display(), e);
            log::error!("{}", err_msg);
            Err(err_msg)
        }
    }
}
