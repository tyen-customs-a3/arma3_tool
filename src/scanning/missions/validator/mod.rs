pub mod types;
mod validator;

use std::collections::HashSet;
use anyhow::Result;
use log::info;

pub use types::ClassExistenceReport;
use crate::scanning::classes::processor::ProcessedClass;
use crate::scanning::missions::analyzer::types::MissionDependencyResult;

/// Validates if classes used in missions exist in the scanned database
pub struct ClassExistenceValidator {
    /// Processed classes for searching
    processed_classes: Vec<ProcessedClass>,
    /// Flag indicating if the class database has been loaded
    db_loaded: bool,
}

impl ClassExistenceValidator {
    /// Create a new validator
    pub fn new() -> Self {
        Self {
            processed_classes: Vec::new(),
            db_loaded: false,
        }
    }
    
    /// Load the class database from in-memory processed classes
    pub fn load_class_database_from_memory(&mut self, processed_classes: &[ProcessedClass]) -> Result<()> {
        validator::load_class_database_from_memory(self, processed_classes)
    }
    
    /// Validate if classes used in missions exist in the database
    pub fn validate_mission_classes(&self, mission_results: &[MissionDependencyResult]) -> Result<ClassExistenceReport> {
        validator::validate_mission_classes(self, mission_results)
    }
} 