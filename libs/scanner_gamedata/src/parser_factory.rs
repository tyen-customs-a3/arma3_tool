use gamedata_scanner_models::FileParser;
use std::sync::Arc;
use std::path::Path; // Import Path

/// Returns a file parser implementation based on the parser type
pub fn get_parser(project_root_dir: &Path, use_advanced: bool) -> Result<Arc<dyn FileParser>, String> {
    if use_advanced {
        match ::parser_advanced::AdvancedProjectParser::new(
            project_root_dir,
            None, // Auto-discover hemtt.toml in project_root_dir
        ) {
            Ok(project_parser) => {
                let wrapper = ::parser_advanced::AdvancedFileParserWrapper::new(
                    Arc::new(project_parser),
                    project_root_dir,
                );
                Ok(Arc::new(wrapper))
            }
            Err(e) => {
                let err_msg = format!("Failed to create AdvancedProjectParser for root '{}': {:?}", project_root_dir.display(), e);
                log::error!("{}", err_msg);
                // Optionally, you could still fallback to simple parser here,
                // but returning an error is cleaner for the new API.
                // Ok(Arc::new(::parser_simple::SimpleFileParser::new()))
                Err(err_msg)
            }
        }
    } else {
        Ok(Arc::new(::parser_simple::SimpleFileParser::new()))
    }
}
