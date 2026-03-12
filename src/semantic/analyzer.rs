//! Main semantic analyzer.
//!
//! Orchestrates the semantic analysis pipeline: build symbol table,
//! resolve references, trace FFI, detect patterns, extract concepts,
//! and build the final CodeGraph.

use std::collections::HashMap;

use crate::graph::CodeGraph;
use crate::parse::{RawCodeUnit, ReferenceKind};
use crate::types::{
    AcbResult, CodeUnit, CodeUnitType, Edge, EdgeType, Language, DEFAULT_DIMENSION,
};

use super::concept_extractor::{ConceptExtractor, ExtractedConcept};
use super::ffi_tracer::{FfiEdge, FfiTracer};
use super::pattern_detector::{PatternDetector, PatternInstance};
use super::resolver::{Resolution, ResolvedUnit, Resolver, SymbolTable};

/// Options for the semantic analysis pass.
#[derive(Debug, Clone)]
pub struct AnalyzeOptions {
    /// Detect design patterns.
    pub detect_patterns: bool,
    /// Extract high-level concepts.
    pub extract_concepts: bool,
    /// Trace FFI boundaries.
    pub trace_ffi: bool,
}

impl Default for AnalyzeOptions {
    fn default() -> Self {
        Self {
            detect_patterns: true,
            extract_concepts: true,
            trace_ffi: true,
        }
    }
}

/// The main semantic analysis orchestrator.
pub struct SemanticAnalyzer {
    resolver: Resolver,
    ffi_tracer: FfiTracer,
    pattern_detector: PatternDetector,
    concept_extractor: ConceptExtractor,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer.
    pub fn new() -> Self {
        Self {
            resolver: Resolver::new(),
            ffi_tracer: FfiTracer::new(),
            pattern_detector: PatternDetector::new(),
            concept_extractor: ConceptExtractor::new(),
        }
    }

    /// Analyze raw units and produce a CodeGraph.
    pub fn analyze(
        &self,
        raw_units: Vec<RawCodeUnit>,
        options: &AnalyzeOptions,
    ) -> AcbResult<CodeGraph> {
        self.analyze_with_progress(raw_units, options, |_step, _total_steps| {})
    }

    /// Analyze with coarse-grained phase progress callbacks.
    pub fn analyze_with_progress<F>(
        &self,
        raw_units: Vec<RawCodeUnit>,
        options: &AnalyzeOptions,
        mut on_progress: F,
    ) -> AcbResult<CodeGraph>
    where
        F: FnMut(usize, usize),
    {
        const TOTAL_STEPS: usize = 6;
        on_progress(0, TOTAL_STEPS);

        // Phase 1: Build symbol table
        let symbol_table = SymbolTable::build(&raw_units)?;
        on_progress(1, TOTAL_STEPS);

        // Phase 2: Resolve references
        let resolved = self.resolver.resolve_all(&raw_units, &symbol_table)?;
        on_progress(2, TOTAL_STEPS);

        // Phase 3: Trace FFI boundaries
        let ffi_edges = if options.trace_ffi {
            self.ffi_tracer.trace(&resolved)?
        } else {
            Vec::new()
        };
        on_progress(3, TOTAL_STEPS);

        // Phase 4: Detect patterns
        let patterns = if options.detect_patterns {
            self.pattern_detector.detect(&resolved)?
        } else {
            Vec::new()
        };
        on_progress(4, TOTAL_STEPS);

        // Phase 5: Extract concepts
        let concepts = if options.extract_concepts {
            self.concept_extractor.extract(&resolved)?
        } else {
            Vec::new()
        };
        on_progress(5, TOTAL_STEPS);

        // Phase 6: Build final graph
        let graph = self.build_graph(resolved, ffi_edges, patterns, concepts)?;
        on_progress(6, TOTAL_STEPS);
        Ok(graph)
    }

    /// Build the CodeGraph from resolved units and analysis results.
    fn build_graph(
        &self,
        resolved: Vec<ResolvedUnit>,
        ffi_edges: Vec<FfiEdge>,
        patterns: Vec<PatternInstance>,
        _concepts: Vec<ExtractedConcept>,
    ) -> AcbResult<CodeGraph> {
        let mut graph = CodeGraph::new(DEFAULT_DIMENSION);

        // Map from temp_id → graph_id
        let mut id_map: HashMap<u64, u64> = HashMap::new();

        // Add all units to the graph
        for runit in &resolved {
            let raw = &runit.unit;
            let code_unit = CodeUnit::new(
                raw.unit_type,
                raw.language,
                raw.name.clone(),
                raw.qualified_name.clone(),
                raw.file_path.clone(),
                raw.span,
            );
            let mut cu = code_unit;
            cu.signature = raw.signature.clone();
            cu.doc_summary = raw.doc.clone();
            cu.visibility = raw.visibility;
            cu.complexity = raw.complexity;
            cu.is_async = raw.is_async;
            cu.is_generator = raw.is_generator;

            let graph_id = graph.add_unit(cu);
            id_map.insert(raw.temp_id, graph_id);
        }

        // Add edges from resolved references
        for runit in &resolved {
            let source_temp = runit.unit.temp_id;
            let source_graph_id = match id_map.get(&source_temp) {
                Some(&id) => id,
                None => continue,
            };

            for ref_info in &runit.resolved_refs {
                match &ref_info.resolution {
                    Resolution::Local(target_temp) => {
                        if let Some(&target_graph_id) = id_map.get(target_temp) {
                            if source_graph_id == target_graph_id {
                                continue; // skip self-edges
                            }
                            let edge_type = reference_to_edge_type(ref_info.raw.kind);
                            let edge = Edge::new(source_graph_id, target_graph_id, edge_type);
                            // Ignore duplicate edge errors
                            let _ = graph.add_edge(edge);
                        }
                    }
                    Resolution::Imported(imported) => {
                        if let Some(&target_graph_id) = id_map.get(&imported.unit_id) {
                            if source_graph_id == target_graph_id {
                                continue;
                            }
                            let edge =
                                Edge::new(source_graph_id, target_graph_id, EdgeType::Imports);
                            let _ = graph.add_edge(edge);
                        }
                    }
                    Resolution::External(_) | Resolution::Unresolved => {
                        // No edges to external/unresolved
                    }
                }
            }
        }

        // Add containment edges: module → its children (by file co-location)
        self.add_containment_edges(&resolved, &id_map, &mut graph);

        // Add FFI edges
        for ffi_edge in ffi_edges {
            if let Some(&source_id) = id_map.get(&ffi_edge.source_id) {
                if let Some(target_temp) = ffi_edge.target_id {
                    if let Some(&target_id) = id_map.get(&target_temp) {
                        if source_id != target_id {
                            let edge = Edge::new(source_id, target_id, EdgeType::FfiBinds);
                            let _ = graph.add_edge(edge);
                        }
                    }
                }
            }
        }

        // Add pattern edges
        for pattern in patterns {
            if let Some(&primary_id) = id_map.get(&pattern.primary_unit) {
                // Create a pattern node
                let pattern_unit = CodeUnit::new(
                    CodeUnitType::Pattern,
                    Language::Unknown,
                    pattern.pattern_name.clone(),
                    format!("pattern::{}", pattern.pattern_name),
                    std::path::PathBuf::new(),
                    crate::types::Span::point(0, 0),
                );
                let pattern_graph_id = graph.add_unit(pattern_unit);
                let edge = Edge::new(primary_id, pattern_graph_id, EdgeType::PatternOf);
                let _ = graph.add_edge(edge);
            }
        }

        Ok(graph)
    }

    /// Add containment edges: module → functions/classes/etc. in the same file.
    fn add_containment_edges(
        &self,
        resolved: &[ResolvedUnit],
        id_map: &HashMap<u64, u64>,
        graph: &mut CodeGraph,
    ) {
        // Group units by file
        let mut file_groups: HashMap<String, Vec<&ResolvedUnit>> = HashMap::new();
        for runit in resolved {
            let key = runit.unit.file_path.to_string_lossy().to_string();
            file_groups.entry(key).or_default().push(runit);
        }

        for units_in_file in file_groups.values() {
            // Find the module unit for this file
            let module = units_in_file
                .iter()
                .find(|u| u.unit.unit_type == CodeUnitType::Module);

            if let Some(module_unit) = module {
                let module_graph_id = match id_map.get(&module_unit.unit.temp_id) {
                    Some(&id) => id,
                    None => continue,
                };

                // All other non-module, non-import units in this file are children
                for other in units_in_file {
                    if other.unit.temp_id == module_unit.unit.temp_id {
                        continue;
                    }
                    if other.unit.unit_type == CodeUnitType::Import {
                        continue;
                    }
                    if let Some(&other_id) = id_map.get(&other.unit.temp_id) {
                        let edge = Edge::new(module_graph_id, other_id, EdgeType::Contains);
                        let _ = graph.add_edge(edge);
                    }
                }
            }
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a raw reference kind to an edge type.
fn reference_to_edge_type(kind: ReferenceKind) -> EdgeType {
    match kind {
        ReferenceKind::Call => EdgeType::Calls,
        ReferenceKind::Import => EdgeType::Imports,
        ReferenceKind::TypeUse => EdgeType::UsesType,
        ReferenceKind::Inherit => EdgeType::Inherits,
        ReferenceKind::Implement => EdgeType::Implements,
        ReferenceKind::Access => EdgeType::References,
    }
}
