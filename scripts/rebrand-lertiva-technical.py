#!/usr/bin/env python3
"""Teknik tanımlayıcılar: lertiva_archive → lertiva_archive, vb."""
from __future__ import annotations

import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
SKIP_DIR_NAMES = {".git", "node_modules", "target", "dist", "gen"}
TEXT_EXT = {
    ".rs", ".toml", ".md", ".html", ".json", ".js", ".css", ".svg", ".xml", ".txt",
    ".yml", ".yaml", ".sh", ".ps1", ".nsh", ".admx", ".adml", ".reg", ".tsx", ".ts", ".py",
}

REPLACEMENTS = [
    ("lertiva_archive", "lertiva_archive"),
    ("lertiva-archive", "lertiva-archive"),
    ("@lertiva/", "@lertiva/"),
    ("com.lertiva.", "com.lertiva."),
]


def protect_urls(content: str) -> tuple[str, list[tuple[str, str]]]:
    """GitHub repo ve eski release tag'lerini koru."""
    tokens: list[tuple[str, str]] = []

    def stash(m: re.Match[str]) -> str:
        key = f"\x00STASH{len(tokens)}\x00"
        tokens.append((key, m.group(0)))
        return key

    protected = content
    for pat in (
        r"github\.com/kutluhanakinci8-ux/winlrt-[^\s\"')>]+",
        r"winlrt-zip-beta[^\s\"')>]*",
        r"winlrt-rar-beta[^\s\"')>]*",
        r"winlrtZIP_[^\s\"')>]+",
        r"winlrtRAR_[^\s\"')>]+",
    ):
        protected = re.sub(pat, stash, protected)
    return protected, tokens


def unstash(content: str, tokens: list[tuple[str, str]]) -> str:
    out = content
    for key, val in tokens:
        out = out.replace(key, val)
    return out


def transform(content: str) -> str:
    protected, tokens = protect_urls(content)
    out = protected
    for old, new in REPLACEMENTS:
        out = out.replace(old, new)
    # CLI/binary adları (beta tag korundu)
    out = out.replace("bin lertiva-zip", "bin lertiva-zip")
    out = out.replace('"lertiva-zip"', '"lertiva-zip"')
    out = out.replace("name = \"winlrt-zip\"", "name = \"lertiva-zip\"")
    out = out.replace("name = \"lertiva-archive\"", "name = \"lertiva-archive\"")
    out = out.replace("default-run = \"lertiva-archive\"", 'default-run = "lertiva"')
    out = out.replace("name = \"lertiva-archive\"", "name = \"lertiva\"")
    out = out.replace("-p lertiva-archive", "-p lertiva-archive")
    out = out.replace("cargo run -p lertiva-archive", "cargo run -p lertiva-archive")
    out = out.replace("apps/lertiva-zip", "apps/lertiva-zip")
    out = out.replace("apps/lertiva-arc", "apps/lertiva-arc")
    out = out.replace("crates/lertiva-archive", "crates/lertiva-archive")
    return unstash(out, tokens)


def main() -> int:
    changed = 0
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIR_NAMES]
        for fn in filenames:
            path = os.path.join(dirpath, fn)
            if "/apps/web/dist/" in path.replace("\\", "/"):
                continue
            _, ext = os.path.splitext(fn)
            if ext.lower() not in TEXT_EXT and fn not in ("Cargo.lock",):
                continue
            try:
                with open(path, "r", encoding="utf-8", errors="surrogateescape") as f:
                    original = f.read()
            except (OSError, UnicodeDecodeError):
                continue
            updated = transform(original)
            if updated != original:
                with open(path, "w", encoding="utf-8", errors="surrogateescape") as f:
                    f.write(updated)
                changed += 1
    print(f"OK: technical rebrand — {changed} files updated")
    return 0


if __name__ == "__main__":
    sys.exit(main())
