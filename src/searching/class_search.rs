use rayon::prelude::*;
use serde::Serialize;
use crate::scanning::classes::processor::ProcessedClass;

/// Type of match found for the class
#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    ExactMatch,
    CaseInsensitiveMatch,
    PartialMatch,
    NotFound,
}

/// Result of a class search
#[derive(Serialize, Clone)]
pub struct ClassSearchResult {
    pub class_name: String,
    pub found: bool,
    pub file_path: Option<String>,
    pub parent_class: Option<String>,
    pub match_type: MatchType,
    pub actual_class_name: Option<String>,  // The actual name if it differs from search (e.g. case difference)
    pub found_in_nested: bool,
    pub nested_parent: Option<String>,      // If found in nested class, this is the parent class
}

/// Find a class using multiple search strategies - thread safe version
pub fn find_class_parallel(class_name: &str, classes: &[ProcessedClass]) -> ClassSearchResult {
    let lowercase_search = class_name.to_lowercase();

    // Strategy 1: Exact match of class name
    if let Some(class) = classes.par_iter().find_any(|c| c.name == class_name) {
        return ClassSearchResult {
            class_name: class_name.to_string(),
            found: true,
            file_path: class.file_path.as_ref().map(|p| p.to_string_lossy().into_owned()),
            parent_class: None,
            match_type: MatchType::ExactMatch,
            actual_class_name: None,
            found_in_nested: false,
            nested_parent: None,
        };
    }

    // Strategy 3: Case-insensitive match of class name
    if let Some(class) = classes.par_iter().find_any(|c| c.name.to_lowercase() == lowercase_search) {
        return ClassSearchResult {
            class_name: class_name.to_string(),
            found: true,
            file_path: class.file_path.as_ref().map(|p| p.to_string_lossy().into_owned()),
            parent_class: None,
            match_type: MatchType::CaseInsensitiveMatch,
            actual_class_name: Some(class.name.clone()),
            found_in_nested: false,
            nested_parent: None,
        };
    }

    // Not found by any strategy
    ClassSearchResult {
        class_name: class_name.to_string(),
        found: false,
        file_path: None,
        parent_class: None,
        match_type: MatchType::NotFound,
        actual_class_name: None,
        found_in_nested: false,
        nested_parent: None,
    }
}

/// Search for multiple classes in parallel
pub fn search_classes_parallel(class_names: &[String], classes: &[ProcessedClass]) -> Vec<ClassSearchResult> {
    class_names.par_iter()
        .map(|class_name| find_class_parallel(class_name, classes))
        .collect()
} 