use hemtt_common::config::{PDriveOption, ProjectConfig};
use hemtt_workspace::{LayerType, Workspace, WorkspacePath};
use std::path::Path;
use crate::error::ParseError;
use log::debug;

pub struct WorkspaceManager {
    workspace_root_wpath: WorkspacePath, // This is the VFS root path
    #[allow(dead_code)]
    project_config: Option<ProjectConfig>,
}

impl WorkspaceManager {
    pub fn new(
        project_root_dir: &Path,
        project_config: Option<ProjectConfig>,
        pdrive_option: &PDriveOption,
    ) -> Result<Self, ParseError> {
        debug!(
            "Initializing WorkspaceManager for project root: {}",
            project_root_dir.display()
        );

        let workspace_root_wpath = Workspace::builder()
            .physical(&project_root_dir.to_path_buf(), LayerType::Source)
            .finish(project_config.clone(), true, pdrive_option)?;

        Ok(Self {
            workspace_root_wpath,
            project_config,
        })
    }

    pub fn get_workspace_path_for_relative(
        &self,
        relative_path: &Path,
    ) -> Result<WorkspacePath, ParseError> {
        let relative_path_str = relative_path.to_str().ok_or_else(|| {
            ParseError::PathConversionError(relative_path.to_path_buf())
        })?;

        let full_wpath = self.workspace_root_wpath.join(relative_path_str)?;

        if !full_wpath.exists()? {
            return Err(ParseError::FileNotFoundInWorkspace(
                full_wpath.as_str().to_string(),
            ));
        }
        Ok(full_wpath)
    }

    #[allow(dead_code)]
    pub fn project_config(&self) -> Option<&ProjectConfig> {
        self.project_config.as_ref()
    }

    #[allow(dead_code)]
    pub fn workspace_root(&self) -> &WorkspacePath {
        &self.workspace_root_wpath
    }
}