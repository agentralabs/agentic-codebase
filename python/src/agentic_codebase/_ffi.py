"""Low-level ctypes bindings for ``libagentic_codebase``.

This module loads the shared library and declares the C function signatures
exactly as exported by the Rust ``agentic-codebase`` crate's FFI module.

All memory management rules:

* Handles returned by :func:`acb_graph_open` **must** be freed with
  :func:`acb_graph_free`.
* Buffer-based functions write into caller-owned memory and return the
  number of bytes written (or a negative error code).
"""

from __future__ import annotations

import ctypes
import ctypes.util
import os
import platform
import sys
from pathlib import Path
from typing import Optional

from .errors import AcbError, LibraryNotFoundError

# ---------------------------------------------------------------------------
# Error codes (mirrored from the Rust FFI crate)
# ---------------------------------------------------------------------------

ACB_OK: int = 0
ACB_ERR_IO: int = -1
ACB_ERR_INVALID: int = -2
ACB_ERR_NOT_FOUND: int = -3
ACB_ERR_OVERFLOW: int = -4
ACB_ERR_NULL_PTR: int = -5

_ERROR_MESSAGES: dict[int, str] = {
    ACB_ERR_IO: "A filesystem I/O operation failed",
    ACB_ERR_INVALID: "An invalid argument was provided",
    ACB_ERR_NOT_FOUND: "The requested item was not found",
    ACB_ERR_OVERFLOW: "The output buffer was too small",
    ACB_ERR_NULL_PTR: "A required pointer argument was null",
}

# ---------------------------------------------------------------------------
# Library loading
# ---------------------------------------------------------------------------


def _lib_filename() -> str:
    """Return the platform-specific shared library filename."""
    system = platform.system()
    if system == "Darwin":
        return "libagentic_codebase.dylib"
    elif system == "Windows":
        return "agentic_codebase.dll"
    else:
        return "libagentic_codebase.so"


def _find_library() -> str:
    """Locate the native shared library.

    Search order:

    1. ``AGENTIC_CODEBASE_LIB`` environment variable (explicit path).
    2. ``../target/release/`` relative to this package (development build).
    3. ``../target/debug/`` relative to this package (development build).
    4. System library search path via :func:`ctypes.util.find_library`.
    """
    # 1. Explicit env var.
    env_path = os.environ.get("AGENTIC_CODEBASE_LIB")
    if env_path and os.path.isfile(env_path):
        return env_path

    lib_name = _lib_filename()

    # 2-3. Relative to the repository root.
    repo_root = Path(__file__).resolve().parent.parent.parent.parent
    for profile in ("release", "debug"):
        candidate = repo_root / "target" / profile / lib_name
        if candidate.is_file():
            return str(candidate)

    # 4. System search path.
    found = ctypes.util.find_library("agentic_codebase")
    if found:
        return found

    raise LibraryNotFoundError(
        [str(repo_root / "target" / p / lib_name) for p in ("release", "debug")]
    )


def _load_library() -> ctypes.CDLL:
    """Load the shared library and declare all C function signatures."""
    lib = ctypes.CDLL(_find_library())

    # -- acb_graph_open ----------------------------------------------------
    lib.acb_graph_open.argtypes = [ctypes.c_char_p]
    lib.acb_graph_open.restype = ctypes.c_void_p

    # -- acb_graph_free ----------------------------------------------------
    lib.acb_graph_free.argtypes = [ctypes.c_void_p]
    lib.acb_graph_free.restype = None

    # -- acb_graph_unit_count ----------------------------------------------
    lib.acb_graph_unit_count.argtypes = [ctypes.c_void_p]
    lib.acb_graph_unit_count.restype = ctypes.c_uint64

    # -- acb_graph_edge_count ----------------------------------------------
    lib.acb_graph_edge_count.argtypes = [ctypes.c_void_p]
    lib.acb_graph_edge_count.restype = ctypes.c_uint64

    # -- acb_graph_dimension -----------------------------------------------
    lib.acb_graph_dimension.argtypes = [ctypes.c_void_p]
    lib.acb_graph_dimension.restype = ctypes.c_uint32

    # -- acb_graph_get_unit_name -------------------------------------------
    lib.acb_graph_get_unit_name.argtypes = [
        ctypes.c_void_p,  # graph
        ctypes.c_uint64,  # unit_id
        ctypes.c_char_p,  # buffer
        ctypes.c_uint32,  # buffer_size
    ]
    lib.acb_graph_get_unit_name.restype = ctypes.c_int32

    # -- acb_graph_get_unit_type -------------------------------------------
    lib.acb_graph_get_unit_type.argtypes = [
        ctypes.c_void_p,  # graph
        ctypes.c_uint64,  # unit_id
    ]
    lib.acb_graph_get_unit_type.restype = ctypes.c_int32

    # -- acb_graph_get_unit_file -------------------------------------------
    lib.acb_graph_get_unit_file.argtypes = [
        ctypes.c_void_p,  # graph
        ctypes.c_uint64,  # unit_id
        ctypes.c_char_p,  # buffer
        ctypes.c_uint32,  # buffer_size
    ]
    lib.acb_graph_get_unit_file.restype = ctypes.c_int32

    # -- acb_graph_get_unit_complexity -------------------------------------
    lib.acb_graph_get_unit_complexity.argtypes = [
        ctypes.c_void_p,  # graph
        ctypes.c_uint64,  # unit_id
    ]
    lib.acb_graph_get_unit_complexity.restype = ctypes.c_float

    # -- acb_graph_get_unit_language ----------------------------------------
    lib.acb_graph_get_unit_language.argtypes = [
        ctypes.c_void_p,  # graph
        ctypes.c_uint64,  # unit_id
    ]
    lib.acb_graph_get_unit_language.restype = ctypes.c_int32

    # -- acb_graph_get_unit_stability --------------------------------------
    lib.acb_graph_get_unit_stability.argtypes = [
        ctypes.c_void_p,  # graph
        ctypes.c_uint64,  # unit_id
    ]
    lib.acb_graph_get_unit_stability.restype = ctypes.c_float

    # -- acb_graph_get_edges -----------------------------------------------
    lib.acb_graph_get_edges.argtypes = [
        ctypes.c_void_p,            # graph
        ctypes.c_uint64,            # unit_id
        ctypes.POINTER(ctypes.c_uint64),  # target_ids
        ctypes.POINTER(ctypes.c_uint8),   # edge_types
        ctypes.POINTER(ctypes.c_float),   # weights
        ctypes.c_uint32,            # max_edges
    ]
    lib.acb_graph_get_edges.restype = ctypes.c_int32

    return lib


# Singleton: loaded once on first import.
_lib: Optional[ctypes.CDLL] = None


def _get_lib() -> ctypes.CDLL:
    """Get or lazily load the shared library."""
    global _lib
    if _lib is None:
        _lib = _load_library()
    return _lib


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------


def _check(rc: int) -> None:
    """Raise :class:`AcbError` if *rc* is a negative error code."""
    if rc < 0:
        msg = _ERROR_MESSAGES.get(rc, f"Unknown FFI error code {rc}")
        raise AcbError(msg, code=rc)
