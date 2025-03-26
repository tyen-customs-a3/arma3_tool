use std::collections::{HashMap, HashSet};
use std::path::{PathBuf, Path};
use arma3_tool_shared_models::gamedata::GameDataClass;
use arma3_tool_cache_storage::PboCache;

/// Maps classes to their source PBO files and provides fast search capabilities
#[derive(Debug, Clone)]
pub struct SourceGraph {
    /// Map of class names to their source PBO paths
    class_to_pbo: HashMap<String, PathBuf>,
    
    /// Map of class names to their original PBO paths (from cache index)
    class_to_original_pbo: HashMap<String, PathBuf>,
    
    /// Map of class names to their lowercase version for fast case-insensitive search
    class_names_lower: HashMap<String, String>,
    
    /// Map of words to class names for fast word-based search
    word_index: HashMap<String, HashSet<String>>,
}

impl SourceGraph {
    /// Create a new empty source graph
    pub fn new() -> Self {
        Self {
            class_to_pbo: HashMap::new(),
            class_to_original_pbo: HashMap::new(),
            class_names_lower: HashMap::new(),
            word_index: HashMap::new(),
        }
    }
    
    /// Find the original PBO for a source file using the cache index
    fn find_original_pbo(source_file: &Path, cache: &PboCache) -> Option<PathBuf> {
        for (pbo_path, entry) in &cache.game_data {
            if entry.extracted_files.iter().any(|f| f == source_file) {
                return Some(pbo_path.clone());
            }
        }
        None
    }
    
    /// Extract searchable words from a class name
    fn extract_words(name: &str) -> Vec<String> {
        let mut words = Vec::new();
        
        // Split on common separators and case changes
        let mut current_word = String::new();
        let mut last_char_type = None;
        
        for c in name.chars() {
            let char_type = if c.is_uppercase() {
                Some("upper")
            } else if c.is_lowercase() {
                Some("lower")
            } else if c.is_numeric() {
                Some("number")
            } else {
                None
            };
            
            match (last_char_type, char_type) {
                // Start new word on case change (camelCase, PascalCase)
                (Some("lower"), Some("upper")) |
                // Or when transitioning to/from numbers
                (Some("lower"), Some("number")) |
                (Some("upper"), Some("number")) |
                (Some("number"), Some("upper")) |
                (Some("number"), Some("lower")) |
                // Or on separators
                (_, None) | (None, _) => {
                    if !current_word.is_empty() {
                        words.push(current_word.to_lowercase());
                        current_word = String::new();
                    }
                }
                _ => {}
            }
            
            if char_type.is_some() {
                current_word.push(c);
            }
            
            last_char_type = char_type;
        }
        
        if !current_word.is_empty() {
            words.push(current_word.to_lowercase());
        }
        
        words
    }
    
    /// Build a source graph from a list of classes and their file sources
    pub fn from_classes(classes: &[GameDataClass], file_sources: &[PathBuf], cache: &PboCache) -> Self {
        let mut graph = Self::new();
        
        // Pre-allocate capacity
        let class_count = classes.len();
        graph.class_to_pbo = HashMap::with_capacity(class_count);
        graph.class_to_original_pbo = HashMap::with_capacity(class_count);
        graph.class_names_lower = HashMap::with_capacity(class_count);
        
        // First pass: build class mappings
        for class in classes {
            if let Some(source_idx) = class.source_file_index {
                if let Some(source_path) = file_sources.get(source_idx) {
                    let name = class.name.clone();
                    let name_lower = name.to_lowercase();
                    
                    // Index words
                    for word in Self::extract_words(&name) {
                        graph.word_index
                            .entry(word)
                            .or_insert_with(HashSet::new)
                            .insert(name.clone());
                    }
                    
                    // Store both current source and original PBO
                    graph.class_to_pbo.insert(name.clone(), source_path.clone());
                    if let Some(original_pbo) = Self::find_original_pbo(source_path, cache) {
                        graph.class_to_original_pbo.insert(name.clone(), original_pbo);
                    }
                    
                    graph.class_names_lower.insert(name, name_lower);
                }
            }
        }
        
        graph
    }
    
    /// Get the source PBO path for a class
    pub fn get_source_pbo(&self, class_name: &str) -> Option<&PathBuf> {
        self.class_to_pbo.get(class_name)
    }
    
    /// Get the original PBO path for a class (from cache index)
    pub fn get_original_pbo(&self, class_name: &str) -> Option<&PathBuf> {
        self.class_to_original_pbo.get(class_name)
    }
    
    /// Get all classes defined in a PBO
    pub fn get_classes_in_pbo(&self, pbo_path: &PathBuf) -> Vec<String> {
        self.class_to_pbo.iter()
            .filter(|(_, path)| *path == pbo_path)
            .map(|(class_name, _)| class_name.clone())
            .collect()
    }
    
    /// Get all classes defined in an original PBO
    pub fn get_classes_in_original_pbo(&self, pbo_path: &PathBuf) -> Vec<String> {
        self.class_to_original_pbo.iter()
            .filter(|(_, path)| *path == pbo_path)
            .map(|(class_name, _)| class_name.clone())
            .collect()
    }

    /// Smart search for classes using word-based matching
    pub fn search_classes(&self, query: &str) -> Vec<String> {
        let query_lower = query.to_lowercase();
        let query_words: Vec<_> = Self::extract_words(query);
        
        if query_words.is_empty() {
            // If no valid words, do a simple substring search
            return self.class_names_lower.iter()
                .filter(|(_, lower)| lower.contains(&query_lower))
                .map(|(name, _)| name.clone())
                .collect();
        }
        
        // Find matches for each word
        let mut word_matches: Vec<HashSet<String>> = Vec::new();
        
        for word in query_words {
            // Get exact word matches
            if let Some(matches) = self.word_index.get(&word) {
                word_matches.push(matches.clone());
                continue;
            }
            
            // If no exact match, find partial word matches
            let partial_matches: HashSet<_> = self.word_index.iter()
                .filter(|(indexed_word, _)| indexed_word.contains(&word))
                .flat_map(|(_, classes)| classes.iter().cloned())
                .collect();
                
            if !partial_matches.is_empty() {
                word_matches.push(partial_matches);
            }
        }
        
        // Intersect all word matches
        let mut results = if let Some(first) = word_matches.first() {
            first.clone()
        } else {
            HashSet::new()
        };
        
        for matches in word_matches.iter().skip(1) {
            results.retain(|name| matches.contains(name));
        }
        
        // Final filtering to ensure complete match
        let mut results: Vec<_> = results.into_iter()
            .filter(|name| {
                self.class_names_lower.get(name)
                    .map_or(false, |lower| lower.contains(&query_lower))
            })
            .collect();
            
        // Sort results by relevance (exact matches first)
        results.sort_by(|a, b| {
            let a_lower = self.class_names_lower.get(a).unwrap();
            let b_lower = self.class_names_lower.get(b).unwrap();
            
            // Prioritize exact matches
            let a_exact = a_lower == &query_lower;
            let b_exact = b_lower == &query_lower;
            
            if a_exact != b_exact {
                return b_exact.cmp(&a_exact);
            }
            
            // Then prioritize starts-with matches
            let a_starts = a_lower.starts_with(&query_lower);
            let b_starts = b_lower.starts_with(&query_lower);
            
            if a_starts != b_starts {
                return b_starts.cmp(&a_starts);
            }
            
            // Finally sort by length and alphabetically
            a_lower.len().cmp(&b_lower.len())
                .then_with(|| a_lower.cmp(b_lower))
        });
        
        results
    }

    /// Get total number of indexed classes
    pub fn class_count(&self) -> usize {
        self.class_to_pbo.len()
    }
} 