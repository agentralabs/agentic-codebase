//! Multi-language parsing engine using tree-sitter.
//!
//! Converts source code into raw syntax information. One module per language.
//! No semantic analysis here — just syntax extraction.

pub mod cpp;
pub mod csharp;
pub mod go;
pub mod java;
pub mod parser;
pub mod python;
pub mod rust;
pub mod treesitter;
pub mod typescript;

pub use parser::{ParseOptions, ParseResult, ParseStats, Parser};

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::types::{AcbResult, CodeUnitType, Language, Span, Visibility};

/// A code unit extracted from parsing, before semantic analysis.
#[derive(Debug, Clone)]
pub struct RawCodeUnit {
    /// Temporary ID (reassigned during graph building).
    pub temp_id: u64,
    /// Type of code unit.
    pub unit_type: CodeUnitType,
    /// Programming language.
    pub language: Language,
    /// Simple name.
    pub name: String,
    /// Qualified name (may be partial, completed by semantic).
    pub qualified_name: String,
    /// Source file path.
    pub file_path: PathBuf,
    /// Location in source.
    pub span: Span,
    /// Type signature (raw, may need resolution).
    pub signature: Option<String>,
    /// Documentation.
    pub doc: Option<String>,
    /// Visibility.
    pub visibility: Visibility,
    /// Is async.
    pub is_async: bool,
    /// Is generator.
    pub is_generator: bool,
    /// Cyclomatic complexity.
    pub complexity: u32,
    /// Raw references found (names, not resolved IDs).
    pub references: Vec<RawReference>,
    /// Children temp_ids (for containers like modules, classes).
    pub children: Vec<u64>,
    /// Parent temp_id (for nested items).
    pub parent: Option<u64>,
    /// Language-specific metadata.
    pub metadata: HashMap<String, String>,
}

impl RawCodeUnit {
    /// Create a new raw code unit with minimal required fields.
    pub fn new(
        unit_type: CodeUnitType,
        language: Language,
        name: String,
        file_path: PathBuf,
        span: Span,
    ) -> Self {
        let qualified_name = name.clone();
        Self {
            temp_id: 0,
            unit_type,
            language,
            name,
            qualified_name,
            file_path,
            span,
            signature: None,
            doc: None,
            visibility: Visibility::Unknown,
            is_async: false,
            is_generator: false,
            complexity: 0,
            references: Vec::new(),
            children: Vec::new(),
            parent: None,
            metadata: HashMap::new(),
        }
    }
}

/// A raw reference found during parsing.
#[derive(Debug, Clone)]
pub struct RawReference {
    /// The name being referenced.
    pub name: String,
    /// The kind of reference.
    pub kind: ReferenceKind,
    /// Where in the source.
    pub span: Span,
}

/// The kind of a raw reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceKind {
    /// Import statement.
    Import,
    /// Function call.
    Call,
    /// Type usage.
    TypeUse,
    /// Inheritance.
    Inherit,
    /// Interface implementation.
    Implement,
    /// Attribute/field access.
    Access,
}

/// Parse error with severity.
#[derive(Debug, Clone)]
pub struct ParseFileError {
    /// File path.
    pub path: PathBuf,
    /// Location in source.
    pub span: Option<Span>,
    /// Error message.
    pub message: String,
    /// Severity level.
    pub severity: Severity,
}

/// Severity of a parse issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Could not parse at all.
    Error,
    /// Parsed but something odd.
    Warning,
    /// Informational.
    Info,
}

/// Trait for language-specific parsers.
pub trait LanguageParser: Send + Sync {
    /// Extract code units from a parsed tree.
    fn extract_units(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
        file_path: &Path,
    ) -> AcbResult<Vec<RawCodeUnit>>;

    /// Check if a file is a test file.
    fn is_test_file(&self, path: &Path, source: &str) -> bool;
}
