"""AgenticCodebase — Semantic code compiler for AI agents.

Public API
----------

Models (pure Python — always available):

* :class:`CodeUnit`, :class:`Edge`, :class:`GraphInfo`, :class:`ImpactResult`
* :class:`UnitType`, :class:`EdgeType`, :class:`Language`
* :func:`unit_type_from_u8`, :func:`edge_type_from_u8`, :func:`language_from_u8`
* :func:`parse_code_unit`, :func:`parse_graph_info`

Errors:

* :class:`AcbError` (base), :class:`GraphNotFoundError`,
  :class:`UnitNotFoundError`, :class:`CompileError`, :class:`StorageError`
* :class:`LibraryNotFoundError`, :class:`ValidationError`,
  :class:`OverflowError`

High-level wrapper (requires native library):

* :class:`CodebaseGraph`
"""

from __future__ import annotations

__version__ = "0.1.0"

# -- errors (always available) --------------------------------------------
from .errors import (
    AcbError,
    CompileError,
    GraphNotFoundError,
    LibraryNotFoundError,
    OverflowError,
    StorageError,
    UnitNotFoundError,
    ValidationError,
)

# -- models (always available) --------------------------------------------
from .models import (
    CodeUnit,
    Edge,
    EdgeType,
    GraphInfo,
    ImpactResult,
    Language,
    UnitType,
    edge_type_from_u8,
    language_from_u8,
    parse_code_unit,
    parse_graph_info,
    unit_type_from_u8,
)

# -- high-level wrapper (requires native library) -------------------------
from .graph import CodebaseGraph

__all__ = [
    # package
    "__version__",
    # errors
    "AcbError",
    "CompileError",
    "GraphNotFoundError",
    "LibraryNotFoundError",
    "OverflowError",
    "StorageError",
    "UnitNotFoundError",
    "ValidationError",
    # models
    "CodeUnit",
    "Edge",
    "EdgeType",
    "GraphInfo",
    "ImpactResult",
    "Language",
    "UnitType",
    "edge_type_from_u8",
    "language_from_u8",
    "parse_code_unit",
    "parse_graph_info",
    "unit_type_from_u8",
    # high-level
    "CodebaseGraph",
]
