use crate::GameClass;
use log::{debug, trace};
use regex::Regex;
use std::fs;
use std::path::Path;

/// A simple parser that scans for "class NAME" and "class NAME : PARENT" patterns
pub struct SimpleClassScanner {
    class_regex: Regex,
}

impl SimpleClassScanner {
    pub fn new() -> Self {
        // Pattern to match:
        // - Optional whitespace
        // - "class" keyword
        // - Whitespace
        // - Class name (capturing group 1)
        // - Optional whitespace
        // - Optional ":" followed by whitespace and parent class name (capturing group 2)
        // - Optional whitespace
        // - Either "{" or ";" (capturing group 3)
        let pattern = r"(?m)^\s*class\s+(\w+)(?:\s*:\s*(\w+))?\s*([{;])";
        Self {
            class_regex: Regex::new(pattern).unwrap(),
        }
    }

    pub fn scan_file(&self, file_path: &Path) -> Vec<GameClass> {
        debug!("Simple scanning file: {}", file_path.display());

        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                debug!("Failed to read file: {}", e);
                return Vec::new();
            }
        };

        let mut classes = Vec::new();

        for capture in self.class_regex.captures_iter(&content) {
            let name = capture.get(1).unwrap().as_str().to_string();
            let parent = capture.get(2).map(|m| m.as_str().to_string());
            let is_forward_declaration = capture.get(3).map_or(false, |m| m.as_str() == ";");

            trace!(
                "Found class: {} (parent: {:?}, forward: {})",
                name,
                parent,
                is_forward_declaration
            );

            let class = GameClass {
                name,
                parent,
                file_path: file_path.to_path_buf(),
                container_class: None,
                properties: Vec::new(),
                is_forward_declaration,
            };

            classes.push(class);
        }

        debug!("Found {} classes in {}", classes.len(), file_path.display());
        classes
    }
}

/// Parse a single file and return all classes found in it using the simple parser
pub fn parse_file_simple(file_path: &Path) -> Vec<GameClass> {
    debug!("Parsing file with simple parser: {}", file_path.display());
    let scanner = SimpleClassScanner::new();
    scanner.scan_file(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_basic_class_detection() {
        let content = r#"
class BaseMan {
    displayName = "Base";
};
class Rifleman : BaseMan {
    displayName = "Rifleman";
};
"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), content).unwrap();

        let scanner = SimpleClassScanner::new();
        let classes = scanner.scan_file(temp_file.path());

        assert_eq!(classes.len(), 2);
        assert_eq!(classes[0].name, "BaseMan");
        assert!(classes[0].parent.is_none());
        assert_eq!(classes[1].name, "Rifleman");
        assert_eq!(classes[1].parent.as_deref(), Some("BaseMan"));
    }

    #[test]
    fn test_includes_forward_declarations_with_flag() {
        let content = r#"
// Forward declarations
class ForwardDeclaredClass1;
class ForwardDeclaredClass2 : SomeBase;

// Regular class definition
class RealClass {
    value = 1;
};

// Another forward declaration with potential whitespace
    class AnotherForwardDecl ;

// Class inheriting from a forward-declared class
class DerivedClass : ForwardDeclaredClass1 {
    value = 2;
};
"#;

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), content).unwrap();

        let scanner = SimpleClassScanner::new();
        let classes = scanner.scan_file(temp_file.path());

        // Should find all 5 declarations
        assert_eq!(classes.len(), 5);

        let mut actual: Vec<_> = classes
            .iter()
            .map(|c| {
                (
                    c.name.as_str(),
                    c.is_forward_declaration,
                    c.parent.as_deref(),
                )
            })
            .collect();

        // Sort both vectors for consistent comparison as order isn't guaranteed
        actual.sort_by(|a, b| a.0.cmp(b.0));

        // Simple check for names first
        let names: Vec<&str> = classes.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"ForwardDeclaredClass1"));
        assert!(names.contains(&"ForwardDeclaredClass2"));
        assert!(names.contains(&"RealClass"));
        assert!(names.contains(&"AnotherForwardDecl"));
        assert!(names.contains(&"DerivedClass"));

        // Find and assert specific classes
        let fwd1 = classes
            .iter()
            .find(|c| c.name == "ForwardDeclaredClass1")
            .unwrap();
        assert!(fwd1.is_forward_declaration);
        assert!(fwd1.parent.is_none());

        let fwd2 = classes
            .iter()
            .find(|c| c.name == "ForwardDeclaredClass2")
            .unwrap();
        assert!(fwd2.is_forward_declaration);
        assert_eq!(fwd2.parent.as_deref(), Some("SomeBase"));

        let real = classes.iter().find(|c| c.name == "RealClass").unwrap();
        assert!(!real.is_forward_declaration);
        assert!(real.parent.is_none());

        let another_fwd = classes
            .iter()
            .find(|c| c.name == "AnotherForwardDecl")
            .unwrap();
        assert!(another_fwd.is_forward_declaration);
        assert!(another_fwd.parent.is_none());

        let derived = classes.iter().find(|c| c.name == "DerivedClass").unwrap();
        assert!(!derived.is_forward_declaration);
        assert_eq!(derived.parent.as_deref(), Some("ForwardDeclaredClass1"));
    }
}