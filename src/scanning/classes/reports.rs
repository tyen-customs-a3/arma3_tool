use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use serde_json;
use crate::scanning::classes::processor::ProcessedClass;

pub struct ReportWriter {
    output_dir: PathBuf,
}

impl ReportWriter {
    pub fn new(output_dir: &Path) -> Self {
        Self {
            output_dir: output_dir.to_owned(),
        }
    }

    pub fn write_all_reports(&self, classes: &[ProcessedClass]) -> Result<()> {
        fs::create_dir_all(&self.output_dir)?;
        
        // Write JSON report
        let json_path = self.output_dir.join("classes.json");
        fs::write(&json_path, serde_json::to_string_pretty(classes)?)?;
        
        Ok(())
    }
}