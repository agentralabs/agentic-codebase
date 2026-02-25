"""Tests for package-level imports and metadata.

These tests verify that the public API surface is correctly exported
and that the package metadata (version, __all__) is valid.
"""

from __future__ import annotations


class TestImports:
    def test_import_package(self) -> None:
        import agentic_codebase

        assert hasattr(agentic_codebase, "__version__")

    def test_version_is_semver(self) -> None:
        from agentic_codebase import __version__

        parts = __version__.split(".")
        assert len(parts) == 3
        for part in parts:
            assert part.isdigit()

    def test_acb_error_importable(self) -> None:
        from agentic_codebase import AcbError

        assert issubclass(AcbError, Exception)

    def test_unit_type_importable(self) -> None:
        from agentic_codebase import UnitType

        assert UnitType.FUNCTION == "function"

    def test_edge_type_importable(self) -> None:
        from agentic_codebase import EdgeType

        assert EdgeType.CALLS == "calls"

    def test_language_importable(self) -> None:
        from agentic_codebase import Language

        assert Language.PYTHON == "Python"

    def test_models_importable(self) -> None:
        from agentic_codebase import (
            CodeUnit,
            Edge,
            GraphInfo,
            ImpactResult,
        )

        assert CodeUnit is not None
        assert Edge is not None
        assert GraphInfo is not None
        assert ImpactResult is not None

    def test_errors_importable(self) -> None:
        from agentic_codebase import (
            AcbError,
            CompileError,
            GraphNotFoundError,
            LibraryNotFoundError,
            StorageError,
            UnitNotFoundError,
            ValidationError,
        )

        assert AcbError is not None
        assert CompileError is not None
        assert GraphNotFoundError is not None
        assert LibraryNotFoundError is not None
        assert StorageError is not None
        assert UnitNotFoundError is not None
        assert ValidationError is not None

    def test_parse_helpers_importable(self) -> None:
        from agentic_codebase import (
            edge_type_from_u8,
            language_from_u8,
            parse_code_unit,
            parse_graph_info,
            unit_type_from_u8,
        )

        assert callable(edge_type_from_u8)
        assert callable(language_from_u8)
        assert callable(parse_code_unit)
        assert callable(parse_graph_info)
        assert callable(unit_type_from_u8)

    def test_codebase_graph_importable(self) -> None:
        from agentic_codebase import CodebaseGraph

        assert CodebaseGraph is not None


class TestAll:
    def test_all_items_are_importable(self) -> None:
        import agentic_codebase

        for name in agentic_codebase.__all__:
            assert hasattr(agentic_codebase, name), f"{name} in __all__ but not importable"

    def test_all_is_nonempty(self) -> None:
        import agentic_codebase

        assert len(agentic_codebase.__all__) > 15
