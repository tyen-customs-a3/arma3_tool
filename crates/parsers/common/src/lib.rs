//! Common parser traits, infrastructure, and utilities
//! 
//! This crate provides the foundational traits and utilities that all
//! Arma3 parsers implement, ensuring consistent parsing interfaces.

use arma3_errors::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Common trait for all Arma3 parsers
pub trait Parser<T> {
    /// Parse content from a string
    fn parse_str(&self, content: &str) -> Result<T>;
    
    /// Parse content from a file
    fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<T> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| Error::io(format!("Failed to read file: {}", e)))?;
        self.parse_str(&content)
    }
    
    /// Get the file extensions this parser supports
    fn supported_extensions(&self) -> &[&str];
    
    /// Check if this parser can handle the given file
    fn can_parse<P: AsRef<Path>>(&self, path: P) -> bool {
        if let Some(ext) = path.as_ref().extension().and_then(|s| s.to_str()) {
            self.supported_extensions()
                .iter()
                .any(|&supported| ext.eq_ignore_ascii_case(supported))
        } else {
            false
        }
    }
}

/// Common trait for serializing parsed data back to string format
pub trait Serializer<T> {
    /// Serialize data to string format
    fn serialize(&self, data: &T) -> Result<String>;
    
    /// Serialize data to a file
    fn serialize_to_file<P: AsRef<Path>>(&self, data: &T, path: P) -> Result<()> {
        let content = self.serialize(data)?;
        std::fs::write(path.as_ref(), content)
            .map_err(|e| Error::io(format!("Failed to write file: {}", e)))?;
        Ok(())
    }
}

/// Position information for parse errors
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
    
    /// Create position from offset in text
    pub fn from_offset(text: &str, offset: usize) -> Self {
        let mut line = 1;
        let mut column = 1;
        
        for (i, ch) in text.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
        }
        
        Self { line, column, offset }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Span representing a range in the source text
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    
    /// Create a span from start and end offsets
    pub fn from_offsets(text: &str, start_offset: usize, end_offset: usize) -> Self {
        Self {
            start: Position::from_offset(text, start_offset),
            end: Position::from_offset(text, end_offset),
        }
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

/// Enhanced parse error with position information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseError {
    pub message: String,
    pub position: Option<Position>,
    pub span: Option<Span>,
    pub context: Option<String>,
}

impl ParseError {
    /// Create a new parse error with a message
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            position: None,
            span: None,
            context: None,
        }
    }
    
    /// Create a parse error with position information
    pub fn at_position<S: Into<String>>(message: S, position: Position) -> Self {
        Self {
            message: message.into(),
            position: Some(position),
            span: None,
            context: None,
        }
    }
    
    /// Create a parse error with span information
    pub fn at_span<S: Into<String>>(message: S, span: Span) -> Self {
        Self {
            message: message.into(),
            position: Some(span.start.clone()),
            span: Some(span),
            context: None,
        }
    }
    
    /// Add context to the parse error
    pub fn with_context<S: Into<String>>(mut self, context: S) -> Self {
        self.context = Some(context.into());
        self
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.message)?;
        
        if let Some(position) = &self.position {
            write!(f, " at {}", position)?;
        }
        
        if let Some(context) = &self.context {
            write!(f, " in {}", context)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for ParseError {}

/// Convert ParseError to our common Error type
impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Error::parse("", err.to_string())
    }
}

/// Common parsing utilities
pub mod utils {
    use super::*;
    
    /// Skip whitespace and comments in text
    pub fn skip_whitespace_and_comments(text: &str, mut pos: usize) -> usize {
        let chars: Vec<char> = text.chars().collect();
        
        while pos < chars.len() {
            // Skip whitespace
            if chars[pos].is_whitespace() {
                pos += 1;
                continue;
            }
            
            // Skip single-line comments (//)
            if pos + 1 < chars.len() && chars[pos] == '/' && chars[pos + 1] == '/' {
                // Skip to end of line
                while pos < chars.len() && chars[pos] != '\n' {
                    pos += 1;
                }
                continue;
            }
            
            // Skip multi-line comments (/* ... */)
            if pos + 1 < chars.len() && chars[pos] == '/' && chars[pos + 1] == '*' {
                pos += 2; // Skip /*
                while pos + 1 < chars.len() {
                    if chars[pos] == '*' && chars[pos + 1] == '/' {
                        pos += 2; // Skip */
                        break;
                    }
                    pos += 1;
                }
                continue;
            }
            
            break;
        }
        
        pos
    }
    
    /// Find matching closing bracket/brace/parenthesis
    pub fn find_matching_delimiter(text: &str, start: usize, open: char, close: char) -> Option<usize> {
        let chars: Vec<char> = text.chars().collect();
        let mut depth = 1;
        let mut pos = start + 1;
        
        while pos < chars.len() && depth > 0 {
            match chars[pos] {
                c if c == open => depth += 1,
                c if c == close => depth -= 1,
                _ => {}
            }
            pos += 1;
        }
        
        if depth == 0 {
            Some(pos - 1)
        } else {
            None
        }
    }
    
    /// Extract a quoted string, handling escape sequences
    pub fn parse_quoted_string(text: &str, start: usize) -> Result<(String, usize)> {
        let chars: Vec<char> = text.chars().collect();
        if start >= chars.len() || chars[start] != '"' {
            return Err(Error::parse("", "Expected quoted string"));
        }
        
        let mut result = String::new();
        let mut pos = start + 1; // Skip opening quote
        
        while pos < chars.len() {
            match chars[pos] {
                '"' => {
                    // End of string
                    return Ok((result, pos + 1));
                }
                '\\' if pos + 1 < chars.len() => {
                    // Escape sequence
                    pos += 1;
                    match chars[pos] {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '"' => result.push('"'),
                        c => {
                            result.push('\\');
                            result.push(c);
                        }
                    }
                }
                c => result.push(c),
            }
            pos += 1;
        }
        
        Err(Error::parse("", "Unterminated quoted string"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_position() {
        let text = "line1\nline2\nline3";
        let pos = Position::from_offset(text, 6); // Start of "line2"
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 1);
        assert_eq!(pos.offset, 6);
    }
    
    #[test]
    fn test_span() {
        let text = "hello\nworld";
        let span = Span::from_offsets(text, 0, 5);
        assert_eq!(span.start.line, 1);
        assert_eq!(span.end.line, 1);
        assert_eq!(span.end.column, 6);
    }
    
    #[test]
    fn test_parse_error() {
        let err = ParseError::new("test error");
        assert_eq!(err.message, "test error");
        assert!(err.position.is_none());
        
        let pos = Position::new(1, 5, 4);
        let err = ParseError::at_position("syntax error", pos);
        assert!(err.position.is_some());
    }
    
    #[test]
    fn test_skip_whitespace() {
        let text = "  \t\n  hello";
        let pos = utils::skip_whitespace_and_comments(text, 0);
        assert_eq!(pos, 6); // Position of 'h'
    }
    
    #[test]
    fn test_skip_comments() {
        let text = "// comment\nhello";
        let pos = utils::skip_whitespace_and_comments(text, 0);
        assert_eq!(pos, 11); // Position of 'h'
        
        let text = "/* multi\nline */hello";
        let pos = utils::skip_whitespace_and_comments(text, 0);
        assert_eq!(pos, 15); // Position of 'h'
    }
    
    #[test]
    fn test_find_matching_delimiter() {
        let text = "{ nested { } }";
        let pos = utils::find_matching_delimiter(text, 0, '{', '}');
        assert_eq!(pos, Some(13));
    }
    
    #[test]
    fn test_parse_quoted_string() {
        let text = r#""hello world""#;
        let (result, pos) = utils::parse_quoted_string(text, 0).unwrap();
        assert_eq!(result, "hello world");
        assert_eq!(pos, 13);
        
        let text = r#""escaped \"quote\"""#;
        let (result, _) = utils::parse_quoted_string(text, 0).unwrap();
        assert_eq!(result, r#"escaped "quote""#);
    }
}