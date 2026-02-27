---
status: stable
---

# FFI Reference

AgenticCodebase exposes a C-compatible FFI layer through the `ffi::c_api` module. This enables integration from any language that supports C function calls (Python ctypes, Node.js ffi-napi, Ruby FFI, Go cgo, etc.).

## Shared Library

Build the shared library:

```bash
cargo build --release -p agentic-codebase
# Output: target/release/libagentic_codebase.{so,dylib,dll}
```

The core crate is configured with `crate-type = ["lib", "cdylib", "staticlib"]`, so it produces both dynamic and static libraries.

## Error Codes

| Constant | Value | Description |
|----------|-------|-------------|
| `ACB_OK` | `0` | Success |
| `ACB_ERR_IO` | `-1` | I/O error |
| `ACB_ERR_INVALID` | `-2` | Invalid argument |
| `ACB_ERR_NOT_FOUND` | `-3` | Unit not found |
| `ACB_ERR_OVERFLOW` | `-4` | Buffer overflow (buffer too small) |
| `ACB_ERR_NULL_PTR` | `-5` | Null pointer passed |

## Functions

### `acb_graph_open`

Load a code graph from an `.acb` file.

```c
void* acb_graph_open(const char* path);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | `const char*` | Path to `.acb` file (must be valid, non-null, null-terminated) |

**Returns:** Opaque handle pointer, or `NULL` on error.

### `acb_graph_free`

Free a graph handle.

```c
void acb_graph_free(void* graph);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `graph` | `void*` | Handle from `acb_graph_open`, or null (no-op) |

Must not be called more than once for the same handle.

### `acb_graph_unit_count`

Get the number of code units in the graph.

```c
uint64_t acb_graph_unit_count(void* graph);
```

**Returns:** Unit count, or `0` if graph is null.

### `acb_graph_edge_count`

Get the number of edges in the graph.

```c
uint64_t acb_graph_edge_count(void* graph);
```

**Returns:** Edge count, or `0` if graph is null.

### `acb_graph_dimension`

Get the embedding dimension.

```c
uint32_t acb_graph_dimension(void* graph);
```

**Returns:** Dimension, or `0` if graph is null.

### `acb_graph_get_unit_name`

Get a unit's name. Writes a null-terminated string to the buffer.

```c
int32_t acb_graph_get_unit_name(
    void* graph,
    uint64_t unit_id,
    char* buffer,
    uint32_t buffer_size
);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `graph` | `void*` | Graph handle |
| `unit_id` | `uint64_t` | Code unit ID |
| `buffer` | `char*` | Output buffer for the name |
| `buffer_size` | `uint32_t` | Size of the buffer in bytes |

**Returns:** Name length on success, or error code (`ACB_ERR_NOT_FOUND`, `ACB_ERR_OVERFLOW`, `ACB_ERR_NULL_PTR`).

### `acb_graph_get_unit_type`

Get a unit's type as an integer.

```c
int32_t acb_graph_get_unit_type(void* graph, uint64_t unit_id);
```

**Returns:** Unit type as `i32`, or `-1` if not found.

### `acb_graph_get_unit_file`

Get a unit's file path. Writes a null-terminated string to the buffer.

```c
int32_t acb_graph_get_unit_file(
    void* graph,
    uint64_t unit_id,
    char* buffer,
    uint32_t buffer_size
);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `graph` | `void*` | Graph handle |
| `unit_id` | `uint64_t` | Code unit ID |
| `buffer` | `char*` | Output buffer for the file path |
| `buffer_size` | `uint32_t` | Size of the buffer in bytes |

**Returns:** Path length on success, or error code.

### `acb_graph_get_unit_complexity`

Get a unit's complexity score.

```c
float acb_graph_get_unit_complexity(void* graph, uint64_t unit_id);
```

**Returns:** Complexity score (>= 0.0), or `-1.0` if not found.

### `acb_graph_get_unit_stability`

Get a unit's stability score.

```c
float acb_graph_get_unit_stability(void* graph, uint64_t unit_id);
```

**Returns:** Stability score (0.0 to 1.0), or `-1.0` if not found.

### `acb_graph_get_unit_language`

Get a unit's language as an integer.

```c
int32_t acb_graph_get_unit_language(void* graph, uint64_t unit_id);
```

**Returns:** Language as `i32`, or `-1` if not found.

### `acb_graph_get_edges`

Get outgoing edges from a unit. Writes to parallel output arrays.

```c
int32_t acb_graph_get_edges(
    void* graph,
    uint64_t unit_id,
    uint64_t* target_ids,
    uint8_t* edge_types,
    float* weights,
    uint32_t max_edges
);
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `graph` | `void*` | Graph handle |
| `unit_id` | `uint64_t` | Source unit ID |
| `target_ids` | `uint64_t*` | Output array for target unit IDs |
| `edge_types` | `uint8_t*` | Output array for edge type codes |
| `weights` | `float*` | Output array for edge weights |
| `max_edges` | `uint32_t` | Maximum edges to return (array capacity) |

**Returns:** Number of edges written, or error code.

## Example: Python ctypes

```python
import ctypes

lib = ctypes.CDLL("libagentic_codebase.dylib")

lib.acb_graph_open.restype = ctypes.c_void_p
lib.acb_graph_open.argtypes = [ctypes.c_char_p]

lib.acb_graph_unit_count.restype = ctypes.c_uint64
lib.acb_graph_unit_count.argtypes = [ctypes.c_void_p]

lib.acb_graph_free.argtypes = [ctypes.c_void_p]

handle = lib.acb_graph_open(b"project.acb")
if handle:
    count = lib.acb_graph_unit_count(handle)
    print(f"Units: {count}")
    lib.acb_graph_free(handle)
```

## Example: Reading a Unit Name

```python
lib.acb_graph_get_unit_name.restype = ctypes.c_int32
lib.acb_graph_get_unit_name.argtypes = [
    ctypes.c_void_p, ctypes.c_uint64, ctypes.c_char_p, ctypes.c_uint32
]

buf = ctypes.create_string_buffer(1024)
result = lib.acb_graph_get_unit_name(handle, 42, buf, 1024)
if result >= 0:
    print(f"Unit name: {buf.value.decode()}")
```

## Thread Safety

All FFI functions are thread-safe when called with different handles. All functions use `panic::catch_unwind` to prevent Rust panics from crossing the FFI boundary. Concurrent access to the same handle from multiple threads requires external synchronization.
