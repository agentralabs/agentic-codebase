"""Tests for the agentic_codebase.errors module.

Validates the error hierarchy and attributes.
"""

from __future__ import annotations

import pytest

from agentic_codebase.errors import (
    AcbError,
    CompileError,
    GraphNotFoundError,
    LibraryNotFoundError,
    OverflowError,
    StorageError,
    UnitNotFoundError,
    ValidationError,
)


class TestAcbError:
    def test_base_error(self) -> None:
        err = AcbError("test error")
        assert str(err) == "test error"
        assert err.code == -1

    def test_custom_code(self) -> None:
        err = AcbError("fail", code=-42)
        assert err.code == -42

    def test_is_exception(self) -> None:
        assert issubclass(AcbError, Exception)


class TestGraphNotFoundError:
    def test_stores_path(self) -> None:
        err = GraphNotFoundError("/tmp/project.acb")
        assert err.path == "/tmp/project.acb"
        assert "/tmp/project.acb" in str(err)
        assert err.code == -3

    def test_is_acb_error(self) -> None:
        assert issubclass(GraphNotFoundError, AcbError)


class TestUnitNotFoundError:
    def test_stores_id(self) -> None:
        err = UnitNotFoundError(42)
        assert err.unit_id == 42
        assert "42" in str(err)
        assert err.code == -3

    def test_is_acb_error(self) -> None:
        assert issubclass(UnitNotFoundError, AcbError)


class TestCompileError:
    def test_default(self) -> None:
        err = CompileError()
        assert "Compilation" in str(err)
        assert err.code == -2

    def test_custom(self) -> None:
        err = CompileError("parse error in main.rs")
        assert "parse error" in str(err)

    def test_is_acb_error(self) -> None:
        assert issubclass(CompileError, AcbError)


class TestStorageError:
    def test_with_path(self) -> None:
        err = StorageError("/tmp/project.acb")
        assert err.path == "/tmp/project.acb"
        assert "/tmp/project.acb" in str(err)
        assert err.code == -1

    def test_is_acb_error(self) -> None:
        assert issubclass(StorageError, AcbError)


class TestLibraryNotFoundError:
    def test_default(self) -> None:
        err = LibraryNotFoundError()
        assert "Native library not found" in str(err)
        assert err.searched == []
        assert err.code == -1

    def test_with_locations(self) -> None:
        err = LibraryNotFoundError(["/usr/lib", "/opt/lib"])
        assert "/usr/lib" in str(err)
        assert "/opt/lib" in str(err)
        assert err.searched == ["/usr/lib", "/opt/lib"]

    def test_is_acb_error(self) -> None:
        assert issubclass(LibraryNotFoundError, AcbError)


class TestValidationError:
    def test_default(self) -> None:
        err = ValidationError()
        assert "Validation" in str(err)
        assert err.code == -6

    def test_is_acb_error(self) -> None:
        assert issubclass(ValidationError, AcbError)

    def test_raise(self) -> None:
        with pytest.raises(AcbError):
            raise ValidationError("invalid query")


class TestOverflowError:
    def test_default(self) -> None:
        err = OverflowError()
        assert "overflow" in str(err).lower()
        assert err.code == -4

    def test_is_acb_error(self) -> None:
        assert issubclass(OverflowError, AcbError)


class TestHierarchy:
    """All errors should be subclasses of both AcbError and Exception."""

    @pytest.mark.parametrize(
        "cls",
        [
            GraphNotFoundError,
            UnitNotFoundError,
            CompileError,
            StorageError,
            LibraryNotFoundError,
            ValidationError,
            OverflowError,
        ],
    )
    def test_is_subclass_of_acb_error(self, cls: type) -> None:
        assert issubclass(cls, AcbError)

    @pytest.mark.parametrize(
        "cls",
        [
            AcbError,
            GraphNotFoundError,
            UnitNotFoundError,
            CompileError,
            StorageError,
            LibraryNotFoundError,
            ValidationError,
            OverflowError,
        ],
    )
    def test_is_subclass_of_exception(self, cls: type) -> None:
        assert issubclass(cls, Exception)
