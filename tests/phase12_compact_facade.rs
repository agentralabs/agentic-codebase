//! Phase 12: compact facade tool routing tests.

use std::path::PathBuf;

use serde_json::{json, Value};

use agentic_codebase::graph::CodeGraph;
use agentic_codebase::mcp::server::McpServer;
use agentic_codebase::types::{CodeUnit, CodeUnitType, Edge, EdgeType, Language, Span};

fn build_test_graph() -> CodeGraph {
    let mut graph = CodeGraph::with_default_dimension();

    let unit_a = CodeUnit::new(
        CodeUnitType::Function,
        Language::Rust,
        "process_data".to_string(),
        "app::process_data".to_string(),
        PathBuf::from("src/app.rs"),
        Span::new(10, 0, 30, 0),
    );
    let id_a = graph.add_unit(unit_a);

    let unit_b = CodeUnit::new(
        CodeUnitType::Function,
        Language::Rust,
        "validate_input".to_string(),
        "app::validate_input".to_string(),
        PathBuf::from("src/app.rs"),
        Span::new(35, 0, 50, 0),
    );
    let id_b = graph.add_unit(unit_b);

    graph
        .add_edge(Edge::new(id_a, id_b, EdgeType::Calls))
        .expect("edge add");

    graph
}

fn create_server() -> McpServer {
    let mut server = McpServer::new();
    server.load_graph("test".to_string(), build_test_graph());
    server
}

fn send(server: &mut McpServer, request: &Value) -> Value {
    let raw = serde_json::to_string(request).expect("serialize request");
    let response_str = server.handle_raw(&raw);
    serde_json::from_str(&response_str).expect("parse response")
}

#[test]
fn test_compact_core_symbol_lookup_routes_to_legacy_tool() {
    let mut server = create_server();

    let response = send(
        &mut server,
        &json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "codebase_core",
                "arguments": {
                    "operation": "symbol_lookup",
                    "params": {
                        "graph": "test",
                        "name": "process",
                        "mode": "prefix"
                    }
                }
            }
        }),
    );

    assert!(
        response.get("result").is_some(),
        "expected success: {response}"
    );
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("text result");
    assert!(
        text.contains("process_data"),
        "expected routed symbol_lookup result: {text}"
    );
}

#[test]
fn test_compact_workspace_create_routes_to_legacy_tool() {
    let mut server = create_server();

    let response = send(
        &mut server,
        &json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "codebase_workspace",
                "arguments": {
                    "operation": "workspace_create",
                    "params": {
                        "name": "compact-workspace"
                    }
                }
            }
        }),
    );

    assert!(
        response.get("result").is_some(),
        "expected success: {response}"
    );
    let text = response["result"]["content"][0]["text"]
        .as_str()
        .expect("text result");
    let parsed: Value = serde_json::from_str(text).expect("workspace json");
    assert_eq!(parsed["name"], "compact-workspace");
    assert!(parsed["workspace_id"].is_string());
}

#[test]
fn test_compact_unknown_operation_returns_invalid_params() {
    let mut server = create_server();

    let response = send(
        &mut server,
        &json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "codebase_core",
                "arguments": {
                    "operation": "not_a_real_operation"
                }
            }
        }),
    );

    assert!(
        response.get("error").is_some(),
        "expected error: {response}"
    );
    assert_eq!(response["error"]["code"], -32602);
}
