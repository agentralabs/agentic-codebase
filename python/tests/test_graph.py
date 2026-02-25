"""FFI integration tests for the CodebaseGraph wrapper.

These tests require the native ``libagentic_codebase`` shared library
and a compiled ``.acb`` graph file.

To generate test data::

    cargo build --release
    ./target/release/acb compile testdata/python -o /tmp/test-codebase.acb
"""

from __future__ import annotations

import os
import subprocess
import tempfile
from pathlib import Path

import pytest

# Skip the entire module if the native library is not available.
try:
    from agentic_codebase._ffi import _find_library

    _find_library()
    HAS_LIB = True
except Exception:
    HAS_LIB = False

pytestmark = pytest.mark.skipif(not HAS_LIB, reason="native library not available")

from agentic_codebase import CodebaseGraph, GraphNotFoundError, UnitNotFoundError
from agentic_codebase.models import EdgeType, Language, UnitType


# ---------------------------------------------------------------------------
# Fixtures
# ---------------------------------------------------------------------------

_ACB_PATH: str | None = None


def _ensure_acb() -> str:
    """Compile testdata if needed, return path to .acb file."""
    global _ACB_PATH
    if _ACB_PATH and Path(_ACB_PATH).exists():
        return _ACB_PATH

    repo_root = Path(__file__).resolve().parent.parent.parent
    acb_bin = repo_root / "target" / "release" / "acb"
    testdata = repo_root / "testdata" / "python"

    if not acb_bin.exists() or not testdata.exists():
        pytest.skip("acb binary or testdata not available")

    out = Path(tempfile.gettempdir()) / "pytest-codebase-test.acb"
    subprocess.run(
        [str(acb_bin), "compile", str(testdata), "-o", str(out)],
        check=True,
        capture_output=True,
    )
    _ACB_PATH = str(out)
    return _ACB_PATH


@pytest.fixture()
def acb_path() -> str:
    """Return path to a compiled .acb test graph."""
    return _ensure_acb()


@pytest.fixture()
def graph(acb_path: str) -> CodebaseGraph:
    """Open a graph and close it after the test."""
    g = CodebaseGraph(acb_path)
    yield g
    g.close()


# ---------------------------------------------------------------------------
# Tests
# ---------------------------------------------------------------------------


class TestGraphLifecycle:
    def test_open_graph(self, acb_path: str) -> None:
        g = CodebaseGraph(acb_path)
        assert g.unit_count > 0
        g.close()

    def test_context_manager(self, acb_path: str) -> None:
        with CodebaseGraph(acb_path) as g:
            assert g.unit_count > 0

    def test_open_nonexistent(self) -> None:
        with pytest.raises(GraphNotFoundError):
            CodebaseGraph("/tmp/does-not-exist-42.acb")


class TestGraphMetadata:
    def test_unit_count(self, graph: CodebaseGraph) -> None:
        assert graph.unit_count >= 1

    def test_edge_count(self, graph: CodebaseGraph) -> None:
        assert graph.edge_count >= 0

    def test_dimension(self, graph: CodebaseGraph) -> None:
        dim = graph.dimension
        assert isinstance(dim, int)
        assert dim >= 0

    def test_info(self, graph: CodebaseGraph) -> None:
        info = graph.info()
        assert info.unit_count == graph.unit_count
        assert info.edge_count == graph.edge_count
        assert info.is_empty is False


class TestUnitAccess:
    def test_get_unit_name(self, graph: CodebaseGraph) -> None:
        name = graph.get_unit_name(0)
        assert isinstance(name, str)
        assert len(name) > 0

    def test_get_unit_type(self, graph: CodebaseGraph) -> None:
        utype = graph.get_unit_type(0)
        assert isinstance(utype, UnitType)

    def test_get_unit_file(self, graph: CodebaseGraph) -> None:
        fpath = graph.get_unit_file(0)
        assert isinstance(fpath, str)

    def test_get_unit_language(self, graph: CodebaseGraph) -> None:
        lang = graph.get_unit_language(0)
        assert isinstance(lang, Language)

    def test_get_unit(self, graph: CodebaseGraph) -> None:
        unit = graph.get_unit(0)
        assert unit.id == 0
        assert isinstance(unit.name, str)
        assert isinstance(unit.unit_type, UnitType)
        assert isinstance(unit.language, Language)

    def test_get_nonexistent_unit(self, graph: CodebaseGraph) -> None:
        with pytest.raises(UnitNotFoundError):
            graph.get_unit_name(999999)


class TestEdgeAccess:
    def test_get_edges(self, graph: CodebaseGraph) -> None:
        edges = graph.get_edges(0)
        assert isinstance(edges, list)
        # Some units may have no edges, that's OK

    def test_edge_types(self, graph: CodebaseGraph) -> None:
        # Find a unit with edges
        for uid in range(min(graph.unit_count, 20)):
            edges = graph.get_edges(uid)
            if edges:
                for edge in edges:
                    assert isinstance(edge.edge_type, EdgeType)
                    assert isinstance(edge.weight, float)
                    assert edge.source_id == uid
                break
