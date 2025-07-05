pub mod args;
pub mod commands;

use log::debug;
use crate::core::api::{PboApi, PboApiOps};
use crate::ops::PboOperationResult;
use self::args::Commands;

pub struct CliProcessor {
    api: PboApi,
}

impl CliProcessor {
    pub fn new(timeout: u32) -> Self {
        debug!("Creating new CliProcessor with timeout: {} seconds", timeout);
        Self {
            api: PboApi::builder()
                .with_timeout(timeout)
                .build(),
        }
    }

    pub async fn process_command(&self, command: Commands) -> PboOperationResult<()> {
        debug!("Processing command: {:?}", command);
        match command {
            Commands::List { pbo_path, brief: _, verbose } => {
                debug!("Listing contents of PBO: {}", pbo_path.display());
                commands::list_contents(&self.api, &pbo_path, verbose).await
            }
            Commands::Extract { pbo_path, output_dir, filter, keep_pbo_name: _, verbose, ignore_warnings: _ } => {
                debug!("Extracting from PBO: {} to {}", pbo_path.display(), output_dir.display());
                commands::extract_contents(&self.api, &pbo_path, &output_dir, filter, verbose).await
            }
            Commands::Properties { pbo_path } => {
                debug!("Getting properties for PBO: {}", pbo_path.display());
                commands::show_properties(&self.api, &pbo_path).await
            }
            Commands::Validate { pbo_path, verbose } => {
                debug!("Validating PBO: {}", pbo_path.display());
                commands::validate_pbo(&self.api, &pbo_path, verbose).await
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use crate::test_utils;

    #[tokio::test]
    async fn test_cli_list_command() {
        test_utils::setup();
        let cli = CliProcessor::new(10);
        let test_pbo = test_utils::get_test_pbo_path();
        let result = cli.process_command(Commands::List { 
            pbo_path: test_pbo,
            brief: false,
            verbose: false,
        }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cli_extract_command() {
        test_utils::setup();
        let cli = CliProcessor::new(10);
        let test_pbo = test_utils::get_test_pbo_path();
        let temp_dir = tempdir().unwrap();
        
        let result = cli.process_command(Commands::Extract { 
            pbo_path: test_pbo,
            output_dir: temp_dir.path().to_path_buf(),
            filter: None,
            keep_pbo_name: false,
            verbose: false,
            ignore_warnings: false,
        }).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cli_with_invalid_paths() {
        test_utils::setup();
        let cli = CliProcessor::new(30);
        let invalid_pbo = PathBuf::from("nonexistent.pbo");
        
        let result = cli.process_command(Commands::List { 
            pbo_path: invalid_pbo.clone(),
            brief: false,
            verbose: false,
        }).await;
        assert!(result.is_err());

        let result = cli.process_command(Commands::Extract { 
            pbo_path: invalid_pbo,
            output_dir: PathBuf::from("output"),
            filter: None,
            keep_pbo_name: false,
            verbose: false,
            ignore_warnings: false,
        }).await;
        assert!(result.is_err());
    }
}
