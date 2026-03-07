# SPEC-PARSING-ENGINE.md

> Multi-language parsing via tree-sitter with custom semantic extraction. Syntax in, structured data out.

---

## Overview

The parsing engine converts source files into raw syntax data that the semantic engine then processes. Each language has its own parser module that uses tree-sitter for syntax analysis and custom logic for language-specific extraction.

---

## Architecture

```
Source File
    │
    ▼
┌─────────────────┐
│  Language       │
│  Detection      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  tree-sitter    │
│  Parser         │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Language       │
│  Extractor      │
│  (Python/Rust/  │
│   TS/Go)        │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  RawCodeUnit    │
│  Collection     │
└─────────────────┘
```

---

## Core Parser Interface

```rust
// src/parse/parser.rs

/// Main parser orchestrator
pub struct Parser {
    parsers: HashMap<Language, LanguageParser>,
}

impl Parser {
    pub fn new() -> Self;
    
    /// Parse a single file
    pub fn parse_file(&self, path: &Path, content: &str) -> AcbResult<Vec<RawCodeUnit>>;
    
    /// Parse all files in a directory
    pub fn parse_directory(&self, 
        root: &Path, 
        options: &ParseOptions
    ) -> AcbResult<ParseResult>;
    
    /// Parse incrementally (only changed files)
    pub fn parse_incremental(&self,
        root: &Path,
        previous: &CodeGraph,
        changed_files: &[PathBuf],
    ) -> AcbResult<ParseResult>;
}

pub struct ParseOptions {
    /// Languages to include (empty = all)
    pub languages: Vec<Language>,
    /// Glob patterns to exclude
    pub exclude: Vec<String>,
    /// Include test files
    pub include_tests: bool,
    /// Maximum file size to parse (bytes)
    pub max_file_size: usize,
    /// Parallel parsing threads
    pub threads: usize,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            languages: vec![],
            exclude: vec![
                "**/node_modules/**".into(),
                "**/target/**".into(),
                "**/.git/**".into(),
                "**/__pycache__/**".into(),
                "**/venv/**".into(),
                "**/.venv/**".into(),
                "**/dist/**".into(),
                "**/build/**".into(),
            ],
            include_tests: true,
            max_file_size: 10 * 1024 * 1024, // 10MB
            threads: num_cpus::get(),
        }
    }
}

pub struct ParseResult {
    pub units: Vec<RawCodeUnit>,
    pub errors: Vec<ParseError>,
    pub stats: ParseStats,
}

pub struct ParseStats {
    pub files_parsed: usize,
    pub files_skipped: usize,
    pub files_errored: usize,
    pub total_lines: usize,
    pub parse_time_ms: u64,
    pub by_language: HashMap<Language, usize>,
}
```

---

## RawCodeUnit (Pre-Semantic)

```rust
// src/parse/mod.rs

/// A code unit extracted from parsing, before semantic analysis
#[derive(Debug, Clone)]
pub struct RawCodeUnit {
    /// Temporary ID (reassigned during graph building)
    pub temp_id: u64,
    
    /// Type of code unit
    pub unit_type: CodeUnitType,
    
    /// Programming language
    pub language: Language,
    
    /// Simple name
    pub name: String,
    
    /// Qualified name (may be partial, completed by semantic)
    pub qualified_name: String,
    
    /// Source file path
    pub file_path: PathBuf,
    
    /// Location in source
    pub span: Span,
    
    /// Type signature (raw, may need resolution)
    pub signature: Option<String>,
    
    /// Documentation
    pub doc: Option<String>,
    
    /// Visibility
    pub visibility: Visibility,
    
    /// Is async
    pub is_async: bool,
    
    /// Is generator
    pub is_generator: bool,
    
    /// Raw references found (names, not resolved IDs)
    pub references: Vec<RawReference>,
    
    /// Children (for containers like modules, classes)
    pub children: Vec<u64>,  // temp_ids
    
    /// Parent (for nested items)
    pub parent: Option<u64>, // temp_id
    
    /// Language-specific metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct RawReference {
    /// The name being referenced
    pub name: String,
    /// The kind of reference
    pub kind: ReferenceKind,
    /// Where in the source
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub enum ReferenceKind {
    Import,
    Call,
    TypeUse,
    Inherit,
    Implement,
    Access,
}
```

---

## Language Parser Trait

```rust
// src/parse/mod.rs

/// Trait for language-specific parsers
pub trait LanguageParser: Send + Sync {
    /// Get the tree-sitter language
    fn tree_sitter_language(&self) -> tree_sitter::Language;
    
    /// Extract code units from a parsed tree
    fn extract_units(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
        file_path: &Path,
    ) -> AcbResult<Vec<RawCodeUnit>>;
    
    /// Check if a file is a test file
    fn is_test_file(&self, path: &Path, source: &str) -> bool;
    
    /// Get language-specific query for finding definitions
    fn definition_query(&self) -> &str;
    
    /// Get language-specific query for finding references
    fn reference_query(&self) -> &str;
}
```

---

## Python Parser

```rust
// src/parse/python.rs

pub struct PythonParser {
    language: tree_sitter::Language,
    def_query: tree_sitter::Query,
    ref_query: tree_sitter::Query,
}

impl PythonParser {
    pub fn new() -> Self {
        let language = tree_sitter_python::language();
        
        // Query for definitions
        let def_query = tree_sitter::Query::new(
            language,
            r#"
            (module) @module
            
            (function_definition
                name: (identifier) @func.name
                parameters: (parameters) @func.params
                return_type: (type)? @func.return
                body: (block) @func.body
            ) @function
            
            (async_function_definition
                name: (identifier) @async_func.name
                parameters: (parameters) @async_func.params
                return_type: (type)? @async_func.return
            ) @async_function
            
            (class_definition
                name: (identifier) @class.name
                superclasses: (argument_list)? @class.bases
                body: (block) @class.body
            ) @class
            
            (decorated_definition
                (decorator)+ @decorators
            ) @decorated
            
            (import_statement) @import
            (import_from_statement) @import_from
            
            (assignment
                left: (identifier) @var.name
                type: (type)? @var.type
            ) @assignment
            "#,
        ).expect("Invalid Python definition query");
        
        // Query for references
        let ref_query = tree_sitter::Query::new(
            language,
            r#"
            (call
                function: (identifier) @call.name
            ) @call
            
            (call
                function: (attribute
                    object: (_) @call.object
                    attribute: (identifier) @call.method
                )
            ) @method_call
            
            (attribute
                object: (identifier) @attr.object
                attribute: (identifier) @attr.name
            ) @attribute
            "#,
        ).expect("Invalid Python reference query");
        
        Self { language, def_query, ref_query }
    }
}

impl LanguageParser for PythonParser {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        self.language
    }
    
    fn extract_units(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
        file_path: &Path,
    ) -> AcbResult<Vec<RawCodeUnit>> {
        let mut units = Vec::new();
        let mut cursor = tree_sitter::QueryCursor::new();
        
        // First pass: extract definitions
        let matches = cursor.matches(&self.def_query, tree.root_node(), source.as_bytes());
        
        for m in matches {
            match self.process_match(&m, source, file_path) {
                Ok(Some(unit)) => units.push(unit),
                Ok(None) => continue,
                Err(e) => log::warn!("Parse warning in {}: {}", file_path.display(), e),
            }
        }
        
        // Second pass: extract references (attached to units)
        self.extract_references(&mut units, tree, source)?;
        
        Ok(units)
    }
    
    fn is_test_file(&self, path: &Path, source: &str) -> bool {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        name.starts_with("test_") || 
        name.ends_with("_test.py") ||
        path.components().any(|c| c.as_os_str() == "tests") ||
        source.contains("import pytest") ||
        source.contains("import unittest")
    }
    
    fn definition_query(&self) -> &str {
        // Return the query string
        "..." 
    }
    
    fn reference_query(&self) -> &str {
        "..."
    }
}

impl PythonParser {
    fn process_match(
        &self,
        m: &tree_sitter::QueryMatch,
        source: &str,
        file_path: &Path,
    ) -> AcbResult<Option<RawCodeUnit>> {
        // Extract based on capture names
        // Build RawCodeUnit from match data
        todo!()
    }
    
    fn extract_references(
        &self,
        units: &mut [RawCodeUnit],
        tree: &tree_sitter::Tree,
        source: &str,
    ) -> AcbResult<()> {
        // For each unit, find references within its span
        todo!()
    }
    
    fn extract_docstring(&self, node: tree_sitter::Node, source: &str) -> Option<String> {
        // Look for first string in function/class body
        let body = node.child_by_field_name("body")?;
        let first_stmt = body.child(0)?;
        
        if first_stmt.kind() == "expression_statement" {
            let expr = first_stmt.child(0)?;
            if expr.kind() == "string" {
                let text = expr.utf8_text(source.as_bytes()).ok()?;
                // Strip quotes and clean up
                return Some(self.clean_docstring(text));
            }
        }
        None
    }
    
    fn clean_docstring(&self, raw: &str) -> String {
        // Remove triple quotes, dedent, take first line
        let trimmed = raw.trim_matches(|c| c == '"' || c == '\'');
        trimmed.lines().next().unwrap_or("").trim().to_string()
    }
    
    fn extract_visibility(&self, node: tree_sitter::Node, name: &str) -> Visibility {
        // Python convention: _private, __very_private, public
        if name.starts_with("__") && !name.ends_with("__") {
            Visibility::Private
        } else if name.starts_with("_") {
            Visibility::Internal
        } else {
            Visibility::Public
        }
    }
    
    fn calculate_complexity(&self, node: tree_sitter::Node, source: &str) -> u32 {
        // Count decision points: if, elif, for, while, and, or, try, except
        let mut complexity = 1; // Base complexity
        let mut cursor = node.walk();
        
        for child in node.descendants(&mut cursor) {
            match child.kind() {
                "if_statement" | "elif_clause" | "for_statement" | 
                "while_statement" | "try_statement" | "except_clause" |
                "with_statement" | "assert_statement" => complexity += 1,
                "boolean_operator" => complexity += 1,
                "conditional_expression" => complexity += 1, // ternary
                _ => {}
            }
        }
        
        complexity
    }
}
```

---

## Rust Parser

```rust
// src/parse/rust.rs

pub struct RustParser {
    language: tree_sitter::Language,
    def_query: tree_sitter::Query,
    ref_query: tree_sitter::Query,
}

impl RustParser {
    pub fn new() -> Self {
        let language = tree_sitter_rust::language();
        
        let def_query = tree_sitter::Query::new(
            language,
            r#"
            (function_item
                name: (identifier) @func.name
                parameters: (parameters) @func.params
                return_type: (_)? @func.return
            ) @function
            
            (struct_item
                name: (type_identifier) @struct.name
            ) @struct
            
            (enum_item
                name: (type_identifier) @enum.name
            ) @enum
            
            (trait_item
                name: (type_identifier) @trait.name
            ) @trait
            
            (impl_item
                trait: (type_identifier)? @impl.trait
                type: (type_identifier) @impl.type
            ) @impl
            
            (mod_item
                name: (identifier) @mod.name
            ) @mod
            
            (use_declaration) @use
            
            (macro_definition
                name: (identifier) @macro.name
            ) @macro
            
            (const_item
                name: (identifier) @const.name
            ) @const
            
            (static_item
                name: (identifier) @static.name
            ) @static
            "#,
        ).expect("Invalid Rust definition query");
        
        let ref_query = tree_sitter::Query::new(
            language,
            r#"
            (call_expression
                function: (identifier) @call.name
            ) @call
            
            (call_expression
                function: (field_expression
                    value: (_) @call.receiver
                    field: (field_identifier) @call.method
                )
            ) @method_call
            
            (macro_invocation
                macro: (identifier) @macro.name
            ) @macro_call
            "#,
        ).expect("Invalid Rust reference query");
        
        Self { language, def_query, ref_query }
    }
}

impl LanguageParser for RustParser {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        self.language
    }
    
    fn extract_units(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
        file_path: &Path,
    ) -> AcbResult<Vec<RawCodeUnit>> {
        let mut units = Vec::new();
        // Similar structure to Python parser
        todo!()
    }
    
    fn is_test_file(&self, path: &Path, source: &str) -> bool {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        path.components().any(|c| c.as_os_str() == "tests") ||
        name.ends_with("_test.rs") ||
        source.contains("#[cfg(test)]") ||
        source.contains("#[test]")
    }
    
    fn definition_query(&self) -> &str { "..." }
    fn reference_query(&self) -> &str { "..." }
}

impl RustParser {
    fn extract_visibility(&self, node: tree_sitter::Node) -> Visibility {
        // Check for pub, pub(crate), pub(super), etc.
        if let Some(vis) = node.child_by_field_name("visibility") {
            let text = vis.utf8_text(&[]).unwrap_or("");
            match text {
                "pub" => Visibility::Public,
                s if s.contains("crate") => Visibility::Internal,
                s if s.contains("super") => Visibility::Protected,
                _ => Visibility::Private,
            }
        } else {
            Visibility::Private
        }
    }
    
    fn extract_doc_comment(&self, node: tree_sitter::Node, source: &str) -> Option<String> {
        // Look for preceding /// or //! comments
        let mut current = node;
        while let Some(prev) = current.prev_sibling() {
            if prev.kind() == "line_comment" {
                let text = prev.utf8_text(source.as_bytes()).ok()?;
                if text.starts_with("///") || text.starts_with("//!") {
                    return Some(text[3..].trim().to_string());
                }
            }
            current = prev;
        }
        None
    }
}
```

---

## TypeScript Parser

```rust
// src/parse/typescript.rs

pub struct TypeScriptParser {
    language: tree_sitter::Language,
    jsx_language: tree_sitter::Language,
    def_query: tree_sitter::Query,
    ref_query: tree_sitter::Query,
}

impl TypeScriptParser {
    pub fn new() -> Self {
        let language = tree_sitter_typescript::language_typescript();
        let jsx_language = tree_sitter_typescript::language_tsx();
        
        let def_query = tree_sitter::Query::new(
            language,
            r#"
            (function_declaration
                name: (identifier) @func.name
                parameters: (formal_parameters) @func.params
                return_type: (type_annotation)? @func.return
            ) @function
            
            (arrow_function
                parameters: (formal_parameters) @arrow.params
                return_type: (type_annotation)? @arrow.return
            ) @arrow
            
            (class_declaration
                name: (type_identifier) @class.name
                (class_heritage)? @class.heritage
            ) @class
            
            (interface_declaration
                name: (type_identifier) @interface.name
            ) @interface
            
            (type_alias_declaration
                name: (type_identifier) @type.name
            ) @type_alias
            
            (enum_declaration
                name: (identifier) @enum.name
            ) @enum
            
            (import_statement) @import
            (export_statement) @export
            
            (method_definition
                name: (property_identifier) @method.name
            ) @method
            "#,
        ).expect("Invalid TypeScript definition query");
        
        let ref_query = tree_sitter::Query::new(
            language,
            r#"
            (call_expression
                function: (identifier) @call.name
            ) @call
            
            (call_expression
                function: (member_expression
                    object: (_) @call.object
                    property: (property_identifier) @call.property
                )
            ) @member_call
            
            (new_expression
                constructor: (identifier) @new.name
            ) @new
            "#,
        ).expect("Invalid TypeScript reference query");
        
        Self { language, jsx_language, def_query, ref_query }
    }
    
    fn get_parser_for_file(&self, path: &Path) -> tree_sitter::Language {
        match path.extension().and_then(|e| e.to_str()) {
            Some("tsx") | Some("jsx") => self.jsx_language,
            _ => self.language,
        }
    }
}

impl LanguageParser for TypeScriptParser {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        self.language
    }
    
    fn extract_units(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
        file_path: &Path,
    ) -> AcbResult<Vec<RawCodeUnit>> {
        todo!()
    }
    
    fn is_test_file(&self, path: &Path, source: &str) -> bool {
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        
        name.ends_with(".test.ts") ||
        name.ends_with(".test.tsx") ||
        name.ends_with(".spec.ts") ||
        name.ends_with(".spec.tsx") ||
        path.components().any(|c| {
            let s = c.as_os_str().to_str().unwrap_or("");
            s == "__tests__" || s == "tests" || s == "test"
        }) ||
        source.contains("describe(") ||
        source.contains("it(") ||
        source.contains("test(")
    }
    
    fn definition_query(&self) -> &str { "..." }
    fn reference_query(&self) -> &str { "..." }
}

impl TypeScriptParser {
    fn detect_react_component(&self, node: tree_sitter::Node, source: &str) -> bool {
        // Check if this is a React component
        // - Returns JSX
        // - Name is PascalCase
        // - Has props parameter or uses hooks
        todo!()
    }
}
```

---

## Go Parser

```rust
// src/parse/go.rs

pub struct GoParser {
    language: tree_sitter::Language,
    def_query: tree_sitter::Query,
    ref_query: tree_sitter::Query,
}

impl GoParser {
    pub fn new() -> Self {
        let language = tree_sitter_go::language();
        
        let def_query = tree_sitter::Query::new(
            language,
            r#"
            (function_declaration
                name: (identifier) @func.name
                parameters: (parameter_list) @func.params
                result: (_)? @func.result
            ) @function
            
            (method_declaration
                receiver: (parameter_list) @method.receiver
                name: (field_identifier) @method.name
            ) @method
            
            (type_declaration
                (type_spec
                    name: (type_identifier) @type.name
                    type: (_) @type.def
                )
            ) @type
            
            (import_declaration) @import
            
            (package_clause
                (package_identifier) @package.name
            ) @package
            
            (const_declaration) @const
            (var_declaration) @var
            "#,
        ).expect("Invalid Go definition query");
        
        let ref_query = tree_sitter::Query::new(
            language,
            r#"
            (call_expression
                function: (identifier) @call.name
            ) @call
            
            (call_expression
                function: (selector_expression
                    operand: (_) @call.object
                    field: (field_identifier) @call.field
                )
            ) @selector_call
            "#,
        ).expect("Invalid Go reference query");
        
        Self { language, def_query, ref_query }
    }
}

impl LanguageParser for GoParser {
    fn tree_sitter_language(&self) -> tree_sitter::Language {
        self.language
    }
    
    fn extract_units(
        &self,
        tree: &tree_sitter::Tree,
        source: &str,
        file_path: &Path,
    ) -> AcbResult<Vec<RawCodeUnit>> {
        todo!()
    }
    
    fn is_test_file(&self, path: &Path, _source: &str) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.ends_with("_test.go"))
            .unwrap_or(false)
    }
    
    fn definition_query(&self) -> &str { "..." }
    fn reference_query(&self) -> &str { "..." }
}
```

---

## tree-sitter Utilities

```rust
// src/parse/treesitter.rs

/// Helper functions for working with tree-sitter

pub fn get_node_text<'a>(node: tree_sitter::Node, source: &'a str) -> &'a str {
    &source[node.byte_range()]
}

pub fn node_to_span(node: tree_sitter::Node) -> Span {
    let start = node.start_position();
    let end = node.end_position();
    Span::new(
        start.row as u32 + 1,
        start.column as u32,
        end.row as u32 + 1,
        end.column as u32,
    )
}

pub fn find_child_by_kind<'a>(
    node: tree_sitter::Node<'a>,
    kind: &str,
) -> Option<tree_sitter::Node<'a>> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == kind {
            return Some(child);
        }
    }
    None
}

pub fn collect_children_by_kind<'a>(
    node: tree_sitter::Node<'a>,
    kind: &str,
) -> Vec<tree_sitter::Node<'a>> {
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter(|c| c.kind() == kind)
        .collect()
}

/// Parse with error recovery - tree-sitter handles malformed code gracefully
pub fn parse_with_recovery(
    parser: &mut tree_sitter::Parser,
    source: &str,
) -> AcbResult<tree_sitter::Tree> {
    parser.parse(source, None)
        .ok_or_else(|| AcbError::ParseError {
            path: PathBuf::new(),
            message: "Failed to parse source".into(),
        })
}
```

---

## Parallel Parsing

```rust
// src/parse/parser.rs

impl Parser {
    pub fn parse_directory_parallel(
        &self,
        root: &Path,
        options: &ParseOptions,
    ) -> AcbResult<ParseResult> {
        use rayon::prelude::*;
        
        // Collect files to parse
        let files = self.collect_files(root, options)?;
        
        // Parse in parallel
        let results: Vec<_> = files
            .par_iter()
            .map(|path| {
                let content = std::fs::read_to_string(path)?;
                let language = Language::from_path(path);
                
                if let Some(parser) = self.parsers.get(&language) {
                    self.parse_file_with_parser(path, &content, parser)
                } else {
                    Ok(vec![])
                }
            })
            .collect();
        
        // Aggregate results
        let mut all_units = Vec::new();
        let mut all_errors = Vec::new();
        
        for result in results {
            match result {
                Ok(units) => all_units.extend(units),
                Err(e) => all_errors.push(e),
            }
        }
        
        Ok(ParseResult {
            units: all_units,
            errors: all_errors,
            stats: self.compute_stats(&all_units),
        })
    }
    
    fn collect_files(&self, root: &Path, options: &ParseOptions) -> AcbResult<Vec<PathBuf>> {
        use ignore::WalkBuilder;
        
        let mut files = Vec::new();
        
        let walker = WalkBuilder::new(root)
            .hidden(true)
            .git_ignore(true)
            .build();
        
        for entry in walker {
            let entry = entry.map_err(|e| AcbError::Io(e.into()))?;
            let path = entry.path();
            
            if !path.is_file() {
                continue;
            }
            
            let language = Language::from_path(path);
            if language == Language::Unknown {
                continue;
            }
            
            // Check language filter
            if !options.languages.is_empty() && !options.languages.contains(&language) {
                continue;
            }
            
            // Check exclude patterns
            if self.is_excluded(path, &options.exclude) {
                continue;
            }
            
            // Check file size
            if let Ok(meta) = path.metadata() {
                if meta.len() as usize > options.max_file_size {
                    log::warn!("Skipping large file: {}", path.display());
                    continue;
                }
            }
            
            files.push(path.to_path_buf());
        }
        
        Ok(files)
    }
}
```

---

## Error Handling

```rust
#[derive(Debug)]
pub struct ParseError {
    pub path: PathBuf,
    pub span: Option<Span>,
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Error,   // Couldn't parse at all
    Warning, // Parsed but something odd
    Info,    // Informational
}
```

Tree-sitter is error-tolerant — it will parse what it can and mark error nodes. We collect these but continue processing:

```rust
fn check_for_errors(tree: &tree_sitter::Tree, path: &Path) -> Vec<ParseError> {
    let mut errors = Vec::new();
    
    fn visit(node: tree_sitter::Node, errors: &mut Vec<ParseError>, path: &Path) {
        if node.is_error() || node.is_missing() {
            errors.push(ParseError {
                path: path.to_path_buf(),
                span: Some(node_to_span(node)),
                message: format!("Syntax error: {}", node.kind()),
                severity: Severity::Warning,
            });
        }
        
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            visit(child, errors, path);
        }
    }
    
    visit(tree.root_node(), &mut errors, path);
    errors
}
```

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Parse 1K LOC file | <50ms |
| Parse 10K files parallel | <10s |
| Memory per file | <10MB peak |
| tree-sitter init per language | <10ms |
