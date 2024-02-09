from collections.abc import Callable

from file_util_lib import read_hashes as _read_hashes
from file_util_lib import save_hashes as _save_hashes

_save_hashes: Callable[[list[tuple[str, bytes]]], None]
_read_hashes: Callable[[], list[tuple[str, bytes]]]


def save_hashes(hashes: list[tuple[str, bytes]]):
    """Save the given hashes to a file."""
    _save_hashes(hashes)


def read_hashes() -> list[tuple[str, bytes]]:
    """Read hashes from a file and return them."""
    return _read_hashes()