"""Error hierarchy for the agentic_codebase package.

All exceptions raised by this library inherit from :class:`AcbError`,
making it easy to catch codebase-specific errors in a single ``except`` block.
"""

from __future__ import annotations


class AcbError(Exception):
    """Base exception for all AgenticCodebase operations.

    Parameters
    ----------
    message:
        Human-readable error description.
    code:
        Numeric error code (mirrors FFI error codes). Defaults to ``-1``.
    """

    def __init__(self, message: str = "", *, code: int = -1) -> None:
        self.code = code
        super().__init__(message)


class GraphNotFoundError(AcbError):
    """Raised when a ``.acb`` graph file cannot be found.

    Parameters
    ----------
    path:
        Filesystem path that was not found.
    """

    def __init__(self, path: str = "") -> None:
        self.path = path
        super().__init__(f"Graph not found: {path}", code=-3)


class UnitNotFoundError(AcbError):
    """Raised when a code unit ID does not exist in the graph.

    Parameters
    ----------
    unit_id:
        The code unit identifier that was not found.
    """

    def __init__(self, unit_id: int = 0) -> None:
        self.unit_id = unit_id
        super().__init__(f"Code unit not found: {unit_id}", code=-3)


class CompileError(AcbError):
    """Raised when a codebase compilation step fails."""

    def __init__(self, message: str = "Compilation failed") -> None:
        super().__init__(message, code=-2)


class StorageError(AcbError):
    """Raised when a storage I/O operation fails.

    Parameters
    ----------
    path:
        Filesystem path that caused the error, if applicable.
    """

    def __init__(self, path: str = "", message: str = "") -> None:
        self.path = path
        msg = message or f"Storage error: {path}" if path else "Storage error"
        super().__init__(msg, code=-1)


class LibraryNotFoundError(AcbError):
    """Raised when the native shared library cannot be located.

    Parameters
    ----------
    searched:
        Filesystem paths that were searched.
    """

    def __init__(self, searched: list[str] | None = None) -> None:
        self.searched = searched or []
        paths = ", ".join(self.searched) if self.searched else "(none)"
        super().__init__(
            f"Native library not found. Searched: {paths}",
            code=-1,
        )


class ValidationError(AcbError):
    """Raised when input validation fails."""

    def __init__(self, message: str = "Validation failed") -> None:
        super().__init__(message, code=-6)


class OverflowError(AcbError):
    """Raised when an FFI buffer is too small for the result."""

    def __init__(self, message: str = "Buffer overflow") -> None:
        super().__init__(message, code=-4)
