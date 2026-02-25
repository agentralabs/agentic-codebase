"""Shared fixtures for the AgenticCodebase test suite."""

from __future__ import annotations

import tempfile
from pathlib import Path

import pytest


@pytest.fixture()
def tmp_dir():
    """Yield a temporary directory that is cleaned up after the test."""
    with tempfile.TemporaryDirectory() as d:
        yield Path(d)
