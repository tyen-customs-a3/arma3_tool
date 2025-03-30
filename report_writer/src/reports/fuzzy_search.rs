use crate::utils;
use crate::error::Result;
use crate::utils::mission_dependency_builder::{Dependency, ScanReport};
use arma3_tool_shared_models::GameDataClasses;
use prettytable::{Row, Cell};
use rayon::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use strsim::jaro_winkler;

const SIMILARITY_THRESHOLD: f64 = 0.85;

#[derive(Debug, Clone)]
pub struct SimilarClass {
    pub name: String,
    pub similarity: f64,
    pub source_file: String,
}

pub struct FuzzySearchReport {
    pub missing_classes: Vec<Dependency>,
    pub similar_classes: HashMap<String, Vec<SimilarClass>>,
}

pub struct FuzzySearchReportWriter<'a> {
    report: &'a ScanReport,
    game_data: &'a GameDataClasses,
}

impl<'a> FuzzySearchReportWriter<'a> {
    pub fn new(report: &'a ScanReport, game_data: &'a GameDataClasses) -> Self {
        Self {
            report,
            game_data,
        }
    }

    pub fn write_report(&self, output_path: &Path) -> Result<FuzzySearchReport> {
        let mut fuzzy_report = FuzzySearchReport {
            missing_classes: self.report.missing.clone(),
            similar_classes: HashMap::new(),
        };

        // Process each missing dependency
        for missing_dep in &self.report.missing {

            let similar = self.find_similar_classes(&missing_dep.class_name);
            if !similar.is_empty() {
                fuzzy_report.similar_classes.insert(missing_dep.class_name.clone(), similar);
            }
        }

        // Write the report
        self.write_fuzzy_text_report(&fuzzy_report, output_path)?;

        Ok(fuzzy_report)
    }

    fn find_similar_classes(&self, class_name: &str) -> Vec<SimilarClass> {
        let class_name_lower = class_name.to_lowercase();
        
        self.game_data.classes.par_iter()
            .filter_map(|class| {
                let similarity = jaro_winkler(&class_name_lower, &class.name.to_lowercase());
                if similarity >= SIMILARITY_THRESHOLD {
                    let source_file = class.source_file_index
                        .and_then(|idx| self.game_data.get_file_source(idx))
                        .map(|path| path.display().to_string())
                        .unwrap_or_else(|| "Unknown".to_string());

                    Some(SimilarClass {
                        name: class.name.clone(),
                        similarity,
                        source_file,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    fn write_fuzzy_text_report(&self, report: &FuzzySearchReport, output_path: &Path) -> Result<()> {
        let mut report_text = String::new();
        
        // Create report header
        report_text.push_str("=== FUZZY SEARCH REPORT ===\n\n");
        
        // Create summary table
        let mut summary_table = utils::create_summary_table();
        summary_table.add_row(Row::new(vec![
            Cell::new("Total Missing Dependencies"),
            Cell::new(&report.missing_classes.len().to_string()),
        ]));
        summary_table.add_row(Row::new(vec![
            Cell::new("Dependencies with Similar Classes Found"),
            Cell::new(&report.similar_classes.len().to_string()),
        ]));
        
        report_text.push_str(&utils::table_to_string(&summary_table));
        report_text.push_str("\n\n");

        // Create detailed results table
        if !report.similar_classes.is_empty() {
            report_text.push_str("=== SIMILAR CLASSES FOUND ===\n");
            
            let mut results_table = utils::create_standard_table();
            results_table.set_titles(Row::new(vec![
                Cell::new("Missing Class"),
                Cell::new("Similar Class Found"),
                Cell::new("Similarity Score"),
                Cell::new("Defined In"),
            ]));

            // Sort missing classes for consistent output
            let mut sorted_missing = report.similar_classes.keys().collect::<Vec<_>>();
            sorted_missing.sort();

            for missing_class in sorted_missing {
                if let Some(similar) = report.similar_classes.get(missing_class) {
                    // Sort similar classes by similarity score (descending)
                    let mut sorted_similar = similar.to_vec();
                    sorted_similar.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap());

                    for similar_class in sorted_similar {
                        results_table.add_row(Row::new(vec![
                            Cell::new(missing_class),
                            Cell::new(&similar_class.name),
                            Cell::new(&format!("{:.2}%", similar_class.similarity * 100.0)),
                            Cell::new(&similar_class.source_file),
                        ]));
                    }
                }
            }
            
            report_text.push_str(&utils::table_to_string(&results_table));
            report_text.push_str("\n\n");
        }

        // Write to file
        utils::write_report(output_path, "fuzzy_search_report.txt", &report_text)?;

        Ok(())
    }
} 