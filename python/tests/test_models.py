"""Tests for the agentic_codebase.models module.

Validates frozen dataclasses, enums, and parsing helpers.
"""

from __future__ import annotations

import dataclasses

import pytest

from agentic_codebase.models import (
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


# ---------------------------------------------------------------------------
# Enums
# ---------------------------------------------------------------------------


class TestUnitType:
    def test_values(self) -> None:
        assert UnitType.MODULE == "module"
        assert UnitType.SYMBOL == "symbol"
        assert UnitType.TYPE == "type"
        assert UnitType.FUNCTION == "function"
        assert UnitType.PARAMETER == "parameter"
        assert UnitType.IMPORT == "import"
        assert UnitType.TEST == "test"
        assert UnitType.DOC == "doc"
        assert UnitType.CONFIG == "config"
        assert UnitType.PATTERN == "pattern"
        assert UnitType.TRAIT == "trait"
        assert UnitType.IMPL == "impl"
        assert UnitType.MACRO == "macro"

    def test_is_str(self) -> None:
        assert isinstance(UnitType.FUNCTION, str)

    def test_from_string(self) -> None:
        assert UnitType("function") == UnitType.FUNCTION

    def test_all_members(self) -> None:
        assert len(UnitType) == 13


class TestEdgeType:
    def test_values(self) -> None:
        assert EdgeType.CALLS == "calls"
        assert EdgeType.IMPORTS == "imports"
        assert EdgeType.INHERITS == "inherits"
        assert EdgeType.IMPLEMENTS == "implements"
        assert EdgeType.CONTAINS == "contains"
        assert EdgeType.TESTS == "tests"
        assert EdgeType.COUPLES_WITH == "couples_with"
        assert EdgeType.FFI_BINDS == "ffi_binds"

    def test_is_str(self) -> None:
        assert isinstance(EdgeType.CALLS, str)

    def test_all_members(self) -> None:
        assert len(EdgeType) == 18


class TestLanguage:
    def test_values(self) -> None:
        assert Language.PYTHON == "Python"
        assert Language.RUST == "Rust"
        assert Language.TYPESCRIPT == "TypeScript"
        assert Language.JAVASCRIPT == "JavaScript"
        assert Language.GO == "Go"
        assert Language.UNKNOWN == "Unknown"

    def test_is_str(self) -> None:
        assert isinstance(Language.PYTHON, str)

    def test_all_members(self) -> None:
        assert len(Language) == 6


# ---------------------------------------------------------------------------
# u8 conversion helpers
# ---------------------------------------------------------------------------


class TestU8Conversion:
    def test_unit_type_from_u8(self) -> None:
        assert unit_type_from_u8(0) == UnitType.MODULE
        assert unit_type_from_u8(3) == UnitType.FUNCTION
        assert unit_type_from_u8(12) == UnitType.MACRO
        assert unit_type_from_u8(99) == UnitType.SYMBOL  # fallback

    def test_edge_type_from_u8(self) -> None:
        assert edge_type_from_u8(0) == EdgeType.CALLS
        assert edge_type_from_u8(1) == EdgeType.IMPORTS
        assert edge_type_from_u8(14) == EdgeType.FFI_BINDS
        assert edge_type_from_u8(99) == EdgeType.REFERENCES  # fallback

    def test_language_from_u8(self) -> None:
        assert language_from_u8(0) == Language.PYTHON
        assert language_from_u8(1) == Language.RUST
        assert language_from_u8(4) == Language.GO
        assert language_from_u8(255) == Language.UNKNOWN
        assert language_from_u8(99) == Language.UNKNOWN  # fallback


# ---------------------------------------------------------------------------
# CodeUnit
# ---------------------------------------------------------------------------


class TestCodeUnit:
    def test_create(self) -> None:
        unit = CodeUnit(
            id=1,
            name="main",
            unit_type=UnitType.FUNCTION,
            file_path="src/main.rs",
            language=Language.RUST,
            complexity=5.0,
            stability=0.9,
        )
        assert unit.id == 1
        assert unit.name == "main"
        assert unit.unit_type == UnitType.FUNCTION
        assert unit.language == Language.RUST

    def test_defaults(self) -> None:
        unit = CodeUnit()
        assert unit.id == 0
        assert unit.name == ""
        assert unit.unit_type == UnitType.SYMBOL
        assert unit.language == Language.UNKNOWN

    def test_frozen(self) -> None:
        unit = CodeUnit(id=1)
        with pytest.raises(dataclasses.FrozenInstanceError):
            unit.id = 2  # type: ignore[misc]


# ---------------------------------------------------------------------------
# Edge
# ---------------------------------------------------------------------------


class TestEdge:
    def test_create(self) -> None:
        edge = Edge(
            source_id=1,
            target_id=2,
            edge_type=EdgeType.CALLS,
            weight=0.8,
        )
        assert edge.source_id == 1
        assert edge.target_id == 2
        assert edge.edge_type == EdgeType.CALLS

    def test_is_dependency_true(self) -> None:
        edge = Edge(edge_type=EdgeType.CALLS)
        assert edge.is_dependency is True

    def test_is_dependency_false(self) -> None:
        edge = Edge(edge_type=EdgeType.DOCUMENTS)
        assert edge.is_dependency is False

    def test_is_temporal_true(self) -> None:
        edge = Edge(edge_type=EdgeType.COUPLES_WITH)
        assert edge.is_temporal is True

    def test_is_temporal_false(self) -> None:
        edge = Edge(edge_type=EdgeType.CALLS)
        assert edge.is_temporal is False

    def test_frozen(self) -> None:
        edge = Edge()
        with pytest.raises(dataclasses.FrozenInstanceError):
            edge.weight = 0.5  # type: ignore[misc]


# ---------------------------------------------------------------------------
# GraphInfo
# ---------------------------------------------------------------------------


class TestGraphInfo:
    def test_create(self) -> None:
        info = GraphInfo(
            path="/tmp/project.acb",
            unit_count=1000,
            edge_count=5000,
            dimension=128,
        )
        assert info.unit_count == 1000
        assert info.edge_count == 5000
        assert info.is_empty is False

    def test_empty(self) -> None:
        info = GraphInfo()
        assert info.is_empty is True

    def test_frozen(self) -> None:
        info = GraphInfo()
        with pytest.raises(dataclasses.FrozenInstanceError):
            info.unit_count = 5  # type: ignore[misc]


# ---------------------------------------------------------------------------
# ImpactResult
# ---------------------------------------------------------------------------


class TestImpactResult:
    def test_create(self) -> None:
        result = ImpactResult(
            root_id=1,
            affected=(2, 3, 4, 5),
            max_depth=3,
        )
        assert result.root_id == 1
        assert result.count == 4

    def test_empty(self) -> None:
        result = ImpactResult()
        assert result.count == 0

    def test_frozen(self) -> None:
        result = ImpactResult()
        with pytest.raises(dataclasses.FrozenInstanceError):
            result.root_id = 1  # type: ignore[misc]


# ---------------------------------------------------------------------------
# Parsing helpers
# ---------------------------------------------------------------------------


class TestParseCodeUnit:
    def test_full_data(self) -> None:
        data = {
            "id": 42,
            "name": "process_request",
            "unit_type": "function",
            "file_path": "src/handler.py",
            "language": "Python",
            "complexity": 8.5,
            "stability": 0.7,
        }
        unit = parse_code_unit(data)
        assert unit.id == 42
        assert unit.name == "process_request"
        assert unit.unit_type == UnitType.FUNCTION
        assert unit.language == Language.PYTHON
        assert unit.complexity == 8.5

    def test_minimal_data(self) -> None:
        unit = parse_code_unit({})
        assert unit.id == 0
        assert unit.name == ""
        assert unit.unit_type == UnitType.SYMBOL
        assert unit.language == Language.UNKNOWN


class TestParseGraphInfo:
    def test_full_data(self) -> None:
        data = {
            "unit_count": 500,
            "edge_count": 2000,
            "dimension": 128,
        }
        info = parse_graph_info(data, path="/tmp/test.acb")
        assert info.path == "/tmp/test.acb"
        assert info.unit_count == 500
        assert info.is_empty is False

    def test_minimal_data(self) -> None:
        info = parse_graph_info({})
        assert info.unit_count == 0
        assert info.is_empty is True
