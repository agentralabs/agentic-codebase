"""Data models for the agentic_codebase package.

All models are frozen dataclasses — immutable and thread-safe.
Enums use ``(str, Enum)`` for natural JSON serialization.
"""

from __future__ import annotations

import dataclasses
from dataclasses import dataclass, field
from enum import Enum
from typing import Optional


# ---------------------------------------------------------------------------
# Enums
# ---------------------------------------------------------------------------


class UnitType(str, Enum):
    """Type of code unit stored in a graph node."""

    MODULE = "module"
    SYMBOL = "symbol"
    TYPE = "type"
    FUNCTION = "function"
    PARAMETER = "parameter"
    IMPORT = "import"
    TEST = "test"
    DOC = "doc"
    CONFIG = "config"
    PATTERN = "pattern"
    TRAIT = "trait"
    IMPL = "impl"
    MACRO = "macro"


class EdgeType(str, Enum):
    """Type of directed relationship between code units."""

    CALLS = "calls"
    IMPORTS = "imports"
    INHERITS = "inherits"
    IMPLEMENTS = "implements"
    OVERRIDES = "overrides"
    CONTAINS = "contains"
    REFERENCES = "references"
    TESTS = "tests"
    DOCUMENTS = "documents"
    CONFIGURES = "configures"
    COUPLES_WITH = "couples_with"
    BREAKS_WITH = "breaks_with"
    PATTERN_OF = "pattern_of"
    VERSION_OF = "version_of"
    FFI_BINDS = "ffi_binds"
    USES_TYPE = "uses_type"
    RETURNS = "returns"
    PARAM_TYPE = "param_type"


class Language(str, Enum):
    """Supported programming languages."""

    PYTHON = "Python"
    RUST = "Rust"
    TYPESCRIPT = "TypeScript"
    JAVASCRIPT = "JavaScript"
    GO = "Go"
    UNKNOWN = "Unknown"


# ---------------------------------------------------------------------------
# Reverse lookup tables for FFI u8 ↔ enum conversion
# ---------------------------------------------------------------------------

_UNIT_TYPE_FROM_U8: dict[int, UnitType] = {
    0: UnitType.MODULE,
    1: UnitType.SYMBOL,
    2: UnitType.TYPE,
    3: UnitType.FUNCTION,
    4: UnitType.PARAMETER,
    5: UnitType.IMPORT,
    6: UnitType.TEST,
    7: UnitType.DOC,
    8: UnitType.CONFIG,
    9: UnitType.PATTERN,
    10: UnitType.TRAIT,
    11: UnitType.IMPL,
    12: UnitType.MACRO,
}

_EDGE_TYPE_FROM_U8: dict[int, EdgeType] = {
    0: EdgeType.CALLS,
    1: EdgeType.IMPORTS,
    2: EdgeType.INHERITS,
    3: EdgeType.IMPLEMENTS,
    4: EdgeType.OVERRIDES,
    5: EdgeType.CONTAINS,
    6: EdgeType.REFERENCES,
    7: EdgeType.TESTS,
    8: EdgeType.DOCUMENTS,
    9: EdgeType.CONFIGURES,
    10: EdgeType.COUPLES_WITH,
    11: EdgeType.BREAKS_WITH,
    12: EdgeType.PATTERN_OF,
    13: EdgeType.VERSION_OF,
    14: EdgeType.FFI_BINDS,
    15: EdgeType.USES_TYPE,
    16: EdgeType.RETURNS,
    17: EdgeType.PARAM_TYPE,
}

_LANGUAGE_FROM_U8: dict[int, Language] = {
    0: Language.PYTHON,
    1: Language.RUST,
    2: Language.TYPESCRIPT,
    3: Language.JAVASCRIPT,
    4: Language.GO,
    255: Language.UNKNOWN,
}


# ---------------------------------------------------------------------------
# Core data models
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class CodeUnit:
    """A single code unit (node) in the code graph."""

    id: int = 0
    name: str = ""
    unit_type: UnitType = UnitType.SYMBOL
    file_path: str = ""
    language: Language = Language.UNKNOWN
    complexity: float = 0.0
    stability: float = 0.0


@dataclass(frozen=True)
class Edge:
    """A directed edge between two code units."""

    source_id: int = 0
    target_id: int = 0
    edge_type: EdgeType = EdgeType.REFERENCES
    weight: float = 1.0

    @property
    def is_dependency(self) -> bool:
        """True if this edge represents a compile-time dependency."""
        return self.edge_type in {
            EdgeType.CALLS,
            EdgeType.IMPORTS,
            EdgeType.INHERITS,
            EdgeType.IMPLEMENTS,
            EdgeType.USES_TYPE,
            EdgeType.FFI_BINDS,
        }

    @property
    def is_temporal(self) -> bool:
        """True if this edge was derived from git history analysis."""
        return self.edge_type in {
            EdgeType.COUPLES_WITH,
            EdgeType.BREAKS_WITH,
            EdgeType.VERSION_OF,
        }


@dataclass(frozen=True)
class GraphInfo:
    """Summary statistics for a loaded code graph."""

    path: str = ""
    unit_count: int = 0
    edge_count: int = 0
    dimension: int = 0

    @property
    def is_empty(self) -> bool:
        """True if the graph has no code units."""
        return self.unit_count == 0


@dataclass(frozen=True)
class ImpactResult:
    """Result of an impact analysis query."""

    root_id: int = 0
    affected: tuple[int, ...] = ()
    max_depth: int = 0

    @property
    def count(self) -> int:
        """Number of affected code units."""
        return len(self.affected)


# ---------------------------------------------------------------------------
# Parsing helpers
# ---------------------------------------------------------------------------


def unit_type_from_u8(value: int) -> UnitType:
    """Convert an FFI ``u8`` to :class:`UnitType`."""
    return _UNIT_TYPE_FROM_U8.get(value, UnitType.SYMBOL)


def edge_type_from_u8(value: int) -> EdgeType:
    """Convert an FFI ``u8`` to :class:`EdgeType`."""
    return _EDGE_TYPE_FROM_U8.get(value, EdgeType.REFERENCES)


def language_from_u8(value: int) -> Language:
    """Convert an FFI ``u8`` to :class:`Language`."""
    return _LANGUAGE_FROM_U8.get(value, Language.UNKNOWN)


def parse_code_unit(data: dict) -> CodeUnit:  # type: ignore[type-arg]
    """Build a :class:`CodeUnit` from a raw JSON dict."""
    return CodeUnit(
        id=data.get("id", 0),
        name=data.get("name", ""),
        unit_type=UnitType(data["unit_type"]) if "unit_type" in data else UnitType.SYMBOL,
        file_path=data.get("file_path", ""),
        language=Language(data["language"]) if "language" in data else Language.UNKNOWN,
        complexity=data.get("complexity", 0.0),
        stability=data.get("stability", 0.0),
    )


def parse_graph_info(data: dict, path: str = "") -> GraphInfo:  # type: ignore[type-arg]
    """Build a :class:`GraphInfo` from a raw JSON dict."""
    return GraphInfo(
        path=path or data.get("path", ""),
        unit_count=data.get("unit_count", 0),
        edge_count=data.get("edge_count", 0),
        dimension=data.get("dimension", 0),
    )
