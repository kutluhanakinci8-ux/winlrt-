#!/usr/bin/env python3
"""Metin dosyalarında Lertiva Arc/Lertiva Zip → Lertiva Arc / Lertiva Zip."""
from __future__ import annotations

import os
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))

SKIP_DIR_NAMES = {
    ".git",
    "node_modules",
    "target",
    "dist",
    "gen",
}

SKIP_PATH_PARTS = (
    "/apps/web/dist/",
    "/apps/lertiva-zip/src-tauri/gen/",
    "/apps/lertiva-zip/src-tauri/gen/",
)

TEXT_EXT = {
    ".rs", ".toml", ".md", ".html", ".htm", ".json", ".js", ".css", ".svg",
    ".xml", ".txt", ".yml", ".yaml", ".sh", ".ps1", ".nsh", ".admx", ".adml",
    ".reg", ".tsx", ".ts", ".lock", ".py", ".conf",
}

# Sıra önemli: uzun eşleşmeler önce
REPLACEMENTS = [
    ("Lertiva Arc", "Lertiva Arc"),
    ("Lertiva Zip", "Lertiva Zip"),
]

# Bu alt dizinlerde Lertiva Arc/ZIP araması yok sayılır (eski release tag metinleri kalabilir)
ALLOW_SUBSTR = (
    "github.com/kutluhanakinci8-ux/winlrt-",
    "winlrt.com.tr",
    "winlrt-zip-beta",
    "winlrt-rar-beta",
    "winlrt_rar",
    "winlrt_zip",
)


def should_skip_dir(name: str) -> bool:
    return name in SKIP_DIR_NAMES


def should_process_file(path: str) -> bool:
    for part in SKIP_PATH_PARTS:
        if part in path.replace("\\", "/"):
            return False
    _, ext = os.path.splitext(path)
    if ext.lower() in TEXT_EXT:
        return True
    base = os.path.basename(path)
    if base in ("Cargo.lock", "README", "LICENSE"):
        return True
    return False


def transform(content: str, path: str) -> str:
    out = content
    for old, new in REPLACEMENTS:
        out = out.replace(old, new)
    return out


def main() -> int:
    changed = 0
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if not should_skip_dir(d)]
        for fn in filenames:
            path = os.path.join(dirpath, fn)
            if not should_process_file(path):
                continue
            try:
                with open(path, "r", encoding="utf-8", errors="surrogateescape") as f:
                    original = f.read()
            except (OSError, UnicodeDecodeError):
                continue
            updated = transform(original, path)
            if updated != original:
                with open(path, "w", encoding="utf-8", errors="surrogateescape") as f:
                    f.write(updated)
                changed += 1
    print(f"OK: rebrand text pass — {changed} files updated")
    return 0


if __name__ == "__main__":
    sys.exit(main())
