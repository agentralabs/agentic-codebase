# SPEC-SEMANTIC-ENGINE.md

> Transform syntax into meaning. Resolve references across files, trace FFI boundaries, detect patterns.

---

## Overview

The semantic engine takes raw parsed units and:
1. **Resolves** references across files (what does this import point to?)
2. **Traces** FFI boundaries (Python calling Rust calling C)
3. **Detects** design patterns (Singleton, Factory, Repository)
4. **Extracts** high-level concepts (authentication, payments, user management)
5. **Builds** the final CodeGraph with all edges

---

## Architecture

```
RawCodeUnit[]
    │
    ▼
┌─────────────────┐
│  Symbol Table   │
│  Builder        │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Reference      │
│  Resolver       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  FFI Tracer     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Pattern        │
│  Detector       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Concept        │
│  Extractor      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Graph Builder  │
└────────┬────────┘
         │
         ▼
    CodeGraph
```

---

## Semantic Analyzer

```rust
// src/semantic/analyzer.rs

pub struct SemanticAnalyzer {
    resolver: Resolver,
    ffi_tracer: FfiTracer,
    pattern_detector: PatternDetector,
    concept_extractor: ConceptExtractor,
}

impl SemanticAnalyzer {
    pub fn new() -> Self;
    
    /// Analyze raw units and produce a CodeGraph
    pub fn analyze(
        &self,
        raw_units: Vec<RawCodeUnit>,
        options: &AnalyzeOptions,
    ) -> AcbResult<CodeGraph> {
        // Phase 1: Build symbol table
        let mut symbol_table = self.build_symbol_table(&raw_units)?;
        
        // Phase 2: Resolve references
        let resolved = self.resolver.resolve_all(&raw_units, &symbol_table)?;
        
        // Phase 3: Trace FFI boundaries
        let ffi_edges = self.ffi_tracer.trace(&resolved, &symbol_table)?;
        
        // Phase 4: Detect patterns
        let patterns = self.pattern_detector.detect(&resolved)?;
        
        // Phase 5: Extract concepts
        let concepts = self.concept_extractor.extract(&resolved)?;
        
        // Phase 6: Build final graph
        self.build_graph(resolved, ffi_edges, patterns, concepts)
    }
}

pub struct AnalyzeOptions {
    /// Detect design patterns
    pub detect_patterns: bool,
    /// Extract high-level concepts
    pub extract_concepts: bool,
    /// Trace FFI boundaries
    pub trace_ffi: bool,
    /// External library hints (for better resolution)
    pub library_hints: Vec<LibraryHint>,
}

pub struct LibraryHint {
    pub name: String,
    pub language: Language,
    pub symbols: Vec<String>,
}
```

---

## Symbol Table

```rust
// src/semantic/resolver.rs

/// A hierarchical symbol table for name resolution
pub struct SymbolTable {
    /// Root scope (global/module level)
    root: Scope,
    /// Scope stack for current resolution context
    scope_stack: Vec<ScopeId>,
    /// All scopes by ID
    scopes: HashMap<ScopeId, Scope>,
    /// Symbol to unit ID mapping
    symbol_map: HashMap<QualifiedName, u64>,
    /// Import resolution cache
    import_cache: HashMap<ImportKey, ResolvedImport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QualifiedName {
    pub parts: Vec<String>,
}

impl QualifiedName {
    pub fn new(parts: Vec<String>) -> Self {
        Self { parts }
    }
    
    pub fn from_str(s: &str) -> Self {
        Self {
            parts: s.split('.').map(String::from).collect(),
        }
    }
    
    pub fn push(&mut self, part: String) {
        self.parts.push(part);
    }
    
    pub fn parent(&self) -> Option<Self> {
        if self.parts.len() > 1 {
            Some(Self {
                parts: self.parts[..self.parts.len()-1].to_vec(),
            })
        } else {
            None
        }
    }
    
    pub fn as_string(&self) -> String {
        self.parts.join(".")
    }
}

#[derive(Debug)]
pub struct Scope {
    pub id: ScopeId,
    pub kind: ScopeKind,
    pub parent: Option<ScopeId>,
    pub symbols: HashMap<String, SymbolEntry>,
    pub children: Vec<ScopeId>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScopeKind {
    Global,
    Module,
    Class,
    Function,
    Block,
}

#[derive(Debug)]
pub struct SymbolEntry {
    pub name: String,
    pub unit_id: u64,
    pub kind: CodeUnitType,
    pub visibility: Visibility,
}

impl SymbolTable {
    pub fn new() -> Self;
    
    /// Build symbol table from raw units
    pub fn build(units: &[RawCodeUnit]) -> AcbResult<Self> {
        let mut table = Self::new();
        
        // First pass: create scopes and register symbols
        for unit in units {
            table.register_unit(unit)?;
        }
        
        // Second pass: link parent-child relationships
        table.link_scopes(units)?;
        
        Ok(table)
    }
    
    /// Resolve a name in the current scope
    pub fn resolve(&self, name: &str, scope: ScopeId) -> Option<&SymbolEntry> {
        let mut current = Some(scope);
        
        while let Some(scope_id) = current {
            if let Some(scope) = self.scopes.get(&scope_id) {
                if let Some(entry) = scope.symbols.get(name) {
                    return Some(entry);
                }
                current = scope.parent;
            } else {
                break;
            }
        }
        
        None
    }
    
    /// Resolve a qualified name from root
    pub fn resolve_qualified(&self, qname: &QualifiedName) -> Option<&SymbolEntry> {
        let mut current_scope = &self.root;
        
        for (i, part) in qname.parts.iter().enumerate() {
            if i == qname.parts.len() - 1 {
                // Last part - look for symbol
                return current_scope.symbols.get(part);
            } else {
                // Intermediate part - look for child scope
                let child_id = current_scope.children.iter()
                    .find(|id| {
                        self.scopes.get(id)
                            .map(|s| s.symbols.contains_key(part))
                            .unwrap_or(false)
                    })?;
                current_scope = self.scopes.get(child_id)?;
            }
        }
        
        None
    }
}
```

---

## Reference Resolver

```rust
// src/semantic/resolver.rs

pub struct Resolver {
    /// Known external libraries and their symbols
    external_libs: HashMap<String, ExternalLibrary>,
}

#[derive(Debug)]
pub struct ExternalLibrary {
    pub name: String,
    pub language: Language,
    pub known_symbols: HashSet<String>,
    pub is_stdlib: bool,
}

impl Resolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            external_libs: HashMap::new(),
        };
        
        // Register standard libraries
        resolver.register_python_stdlib();
        resolver.register_rust_stdlib();
        resolver.register_node_builtins();
        
        resolver
    }
    
    /// Resolve all references in the raw units
    pub fn resolve_all(
        &self,
        units: &[RawCodeUnit],
        symbol_table: &SymbolTable,
    ) -> AcbResult<Vec<ResolvedUnit>> {
        let mut resolved = Vec::with_capacity(units.len());
        
        for unit in units {
            let resolved_refs = self.resolve_unit_references(unit, symbol_table)?;
            resolved.push(ResolvedUnit {
                unit: unit.clone(),
                resolved_refs,
            });
        }
        
        Ok(resolved)
    }
    
    fn resolve_unit_references(
        &self,
        unit: &RawCodeUnit,
        symbol_table: &SymbolTable,
    ) -> AcbResult<Vec<ResolvedReference>> {
        let mut resolved = Vec::new();
        
        for raw_ref in &unit.references {
            let resolution = self.resolve_reference(raw_ref, unit, symbol_table);
            resolved.push(ResolvedReference {
                raw: raw_ref.clone(),
                resolution,
            });
        }
        
        Ok(resolved)
    }
    
    fn resolve_reference(
        &self,
        raw_ref: &RawReference,
        unit: &RawCodeUnit,
        symbol_table: &SymbolTable,
    ) -> Resolution {
        // Try local resolution first
        if let Some(local) = self.resolve_local(&raw_ref.name, unit, symbol_table) {
            return Resolution::Local(local);
        }
        
        // Try imported symbols
        if let Some(imported) = self.resolve_imported(&raw_ref.name, unit, symbol_table) {
            return Resolution::Imported(imported);
        }
        
        // Try external libraries
        if let Some(external) = self.resolve_external(&raw_ref.name, unit.language) {
            return Resolution::External(external);
        }
        
        // Unresolved
        Resolution::Unresolved
    }
    
    fn resolve_local(
        &self,
        name: &str,
        unit: &RawCodeUnit,
        symbol_table: &SymbolTable,
    ) -> Option<u64> {
        // Look in current scope and parent scopes
        let scope = symbol_table.find_scope_for_unit(unit.temp_id)?;
        symbol_table.resolve(name, scope).map(|e| e.unit_id)
    }
    
    fn resolve_imported(
        &self,
        name: &str,
        unit: &RawCodeUnit,
        symbol_table: &SymbolTable,
    ) -> Option<ImportedSymbol> {
        // Check imports in this file
        // Match name against imported symbols
        // Resolve the import target
        todo!()
    }
    
    fn resolve_external(
        &self,
        name: &str,
        language: Language,
    ) -> Option<ExternalSymbol> {
        // Check if this is a known stdlib or common library symbol
        for lib in self.external_libs.values() {
            if lib.language == language && lib.known_symbols.contains(name) {
                return Some(ExternalSymbol {
                    library: lib.name.clone(),
                    symbol: name.to_string(),
                    is_stdlib: lib.is_stdlib,
                });
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct ResolvedUnit {
    pub unit: RawCodeUnit,
    pub resolved_refs: Vec<ResolvedReference>,
}

#[derive(Debug)]
pub struct ResolvedReference {
    pub raw: RawReference,
    pub resolution: Resolution,
}

#[derive(Debug)]
pub enum Resolution {
    /// Resolved to a local unit
    Local(u64),
    /// Resolved to an imported unit
    Imported(ImportedSymbol),
    /// Resolved to an external library
    External(ExternalSymbol),
    /// Could not resolve
    Unresolved,
}

#[derive(Debug)]
pub struct ImportedSymbol {
    pub unit_id: u64,
    pub import_path: String,
}

#[derive(Debug)]
pub struct ExternalSymbol {
    pub library: String,
    pub symbol: String,
    pub is_stdlib: bool,
}
```

---

## FFI Tracer

```rust
// src/semantic/ffi_tracer.rs

/// Traces function calls across language boundaries
pub struct FfiTracer {
    /// Known FFI patterns per language pair
    patterns: Vec<FfiPattern>,
}

#[derive(Debug)]
pub struct FfiPattern {
    pub source_lang: Language,
    pub target_lang: Language,
    pub pattern_type: FfiPatternType,
    pub detector: Box<dyn FfiDetector>,
}

#[derive(Debug, Clone, Copy)]
pub enum FfiPatternType {
    /// Python calling Rust via PyO3
    PyO3,
    /// Python calling C via ctypes
    Ctypes,
    /// Python calling C via cffi
    Cffi,
    /// Rust calling C via FFI
    RustCFfi,
    /// Node.js calling native via N-API
    NodeNapi,
    /// WebAssembly boundary
    Wasm,
    /// HTTP/RPC call
    HttpRpc,
}

pub trait FfiDetector: Send + Sync {
    fn detect(
        &self,
        unit: &ResolvedUnit,
        all_units: &[ResolvedUnit],
    ) -> Vec<FfiCall>;
}

#[derive(Debug)]
pub struct FfiCall {
    pub source_id: u64,
    pub target_id: Option<u64>,  // None if external
    pub ffi_type: FfiPatternType,
    pub binding_info: String,
}

impl FfiTracer {
    pub fn new() -> Self {
        let mut tracer = Self { patterns: Vec::new() };
        
        // Register known FFI patterns
        tracer.register_pyo3_pattern();
        tracer.register_ctypes_pattern();
        tracer.register_napi_pattern();
        
        tracer
    }
    
    pub fn trace(
        &self,
        units: &[ResolvedUnit],
        symbol_table: &SymbolTable,
    ) -> AcbResult<Vec<FfiEdge>> {
        let mut edges = Vec::new();
        
        for unit in units {
            for pattern in &self.patterns {
                if unit.unit.language == pattern.source_lang {
                    let calls = pattern.detector.detect(unit, units);
                    for call in calls {
                        edges.push(FfiEdge {
                            source_id: call.source_id,
                            target_id: call.target_id,
                            ffi_type: call.ffi_type,
                            binding_info: call.binding_info,
                        });
                    }
                }
            }
        }
        
        Ok(edges)
    }
    
    fn register_pyo3_pattern(&mut self) {
        // Detect #[pyfunction], #[pyclass] in Rust
        // Match to Python imports of the module
        self.patterns.push(FfiPattern {
            source_lang: Language::Python,
            target_lang: Language::Rust,
            pattern_type: FfiPatternType::PyO3,
            detector: Box::new(PyO3Detector),
        });
    }
}

#[derive(Debug)]
pub struct FfiEdge {
    pub source_id: u64,
    pub target_id: Option<u64>,
    pub ffi_type: FfiPatternType,
    pub binding_info: String,
}

struct PyO3Detector;

impl FfiDetector for PyO3Detector {
    fn detect(
        &self,
        unit: &ResolvedUnit,
        all_units: &[ResolvedUnit],
    ) -> Vec<FfiCall> {
        let mut calls = Vec::new();
        
        // Look for Python imports of native modules
        for ref_info in &unit.resolved_refs {
            if ref_info.raw.kind == ReferenceKind::Import {
                // Check if the import target is a Rust PyO3 module
                if let Some(rust_module) = self.find_pyo3_module(&ref_info.raw.name, all_units) {
                    calls.push(FfiCall {
                        source_id: unit.unit.temp_id,
                        target_id: Some(rust_module),
                        ffi_type: FfiPatternType::PyO3,
                        binding_info: format!("import {}", ref_info.raw.name),
                    });
                }
            }
        }
        
        calls
    }
}

impl PyO3Detector {
    fn find_pyo3_module(&self, name: &str, units: &[ResolvedUnit]) -> Option<u64> {
        // Look for Rust code with #[pymodule] that exports this name
        for unit in units {
            if unit.unit.language == Language::Rust {
                if let Some(meta) = unit.unit.metadata.get("pymodule") {
                    if meta == name {
                        return Some(unit.unit.temp_id);
                    }
                }
            }
        }
        None
    }
}
```

---

## Pattern Detector

```rust
// src/semantic/pattern_detector.rs

/// Detects common design patterns in code
pub struct PatternDetector {
    patterns: Vec<Box<dyn PatternMatcher>>,
}

pub trait PatternMatcher: Send + Sync {
    fn name(&self) -> &str;
    fn detect(&self, units: &[ResolvedUnit]) -> Vec<PatternInstance>;
}

#[derive(Debug)]
pub struct PatternInstance {
    pub pattern_name: String,
    pub primary_unit: u64,
    pub participating_units: Vec<u64>,
    pub confidence: f32,
}

impl PatternDetector {
    pub fn new() -> Self {
        let mut detector = Self { patterns: Vec::new() };
        
        detector.patterns.push(Box::new(SingletonMatcher));
        detector.patterns.push(Box::new(FactoryMatcher));
        detector.patterns.push(Box::new(RepositoryMatcher));
        detector.patterns.push(Box::new(DecoratorMatcher));
        detector.patterns.push(Box::new(ObserverMatcher));
        detector.patterns.push(Box::new(StrategyMatcher));
        
        detector
    }
    
    pub fn detect(&self, units: &[ResolvedUnit]) -> AcbResult<Vec<PatternInstance>> {
        let mut instances = Vec::new();
        
        for matcher in &self.patterns {
            instances.extend(matcher.detect(units));
        }
        
        Ok(instances)
    }
}

struct SingletonMatcher;

impl PatternMatcher for SingletonMatcher {
    fn name(&self) -> &str { "Singleton" }
    
    fn detect(&self, units: &[ResolvedUnit]) -> Vec<PatternInstance> {
        let mut instances = Vec::new();
        
        for unit in units {
            if unit.unit.unit_type != CodeUnitType::Type {
                continue;
            }
            
            // Check for singleton indicators:
            // 1. Private constructor
            // 2. Static instance field
            // 3. get_instance() or similar method
            // 4. __new__ override in Python
            
            let has_private_init = self.has_private_constructor(unit, units);
            let has_static_instance = self.has_static_instance_field(unit, units);
            let has_get_instance = self.has_get_instance_method(unit, units);
            
            let score = [has_private_init, has_static_instance, has_get_instance]
                .iter()
                .filter(|&&x| x)
                .count();
            
            if score >= 2 {
                instances.push(PatternInstance {
                    pattern_name: "Singleton".to_string(),
                    primary_unit: unit.unit.temp_id,
                    participating_units: vec![unit.unit.temp_id],
                    confidence: score as f32 / 3.0,
                });
            }
        }
        
        instances
    }
}

impl SingletonMatcher {
    fn has_private_constructor(&self, unit: &ResolvedUnit, all: &[ResolvedUnit]) -> bool {
        // Find __init__ or constructor and check visibility
        todo!()
    }
    
    fn has_static_instance_field(&self, unit: &ResolvedUnit, all: &[ResolvedUnit]) -> bool {
        todo!()
    }
    
    fn has_get_instance_method(&self, unit: &ResolvedUnit, all: &[ResolvedUnit]) -> bool {
        todo!()
    }
}

struct FactoryMatcher;
struct RepositoryMatcher;
struct DecoratorMatcher;
struct ObserverMatcher;
struct StrategyMatcher;

// Similar implementations for each pattern...
```

---

## Concept Extractor

```rust
// src/semantic/concept_extractor.rs

/// Extracts high-level concepts from code (authentication, payments, etc.)
pub struct ConceptExtractor {
    /// Concept definitions with keywords and patterns
    concepts: Vec<ConceptDefinition>,
}

#[derive(Debug)]
pub struct ConceptDefinition {
    pub name: String,
    pub keywords: Vec<String>,
    pub patterns: Vec<String>,
    pub typical_types: Vec<CodeUnitType>,
}

#[derive(Debug)]
pub struct ExtractedConcept {
    pub name: String,
    pub units: Vec<ConceptUnit>,
    pub confidence: f32,
}

#[derive(Debug)]
pub struct ConceptUnit {
    pub unit_id: u64,
    pub role: ConceptRole,
    pub score: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum ConceptRole {
    Definition,
    Implementation,
    Usage,
    Test,
}

impl ConceptExtractor {
    pub fn new() -> Self {
        let mut extractor = Self { concepts: Vec::new() };
        
        // Built-in concept definitions
        extractor.concepts.push(ConceptDefinition {
            name: "Authentication".to_string(),
            keywords: vec![
                "auth", "login", "logout", "session", "token", 
                "jwt", "oauth", "password", "credential",
            ].into_iter().map(String::from).collect(),
            patterns: vec![
                r"authenticate\w*".to_string(),
                r"verify_\w*".to_string(),
            ],
            typical_types: vec![CodeUnitType::Function, CodeUnitType::Type],
        });
        
        extractor.concepts.push(ConceptDefinition {
            name: "Payment".to_string(),
            keywords: vec![
                "payment", "charge", "refund", "transaction",
                "stripe", "paypal", "billing", "invoice",
            ].into_iter().map(String::from).collect(),
            patterns: vec![],
            typical_types: vec![CodeUnitType::Function, CodeUnitType::Type],
        });
        
        extractor.concepts.push(ConceptDefinition {
            name: "UserManagement".to_string(),
            keywords: vec![
                "user", "account", "profile", "registration",
                "signup", "settings", "preferences",
            ].into_iter().map(String::from).collect(),
            patterns: vec![],
            typical_types: vec![CodeUnitType::Type, CodeUnitType::Function],
        });
        
        // Add more concepts...
        
        extractor
    }
    
    pub fn extract(&self, units: &[ResolvedUnit]) -> AcbResult<Vec<ExtractedConcept>> {
        let mut extracted = Vec::new();
        
        for concept_def in &self.concepts {
            let mut concept_units = Vec::new();
            
            for unit in units {
                let score = self.score_unit_for_concept(unit, concept_def);
                if score > 0.3 {
                    concept_units.push(ConceptUnit {
                        unit_id: unit.unit.temp_id,
                        role: self.determine_role(unit, concept_def),
                        score,
                    });
                }
            }
            
            if !concept_units.is_empty() {
                let avg_score = concept_units.iter()
                    .map(|u| u.score)
                    .sum::<f32>() / concept_units.len() as f32;
                
                extracted.push(ExtractedConcept {
                    name: concept_def.name.clone(),
                    units: concept_units,
                    confidence: avg_score,
                });
            }
        }
        
        Ok(extracted)
    }
    
    fn score_unit_for_concept(
        &self,
        unit: &ResolvedUnit,
        concept: &ConceptDefinition,
    ) -> f32 {
        let mut score = 0.0;
        
        let name_lower = unit.unit.name.to_lowercase();
        let qname_lower = unit.unit.qualified_name.to_lowercase();
        
        // Keyword matching
        for keyword in &concept.keywords {
            if name_lower.contains(keyword) {
                score += 0.3;
            } else if qname_lower.contains(keyword) {
                score += 0.2;
            }
        }
        
        // Pattern matching
        for pattern in &concept.patterns {
            if let Ok(re) = regex::Regex::new(pattern) {
                if re.is_match(&name_lower) {
                    score += 0.3;
                }
            }
        }
        
        // Type bonus
        if concept.typical_types.contains(&unit.unit.unit_type) {
            score += 0.1;
        }
        
        score.min(1.0)
    }
    
    fn determine_role(
        &self,
        unit: &ResolvedUnit,
        concept: &ConceptDefinition,
    ) -> ConceptRole {
        match unit.unit.unit_type {
            CodeUnitType::Type | CodeUnitType::Trait => ConceptRole::Definition,
            CodeUnitType::Test => ConceptRole::Test,
            CodeUnitType::Function | CodeUnitType::Impl => ConceptRole::Implementation,
            _ => ConceptRole::Usage,
        }
    }
}
```

---

## Graph Builder

```rust
// src/semantic/analyzer.rs

impl SemanticAnalyzer {
    fn build_graph(
        &self,
        resolved: Vec<ResolvedUnit>,
        ffi_edges: Vec<FfiEdge>,
        patterns: Vec<PatternInstance>,
        concepts: Vec<ExtractedConcept>,
    ) -> AcbResult<CodeGraph> {
        let mut graph = CodeGraph::new(DEFAULT_DIMENSION);
        
        // Add all units
        let id_map = self.add_units_to_graph(&mut graph, &resolved)?;
        
        // Add edges from resolved references
        for unit in &resolved {
            for ref_info in &unit.resolved_refs {
                if let Some(edge) = self.create_edge_from_reference(
                    &ref_info,
                    unit.unit.temp_id,
                    &id_map,
                ) {
                    graph.add_edge(edge)?;
                }
            }
        }
        
        // Add FFI edges
        for ffi_edge in ffi_edges {
            let source = id_map.get(&ffi_edge.source_id).copied();
            let target = ffi_edge.target_id.and_then(|t| id_map.get(&t).copied());
            
            if let (Some(s), Some(t)) = (source, target) {
                graph.add_edge(Edge::new(s, t, EdgeType::FfiBinds))?;
            }
        }
        
        // Add pattern edges
        for pattern in patterns {
            let primary = id_map.get(&pattern.primary_unit).copied();
            if let Some(p) = primary {
                // Create a pattern node
                let pattern_id = graph.add_pattern_node(&pattern.pattern_name)?;
                graph.add_edge(Edge::new(p, pattern_id, EdgeType::PatternOf))?;
            }
        }
        
        Ok(graph)
    }
    
    fn create_edge_from_reference(
        &self,
        ref_info: &ResolvedReference,
        source_temp_id: u64,
        id_map: &HashMap<u64, u64>,
    ) -> Option<Edge> {
        let source_id = *id_map.get(&source_temp_id)?;
        
        match &ref_info.resolution {
            Resolution::Local(target_temp) => {
                let target_id = *id_map.get(target_temp)?;
                let edge_type = match ref_info.raw.kind {
                    ReferenceKind::Call => EdgeType::Calls,
                    ReferenceKind::Import => EdgeType::Imports,
                    ReferenceKind::TypeUse => EdgeType::UsesType,
                    ReferenceKind::Inherit => EdgeType::Inherits,
                    ReferenceKind::Implement => EdgeType::Implements,
                    ReferenceKind::Access => EdgeType::References,
                };
                Some(Edge::new(source_id, target_id, edge_type))
            }
            Resolution::Imported(imported) => {
                let target_id = *id_map.get(&imported.unit_id)?;
                Some(Edge::new(source_id, target_id, EdgeType::Imports))
            }
            Resolution::External(_) | Resolution::Unresolved => None,
        }
    }
}
```

---

## Performance Targets

| Operation | Target |
|-----------|--------|
| Symbol table build (100K units) | <500ms |
| Reference resolution (100K units) | <1s |
| FFI tracing | <100ms |
| Pattern detection | <200ms |
| Concept extraction | <200ms |
| Graph building | <500ms |
| **Total semantic analysis** | **<3s for 100K units** |
