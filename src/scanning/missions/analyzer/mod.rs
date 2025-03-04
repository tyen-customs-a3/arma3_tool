pub mod types;
mod parser;
mod sqf_parser;
mod sqm_parser;
mod cpp_parser;

use std::path::Path;
use anyhow::Result;
use log::{info, debug};

pub use types::{MissionDependencyResult, ClassDependency, ReferenceType};
use crate::scanning::missions::extractor::types::MissionExtractionResult;

/// Analyzes mission dependencies from extracted mission files
pub struct DependencyAnalyzer<'a> {
    cache_dir: &'a Path,
    sqf_parser: ::sqf_parser::SqfClassParser,
}

impl<'a> DependencyAnalyzer<'a> {
    /// Create a new dependency analyzer
    pub fn new(cache_dir: &'a Path) -> Self {
        Self {
            cache_dir,
            sqf_parser: ::sqf_parser::SqfClassParser::new(),
        }
    }
    
    /// Analyze dependencies in multiple missions
    pub fn analyze_missions(
        &self,
        extraction_results: &[MissionExtractionResult],
    ) -> Result<Vec<MissionDependencyResult>> {
        info!("Analyzing dependencies for {} missions", extraction_results.len());
        
        // Process each mission in parallel
        let results: Vec<MissionDependencyResult> = extraction_results.iter()
            .filter_map(|extraction| {
                match self.analyze_single_mission(extraction) {
                    Ok(result) => Some(result),
                    Err(e) => {
                        debug!("Failed to analyze mission {}: {}", extraction.mission_name, e);
                        None
                    }
                }
            })
            .collect();
        
        info!("Successfully analyzed {} missions", results.len());
        
        Ok(results)
    }
    
    /// Analyze a single mission's dependencies
    fn analyze_single_mission(&self, extraction: &MissionExtractionResult) -> Result<MissionDependencyResult> {
        // Delegate to the parser module
        parser::analyze_mission(self.cache_dir, &self.sqf_parser, extraction)
    }
} 