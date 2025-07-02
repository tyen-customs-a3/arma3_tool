use hemtt_workspace::reporting::Codes;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("HEMTT Workspace error: {0}")]
    Workspace(#[from] hemtt_workspace::Error),

    #[error("HEMTT Preprocessor error: {0} (Included files: {1:?})")]
    Preprocessor(hemtt_preprocessor::Error, Vec<hemtt_workspace::WorkspacePath>),

    #[error("HEMTT Config parsing failed with multiple error codes")]
    ConfigParse(Codes), // Vec<Arc<dyn Code>>

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File not found in workspace: {0}")]
    FileNotFoundInWorkspace(String),

    #[error("Project config (hemtt.toml) not found or failed to load at {0}: {1}")]
    ProjectConfigLoad(PathBuf, hemtt_common::error::Error),

    #[error("The provided path '{0}' is not within the project root '{1}'")]
    PathNotInProject(PathBuf, PathBuf),

    #[error("Could not convert path to relative: {0}")]
    PathConversionError(PathBuf),
}

impl From<(hemtt_preprocessor::Error, Vec<hemtt_workspace::WorkspacePath>)> for ParseError {
    fn from(value: (hemtt_preprocessor::Error, Vec<hemtt_workspace::WorkspacePath>)) -> Self {
        ParseError::Preprocessor(value.0, value.1)
    }
}

impl ParseError {
    /// Get the number of codes in a ConfigParse error, if applicable
    pub fn code_count(&self) -> Option<usize> {
        match self {
            ParseError::ConfigParse(codes) => Some(codes.len()),
            _ => None,
        }
    }
}

// You might want a method here to get a Vec<Arc<dyn Code>> from ParseError
// for consistent diagnostic reporting upstream, but that's more involved.
