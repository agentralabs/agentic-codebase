# SPEC-FFI.md

> C bindings for cross-language integration. Python, Node.js, Go — everyone gets access.

---

## Overview

The FFI layer provides C-compatible bindings so AgenticCodebase can be used from:
- **Python** via ctypes or cffi
- **Node.js** via N-API or node-ffi
- **Go** via cgo
- **Any language** with C FFI support

---

## Design Principles

1. **Opaque pointers**: No Rust types exposed across FFI boundary
2. **Explicit memory management**: Caller creates, caller destroys
3. **Error codes**: Return integers, not exceptions
4. **String handling**: UTF-8, null-terminated, with length variants
5. **Thread safety**: All functions are thread-safe unless noted
6. **No callbacks in v1**: Keeps the interface simple

---

## Header File

```c
// ffi/agentic_codebase.h

#ifndef AGENTIC_CODEBASE_H
#define AGENTIC_CODEBASE_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// =============================================================================
// Opaque Types
// =============================================================================

/// Handle to a compiled code graph
typedef struct AcbGraph AcbGraph;

/// Handle to a query result
typedef struct AcbQueryResult AcbQueryResult;

/// Handle to a code unit
typedef struct AcbCodeUnit AcbCodeUnit;

/// Handle to an edge
typedef struct AcbEdge AcbEdge;

/// Handle to compilation options
typedef struct AcbCompileOptions AcbCompileOptions;

/// Handle to query options
typedef struct AcbQueryOptions AcbQueryOptions;

// =============================================================================
// Error Codes
// =============================================================================

typedef enum AcbError {
    ACB_OK = 0,
    ACB_ERR_NULL_POINTER = 1,
    ACB_ERR_INVALID_PATH = 2,
    ACB_ERR_PARSE_ERROR = 3,
    ACB_ERR_FILE_NOT_FOUND = 4,
    ACB_ERR_INVALID_FORMAT = 5,
    ACB_ERR_UNIT_NOT_FOUND = 6,
    ACB_ERR_QUERY_FAILED = 7,
    ACB_ERR_IO_ERROR = 8,
    ACB_ERR_OUT_OF_MEMORY = 9,
    ACB_ERR_INVALID_ARGUMENT = 10,
    ACB_ERR_GIT_ERROR = 11,
    ACB_ERR_UNKNOWN = 255,
} AcbError;

// =============================================================================
// Code Unit Types
// =============================================================================

typedef enum AcbUnitType {
    ACB_UNIT_MODULE = 0,
    ACB_UNIT_SYMBOL = 1,
    ACB_UNIT_TYPE = 2,
    ACB_UNIT_FUNCTION = 3,
    ACB_UNIT_PARAMETER = 4,
    ACB_UNIT_IMPORT = 5,
    ACB_UNIT_TEST = 6,
    ACB_UNIT_DOC = 7,
    ACB_UNIT_CONFIG = 8,
    ACB_UNIT_PATTERN = 9,
    ACB_UNIT_TRAIT = 10,
    ACB_UNIT_IMPL = 11,
    ACB_UNIT_MACRO = 12,
} AcbUnitType;

// =============================================================================
// Edge Types
// =============================================================================

typedef enum AcbEdgeType {
    ACB_EDGE_CALLS = 0,
    ACB_EDGE_IMPORTS = 1,
    ACB_EDGE_INHERITS = 2,
    ACB_EDGE_IMPLEMENTS = 3,
    ACB_EDGE_OVERRIDES = 4,
    ACB_EDGE_CONTAINS = 5,
    ACB_EDGE_REFERENCES = 6,
    ACB_EDGE_TESTS = 7,
    ACB_EDGE_DOCUMENTS = 8,
    ACB_EDGE_CONFIGURES = 9,
    ACB_EDGE_COUPLES_WITH = 10,
    ACB_EDGE_BREAKS_WITH = 11,
    ACB_EDGE_PATTERN_OF = 12,
    ACB_EDGE_VERSION_OF = 13,
    ACB_EDGE_FFI_BINDS = 14,
    ACB_EDGE_USES_TYPE = 15,
    ACB_EDGE_RETURNS = 16,
    ACB_EDGE_PARAM_TYPE = 17,
} AcbEdgeType;

// =============================================================================
// Languages
// =============================================================================

typedef enum AcbLanguage {
    ACB_LANG_PYTHON = 0,
    ACB_LANG_RUST = 1,
    ACB_LANG_TYPESCRIPT = 2,
    ACB_LANG_JAVASCRIPT = 3,
    ACB_LANG_GO = 4,
    ACB_LANG_UNKNOWN = 255,
} AcbLanguage;

// =============================================================================
// Lifecycle Functions
// =============================================================================

/// Get library version string
/// Returns: null-terminated version string (e.g., "0.1.0")
/// Lifetime: static, do not free
const char* acb_version(void);

/// Get last error message
/// Returns: null-terminated error message, or NULL if no error
/// Lifetime: valid until next API call on same thread
const char* acb_last_error(void);

// =============================================================================
// Compilation
// =============================================================================

/// Create default compilation options
/// Returns: options handle, or NULL on error
AcbCompileOptions* acb_compile_options_new(void);

/// Free compilation options
void acb_compile_options_free(AcbCompileOptions* options);

/// Set output path for .acb file
AcbError acb_compile_options_set_output(
    AcbCompileOptions* options,
    const char* path
);

/// Add language to include (call multiple times for multiple languages)
AcbError acb_compile_options_add_language(
    AcbCompileOptions* options,
    AcbLanguage language
);

/// Add exclude pattern (glob)
AcbError acb_compile_options_add_exclude(
    AcbCompileOptions* options,
    const char* pattern
);

/// Set whether to include test files
AcbError acb_compile_options_set_include_tests(
    AcbCompileOptions* options,
    int include  // 0 = false, non-zero = true
);

/// Set whether to extract git history
AcbError acb_compile_options_set_git_history(
    AcbCompileOptions* options,
    int include
);

/// Compile a repository into a code graph
/// path: path to repository root (UTF-8)
/// options: compilation options (may be NULL for defaults)
/// Returns: graph handle, or NULL on error (check acb_last_error)
AcbGraph* acb_compile(
    const char* path,
    const AcbCompileOptions* options
);

// =============================================================================
// File I/O
// =============================================================================

/// Load a graph from an .acb file
/// Returns: graph handle, or NULL on error
AcbGraph* acb_load(const char* path);

/// Save a graph to an .acb file
AcbError acb_save(const AcbGraph* graph, const char* path);

/// Free a graph
void acb_graph_free(AcbGraph* graph);

// =============================================================================
// Graph Information
// =============================================================================

/// Get number of code units in graph
uint64_t acb_graph_unit_count(const AcbGraph* graph);

/// Get number of edges in graph
uint64_t acb_graph_edge_count(const AcbGraph* graph);

/// Get count of units by language
uint64_t acb_graph_unit_count_by_language(
    const AcbGraph* graph,
    AcbLanguage language
);

/// Get count of units by type
uint64_t acb_graph_unit_count_by_type(
    const AcbGraph* graph,
    AcbUnitType unit_type
);

// =============================================================================
// Code Unit Access
// =============================================================================

/// Get a code unit by ID
/// Returns: unit handle, or NULL if not found
/// Lifetime: valid until graph is freed
const AcbCodeUnit* acb_graph_get_unit(
    const AcbGraph* graph,
    uint64_t unit_id
);

/// Get code unit ID
uint64_t acb_unit_id(const AcbCodeUnit* unit);

/// Get code unit type
AcbUnitType acb_unit_type(const AcbCodeUnit* unit);

/// Get code unit language
AcbLanguage acb_unit_language(const AcbCodeUnit* unit);

/// Get code unit name (simple name)
/// Returns: null-terminated string
/// Lifetime: valid until graph is freed
const char* acb_unit_name(const AcbCodeUnit* unit);

/// Get code unit qualified name
const char* acb_unit_qualified_name(const AcbCodeUnit* unit);

/// Get code unit file path
const char* acb_unit_file_path(const AcbCodeUnit* unit);

/// Get code unit signature (may be NULL)
const char* acb_unit_signature(const AcbCodeUnit* unit);

/// Get code unit doc summary (may be NULL)
const char* acb_unit_doc(const AcbCodeUnit* unit);

/// Get code unit span (start_line, start_col, end_line, end_col)
void acb_unit_span(
    const AcbCodeUnit* unit,
    uint32_t* start_line,
    uint32_t* start_col,
    uint32_t* end_line,
    uint32_t* end_col
);

/// Get code unit complexity
uint32_t acb_unit_complexity(const AcbCodeUnit* unit);

/// Check if code unit is async
int acb_unit_is_async(const AcbCodeUnit* unit);

/// Get code unit stability score (0.0 - 1.0)
float acb_unit_stability(const AcbCodeUnit* unit);

/// Get code unit change count
uint32_t acb_unit_change_count(const AcbCodeUnit* unit);

// =============================================================================
// Edge Access
// =============================================================================

/// Get edges from a unit
/// out_edges: array to fill (caller allocates)
/// max_edges: size of out_edges array
/// Returns: number of edges written (may be less than total if array too small)
size_t acb_unit_edges_from(
    const AcbGraph* graph,
    uint64_t unit_id,
    AcbEdge** out_edges,
    size_t max_edges
);

/// Get total edge count from a unit
size_t acb_unit_edge_count_from(
    const AcbGraph* graph,
    uint64_t unit_id
);

/// Get edges to a unit (reverse edges)
size_t acb_unit_edges_to(
    const AcbGraph* graph,
    uint64_t unit_id,
    AcbEdge** out_edges,
    size_t max_edges
);

/// Get edge source ID
uint64_t acb_edge_source(const AcbEdge* edge);

/// Get edge target ID
uint64_t acb_edge_target(const AcbEdge* edge);

/// Get edge type
AcbEdgeType acb_edge_type(const AcbEdge* edge);

/// Get edge weight
float acb_edge_weight(const AcbEdge* edge);

// =============================================================================
// Queries
// =============================================================================

/// Create query options
AcbQueryOptions* acb_query_options_new(void);

/// Free query options
void acb_query_options_free(AcbQueryOptions* options);

/// Set max results for query
AcbError acb_query_options_set_max_results(
    AcbQueryOptions* options,
    size_t max_results
);

/// Set depth for traversal queries
AcbError acb_query_options_set_depth(
    AcbQueryOptions* options,
    uint32_t depth
);

/// Symbol lookup query
/// name: symbol name to search for
/// Returns: query result, or NULL on error
AcbQueryResult* acb_query_symbol_lookup(
    const AcbGraph* graph,
    const char* name,
    const AcbQueryOptions* options
);

/// Dependency graph query
AcbQueryResult* acb_query_dependencies(
    const AcbGraph* graph,
    uint64_t unit_id,
    const AcbQueryOptions* options
);

/// Reverse dependency query
AcbQueryResult* acb_query_reverse_dependencies(
    const AcbGraph* graph,
    uint64_t unit_id,
    const AcbQueryOptions* options
);

/// Impact analysis query
AcbQueryResult* acb_query_impact(
    const AcbGraph* graph,
    uint64_t unit_id,
    const AcbQueryOptions* options
);

/// Similarity search query
/// query_vec: feature vector (length must match graph dimension)
/// vec_len: length of query_vec
AcbQueryResult* acb_query_similar(
    const AcbGraph* graph,
    const float* query_vec,
    size_t vec_len,
    const AcbQueryOptions* options
);

/// Call graph query
/// direction: 0 = callers, 1 = callees, 2 = both
AcbQueryResult* acb_query_call_graph(
    const AcbGraph* graph,
    uint64_t unit_id,
    int direction,
    const AcbQueryOptions* options
);

/// Shortest path query
AcbQueryResult* acb_query_shortest_path(
    const AcbGraph* graph,
    uint64_t from_id,
    uint64_t to_id
);

/// Free query result
void acb_query_result_free(AcbQueryResult* result);

/// Get number of results
size_t acb_query_result_count(const AcbQueryResult* result);

/// Get unit ID at index
uint64_t acb_query_result_unit_id(
    const AcbQueryResult* result,
    size_t index
);

/// Get score/similarity at index (for scored queries)
float acb_query_result_score(
    const AcbQueryResult* result,
    size_t index
);

/// Check if query result has error
int acb_query_result_has_error(const AcbQueryResult* result);

/// Get query result error message
const char* acb_query_result_error(const AcbQueryResult* result);

// =============================================================================
// Temporal Analysis
// =============================================================================

/// Get stability analysis for a unit
/// Returns: JSON string with stability factors (caller must free with acb_free_string)
char* acb_analyze_stability(
    const AcbGraph* graph,
    uint64_t unit_id
);

/// Get coupling analysis
/// Returns: JSON string with couplings
char* acb_analyze_coupling(
    const AcbGraph* graph,
    uint64_t unit_id  // 0 for all units
);

/// Get prophecy predictions
/// Returns: JSON string with predictions
char* acb_analyze_prophecy(
    const AcbGraph* graph
);

/// Free a string returned by analysis functions
void acb_free_string(char* str);

// =============================================================================
// Feature Vectors
// =============================================================================

/// Get feature vector dimension for graph
size_t acb_graph_dimension(const AcbGraph* graph);

/// Get feature vector for a unit
/// out_vec: array to fill (must be at least dimension size)
/// Returns: ACB_OK on success
AcbError acb_unit_feature_vector(
    const AcbGraph* graph,
    uint64_t unit_id,
    float* out_vec
);

// =============================================================================
// Utility
// =============================================================================

/// Set log level (0=off, 1=error, 2=warn, 3=info, 4=debug, 5=trace)
void acb_set_log_level(int level);

#ifdef __cplusplus
}
#endif

#endif // AGENTIC_CODEBASE_H
```

---

## Rust Implementation

```rust
// src/ffi/c_api.rs

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_float, c_int};
use std::ptr;
use std::sync::Mutex;

use crate::*;

// Thread-local error storage
thread_local! {
    static LAST_ERROR: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

fn set_last_error(err: impl ToString) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = Some(err.to_string());
    });
}

fn clear_last_error() {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = None;
    });
}

// =============================================================================
// Opaque Types
// =============================================================================

pub struct AcbGraph {
    inner: CodeGraph,
}

pub struct AcbQueryResult {
    unit_ids: Vec<u64>,
    scores: Vec<f32>,
    error: Option<String>,
}

pub struct AcbCompileOptions {
    inner: CompileOptions,
}

pub struct AcbQueryOptions {
    max_results: usize,
    depth: u32,
}

// =============================================================================
// Lifecycle
// =============================================================================

#[no_mangle]
pub extern "C" fn acb_version() -> *const c_char {
    static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");
    VERSION.as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn acb_last_error() -> *const c_char {
    LAST_ERROR.with(|e| {
        match &*e.borrow() {
            Some(s) => s.as_ptr() as *const c_char,
            None => ptr::null(),
        }
    })
}

// =============================================================================
// Compilation
// =============================================================================

#[no_mangle]
pub extern "C" fn acb_compile_options_new() -> *mut AcbCompileOptions {
    Box::into_raw(Box::new(AcbCompileOptions {
        inner: CompileOptions::default(),
    }))
}

#[no_mangle]
pub extern "C" fn acb_compile_options_free(options: *mut AcbCompileOptions) {
    if !options.is_null() {
        unsafe { drop(Box::from_raw(options)) };
    }
}

#[no_mangle]
pub extern "C" fn acb_compile_options_set_output(
    options: *mut AcbCompileOptions,
    path: *const c_char,
) -> AcbError {
    if options.is_null() || path.is_null() {
        return AcbError::ACB_ERR_NULL_POINTER;
    }
    
    let path_str = unsafe { CStr::from_ptr(path) };
    let path = match path_str.to_str() {
        Ok(s) => s,
        Err(_) => return AcbError::ACB_ERR_INVALID_ARGUMENT,
    };
    
    unsafe {
        (*options).inner.output = Some(PathBuf::from(path));
    }
    
    AcbError::ACB_OK
}

#[no_mangle]
pub extern "C" fn acb_compile_options_add_language(
    options: *mut AcbCompileOptions,
    language: AcbLanguage,
) -> AcbError {
    if options.is_null() {
        return AcbError::ACB_ERR_NULL_POINTER;
    }
    
    let lang = match language {
        AcbLanguage::ACB_LANG_PYTHON => Language::Python,
        AcbLanguage::ACB_LANG_RUST => Language::Rust,
        AcbLanguage::ACB_LANG_TYPESCRIPT => Language::TypeScript,
        AcbLanguage::ACB_LANG_JAVASCRIPT => Language::JavaScript,
        AcbLanguage::ACB_LANG_GO => Language::Go,
        _ => return AcbError::ACB_ERR_INVALID_ARGUMENT,
    };
    
    unsafe {
        (*options).inner.languages.push(lang);
    }
    
    AcbError::ACB_OK
}

#[no_mangle]
pub extern "C" fn acb_compile(
    path: *const c_char,
    options: *const AcbCompileOptions,
) -> *mut AcbGraph {
    clear_last_error();
    
    if path.is_null() {
        set_last_error("path is null");
        return ptr::null_mut();
    }
    
    let path_str = unsafe { CStr::from_ptr(path) };
    let path = match path_str.to_str() {
        Ok(s) => PathBuf::from(s),
        Err(e) => {
            set_last_error(e);
            return ptr::null_mut();
        }
    };
    
    let opts = if options.is_null() {
        CompileOptions::default()
    } else {
        unsafe { (*options).inner.clone() }
    };
    
    match compile_repo(&path, opts) {
        Ok(graph) => Box::into_raw(Box::new(AcbGraph { inner: graph })),
        Err(e) => {
            set_last_error(e);
            ptr::null_mut()
        }
    }
}

// =============================================================================
// File I/O
// =============================================================================

#[no_mangle]
pub extern "C" fn acb_load(path: *const c_char) -> *mut AcbGraph {
    clear_last_error();
    
    if path.is_null() {
        set_last_error("path is null");
        return ptr::null_mut();
    }
    
    let path_str = unsafe { CStr::from_ptr(path) };
    let path = match path_str.to_str() {
        Ok(s) => PathBuf::from(s),
        Err(e) => {
            set_last_error(e);
            return ptr::null_mut();
        }
    };
    
    match AcbReader::read_from_file(&path) {
        Ok(graph) => Box::into_raw(Box::new(AcbGraph { inner: graph })),
        Err(e) => {
            set_last_error(e);
            ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn acb_save(graph: *const AcbGraph, path: *const c_char) -> AcbError {
    if graph.is_null() || path.is_null() {
        return AcbError::ACB_ERR_NULL_POINTER;
    }
    
    let path_str = unsafe { CStr::from_ptr(path) };
    let path = match path_str.to_str() {
        Ok(s) => PathBuf::from(s),
        Err(_) => return AcbError::ACB_ERR_INVALID_ARGUMENT,
    };
    
    let graph = unsafe { &(*graph).inner };
    
    match AcbWriter::new(graph.dimension()).write_to_file(graph, &path) {
        Ok(_) => AcbError::ACB_OK,
        Err(e) => {
            set_last_error(e);
            AcbError::ACB_ERR_IO_ERROR
        }
    }
}

#[no_mangle]
pub extern "C" fn acb_graph_free(graph: *mut AcbGraph) {
    if !graph.is_null() {
        unsafe { drop(Box::from_raw(graph)) };
    }
}

// =============================================================================
// Graph Information
// =============================================================================

#[no_mangle]
pub extern "C" fn acb_graph_unit_count(graph: *const AcbGraph) -> u64 {
    if graph.is_null() {
        return 0;
    }
    unsafe { (*graph).inner.unit_count() as u64 }
}

#[no_mangle]
pub extern "C" fn acb_graph_edge_count(graph: *const AcbGraph) -> u64 {
    if graph.is_null() {
        return 0;
    }
    unsafe { (*graph).inner.edge_count() as u64 }
}

// =============================================================================
// Code Unit Access
// =============================================================================

#[no_mangle]
pub extern "C" fn acb_graph_get_unit(
    graph: *const AcbGraph,
    unit_id: u64,
) -> *const AcbCodeUnit {
    if graph.is_null() {
        return ptr::null();
    }
    
    let graph = unsafe { &(*graph).inner };
    match graph.get_unit(unit_id) {
        Some(unit) => unit as *const CodeUnit as *const AcbCodeUnit,
        None => ptr::null(),
    }
}

// Cast AcbCodeUnit* to CodeUnit* for internal use
#[inline]
fn unit_ref(unit: *const AcbCodeUnit) -> Option<&'static CodeUnit> {
    if unit.is_null() {
        None
    } else {
        Some(unsafe { &*(unit as *const CodeUnit) })
    }
}

#[no_mangle]
pub extern "C" fn acb_unit_id(unit: *const AcbCodeUnit) -> u64 {
    unit_ref(unit).map(|u| u.id).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn acb_unit_type(unit: *const AcbCodeUnit) -> AcbUnitType {
    unit_ref(unit)
        .map(|u| unsafe { std::mem::transmute(u.unit_type as u8) })
        .unwrap_or(AcbUnitType::ACB_UNIT_MODULE)
}

#[no_mangle]
pub extern "C" fn acb_unit_name(unit: *const AcbCodeUnit) -> *const c_char {
    unit_ref(unit)
        .map(|u| u.name.as_ptr() as *const c_char)
        .unwrap_or(ptr::null())
}

#[no_mangle]
pub extern "C" fn acb_unit_qualified_name(unit: *const AcbCodeUnit) -> *const c_char {
    unit_ref(unit)
        .map(|u| u.qualified_name.as_ptr() as *const c_char)
        .unwrap_or(ptr::null())
}

#[no_mangle]
pub extern "C" fn acb_unit_complexity(unit: *const AcbCodeUnit) -> u32 {
    unit_ref(unit).map(|u| u.complexity).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn acb_unit_stability(unit: *const AcbCodeUnit) -> f32 {
    unit_ref(unit).map(|u| u.stability_score).unwrap_or(0.0)
}

// =============================================================================
// Queries
// =============================================================================

#[no_mangle]
pub extern "C" fn acb_query_options_new() -> *mut AcbQueryOptions {
    Box::into_raw(Box::new(AcbQueryOptions {
        max_results: 100,
        depth: 5,
    }))
}

#[no_mangle]
pub extern "C" fn acb_query_options_free(options: *mut AcbQueryOptions) {
    if !options.is_null() {
        unsafe { drop(Box::from_raw(options)) };
    }
}

#[no_mangle]
pub extern "C" fn acb_query_symbol_lookup(
    graph: *const AcbGraph,
    name: *const c_char,
    options: *const AcbQueryOptions,
) -> *mut AcbQueryResult {
    clear_last_error();
    
    if graph.is_null() || name.is_null() {
        set_last_error("null argument");
        return ptr::null_mut();
    }
    
    let name_str = unsafe { CStr::from_ptr(name) };
    let name = match name_str.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_last_error(e);
            return ptr::null_mut();
        }
    };
    
    let graph = unsafe { &(*graph).inner };
    let engine = QueryEngine::new();
    
    let params = SymbolLookupParams {
        name: name.to_string(),
        mode: MatchMode::Contains,
        unit_types: vec![],
        languages: vec![],
        limit: if options.is_null() { 100 } else { unsafe { (*options).max_results } },
    };
    
    match engine.symbol_lookup(graph, params) {
        Ok(units) => {
            let result = AcbQueryResult {
                unit_ids: units.iter().map(|u| u.id).collect(),
                scores: vec![1.0; units.len()],
                error: None,
            };
            Box::into_raw(Box::new(result))
        }
        Err(e) => {
            let result = AcbQueryResult {
                unit_ids: vec![],
                scores: vec![],
                error: Some(e.to_string()),
            };
            Box::into_raw(Box::new(result))
        }
    }
}

#[no_mangle]
pub extern "C" fn acb_query_result_free(result: *mut AcbQueryResult) {
    if !result.is_null() {
        unsafe { drop(Box::from_raw(result)) };
    }
}

#[no_mangle]
pub extern "C" fn acb_query_result_count(result: *const AcbQueryResult) -> usize {
    if result.is_null() {
        return 0;
    }
    unsafe { (*result).unit_ids.len() }
}

#[no_mangle]
pub extern "C" fn acb_query_result_unit_id(
    result: *const AcbQueryResult,
    index: usize,
) -> u64 {
    if result.is_null() {
        return 0;
    }
    unsafe { (*result).unit_ids.get(index).copied().unwrap_or(0) }
}

#[no_mangle]
pub extern "C" fn acb_query_result_score(
    result: *const AcbQueryResult,
    index: usize,
) -> f32 {
    if result.is_null() {
        return 0.0;
    }
    unsafe { (*result).scores.get(index).copied().unwrap_or(0.0) }
}

// =============================================================================
// Temporal Analysis (returns JSON)
// =============================================================================

#[no_mangle]
pub extern "C" fn acb_analyze_stability(
    graph: *const AcbGraph,
    unit_id: u64,
) -> *mut c_char {
    if graph.is_null() {
        return ptr::null_mut();
    }
    
    let graph = unsafe { &(*graph).inner };
    
    // Get stability analysis
    // This would call the actual stability analyzer and serialize to JSON
    let json = r#"{"score": 0.75, "factors": []}"#;
    
    match CString::new(json) {
        Ok(s) => s.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn acb_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}

// =============================================================================
// Error Enum
// =============================================================================

#[repr(C)]
pub enum AcbError {
    ACB_OK = 0,
    ACB_ERR_NULL_POINTER = 1,
    ACB_ERR_INVALID_PATH = 2,
    ACB_ERR_PARSE_ERROR = 3,
    ACB_ERR_FILE_NOT_FOUND = 4,
    ACB_ERR_INVALID_FORMAT = 5,
    ACB_ERR_UNIT_NOT_FOUND = 6,
    ACB_ERR_QUERY_FAILED = 7,
    ACB_ERR_IO_ERROR = 8,
    ACB_ERR_OUT_OF_MEMORY = 9,
    ACB_ERR_INVALID_ARGUMENT = 10,
    ACB_ERR_GIT_ERROR = 11,
    ACB_ERR_UNKNOWN = 255,
}

#[repr(C)]
pub enum AcbUnitType {
    ACB_UNIT_MODULE = 0,
    ACB_UNIT_SYMBOL = 1,
    ACB_UNIT_TYPE = 2,
    ACB_UNIT_FUNCTION = 3,
    ACB_UNIT_PARAMETER = 4,
    ACB_UNIT_IMPORT = 5,
    ACB_UNIT_TEST = 6,
    ACB_UNIT_DOC = 7,
    ACB_UNIT_CONFIG = 8,
    ACB_UNIT_PATTERN = 9,
    ACB_UNIT_TRAIT = 10,
    ACB_UNIT_IMPL = 11,
    ACB_UNIT_MACRO = 12,
}

#[repr(C)]
pub enum AcbEdgeType {
    ACB_EDGE_CALLS = 0,
    ACB_EDGE_IMPORTS = 1,
    // ... etc
}

#[repr(C)]
pub enum AcbLanguage {
    ACB_LANG_PYTHON = 0,
    ACB_LANG_RUST = 1,
    ACB_LANG_TYPESCRIPT = 2,
    ACB_LANG_JAVASCRIPT = 3,
    ACB_LANG_GO = 4,
    ACB_LANG_UNKNOWN = 255,
}

// Opaque type aliases for C
pub type AcbCodeUnit = CodeUnit;
pub type AcbEdge = Edge;
```

---

## Cargo.toml Addition

```toml
[lib]
crate-type = ["rlib", "cdylib", "staticlib"]

[features]
ffi = []
```

---

## Python Bindings Example

```python
# python/agentic_codebase.py

import ctypes
from ctypes import c_char_p, c_uint64, c_uint32, c_float, c_int, c_size_t, POINTER, Structure
from pathlib import Path

# Load the library
_lib = ctypes.CDLL("libagentic_codebase.so")

# Define types
class AcbGraph(Structure):
    pass

class AcbQueryResult(Structure):
    pass

AcbGraphPtr = POINTER(AcbGraph)
AcbQueryResultPtr = POINTER(AcbQueryResult)

# Define functions
_lib.acb_version.restype = c_char_p
_lib.acb_last_error.restype = c_char_p

_lib.acb_compile.argtypes = [c_char_p, ctypes.c_void_p]
_lib.acb_compile.restype = AcbGraphPtr

_lib.acb_load.argtypes = [c_char_p]
_lib.acb_load.restype = AcbGraphPtr

_lib.acb_graph_free.argtypes = [AcbGraphPtr]
_lib.acb_graph_unit_count.argtypes = [AcbGraphPtr]
_lib.acb_graph_unit_count.restype = c_uint64

_lib.acb_query_symbol_lookup.argtypes = [AcbGraphPtr, c_char_p, ctypes.c_void_p]
_lib.acb_query_symbol_lookup.restype = AcbQueryResultPtr

_lib.acb_query_result_count.argtypes = [AcbQueryResultPtr]
_lib.acb_query_result_count.restype = c_size_t

_lib.acb_query_result_unit_id.argtypes = [AcbQueryResultPtr, c_size_t]
_lib.acb_query_result_unit_id.restype = c_uint64

_lib.acb_query_result_free.argtypes = [AcbQueryResultPtr]


class CodeGraph:
    """Python wrapper for AgenticCodebase graphs."""
    
    def __init__(self, ptr: AcbGraphPtr):
        self._ptr = ptr
    
    def __del__(self):
        if self._ptr:
            _lib.acb_graph_free(self._ptr)
    
    @classmethod
    def compile(cls, path: str | Path) -> "CodeGraph":
        """Compile a repository into a code graph."""
        path_bytes = str(path).encode("utf-8")
        ptr = _lib.acb_compile(path_bytes, None)
        if not ptr:
            error = _lib.acb_last_error()
            raise RuntimeError(f"Compilation failed: {error.decode() if error else 'unknown'}")
        return cls(ptr)
    
    @classmethod
    def load(cls, path: str | Path) -> "CodeGraph":
        """Load a graph from an .acb file."""
        path_bytes = str(path).encode("utf-8")
        ptr = _lib.acb_load(path_bytes)
        if not ptr:
            error = _lib.acb_last_error()
            raise RuntimeError(f"Load failed: {error.decode() if error else 'unknown'}")
        return cls(ptr)
    
    @property
    def unit_count(self) -> int:
        return _lib.acb_graph_unit_count(self._ptr)
    
    def lookup(self, name: str) -> list[int]:
        """Look up units by name."""
        name_bytes = name.encode("utf-8")
        result = _lib.acb_query_symbol_lookup(self._ptr, name_bytes, None)
        if not result:
            return []
        
        try:
            count = _lib.acb_query_result_count(result)
            return [_lib.acb_query_result_unit_id(result, i) for i in range(count)]
        finally:
            _lib.acb_query_result_free(result)


def version() -> str:
    return _lib.acb_version().decode("utf-8")


# Usage example:
if __name__ == "__main__":
    print(f"AgenticCodebase version: {version()}")
    
    graph = CodeGraph.compile("./my-project")
    print(f"Compiled {graph.unit_count} units")
    
    results = graph.lookup("process_payment")
    print(f"Found {len(results)} matches")
```

---

## Build Instructions

```bash
# Build shared library
cargo build --release --features ffi

# The library will be at:
# - Linux: target/release/libagentic_codebase.so
# - macOS: target/release/libagentic_codebase.dylib
# - Windows: target/release/agentic_codebase.dll

# Generate header (using cbindgen)
cbindgen --config cbindgen.toml --crate agentic-codebase --output ffi/agentic_codebase.h
```

---

## cbindgen.toml

```toml
language = "C"
header = "/* AgenticCodebase FFI - Auto-generated, do not edit */"
include_guard = "AGENTIC_CODEBASE_H"
autogen_warning = "/* Warning: this file was auto-generated by cbindgen. */"
include_version = true
braces = "SameLine"
line_length = 100
tab_width = 4
documentation = true

[export]
include = ["AcbError", "AcbUnitType", "AcbEdgeType", "AcbLanguage"]

[export.rename]
"AcbError" = "AcbError"

[enum]
rename_variants = "ScreamingSnakeCase"
```

---

## Test FFI

```c
// ffi/test_ffi.c

#include <stdio.h>
#include <assert.h>
#include "agentic_codebase.h"

int main() {
    printf("Testing AgenticCodebase FFI\n");
    
    // Test version
    const char* version = acb_version();
    printf("Version: %s\n", version);
    assert(version != NULL);
    
    // Test compile
    AcbGraph* graph = acb_compile("./testdata/python/", NULL);
    if (graph == NULL) {
        printf("Compile failed: %s\n", acb_last_error());
        return 1;
    }
    
    printf("Unit count: %lu\n", acb_graph_unit_count(graph));
    assert(acb_graph_unit_count(graph) > 0);
    
    // Test query
    AcbQueryResult* result = acb_query_symbol_lookup(graph, "test", NULL);
    assert(result != NULL);
    
    size_t count = acb_query_result_count(result);
    printf("Found %zu results for 'test'\n", count);
    
    for (size_t i = 0; i < count; i++) {
        uint64_t id = acb_query_result_unit_id(result, i);
        printf("  - Unit %lu\n", id);
    }
    
    acb_query_result_free(result);
    acb_graph_free(graph);
    
    printf("All tests passed!\n");
    return 0;
}
```

Compile and run:
```bash
gcc -o test_ffi ffi/test_ffi.c -L target/release -lagentic_codebase -lpthread -ldl -lm
LD_LIBRARY_PATH=target/release ./test_ffi
```
