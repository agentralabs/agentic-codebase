//! WASM bindings for the agentic-codebase query engine.
//!
//! This crate provides a pure-Rust, WASM-compatible reimplementation of the
//! agentic-codebase graph query engine. It loads pre-compiled graph data from
//! JSON (exported by the native `acb` CLI) and exposes query operations via
//! `wasm_bindgen`.
//!
//! Tree-sitter parsing and filesystem operations are NOT included — this crate
//! is query-only, operating on pre-built graph data.

use std::collections::{HashMap, HashSet, VecDeque};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// ============================================================================
// Core types — mirrors agentic_codebase::types
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeUnitType {
    Module,
    Symbol,
    Type,
    Function,
    Parameter,
    Import,
    Test,
    Doc,
    Config,
    Pattern,
    Trait,
    Impl,
    Macro,
}

impl CodeUnitType {
    fn label(&self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Symbol => "symbol",
            Self::Type => "type",
            Self::Function => "function",
            Self::Parameter => "parameter",
            Self::Import => "import",
            Self::Test => "test",
            Self::Doc => "doc",
            Self::Config => "config",
            Self::Pattern => "pattern",
            Self::Trait => "trait",
            Self::Impl => "impl",
            Self::Macro => "macro",
        }
    }

    fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "module" => Some(Self::Module),
            "symbol" => Some(Self::Symbol),
            "type" => Some(Self::Type),
            "function" => Some(Self::Function),
            "parameter" => Some(Self::Parameter),
            "import" => Some(Self::Import),
            "test" => Some(Self::Test),
            "doc" => Some(Self::Doc),
            "config" => Some(Self::Config),
            "pattern" => Some(Self::Pattern),
            "trait" => Some(Self::Trait),
            "impl" => Some(Self::Impl),
            "macro" => Some(Self::Macro),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    Calls,
    Imports,
    Inherits,
    Implements,
    Overrides,
    Contains,
    References,
    Tests,
    Documents,
    Configures,
    CouplesWith,
    BreaksWith,
    PatternOf,
    VersionOf,
    FfiBinds,
    UsesType,
    Returns,
    ParamType,
}

impl EdgeType {
    fn label(&self) -> &'static str {
        match self {
            Self::Calls => "calls",
            Self::Imports => "imports",
            Self::Inherits => "inherits",
            Self::Implements => "implements",
            Self::Overrides => "overrides",
            Self::Contains => "contains",
            Self::References => "references",
            Self::Tests => "tests",
            Self::Documents => "documents",
            Self::Configures => "configures",
            Self::CouplesWith => "couples_with",
            Self::BreaksWith => "breaks_with",
            Self::PatternOf => "pattern_of",
            Self::VersionOf => "version_of",
            Self::FfiBinds => "ffi_binds",
            Self::UsesType => "uses_type",
            Self::Returns => "returns",
            Self::ParamType => "param_type",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Language {
    Python,
    Rust,
    TypeScript,
    JavaScript,
    Go,
    Unknown,
}

impl Language {
    fn name(&self) -> &'static str {
        match self {
            Self::Python => "Python",
            Self::Rust => "Rust",
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Go => "Go",
            Self::Unknown => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Public,
    Private,
    Internal,
    Protected,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub start_line: u32,
    pub start_col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeUnit {
    pub id: u64,
    pub unit_type: CodeUnitType,
    pub language: Language,
    pub name: String,
    pub qualified_name: String,
    pub file_path: String,
    pub span: Span,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub doc_summary: Option<String>,
    #[serde(default = "default_visibility")]
    pub visibility: Visibility,
    #[serde(default)]
    pub complexity: u32,
    #[serde(default)]
    pub is_async: bool,
    #[serde(default)]
    pub is_generator: bool,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub last_modified: u64,
    #[serde(default)]
    pub change_count: u32,
    #[serde(default = "default_stability")]
    pub stability_score: f32,
    #[serde(default)]
    pub collective_usage: u64,
    #[serde(default)]
    pub edge_offset: u64,
    #[serde(default)]
    pub edge_count: u32,
}

fn default_visibility() -> Visibility {
    Visibility::Unknown
}

fn default_stability() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source_id: u64,
    pub target_id: u64,
    pub edge_type: EdgeType,
    #[serde(default = "default_weight")]
    pub weight: f32,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub context: u32,
}

fn default_weight() -> f32 {
    1.0
}

// ============================================================================
// Graph — in-memory graph with indexes
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedGraph {
    #[serde(default)]
    pub units: Vec<CodeUnit>,
    #[serde(default)]
    pub edges: Vec<Edge>,
    #[serde(default = "default_dimension")]
    pub dimension: usize,
}

fn default_dimension() -> usize {
    256
}

#[allow(dead_code)]
struct CodeGraph {
    units: Vec<CodeUnit>,
    edges: Vec<Edge>,
    edges_by_source: HashMap<u64, Vec<usize>>,
    edges_by_target: HashMap<u64, Vec<usize>>,
    dimension: usize,
    languages: HashSet<Language>,
}

impl CodeGraph {
    fn from_serialized(sg: SerializedGraph) -> Self {
        let mut edges_by_source: HashMap<u64, Vec<usize>> = HashMap::new();
        let mut edges_by_target: HashMap<u64, Vec<usize>> = HashMap::new();
        let mut languages = HashSet::new();

        for unit in &sg.units {
            languages.insert(unit.language);
        }

        for (idx, edge) in sg.edges.iter().enumerate() {
            edges_by_source
                .entry(edge.source_id)
                .or_default()
                .push(idx);
            edges_by_target
                .entry(edge.target_id)
                .or_default()
                .push(idx);
        }

        Self {
            units: sg.units,
            edges: sg.edges,
            edges_by_source,
            edges_by_target,
            dimension: sg.dimension,
            languages,
        }
    }

    fn unit_count(&self) -> usize {
        self.units.len()
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn get_unit(&self, id: u64) -> Option<&CodeUnit> {
        self.units.get(id as usize)
    }

    fn edges_from(&self, source_id: u64) -> Vec<&Edge> {
        self.edges_by_source
            .get(&source_id)
            .map(|indices| indices.iter().map(|&i| &self.edges[i]).collect())
            .unwrap_or_default()
    }

    fn edges_to(&self, target_id: u64) -> Vec<&Edge> {
        self.edges_by_target
            .get(&target_id)
            .map(|indices| indices.iter().map(|&i| &self.edges[i]).collect())
            .unwrap_or_default()
    }

    fn find_units_by_name_prefix(&self, prefix: &str) -> Vec<&CodeUnit> {
        let prefix_lower = prefix.to_lowercase();
        self.units
            .iter()
            .filter(|u| u.name.to_lowercase().starts_with(&prefix_lower))
            .collect()
    }

    fn find_units_by_exact_name(&self, name: &str) -> Vec<&CodeUnit> {
        self.units.iter().filter(|u| u.name == name).collect()
    }

    fn find_units_by_contains(&self, substr: &str) -> Vec<&CodeUnit> {
        let substr_lower = substr.to_lowercase();
        self.units
            .iter()
            .filter(|u| u.name.to_lowercase().contains(&substr_lower))
            .collect()
    }

    fn find_units_by_type(&self, unit_type: CodeUnitType) -> Vec<&CodeUnit> {
        self.units
            .iter()
            .filter(|u| u.unit_type == unit_type)
            .collect()
    }
}

// ============================================================================
// Levenshtein distance for fuzzy matching
// ============================================================================

fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut prev = vec![0usize; n + 1];
    let mut curr = vec![0usize; n + 1];

    for j in 0..=n {
        prev[j] = j;
    }

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            curr[j] = (prev[j] + 1)
                .min(curr[j - 1] + 1)
                .min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}

// ============================================================================
// Query result types (JSON-serializable)
// ============================================================================

#[derive(Serialize)]
struct UnitInfo {
    id: u64,
    unit_type: String,
    language: String,
    name: String,
    qualified_name: String,
    file_path: String,
    span: SpanInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    doc_summary: Option<String>,
    visibility: String,
    complexity: u32,
    is_async: bool,
    stability_score: f32,
}

#[derive(Serialize)]
struct SpanInfo {
    start_line: u32,
    start_col: u32,
    end_line: u32,
    end_col: u32,
}

impl From<&CodeUnit> for UnitInfo {
    fn from(u: &CodeUnit) -> Self {
        Self {
            id: u.id,
            unit_type: u.unit_type.label().to_string(),
            language: u.language.name().to_string(),
            name: u.name.clone(),
            qualified_name: u.qualified_name.clone(),
            file_path: u.file_path.clone(),
            span: SpanInfo {
                start_line: u.span.start_line,
                start_col: u.span.start_col,
                end_line: u.span.end_line,
                end_col: u.span.end_col,
            },
            signature: u.signature.clone(),
            doc_summary: u.doc_summary.clone(),
            visibility: format!("{:?}", u.visibility).to_lowercase(),
            complexity: u.complexity,
            is_async: u.is_async,
            stability_score: u.stability_score,
        }
    }
}

#[derive(Serialize)]
struct EdgeInfo {
    source_id: u64,
    target_id: u64,
    edge_type: String,
    weight: f32,
}

impl From<&Edge> for EdgeInfo {
    fn from(e: &Edge) -> Self {
        Self {
            source_id: e.source_id,
            target_id: e.target_id,
            edge_type: e.edge_type.label().to_string(),
            weight: e.weight,
        }
    }
}

#[derive(Serialize)]
struct StatsResult {
    unit_count: usize,
    edge_count: usize,
    dimension: usize,
    type_counts: HashMap<String, usize>,
    edge_type_counts: HashMap<String, usize>,
    language_counts: HashMap<String, usize>,
}

#[derive(Serialize)]
struct ImpactNode {
    unit_id: u64,
    name: String,
    unit_type: String,
    depth: u32,
    risk_score: f32,
    has_tests: bool,
}

#[derive(Serialize)]
struct ImpactResult {
    root_id: u64,
    impacted: Vec<ImpactNode>,
    overall_risk: f32,
}

// ============================================================================
// WASM-exported wrapper
// ============================================================================

/// A code graph loaded in WASM memory, supporting query operations.
///
/// Create via `WasmCodeGraph.from_json(jsonStr)` where `jsonStr` is a JSON
/// object with `{ units: [...], edges: [...], dimension: N }`.
#[wasm_bindgen]
pub struct WasmCodeGraph {
    graph: CodeGraph,
}

#[wasm_bindgen]
impl WasmCodeGraph {
    /// Load a code graph from a JSON string.
    ///
    /// The JSON must have the shape:
    /// ```json
    /// {
    ///   "units": [ { "id": 0, "unit_type": "function", ... }, ... ],
    ///   "edges": [ { "source_id": 0, "target_id": 1, "edge_type": "calls", ... }, ... ],
    ///   "dimension": 256
    /// }
    /// ```
    #[wasm_bindgen(js_name = "fromJson")]
    pub fn from_json(json_str: &str) -> Result<WasmCodeGraph, JsError> {
        let sg: SerializedGraph =
            serde_json::from_str(json_str).map_err(|e| JsError::new(&format!("JSON parse error: {}", e)))?;
        Ok(Self {
            graph: CodeGraph::from_serialized(sg),
        })
    }

    /// Look up symbols by name.
    ///
    /// `mode` is one of: "exact", "prefix", "contains", "fuzzy".
    /// Returns a JSON array of matching units.
    #[wasm_bindgen(js_name = "lookupSymbol")]
    pub fn lookup_symbol(
        &self,
        name: &str,
        mode: &str,
        limit: Option<usize>,
    ) -> Result<String, JsError> {
        let limit = limit.unwrap_or(10);

        let matches: Vec<&CodeUnit> = match mode {
            "exact" => self.graph.find_units_by_exact_name(name),
            "prefix" => self.graph.find_units_by_name_prefix(name),
            "contains" => self.graph.find_units_by_contains(name),
            "fuzzy" => {
                let name_lower = name.to_lowercase();
                let mut scored: Vec<(&CodeUnit, usize)> = self
                    .graph
                    .units
                    .iter()
                    .map(|u| (u, levenshtein(&u.name.to_lowercase(), &name_lower)))
                    .filter(|(_, dist)| *dist <= 2)
                    .collect();
                scored.sort_by_key(|(_, dist)| *dist);
                scored.into_iter().map(|(u, _)| u).collect()
            }
            _ => {
                return Err(JsError::new(
                    "Invalid mode. Use: exact, prefix, contains, fuzzy",
                ))
            }
        };

        let results: Vec<UnitInfo> = matches
            .into_iter()
            .take(limit)
            .map(UnitInfo::from)
            .collect();

        serde_json::to_string(&results)
            .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
    }

    /// Analyse the impact of changing a code unit.
    ///
    /// Performs a reverse-dependency BFS from the given unit, tracking which
    /// units would be affected by a change. Returns JSON with impacted units,
    /// depth, risk scores, and test coverage info.
    #[wasm_bindgen(js_name = "impactAnalysis")]
    pub fn impact_analysis(
        &self,
        unit_id: u64,
        max_depth: Option<u32>,
    ) -> Result<String, JsError> {
        let max_depth = max_depth.unwrap_or(3);

        // Validate the unit exists
        if self.graph.get_unit(unit_id).is_none() {
            return Err(JsError::new(&format!("Unit {} not found", unit_id)));
        }

        // BFS backward (reverse dependencies)
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut impacted = Vec::new();

        visited.insert(unit_id);
        queue.push_back((unit_id, 0u32));

        while let Some((current_id, depth)) = queue.pop_front() {
            if depth > 0 {
                let unit = self.graph.get_unit(current_id);
                let name = unit.map(|u| u.name.clone()).unwrap_or_default();
                let unit_type = unit
                    .map(|u| u.unit_type.label().to_string())
                    .unwrap_or_default();

                // Check if this unit has test coverage
                let has_tests = self
                    .graph
                    .edges_to(current_id)
                    .iter()
                    .any(|e| e.edge_type == EdgeType::Tests);

                // Risk score: higher for shallow depth, lower with test coverage
                let depth_factor = 1.0 / (depth as f32);
                let test_factor = if has_tests { 0.5 } else { 1.0 };
                let risk_score = (depth_factor * test_factor).min(1.0);

                impacted.push(ImpactNode {
                    unit_id: current_id,
                    name,
                    unit_type,
                    depth,
                    risk_score,
                    has_tests,
                });
            }

            if depth < max_depth {
                // Follow reverse edges (who depends on current_id)
                for edge in self.graph.edges_to(current_id) {
                    if edge.edge_type == EdgeType::Tests
                        || edge.edge_type == EdgeType::Documents
                    {
                        continue; // skip non-dependency edges
                    }
                    if visited.insert(edge.source_id) {
                        queue.push_back((edge.source_id, depth + 1));
                    }
                }
            }
        }

        // Overall risk
        let overall_risk = if impacted.is_empty() {
            0.0
        } else {
            let sum: f32 = impacted.iter().map(|i| i.risk_score).sum();
            (sum / impacted.len() as f32).min(1.0)
        };

        let result = ImpactResult {
            root_id: unit_id,
            impacted,
            overall_risk,
        };

        serde_json::to_string(&result)
            .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
    }

    /// Get summary statistics about the loaded graph.
    ///
    /// Returns JSON with unit count, edge count, type distribution, etc.
    #[wasm_bindgen(js_name = "stats")]
    pub fn stats(&self) -> Result<String, JsError> {
        let mut type_counts: HashMap<String, usize> = HashMap::new();
        let mut edge_type_counts: HashMap<String, usize> = HashMap::new();
        let mut lang_counts: HashMap<String, usize> = HashMap::new();

        for unit in &self.graph.units {
            *type_counts
                .entry(unit.unit_type.label().to_string())
                .or_default() += 1;
            *lang_counts
                .entry(unit.language.name().to_string())
                .or_default() += 1;
        }
        for edge in &self.graph.edges {
            *edge_type_counts
                .entry(edge.edge_type.label().to_string())
                .or_default() += 1;
        }

        let result = StatsResult {
            unit_count: self.graph.unit_count(),
            edge_count: self.graph.edge_count(),
            dimension: self.graph.dimension,
            type_counts,
            edge_type_counts,
            language_counts: lang_counts,
        };

        serde_json::to_string(&result)
            .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
    }

    /// List code units, optionally filtered by type.
    ///
    /// `type_filter` is an optional unit type string (e.g. "function", "module").
    /// Returns a JSON array of units.
    #[wasm_bindgen(js_name = "listUnits")]
    pub fn list_units(
        &self,
        type_filter: Option<String>,
        limit: Option<usize>,
    ) -> Result<String, JsError> {
        let limit = limit.unwrap_or(50);

        let units: Vec<UnitInfo> = if let Some(ref tf) = type_filter {
            let unit_type = CodeUnitType::from_str_loose(tf)
                .ok_or_else(|| JsError::new(&format!("Unknown unit type: {}", tf)))?;
            self.graph
                .find_units_by_type(unit_type)
                .into_iter()
                .take(limit)
                .map(UnitInfo::from)
                .collect()
        } else {
            self.graph
                .units
                .iter()
                .take(limit)
                .map(UnitInfo::from)
                .collect()
        };

        serde_json::to_string(&units)
            .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
    }

    /// Get a single unit by ID.
    ///
    /// Returns JSON representation of the unit, or an error if not found.
    #[wasm_bindgen(js_name = "getUnit")]
    pub fn get_unit(&self, id: u64) -> Result<String, JsError> {
        let unit = self
            .graph
            .get_unit(id)
            .ok_or_else(|| JsError::new(&format!("Unit {} not found", id)))?;
        let info = UnitInfo::from(unit);
        serde_json::to_string(&info)
            .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
    }

    /// Get all edges from a given unit.
    ///
    /// Returns JSON array of edges originating from the given unit.
    #[wasm_bindgen(js_name = "edgesFrom")]
    pub fn edges_from(&self, unit_id: u64) -> Result<String, JsError> {
        let edges: Vec<EdgeInfo> = self
            .graph
            .edges_from(unit_id)
            .iter()
            .map(|e| EdgeInfo::from(*e))
            .collect();
        serde_json::to_string(&edges)
            .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
    }

    /// Get all edges targeting a given unit.
    ///
    /// Returns JSON array of edges pointing to the given unit.
    #[wasm_bindgen(js_name = "edgesTo")]
    pub fn edges_to(&self, unit_id: u64) -> Result<String, JsError> {
        let edges: Vec<EdgeInfo> = self
            .graph
            .edges_to(unit_id)
            .iter()
            .map(|e| EdgeInfo::from(*e))
            .collect();
        serde_json::to_string(&edges)
            .map_err(|e| JsError::new(&format!("Serialization error: {}", e)))
    }

    /// Find the shortest path between two units using BFS.
    ///
    /// Returns JSON array of unit IDs forming the shortest path, or null if
    /// no path exists.
    #[wasm_bindgen(js_name = "shortestPath")]
    pub fn shortest_path(&self, from_id: u64, to_id: u64) -> Result<String, JsError> {
        if from_id == to_id {
            return serde_json::to_string(&vec![from_id])
                .map_err(|e| JsError::new(&format!("Serialization error: {}", e)));
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent: HashMap<u64, u64> = HashMap::new();

        visited.insert(from_id);
        queue.push_back(from_id);

        while let Some(current) = queue.pop_front() {
            for edge in self.graph.edges_from(current) {
                if visited.insert(edge.target_id) {
                    parent.insert(edge.target_id, current);
                    if edge.target_id == to_id {
                        let mut path = vec![to_id];
                        let mut node = to_id;
                        while let Some(&p) = parent.get(&node) {
                            path.push(p);
                            node = p;
                        }
                        path.reverse();
                        return serde_json::to_string(&path).map_err(|e| {
                            JsError::new(&format!("Serialization error: {}", e))
                        });
                    }
                    queue.push_back(edge.target_id);
                }
            }
        }

        // No path found
        Ok("null".to_string())
    }

    /// Get the total number of units in the graph.
    #[wasm_bindgen(js_name = "unitCount")]
    pub fn unit_count(&self) -> usize {
        self.graph.unit_count()
    }

    /// Get the total number of edges in the graph.
    #[wasm_bindgen(js_name = "edgeCount")]
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}
