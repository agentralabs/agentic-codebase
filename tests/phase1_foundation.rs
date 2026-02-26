//! Phase 1 tests: Foundation — Data structures, file header, CodeGraph, and file I/O.
//!
//! All tests must pass before proceeding to Phase 2.

use std::path::PathBuf;

use agentic_codebase::format::{AcbReader, AcbWriter};
use agentic_codebase::graph::CodeGraph;
use agentic_codebase::graph::GraphBuilder;
use agentic_codebase::types::*;

// ===========================================================================
// Data Structure Tests
// ===========================================================================

#[test]
fn test_code_unit_type_roundtrip() {
    let variants = [
        CodeUnitType::Module,
        CodeUnitType::Symbol,
        CodeUnitType::Type,
        CodeUnitType::Function,
        CodeUnitType::Parameter,
        CodeUnitType::Import,
        CodeUnitType::Test,
        CodeUnitType::Doc,
        CodeUnitType::Config,
        CodeUnitType::Pattern,
        CodeUnitType::Trait,
        CodeUnitType::Impl,
        CodeUnitType::Macro,
    ];
    for variant in &variants {
        let byte = *variant as u8;
        let recovered = CodeUnitType::from_u8(byte).expect("roundtrip failed");
        assert_eq!(
            *variant, recovered,
            "Roundtrip failed for {:?} (byte {})",
            variant, byte
        );
    }
}

#[test]
fn test_code_unit_type_invalid() {
    assert!(CodeUnitType::from_u8(255).is_none());
    assert!(CodeUnitType::from_u8(13).is_none());
    assert!(CodeUnitType::from_u8(200).is_none());
}

#[test]
fn test_code_unit_type_is_callable() {
    assert!(CodeUnitType::Function.is_callable());
    assert!(CodeUnitType::Macro.is_callable());
    assert!(!CodeUnitType::Module.is_callable());
    assert!(!CodeUnitType::Type.is_callable());
}

#[test]
fn test_code_unit_type_is_container() {
    assert!(CodeUnitType::Module.is_container());
    assert!(CodeUnitType::Type.is_container());
    assert!(CodeUnitType::Trait.is_container());
    assert!(CodeUnitType::Impl.is_container());
    assert!(!CodeUnitType::Function.is_container());
    assert!(!CodeUnitType::Parameter.is_container());
}

#[test]
fn test_edge_type_roundtrip() {
    for i in 0..18u8 {
        let et = EdgeType::from_u8(i).unwrap_or_else(|| panic!("EdgeType::from_u8({}) failed", i));
        assert_eq!(et as u8, i, "EdgeType roundtrip failed for byte {}", i);
    }
}

#[test]
fn test_edge_type_invalid() {
    assert!(EdgeType::from_u8(18).is_none());
    assert!(EdgeType::from_u8(255).is_none());
}

#[test]
fn test_edge_type_is_dependency() {
    assert!(EdgeType::Calls.is_dependency());
    assert!(EdgeType::Imports.is_dependency());
    assert!(EdgeType::Inherits.is_dependency());
    assert!(EdgeType::Implements.is_dependency());
    assert!(EdgeType::UsesType.is_dependency());
    assert!(EdgeType::FfiBinds.is_dependency());
    assert!(!EdgeType::Contains.is_dependency());
    assert!(!EdgeType::Tests.is_dependency());
}

#[test]
fn test_edge_type_is_temporal() {
    assert!(EdgeType::CouplesWith.is_temporal());
    assert!(EdgeType::BreaksWith.is_temporal());
    assert!(EdgeType::VersionOf.is_temporal());
    assert!(!EdgeType::Calls.is_temporal());
}

#[test]
fn test_language_detection_python() {
    assert_eq!(Language::from_extension("py"), Language::Python);
    assert_eq!(Language::from_extension("pyi"), Language::Python);
    assert_eq!(Language::from_extension("PY"), Language::Python);
}

#[test]
fn test_language_detection_all() {
    assert_eq!(Language::from_extension("py"), Language::Python);
    assert_eq!(Language::from_extension("pyi"), Language::Python);
    assert_eq!(Language::from_extension("rs"), Language::Rust);
    assert_eq!(Language::from_extension("ts"), Language::TypeScript);
    assert_eq!(Language::from_extension("tsx"), Language::TypeScript);
    assert_eq!(Language::from_extension("js"), Language::JavaScript);
    assert_eq!(Language::from_extension("jsx"), Language::JavaScript);
    assert_eq!(Language::from_extension("mjs"), Language::JavaScript);
    assert_eq!(Language::from_extension("cjs"), Language::JavaScript);
    assert_eq!(Language::from_extension("go"), Language::Go);
    assert_eq!(Language::from_extension("cpp"), Language::Cpp);
    assert_eq!(Language::from_extension("cc"), Language::Cpp);
    assert_eq!(Language::from_extension("cxx"), Language::Cpp);
    assert_eq!(Language::from_extension("hpp"), Language::Cpp);
    assert_eq!(Language::from_extension("h"), Language::Cpp);
    assert_eq!(Language::from_extension("java"), Language::Java);
    assert_eq!(Language::from_extension("cs"), Language::CSharp);
    assert_eq!(Language::from_extension("unknown"), Language::Unknown);
}

#[test]
fn test_language_from_path() {
    assert_eq!(
        Language::from_path(&PathBuf::from("src/main.py")),
        Language::Python
    );
    assert_eq!(
        Language::from_path(&PathBuf::from("lib.rs")),
        Language::Rust
    );
    assert_eq!(
        Language::from_path(&PathBuf::from("no_ext")),
        Language::Unknown
    );
}

#[test]
fn test_language_from_u8_roundtrip() {
    assert_eq!(Language::from_u8(0), Some(Language::Python));
    assert_eq!(Language::from_u8(1), Some(Language::Rust));
    assert_eq!(Language::from_u8(2), Some(Language::TypeScript));
    assert_eq!(Language::from_u8(3), Some(Language::JavaScript));
    assert_eq!(Language::from_u8(4), Some(Language::Go));
    assert_eq!(Language::from_u8(5), Some(Language::Cpp));
    assert_eq!(Language::from_u8(6), Some(Language::Java));
    assert_eq!(Language::from_u8(7), Some(Language::CSharp));
    assert_eq!(Language::from_u8(255), Some(Language::Unknown));
    assert_eq!(Language::from_u8(8), None);
    assert_eq!(Language::from_u8(100), None);
}

#[test]
fn test_span_creation() {
    let span = Span::new(10, 5, 20, 30);
    assert_eq!(span.start_line, 10);
    assert_eq!(span.start_col, 5);
    assert_eq!(span.end_line, 20);
    assert_eq!(span.end_col, 30);
}

#[test]
fn test_span_point() {
    let span = Span::point(42, 7);
    assert_eq!(span.start_line, 42);
    assert_eq!(span.start_col, 7);
    assert_eq!(span.end_line, 42);
    assert_eq!(span.end_col, 7);
    assert_eq!(span.line_count(), 1);
}

#[test]
fn test_span_line_count() {
    assert_eq!(Span::new(1, 0, 1, 10).line_count(), 1);
    assert_eq!(Span::new(1, 0, 10, 10).line_count(), 10);
    assert_eq!(Span::new(5, 0, 15, 0).line_count(), 11);
}

#[test]
fn test_span_contains() {
    let span = Span::new(10, 5, 20, 30);
    // Inside
    assert!(span.contains(15, 10));
    // Start boundary
    assert!(span.contains(10, 5));
    assert!(span.contains(10, 10));
    // End boundary
    assert!(span.contains(20, 30));
    assert!(span.contains(20, 0));
    // Outside
    assert!(!span.contains(9, 10));
    assert!(!span.contains(21, 0));
    assert!(!span.contains(10, 4)); // before start col on start line
    assert!(!span.contains(20, 31)); // after end col on end line
}

#[test]
fn test_visibility_roundtrip() {
    assert_eq!(Visibility::from_u8(0), Some(Visibility::Public));
    assert_eq!(Visibility::from_u8(1), Some(Visibility::Private));
    assert_eq!(Visibility::from_u8(2), Some(Visibility::Internal));
    assert_eq!(Visibility::from_u8(3), Some(Visibility::Protected));
    assert_eq!(Visibility::from_u8(255), Some(Visibility::Unknown));
    assert_eq!(Visibility::from_u8(4), None);
}

#[test]
fn test_code_unit_creation() {
    let unit = CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "my_func".to_string(),
        "mymod.my_func".to_string(),
        PathBuf::from("src/mymod.py"),
        Span::new(10, 0, 25, 0),
    );
    assert_eq!(unit.id, 0); // Default before graph assigns
    assert_eq!(unit.unit_type, CodeUnitType::Function);
    assert_eq!(unit.language, Language::Python);
    assert_eq!(unit.name, "my_func");
    assert_eq!(unit.qualified_name, "mymod.my_func");
    assert_eq!(unit.file_path, PathBuf::from("src/mymod.py"));
    assert_eq!(unit.span.start_line, 10);
    assert_eq!(unit.visibility, Visibility::Unknown);
    assert_eq!(unit.complexity, 0);
    assert!(!unit.is_async);
    assert!(!unit.is_generator);
    assert_eq!(unit.stability_score, 1.0);
    assert_eq!(unit.feature_vec.len(), 256);
}

#[test]
fn test_code_unit_builder() {
    let unit = CodeUnitBuilder::new(
        CodeUnitType::Function,
        Language::Rust,
        "process",
        "engine::process",
        PathBuf::from("src/engine.rs"),
        Span::new(1, 0, 50, 0),
    )
    .signature("fn process(data: &[u8]) -> Result<Output>")
    .doc("Processes raw data into output")
    .visibility(Visibility::Public)
    .complexity(5)
    .async_fn()
    .build();

    assert_eq!(unit.name, "process");
    assert_eq!(unit.qualified_name, "engine::process");
    assert_eq!(
        unit.signature.as_deref(),
        Some("fn process(data: &[u8]) -> Result<Output>")
    );
    assert_eq!(
        unit.doc_summary.as_deref(),
        Some("Processes raw data into output")
    );
    assert_eq!(unit.visibility, Visibility::Public);
    assert_eq!(unit.complexity, 5);
    assert!(unit.is_async);
    assert!(!unit.is_generator);
}

#[test]
fn test_edge_creation() {
    let edge = Edge::new(0, 1, EdgeType::Calls);
    assert_eq!(edge.source_id, 0);
    assert_eq!(edge.target_id, 1);
    assert_eq!(edge.edge_type, EdgeType::Calls);
    assert_eq!(edge.weight, 1.0);
    assert_eq!(edge.context, 0);
}

#[test]
fn test_edge_weight_clamping() {
    let edge = Edge::new(0, 1, EdgeType::Calls).with_weight(2.0);
    assert_eq!(edge.weight, 1.0);

    let edge = Edge::new(0, 1, EdgeType::Calls).with_weight(-1.0);
    assert_eq!(edge.weight, 0.0);

    let edge = Edge::new(0, 1, EdgeType::Calls).with_weight(0.5);
    assert!((edge.weight - 0.5).abs() < f32::EPSILON);
}

#[test]
fn test_edge_with_context() {
    let edge = Edge::new(0, 1, EdgeType::Calls).with_context(42);
    assert_eq!(edge.context, 42);
}

#[test]
fn test_self_edge_detection() {
    let edge = Edge::new(5, 5, EdgeType::Calls);
    assert!(edge.is_self_edge());

    let edge = Edge::new(5, 6, EdgeType::Calls);
    assert!(!edge.is_self_edge());
}

// ===========================================================================
// File Header Tests
// ===========================================================================

#[test]
fn test_header_write_read_roundtrip() {
    let mut header = FileHeader::new(256);
    header.unit_count = 1000;
    header.edge_count = 5000;
    header.language_count = 3;
    header.unit_table_offset = 128;
    header.edge_table_offset = 128 + 1000 * 96;
    header.string_pool_offset = 128 + 1000 * 96 + 5000 * 40;
    header.repo_hash = [0xAB; 32];
    header.compiled_at = 1234567890;

    let bytes = header.to_bytes();
    let header2 = FileHeader::from_bytes(&bytes).unwrap();
    assert_eq!(header, header2);
}

#[test]
fn test_header_size_is_128_bytes() {
    let header = FileHeader::new(256);
    let bytes = header.to_bytes();
    assert_eq!(bytes.len(), 128);
}

#[test]
fn test_header_magic_validation() {
    let header = FileHeader::new(256);
    let mut bytes = header.to_bytes();
    // Corrupt magic
    bytes[0] = 0xFF;
    let result = FileHeader::from_bytes(&bytes);
    assert!(matches!(result, Err(AcbError::InvalidMagic)));
}

#[test]
fn test_header_version_validation() {
    let header = FileHeader::new(256);
    let mut bytes = header.to_bytes();
    // Set version to 99
    bytes[4..8].copy_from_slice(&99u32.to_le_bytes());
    let result = FileHeader::from_bytes(&bytes);
    assert!(matches!(result, Err(AcbError::UnsupportedVersion(99))));
}

#[test]
fn test_header_little_endian() {
    let mut header = FileHeader::new(256);
    header.unit_count = 0x0102030405060708;
    let bytes = header.to_bytes();
    // unit_count is at offset 0x10 (16)
    assert_eq!(bytes[16], 0x08); // LSB first
    assert_eq!(bytes[17], 0x07);
    assert_eq!(bytes[18], 0x06);
    assert_eq!(bytes[19], 0x05);
}

// ===========================================================================
// CodeGraph Basic Tests
// ===========================================================================

#[test]
fn test_empty_graph() {
    let graph = CodeGraph::new(256);
    assert_eq!(graph.unit_count(), 0);
    assert_eq!(graph.edge_count(), 0);
    assert_eq!(graph.dimension(), 256);
}

#[test]
fn test_add_single_unit() {
    let mut graph = CodeGraph::new(256);
    let unit = CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "foo".into(),
        "mod.foo".into(),
        PathBuf::from("mod.py"),
        Span::new(1, 0, 10, 0),
    );
    let id = graph.add_unit(unit);
    assert_eq!(id, 0);
    assert_eq!(graph.unit_count(), 1);
    let u = graph.get_unit(0).unwrap();
    assert_eq!(u.name, "foo");
    assert_eq!(u.id, 0);
}

#[test]
fn test_add_multiple_units() {
    let mut graph = CodeGraph::new(256);
    for i in 0..100 {
        let unit = CodeUnit::new(
            CodeUnitType::Function,
            Language::Python,
            format!("func_{}", i),
            format!("mod.func_{}", i),
            PathBuf::from("mod.py"),
            Span::new(i as u32 * 10, 0, (i as u32 + 1) * 10, 0),
        );
        let id = graph.add_unit(unit);
        assert_eq!(id, i as u64);
    }
    assert_eq!(graph.unit_count(), 100);
    for i in 0..100 {
        let u = graph.get_unit(i).unwrap();
        assert_eq!(u.name, format!("func_{}", i));
    }
}

#[test]
fn test_add_edge() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "a".into(),
        "a".into(),
        PathBuf::from("a.py"),
        Span::new(1, 0, 10, 0),
    ));
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "b".into(),
        "b".into(),
        PathBuf::from("b.py"),
        Span::new(1, 0, 10, 0),
    ));
    graph.add_edge(Edge::new(0, 1, EdgeType::Calls)).unwrap();
    assert_eq!(graph.edge_count(), 1);
    let edges = graph.edges_from(0);
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].target_id, 1);
    assert_eq!(edges[0].edge_type, EdgeType::Calls);
}

#[test]
fn test_edge_validation_self_edge() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "a".into(),
        "a".into(),
        PathBuf::from("a.py"),
        Span::new(1, 0, 10, 0),
    ));
    let result = graph.add_edge(Edge::new(0, 0, EdgeType::Calls));
    assert!(matches!(result, Err(AcbError::SelfEdge(0))));
}

#[test]
fn test_edge_validation_nonexistent_source() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "a".into(),
        "a".into(),
        PathBuf::from("a.py"),
        Span::new(1, 0, 10, 0),
    ));
    let result = graph.add_edge(Edge::new(99, 0, EdgeType::Calls));
    assert!(matches!(result, Err(AcbError::UnitNotFound(99))));
}

#[test]
fn test_edge_validation_nonexistent_target() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "a".into(),
        "a".into(),
        PathBuf::from("a.py"),
        Span::new(1, 0, 10, 0),
    ));
    let result = graph.add_edge(Edge::new(0, 99, EdgeType::Calls));
    assert!(matches!(result, Err(AcbError::InvalidEdgeTarget(99))));
}

#[test]
fn test_edges_from_and_to() {
    let mut graph = CodeGraph::new(256);
    for i in 0..5 {
        graph.add_unit(CodeUnit::new(
            CodeUnitType::Function,
            Language::Python,
            format!("f{}", i),
            format!("f{}", i),
            PathBuf::from("m.py"),
            Span::new(i as u32, 0, i as u32 + 1, 0),
        ));
    }
    // 0 -> 1, 0 -> 2, 3 -> 4, 1 -> 4
    graph.add_edge(Edge::new(0, 1, EdgeType::Calls)).unwrap();
    graph.add_edge(Edge::new(0, 2, EdgeType::Calls)).unwrap();
    graph.add_edge(Edge::new(3, 4, EdgeType::Calls)).unwrap();
    graph.add_edge(Edge::new(1, 4, EdgeType::Calls)).unwrap();

    assert_eq!(graph.edges_from(0).len(), 2);
    assert_eq!(graph.edges_from(3).len(), 1);
    assert_eq!(graph.edges_to(4).len(), 2);
    assert_eq!(graph.edges_to(0).len(), 0);
}

#[test]
fn test_find_units_by_name() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "process_data".into(),
        "process_data".into(),
        PathBuf::from("m.py"),
        Span::new(1, 0, 10, 0),
    ));
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "process_file".into(),
        "process_file".into(),
        PathBuf::from("m.py"),
        Span::new(11, 0, 20, 0),
    ));
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "validate".into(),
        "validate".into(),
        PathBuf::from("m.py"),
        Span::new(21, 0, 30, 0),
    ));

    let results = graph.find_units_by_name("process");
    assert_eq!(results.len(), 2);
    let results = graph.find_units_by_name("val");
    assert_eq!(results.len(), 1);
    let results = graph.find_units_by_name("nonexistent");
    assert_eq!(results.len(), 0);
}

#[test]
fn test_find_units_by_type() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "f".into(),
        "f".into(),
        PathBuf::from("m.py"),
        Span::new(1, 0, 10, 0),
    ));
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Type,
        Language::Python,
        "MyClass".into(),
        "MyClass".into(),
        PathBuf::from("m.py"),
        Span::new(11, 0, 20, 0),
    ));
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "g".into(),
        "g".into(),
        PathBuf::from("m.py"),
        Span::new(21, 0, 30, 0),
    ));

    let funcs = graph.find_units_by_type(CodeUnitType::Function);
    assert_eq!(funcs.len(), 2);
    let types = graph.find_units_by_type(CodeUnitType::Type);
    assert_eq!(types.len(), 1);
}

#[test]
fn test_graph_stats() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "f".into(),
        "f".into(),
        PathBuf::from("m.py"),
        Span::new(1, 0, 10, 0),
    ));
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Type,
        Language::Rust,
        "S".into(),
        "S".into(),
        PathBuf::from("lib.rs"),
        Span::new(1, 0, 10, 0),
    ));
    graph.add_edge(Edge::new(0, 1, EdgeType::Calls)).unwrap();

    let stats = graph.stats();
    assert_eq!(stats.unit_count, 2);
    assert_eq!(stats.edge_count, 1);
    assert_eq!(stats.language_counts[&Language::Python], 1);
    assert_eq!(stats.language_counts[&Language::Rust], 1);
}

#[test]
fn test_graph_builder() {
    let graph = GraphBuilder::new(256)
        .add_unit(CodeUnit::new(
            CodeUnitType::Module,
            Language::Python,
            "mymod".into(),
            "mymod".into(),
            PathBuf::from("mymod.py"),
            Span::new(1, 0, 100, 0),
        ))
        .add_unit(CodeUnit::new(
            CodeUnitType::Function,
            Language::Python,
            "func".into(),
            "mymod.func".into(),
            PathBuf::from("mymod.py"),
            Span::new(5, 0, 15, 0),
        ))
        .add_edge(Edge::new(0, 1, EdgeType::Contains))
        .build()
        .unwrap();

    assert_eq!(graph.unit_count(), 2);
    assert_eq!(graph.edge_count(), 1);
}

// ===========================================================================
// File I/O Tests
// ===========================================================================

fn make_test_graph(num_units: usize, num_edges_per: usize) -> CodeGraph {
    let mut graph = CodeGraph::new(256);
    for i in 0..num_units {
        let unit = CodeUnit::new(
            CodeUnitType::Function,
            Language::Python,
            format!("func_{}", i),
            format!("module.func_{}", i),
            PathBuf::from(format!("src/module_{}.py", i / 10)),
            Span::new((i * 10) as u32, 0, ((i + 1) * 10) as u32, 0),
        );
        graph.add_unit(unit);
    }
    // Add edges: each unit calls the next few units
    for i in 0..num_units {
        for j in 1..=num_edges_per {
            let target = (i + j) % num_units;
            if target != i {
                let _ = graph.add_edge(Edge::new(i as u64, target as u64, EdgeType::Calls));
            }
        }
    }
    graph
}

#[test]
fn test_write_empty_graph() {
    let graph = CodeGraph::new(256);
    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();
    assert!(buf.len() >= 128); // At least the header
                               // Verify we can read back
    let graph2 = AcbReader::read_from_data(&buf).unwrap();
    assert_eq!(graph2.unit_count(), 0);
    assert_eq!(graph2.edge_count(), 0);
}

#[test]
fn test_write_read_single_unit() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "hello".into(),
        "greeting.hello".into(),
        PathBuf::from("greeting.py"),
        Span::new(5, 4, 10, 0),
    ));

    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();
    let graph2 = AcbReader::read_from_data(&buf).unwrap();

    assert_eq!(graph2.unit_count(), 1);
    let u = graph2.get_unit(0).unwrap();
    assert_eq!(u.name, "hello");
    assert_eq!(u.qualified_name, "greeting.hello");
    assert_eq!(u.file_path, PathBuf::from("greeting.py"));
    assert_eq!(u.unit_type, CodeUnitType::Function);
    assert_eq!(u.language, Language::Python);
    assert_eq!(u.span.start_line, 5);
}

#[test]
fn test_write_read_with_edges() {
    let graph = make_test_graph(10, 2);
    assert!(graph.edge_count() > 0);

    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();
    let graph2 = AcbReader::read_from_data(&buf).unwrap();

    assert_eq!(graph2.unit_count(), graph.unit_count());
    assert_eq!(graph2.edge_count(), graph.edge_count());

    // Verify a specific unit
    let u1 = graph.get_unit(0).unwrap();
    let u2 = graph2.get_unit(0).unwrap();
    assert_eq!(u1.name, u2.name);
    assert_eq!(u1.qualified_name, u2.qualified_name);
}

#[test]
fn test_write_read_all_unit_types() {
    let mut graph = CodeGraph::new(256);
    let types = [
        CodeUnitType::Module,
        CodeUnitType::Symbol,
        CodeUnitType::Type,
        CodeUnitType::Function,
        CodeUnitType::Parameter,
        CodeUnitType::Import,
        CodeUnitType::Test,
        CodeUnitType::Doc,
        CodeUnitType::Config,
        CodeUnitType::Pattern,
        CodeUnitType::Trait,
        CodeUnitType::Impl,
        CodeUnitType::Macro,
    ];
    for (i, ut) in types.iter().enumerate() {
        graph.add_unit(CodeUnit::new(
            *ut,
            Language::Python,
            format!("unit_{}", i),
            format!("unit_{}", i),
            PathBuf::from("test.py"),
            Span::new(i as u32, 0, i as u32 + 1, 0),
        ));
    }

    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();
    let graph2 = AcbReader::read_from_data(&buf).unwrap();

    assert_eq!(graph2.unit_count(), types.len());
    for (i, ut) in types.iter().enumerate() {
        assert_eq!(
            graph2.get_unit(i as u64).unwrap().unit_type,
            *ut,
            "Unit type mismatch at index {}",
            i
        );
    }
}

#[test]
fn test_write_read_all_edge_types() {
    let mut graph = CodeGraph::new(256);
    // Create enough units for all edge types (no self-edges)
    for i in 0..36 {
        graph.add_unit(CodeUnit::new(
            CodeUnitType::Function,
            Language::Python,
            format!("u{}", i),
            format!("u{}", i),
            PathBuf::from("test.py"),
            Span::new(i, 0, i + 1, 0),
        ));
    }

    let edge_types = [
        EdgeType::Calls,
        EdgeType::Imports,
        EdgeType::Inherits,
        EdgeType::Implements,
        EdgeType::Overrides,
        EdgeType::Contains,
        EdgeType::References,
        EdgeType::Tests,
        EdgeType::Documents,
        EdgeType::Configures,
        EdgeType::CouplesWith,
        EdgeType::BreaksWith,
        EdgeType::PatternOf,
        EdgeType::VersionOf,
        EdgeType::FfiBinds,
        EdgeType::UsesType,
        EdgeType::Returns,
        EdgeType::ParamType,
    ];

    for (i, et) in edge_types.iter().enumerate() {
        let src = (i * 2) as u64;
        let tgt = (i * 2 + 1) as u64;
        graph.add_edge(Edge::new(src, tgt, *et)).unwrap();
    }

    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();
    let graph2 = AcbReader::read_from_data(&buf).unwrap();

    assert_eq!(graph2.edge_count(), edge_types.len());
}

#[test]
fn test_string_pool_compression() {
    let mut graph = CodeGraph::new(256);
    // Add units with long, repetitive names (good for compression)
    for i in 0..100 {
        graph.add_unit(CodeUnit::new(
            CodeUnitType::Function,
            Language::Python,
            format!("very_long_function_name_number_{}", i),
            format!(
                "very.long.qualified.module.path.very_long_function_name_number_{}",
                i
            ),
            PathBuf::from(format!("very/long/directory/path/module_{}.py", i)),
            Span::new(i as u32, 0, (i + 10) as u32, 0),
        ));
    }

    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();

    // The compressed file should exist and be readable
    let graph2 = AcbReader::read_from_data(&buf).unwrap();
    assert_eq!(graph2.unit_count(), 100);

    // Verify strings survived compression/decompression
    for i in 0..100 {
        let u = graph2.get_unit(i).unwrap();
        assert_eq!(u.name, format!("very_long_function_name_number_{}", i));
    }
}

#[test]
fn test_unicode_strings() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "calcular_pre\u{00e7}o".into(), // calcular_preço
        "m\u{00f3}dulo.calcular_pre\u{00e7}o".into(),
        PathBuf::from("src/m\u{00f3}dulo.py"),
        Span::new(1, 0, 10, 0),
    ));
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "\u{4f60}\u{597d}".into(), // 你好
        "module.\u{4f60}\u{597d}".into(),
        PathBuf::from("src/module.py"),
        Span::new(11, 0, 20, 0),
    ));

    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();
    let graph2 = AcbReader::read_from_data(&buf).unwrap();

    assert_eq!(graph2.unit_count(), 2);
    assert_eq!(graph2.get_unit(0).unwrap().name, "calcular_pre\u{00e7}o");
    assert_eq!(graph2.get_unit(1).unwrap().name, "\u{4f60}\u{597d}");
}

#[test]
fn test_large_graph_write_read() {
    let graph = make_test_graph(1000, 3);
    assert!(graph.unit_count() == 1000);
    assert!(graph.edge_count() > 0);

    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();
    let graph2 = AcbReader::read_from_data(&buf).unwrap();

    assert_eq!(graph2.unit_count(), 1000);
    assert_eq!(graph2.edge_count(), graph.edge_count());

    // Spot check some units
    for i in [0, 100, 500, 999] {
        let u1 = graph.get_unit(i).unwrap();
        let u2 = graph2.get_unit(i).unwrap();
        assert_eq!(u1.name, u2.name);
        assert_eq!(u1.qualified_name, u2.qualified_name);
    }
}

#[test]
fn test_corrupted_file_detection() {
    let graph = make_test_graph(10, 1);
    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();

    // Truncate to just the header
    let truncated = &buf[..128];
    // This should either return an empty graph or an error for truncation
    // because the header says there are 10 units but they're missing
    let result = AcbReader::read_from_data(truncated);
    assert!(result.is_err());
}

#[test]
fn test_wrong_magic_detection() {
    let graph = make_test_graph(5, 1);
    let mut buf = Vec::new();
    AcbWriter::new(256).write_to(&graph, &mut buf).unwrap();

    // Corrupt magic bytes
    buf[0] = 0xFF;
    let result = AcbReader::read_from_data(&buf);
    assert!(matches!(result, Err(AcbError::InvalidMagic)));
}

#[test]
fn test_file_roundtrip_via_tempfile() {
    let graph = make_test_graph(50, 2);
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path().to_path_buf();

    AcbWriter::new(256).write_to_file(&graph, &path).unwrap();
    let graph2 = AcbReader::read_from_file(&path).unwrap();

    assert_eq!(graph2.unit_count(), 50);
    assert_eq!(graph2.edge_count(), graph.edge_count());
}

#[test]
fn test_timestamp_is_nonzero() {
    let ts = agentic_codebase::types::now_micros();
    assert!(ts > 0);
    assert!(ts > 1_700_000_000_000_000); // After 2023
}

#[test]
fn test_has_edge() {
    let mut graph = CodeGraph::new(256);
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "a".into(),
        "a".into(),
        PathBuf::from("a.py"),
        Span::new(1, 0, 10, 0),
    ));
    graph.add_unit(CodeUnit::new(
        CodeUnitType::Function,
        Language::Python,
        "b".into(),
        "b".into(),
        PathBuf::from("b.py"),
        Span::new(1, 0, 10, 0),
    ));
    graph.add_edge(Edge::new(0, 1, EdgeType::Calls)).unwrap();

    assert!(graph.has_edge(0, 1, EdgeType::Calls));
    assert!(!graph.has_edge(0, 1, EdgeType::Imports));
    assert!(!graph.has_edge(1, 0, EdgeType::Calls));
}

#[test]
fn test_default_graph() {
    let graph = CodeGraph::default();
    assert_eq!(graph.unit_count(), 0);
    assert_eq!(graph.dimension(), 256);
}
