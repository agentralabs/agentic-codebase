"""High-level CodebaseGraph API.

Provides the :class:`CodebaseGraph` class for querying ``.acb`` graph files
produced by the ``acb`` CLI compiler.
"""

from __future__ import annotations

import ctypes
from pathlib import Path
from typing import Optional

from .errors import AcbError, GraphNotFoundError, UnitNotFoundError
from .models import (
    CodeUnit,
    Edge,
    EdgeType,
    GraphInfo,
    Language,
    UnitType,
    edge_type_from_u8,
    language_from_u8,
    unit_type_from_u8,
)


_BUFFER_SIZE = 4096


class CodebaseGraph:
    """A loaded code graph for semantic analysis.

    Parameters
    ----------
    path:
        Path to a compiled ``.acb`` graph file.
    """

    def __init__(self, path: str) -> None:
        from ._ffi import _get_lib, _check, ACB_ERR_NOT_FOUND

        if not Path(path).exists():
            raise GraphNotFoundError(path)

        self._lib = _get_lib()
        self._ptr = self._lib.acb_graph_open(path.encode("utf-8"))
        if not self._ptr:
            raise AcbError(f"Failed to open graph: {path}", code=-1)
        self._path = path

    # -- lifecycle ---------------------------------------------------------

    def close(self) -> None:
        """Release the native handle."""
        if self._ptr:
            self._lib.acb_graph_free(self._ptr)
            self._ptr = None

    def __enter__(self) -> "CodebaseGraph":
        return self

    def __exit__(self, *exc: object) -> None:
        self.close()

    def __del__(self) -> None:
        if hasattr(self, "_ptr"):
            self.close()

    # -- metadata ----------------------------------------------------------

    @property
    def unit_count(self) -> int:
        """Number of code units in the graph."""
        if not self._ptr:
            raise AcbError("Graph handle is closed")
        return int(self._lib.acb_graph_unit_count(self._ptr))

    @property
    def edge_count(self) -> int:
        """Number of edges in the graph."""
        if not self._ptr:
            raise AcbError("Graph handle is closed")
        return int(self._lib.acb_graph_edge_count(self._ptr))

    @property
    def dimension(self) -> int:
        """Embedding dimension of the graph."""
        if not self._ptr:
            raise AcbError("Graph handle is closed")
        return int(self._lib.acb_graph_dimension(self._ptr))

    def info(self) -> GraphInfo:
        """Return summary statistics."""
        return GraphInfo(
            path=self._path,
            unit_count=self.unit_count,
            edge_count=self.edge_count,
            dimension=self.dimension,
        )

    # -- unit access -------------------------------------------------------

    def get_unit_name(self, unit_id: int) -> str:
        """Get the name of a code unit by ID."""
        buf = ctypes.create_string_buffer(_BUFFER_SIZE)
        rc = self._lib.acb_graph_get_unit_name(self._ptr, unit_id, buf, _BUFFER_SIZE)
        if rc < 0:
            raise UnitNotFoundError(unit_id)
        return buf.value.decode("utf-8")

    def get_unit_type(self, unit_id: int) -> UnitType:
        """Get the type of a code unit by ID."""
        rc = self._lib.acb_graph_get_unit_type(self._ptr, unit_id)
        if rc < 0:
            raise UnitNotFoundError(unit_id)
        return unit_type_from_u8(rc)

    def get_unit_file(self, unit_id: int) -> str:
        """Get the file path of a code unit by ID."""
        buf = ctypes.create_string_buffer(_BUFFER_SIZE)
        rc = self._lib.acb_graph_get_unit_file(self._ptr, unit_id, buf, _BUFFER_SIZE)
        if rc < 0:
            raise UnitNotFoundError(unit_id)
        return buf.value.decode("utf-8")

    def get_unit_complexity(self, unit_id: int) -> float:
        """Get the complexity score of a code unit."""
        result = self._lib.acb_graph_get_unit_complexity(self._ptr, unit_id)
        if result < 0:
            raise UnitNotFoundError(unit_id)
        return float(result)

    def get_unit_language(self, unit_id: int) -> Language:
        """Get the programming language of a code unit."""
        rc = self._lib.acb_graph_get_unit_language(self._ptr, unit_id)
        if rc < 0:
            raise UnitNotFoundError(unit_id)
        return language_from_u8(rc)

    def get_unit_stability(self, unit_id: int) -> float:
        """Get the stability score of a code unit."""
        result = self._lib.acb_graph_get_unit_stability(self._ptr, unit_id)
        if result < 0:
            raise UnitNotFoundError(unit_id)
        return float(result)

    def get_unit(self, unit_id: int) -> CodeUnit:
        """Get a full :class:`CodeUnit` by ID."""
        return CodeUnit(
            id=unit_id,
            name=self.get_unit_name(unit_id),
            unit_type=self.get_unit_type(unit_id),
            file_path=self.get_unit_file(unit_id),
            language=self.get_unit_language(unit_id),
            complexity=self.get_unit_complexity(unit_id),
            stability=self.get_unit_stability(unit_id),
        )

    # -- edge access -------------------------------------------------------

    def get_edges(self, unit_id: int, max_edges: int = 256) -> list[Edge]:
        """Get outgoing edges from a code unit."""
        target_ids = (ctypes.c_uint64 * max_edges)()
        edge_types = (ctypes.c_uint8 * max_edges)()
        weights = (ctypes.c_float * max_edges)()

        rc = self._lib.acb_graph_get_edges(
            self._ptr,
            unit_id,
            target_ids,
            edge_types,
            weights,
            max_edges,
        )
        if rc < 0:
            raise UnitNotFoundError(unit_id)

        return [
            Edge(
                source_id=unit_id,
                target_id=int(target_ids[i]),
                edge_type=edge_type_from_u8(int(edge_types[i])),
                weight=float(weights[i]),
            )
            for i in range(rc)
        ]
