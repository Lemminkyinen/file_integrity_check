from pathlib import Path

import typer
from wrappers.file_util_lib_wrap import read_hashes, save_hashes
from wrappers.hash_lib_wrap import file_sha256


def main(path: str):
    path_ = convert_to_path(path)
    if path_ is None:
        print(f"Invalid path: {path}")
        exit(1)

    if not path_.exists():
        print(f"Path does not exist: {path}")
        exit(1)

    if path_.is_dir():
        files = find_all_files(path_)
    else:
        files = [path_]

    abs_files = [f.resolve().as_posix() for f in files]
    abs_files.sort()

    old_hashes = read_hashes()
    new_hashes = list(map(get_hash, abs_files))

    if old_hashes != new_hashes:
        print("Changes detected in files.")
        print("Storing new hashes.")
        save_hashes(new_hashes)
    else:
        print("No changes detected in files.")


def get_hash(file_path: str) -> tuple[str, bytes]:
    return (file_path, file_sha256(file_path))


def convert_to_path(s: str) -> Path | None:
    try:
        return Path(s)
    except Exception:
        return None


def find_all_files(dir: Path):
    return [f for f in dir.rglob("*") if f.is_file()]


if __name__ == "__main__":
    typer.run(main)
