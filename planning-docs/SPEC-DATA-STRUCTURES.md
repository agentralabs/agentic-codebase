# SPEC-DATA-STRUCTURES.md

> Every type used in the system. Implement these exactly as specified.

---

## Constants

```rust
// src/types/mod.rs

/// Magic bytes at the start of every .acb file
pub const ACB_MAGIC: [u8; 4] = [0x41, 0x43, 0x44, 0x42]; // "ACDB"

/// Current format version
pub const FORMAT_VERSION: u32 = 1;

/// Default feature vector dimensionality
pub const DEFAULT_DIMENSION: usize = 256;

/// Maximum symbol name length
pub const MAX_SYMBOL_NAME: usize = 1024;

/// Maximum qualified name length  
pub const MAX_QUALIFIED_NAME: usize = 4096;

/// Maximum file path length
pub const MAX_PATH_LENGTH: usize = 4096;

/// Maximum edges per code unit
pub const MAX_EDGES_PER_UNIT: u32 = 16384;

/// Maximum signature length
pub const MAX_SIGNATURE_LENGTH: usize = 2048;

/// Maximum doc summary length
pub const MAX_DOC_LENGTH: usize = 512;
```

---

## CodeUnitType Enum

```rust
// src/types/code_unit.rs

/// The type of code unit stored in a node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum CodeUnitType {
    /// A logical grouping (file, package, namespace, module)
    Module = 0,
    /// A named entity (function, class, variable, constant)
    Symbol = 1,
    /// A type definition (class, struct, interface, enum, type alias)
    Type = 2,
    /// A callable unit (function, method, closure)
    Function = 3,
    /// A function parameter or struct/class field
    Parameter = 4,
    /// A dependency declaration (import, require, use)
    Import = 5,
    /// A test case or test suite
    Test = 6,
    /// Documentation block (docstring, JSDoc, comment block)
    Doc = 7,
    /// Configuration value or constant
    Config = 8,
    /// An identified design pattern (Singleton, Factory, etc.)
    Pattern = 9,
    /// A trait, interface, or protocol definition
    Trait = 10,
    /// An implementation block (impl, class body)
    Impl = 11,
    /// A macro definition or invocation
    Macro = 12,
}

impl CodeUnitType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Module),
            1 => Some(Self::Symbol),
            2 => Some(Self::Type),
            3 => Some(Self::Function),
            4 => Some(Self::Parameter),
            5 => Some(Self::Import),
            6 => Some(Self::Test),
            7 => Some(Self::Doc),
            8 => Some(Self::Config),
            9 => Some(Self::Pattern),
            10 => Some(Self::Trait),
            11 => Some(Self::Impl),
            12 => Some(Self::Macro),
            _ => None,
        }
    }
    
    /// Returns true if this type represents a callable
    pub fn is_callable(&self) -> bool {
        matches!(self, Self::Function | Self::Macro)
    }
    
    /// Returns true if this type can have children
    pub fn is_container(&self) -> bool {
        matches!(self, Self::Module | Self::Type | Self::Trait | Self::Impl)
    }
}
```

---

## EdgeType Enum

```rust
// src/types/edge.rs

/// The type of relationship between two code units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum EdgeType {
    /// Runtime invocation: source calls target
    Calls = 0,
    /// Static dependency: source imports/uses target
    Imports = 1,
    /// Type hierarchy: source extends/inherits target
    Inherits = 2,
    /// Interface conformance: source implements target trait/interface
    Implements = 3,
    /// Method override: source overrides target method
    Overrides = 4,
    /// Structural containment: source contains target (module contains function)
    Contains = 5,
    /// Non-call reference: source references target without calling
    References = 6,
    /// Test coverage: source test covers target code
    Tests = 7,
    /// Documentation: source doc describes target
    Documents = 8,
    /// Configuration: source configures target
    Configures = 9,
    /// Hidden coupling: changes together >70% of time (from history)
    CouplesWith = 10,
    /// Breaking relationship: changing source historically breaks target
    BreaksWith = 11,
    /// Pattern instance: source is an instance of target pattern
    PatternOf = 12,
    /// Temporal: source is newer version of target
    VersionOf = 13,
    /// Cross-language: source binds to target across FFI
    FfiBinds = 14,
    /// Type relationship: source uses target as a type
    UsesType = 15,
    /// Return type: source returns target type
    Returns = 16,
    /// Parameter type: source has parameter of target type
    ParamType = 17,
}

impl EdgeType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Calls),
            1 => Some(Self::Imports),
            2 => Some(Self::Inherits),
            3 => Some(Self::Implements),
            4 => Some(Self::Overrides),
            5 => Some(Self::Contains),
            6 => Some(Self::References),
            7 => Some(Self::Tests),
            8 => Some(Self::Documents),
            9 => Some(Self::Configures),
            10 => Some(Self::CouplesWith),
            11 => Some(Self::BreaksWith),
            12 => Some(Self::PatternOf),
            13 => Some(Self::VersionOf),
            14 => Some(Self::FfiBinds),
            15 => Some(Self::UsesType),
            16 => Some(Self::Returns),
            17 => Some(Self::ParamType),
            _ => None,
        }
    }
    
    /// Returns true if this edge type indicates a dependency
    pub fn is_dependency(&self) -> bool {
        matches!(self, 
            Self::Calls | Self::Imports | Self::Inherits | 
            Self::Implements | Self::UsesType | Self::FfiBinds
        )
    }
    
    /// Returns true if this edge is derived from history analysis
    pub fn is_temporal(&self) -> bool {
        matches!(self, Self::CouplesWith | Self::BreaksWith | Self::VersionOf)
    }
}
```

---

## Language Enum

```rust
// src/types/language.rs

/// Supported programming languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Language {
    Python = 0,
    Rust = 1,
    TypeScript = 2,
    JavaScript = 3,
    Go = 4,
    Unknown = 255,
}

impl Language {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Python),
            1 => Some(Self::Rust),
            2 => Some(Self::TypeScript),
            3 => Some(Self::JavaScript),
            4 => Some(Self::Go),
            255 => Some(Self::Unknown),
            _ => None,
        }
    }
    
    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "py" | "pyi" => Self::Python,
            "rs" => Self::Rust,
            "ts" | "tsx" => Self::TypeScript,
            "js" | "jsx" | "mjs" | "cjs" => Self::JavaScript,
            "go" => Self::Go,
            _ => Self::Unknown,
        }
    }
    
    /// Detect language from file path
    pub fn from_path(path: &Path) -> Self {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(Self::from_extension)
            .unwrap_or(Self::Unknown)
    }
    
    /// Returns the tree-sitter language for this enum
    pub fn tree_sitter_language(&self) -> Option<tree_sitter::Language> {
        match self {
            Self::Python => Some(tree_sitter_python::language()),
            Self::Rust => Some(tree_sitter_rust::language()),
            Self::TypeScript => Some(tree_sitter_typescript::language_typescript()),
            Self::JavaScript => Some(tree_sitter_javascript::language()),
            Self::Go => Some(tree_sitter_go::language()),
            Self::Unknown => None,
        }
    }
}
```

---

## Visibility Enum

```rust
// src/types/code_unit.rs

/// Symbol visibility/accessibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Visibility {
    /// Accessible from anywhere
    Public = 0,
    /// Accessible within module/file
    Private = 1,
    /// Accessible within package/crate
    Internal = 2,
    /// Protected (subclass access)
    Protected = 3,
    /// Unknown visibility
    Unknown = 255,
}

impl Visibility {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Public),
            1 => Some(Self::Private),
            2 => Some(Self::Internal),
            3 => Some(Self::Protected),
            255 => Some(Self::Unknown),
            _ => None,
        }
    }
}
```

---

## Span Struct

```rust
// src/types/span.rs

/// A location range in source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    /// Starting line (1-indexed)
    pub start_line: u32,
    /// Starting column (0-indexed, byte offset)
    pub start_col: u32,
    /// Ending line (1-indexed)
    pub end_line: u32,
    /// Ending column (0-indexed, byte offset)
    pub end_col: u32,
}

impl Span {
    pub fn new(start_line: u32, start_col: u32, end_line: u32, end_col: u32) -> Self {
        Self { start_line, start_col, end_line, end_col }
    }
    
    pub fn point(line: u32, col: u32) -> Self {
        Self::new(line, col, line, col)
    }
    
    pub fn line_count(&self) -> u32 {
        self.end_line - self.start_line + 1
    }
    
    pub fn contains(&self, line: u32, col: u32) -> bool {
        if line < self.start_line || line > self.end_line {
            return false;
        }
        if line == self.start_line && col < self.start_col {
            return false;
        }
        if line == self.end_line && col > self.end_col {
            return false;
        }
        true
    }
}
```

---

## CodeUnit Struct (In-Memory Representation)

```rust
// src/types/code_unit.rs

/// A single code unit — the atomic element of the code graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeUnit {
    /// Unique identifier (assigned sequentially during compilation)
    pub id: u64,
    
    /// Type of code unit
    pub unit_type: CodeUnitType,
    
    /// Programming language
    pub language: Language,
    
    /// Simple name (e.g., "process_payment")
    pub name: String,
    
    /// Fully qualified name (e.g., "payments.stripe.process_payment")
    pub qualified_name: String,
    
    /// Source file path (relative to repo root)
    pub file_path: PathBuf,
    
    /// Location in source file
    pub span: Span,
    
    /// Type signature if applicable (e.g., "(amount: Decimal) -> bool")
    pub signature: Option<String>,
    
    /// First line of documentation
    pub doc_summary: Option<String>,
    
    // === Semantic metadata ===
    
    /// Visibility level
    pub visibility: Visibility,
    
    /// Cyclomatic complexity (0 for non-functions)
    pub complexity: u32,
    
    /// Is this async/await?
    pub is_async: bool,
    
    /// Is this a generator/iterator?
    pub is_generator: bool,
    
    // === Temporal metadata ===
    
    /// First seen timestamp (git commit time, or compile time if no git)
    pub created_at: u64,
    
    /// Last modified timestamp
    pub last_modified: u64,
    
    /// Total changes in git history
    pub change_count: u32,
    
    /// Stability score: 0.0 = constantly changing, 1.0 = never changes
    pub stability_score: f32,
    
    // === Collective metadata ===
    
    /// Global usage count from collective (0 if private code)
    pub collective_usage: u64,
    
    /// Content hash for deduplication
    pub content_hash: [u8; 32],
    
    // === Vector for semantic search ===
    
    /// Feature vector for similarity (dimension = DEFAULT_DIMENSION)
    pub feature_vec: Vec<f32>,
    
    // === Graph position (set by graph builder) ===
    
    /// Byte offset into edge table
    pub edge_offset: u64,
    
    /// Number of outgoing edges
    pub edge_count: u32,
}

impl CodeUnit {
    /// Create a new code unit with required fields only
    pub fn new(
        unit_type: CodeUnitType,
        language: Language,
        name: String,
        qualified_name: String,
        file_path: PathBuf,
        span: Span,
    ) -> Self {
        Self {
            id: 0, // Set by graph
            unit_type,
            language,
            name,
            qualified_name,
            file_path,
            span,
            signature: None,
            doc_summary: None,
            visibility: Visibility::Unknown,
            complexity: 0,
            is_async: false,
            is_generator: false,
            created_at: crate::types::now_micros(),
            last_modified: crate::types::now_micros(),
            change_count: 0,
            stability_score: 1.0,
            collective_usage: 0,
            content_hash: [0u8; 32],
            feature_vec: vec![0.0; DEFAULT_DIMENSION],
            edge_offset: 0,
            edge_count: 0,
        }
    }
}
```

---

## Edge Struct (In-Memory Representation)

```rust
// src/types/edge.rs

/// A directed relationship between two code units.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    /// Source code unit ID
    pub source_id: u64,
    
    /// Target code unit ID
    pub target_id: u64,
    
    /// Type of relationship
    pub edge_type: EdgeType,
    
    /// Relationship strength (0.0 = weak, 1.0 = strong)
    /// For temporal edges, this is the confidence/frequency
    pub weight: f32,
    
    /// When this edge was established
    pub created_at: u64,
    
    /// Additional context (e.g., call site line number)
    pub context: u32,
}

impl Edge {
    pub fn new(source_id: u64, target_id: u64, edge_type: EdgeType) -> Self {
        Self {
            source_id,
            target_id,
            edge_type,
            weight: 1.0,
            created_at: crate::types::now_micros(),
            context: 0,
        }
    }
    
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
        self
    }
    
    pub fn with_context(mut self, context: u32) -> Self {
        self.context = context;
        self
    }
}
```

---

## FileHeader Struct

```rust
// src/types/header.rs

/// Header of an .acb file. Fixed size: 128 bytes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FileHeader {
    /// Magic bytes: [0x41, 0x43, 0x44, 0x42] ("ACDB")
    pub magic: [u8; 4],
    
    /// Format version (currently 1)
    pub version: u32,
    
    /// Feature vector dimensionality
    pub dimension: u32,
    
    /// Number of supported languages in this file
    pub language_count: u32,
    
    /// Total number of code units
    pub unit_count: u64,
    
    /// Total number of edges
    pub edge_count: u64,
    
    /// Byte offset to code unit table
    pub unit_table_offset: u64,
    
    /// Byte offset to edge table
    pub edge_table_offset: u64,
    
    /// Byte offset to string pool
    pub string_pool_offset: u64,
    
    /// Byte offset to feature vectors
    pub feature_vec_offset: u64,
    
    /// Byte offset to temporal block
    pub temporal_offset: u64,
    
    /// Byte offset to index block
    pub index_offset: u64,
    
    /// Repository root path hash (for cache validation)
    pub repo_hash: [u8; 32],
    
    /// Compilation timestamp
    pub compiled_at: u64,
    
    /// Reserved for future use
    pub _reserved: [u8; 16],
}

// Total: 4 + 4 + 4 + 4 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 32 + 8 + 16 = 128 bytes
```

---

## Error Types

```rust
// src/types/error.rs

use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum AcbError {
    #[error("Invalid magic bytes in file header")]
    InvalidMagic,
    
    #[error("Unsupported format version: {0}")]
    UnsupportedVersion(u32),
    
    #[error("Code unit ID {0} not found")]
    UnitNotFound(u64),
    
    #[error("Edge references invalid code unit: {0}")]
    InvalidEdgeTarget(u64),
    
    #[error("Self-edge not allowed on unit {0}")]
    SelfEdge(u64),
    
    #[error("Symbol name too long: {len} > {max}")]
    NameTooLong { len: usize, max: usize },
    
    #[error("Path too long: {len} > {max}")]
    PathTooLong { len: usize, max: usize },
    
    #[error("Feature vector dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },
    
    #[error("Maximum edges per unit exceeded: {0}")]
    TooManyEdges(u32),
    
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),
    
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),
    
    #[error("Parse error in {path}: {message}")]
    ParseError { path: PathBuf, message: String },
    
    #[error("Semantic error: {0}")]
    SemanticError(String),
    
    #[error("Git error: {0}")]
    GitError(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Compression error: {0}")]
    Compression(String),
    
    #[error("File is empty or truncated")]
    Truncated,
    
    #[error("Corrupt data at offset {0}")]
    Corrupt(u64),
    
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Collective sync error: {0}")]
    CollectiveError(String),
}

pub type AcbResult<T> = Result<T, AcbError>;
```

---

## Timestamp Helper

```rust
// src/types/mod.rs

/// Returns the current time as Unix epoch microseconds.
pub fn now_micros() -> u64 {
    chrono::Utc::now().timestamp_micros() as u64
}
```

---

## On-Disk Record Formats

### CodeUnitRecord (96 bytes)

```rust
// src/format/writer.rs (internal)

#[repr(C, packed)]
struct CodeUnitRecord {
    id: u64,                    // 8 bytes
    unit_type: u8,              // 1 byte
    language: u8,               // 1 byte
    visibility: u8,             // 1 byte
    flags: u8,                  // 1 byte (is_async, is_generator, etc.)
    complexity: u32,            // 4 bytes
    
    name_offset: u64,           // 8 bytes (into string pool)
    name_len: u32,              // 4 bytes
    
    qualified_name_offset: u64, // 8 bytes
    qualified_name_len: u32,    // 4 bytes
    
    path_offset: u64,           // 8 bytes
    path_len: u32,              // 4 bytes
    
    span_start_line: u32,       // 4 bytes
    span_start_col: u32,        // 4 bytes
    span_end_line: u32,         // 4 bytes
    span_end_col: u32,          // 4 bytes
    
    created_at: u64,            // 8 bytes
    last_modified: u64,         // 8 bytes
    change_count: u32,          // 4 bytes
    stability_score_bits: u32,  // 4 bytes (f32 as bits)
    
    edge_offset: u64,           // 8 bytes
    edge_count: u32,            // 4 bytes
    _padding: u32,              // 4 bytes
}
// Total: 96 bytes
```

### EdgeRecord (40 bytes)

```rust
#[repr(C, packed)]
struct EdgeRecord {
    source_id: u64,             // 8 bytes
    target_id: u64,             // 8 bytes
    edge_type: u8,              // 1 byte
    _padding1: [u8; 3],         // 3 bytes
    weight_bits: u32,           // 4 bytes (f32 as bits)
    created_at: u64,            // 8 bytes
    context: u32,               // 4 bytes
    _padding2: u32,             // 4 bytes
}
// Total: 40 bytes
```

---

## Builder Pattern

```rust
// src/types/code_unit.rs

pub struct CodeUnitBuilder {
    inner: CodeUnit,
}

impl CodeUnitBuilder {
    pub fn new(
        unit_type: CodeUnitType,
        language: Language,
        name: impl Into<String>,
        qualified_name: impl Into<String>,
        file_path: impl Into<PathBuf>,
        span: Span,
    ) -> Self {
        Self {
            inner: CodeUnit::new(
                unit_type,
                language,
                name.into(),
                qualified_name.into(),
                file_path.into(),
                span,
            ),
        }
    }
    
    pub fn signature(mut self, sig: impl Into<String>) -> Self {
        self.inner.signature = Some(sig.into());
        self
    }
    
    pub fn doc(mut self, doc: impl Into<String>) -> Self {
        self.inner.doc_summary = Some(doc.into());
        self
    }
    
    pub fn visibility(mut self, vis: Visibility) -> Self {
        self.inner.visibility = vis;
        self
    }
    
    pub fn complexity(mut self, c: u32) -> Self {
        self.inner.complexity = c;
        self
    }
    
    pub fn async_fn(mut self) -> Self {
        self.inner.is_async = true;
        self
    }
    
    pub fn generator(mut self) -> Self {
        self.inner.is_generator = true;
        self
    }
    
    pub fn feature_vec(mut self, vec: Vec<f32>) -> Self {
        self.inner.feature_vec = vec;
        self
    }
    
    pub fn build(self) -> CodeUnit {
        self.inner
    }
}
```
