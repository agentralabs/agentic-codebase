//! Programming language detection and enumeration.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Supported programming languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Language {
    /// Python (.py, .pyi)
    Python = 0,
    /// Rust (.rs)
    Rust = 1,
    /// TypeScript (.ts, .tsx)
    TypeScript = 2,
    /// JavaScript (.js, .jsx, .mjs, .cjs)
    JavaScript = 3,
    /// Go (.go)
    Go = 4,
    /// C++ (.cpp, .cc, .cxx, .h, .hpp, .hxx)
    Cpp = 5,
    /// Java (.java)
    Java = 6,
    /// C# (.cs)
    CSharp = 7,
    /// Unknown or unsupported language
    Unknown = 255,
}

impl Language {
    /// Convert from raw byte value.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Python),
            1 => Some(Self::Rust),
            2 => Some(Self::TypeScript),
            3 => Some(Self::JavaScript),
            4 => Some(Self::Go),
            5 => Some(Self::Cpp),
            6 => Some(Self::Java),
            7 => Some(Self::CSharp),
            255 => Some(Self::Unknown),
            _ => None,
        }
    }

    /// Detect language from file extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "py" | "pyi" => Self::Python,
            "rs" => Self::Rust,
            "ts" | "tsx" => Self::TypeScript,
            "js" | "jsx" | "mjs" | "cjs" => Self::JavaScript,
            "go" => Self::Go,
            "cpp" | "cc" | "cxx" | "c++" | "h" | "hpp" | "hxx" | "hh" => Self::Cpp,
            "java" => Self::Java,
            "cs" => Self::CSharp,
            _ => Self::Unknown,
        }
    }

    /// Detect language from file path.
    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(Self::from_extension)
            .unwrap_or(Self::Unknown)
    }

    /// Returns the tree-sitter language for this enum variant.
    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            Self::Python => Some(tree_sitter_python::language()),
            Self::Rust => Some(tree_sitter_rust::language()),
            Self::TypeScript => Some(tree_sitter_typescript::language_typescript()),
            Self::JavaScript => Some(tree_sitter_javascript::language()),
            Self::Go => Some(tree_sitter_go::language()),
            Self::Cpp => Some(tree_sitter_cpp::language()),
            Self::Java => Some(tree_sitter_java::language()),
            Self::CSharp => Some(tree_sitter_c_sharp::language()),
            Self::Unknown => None,
        }
    }

    /// Returns a human-readable name for the language.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Python => "Python",
            Self::Rust => "Rust",
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Go => "Go",
            Self::Cpp => "C++",
            Self::Java => "Java",
            Self::CSharp => "C#",
            Self::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
