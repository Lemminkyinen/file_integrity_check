from collections.abc import Callable
from pathlib import Path

from hash_lib import __doc__
from hash_lib import file_sha256 as _file_sha256

# Typing, docstrings and other metadata are not available yet
# https://pyo3.rs/v0.20.2/python_typing_hints
# Currently the best solution for the problem is to manually maintain
# *.pyi files and ship them along with the package.
_file_sha256: Callable[[str], bytes]

# Module documentation works though
assert __doc__ == "A Python module implemented in Rust."

def file_sha256(file_path: str | Path) -> bytes:
    """Return the SHA-256 hash of the file at the given path."""
    return _file_sha256(file_path)