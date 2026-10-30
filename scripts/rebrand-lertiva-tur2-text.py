#!/usr/bin/env python3
"""2. tur: dosya yolu / referans metinleri (GitHub release tag korunur)."""
from __future__ import annotations

import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
SKIP = {".git", "node_modules", "target", "dist", "gen"}
EXT = {
    ".rs", ".toml", ".md", ".html", ".json", ".js", ".css", ".svg", ".xml", ".txt",
    ".yml", ".yaml", ".sh", ".ps1", ".nsh", ".admx", ".adml", ".reg", ".tsx", ".ts", ".py",
}

REPLACEMENTS = [
    ("winlrtzip-", "lertiva-zip-"),
    ("winlrtrar-", "lertiva-arc-"),
    ("tauri-winlrtzip", "tauri-lertiva-zip"),
    ("tauri-winlrtrar", "tauri-lertiva-arc"),
    ("installer-winlrtzip", "installer-lertiva-zip"),
    ("installer-winlrtrar", "installer-lertiva-arc"),
    ("smartscreen-winlrt", "smartscreen-lertiva"),
    ("winlrtZIP-corporate-gpo", "lertiva-zip-corporate-gpo"),
    ("winlrtRAR-corporate-gpo", "lertiva-arc-corporate-gpo"),
    ("Lertiva Zip-corporate-gpo", "lertiva-zip-corporate-gpo"),
    ("Lertiva Arc-corporate-gpo", "lertiva-arc-corporate-gpo"),
    ("Lertiva Zip.admx", "lertiva-zip.admx"),
    ("Lertiva Arc.admx", "lertiva-arc.admx"),
    ("Lertiva Zip.adml", "lertiva-zip.adml"),
    ("Lertiva Arc.adml", "lertiva-arc.adml"),
    ("README-Lertiva Zip.md", "README-lertiva-zip.md"),
    ("package-winlrtzip-corporate-gpo-zip.sh", "package-lertiva-zip-corporate-gpo-zip.sh"),
    ("winlrtzip-corporate-policy-sample.reg", "lertiva-zip-corporate-policy-sample.reg"),
    ("winlrt-corporate-policy-sample.reg", "lertiva-arc-corporate-policy-sample.reg"),
    ("evaluate-winlrtzip-", "evaluate-lertiva-zip-"),
    ("refresh-winlrtzip-", "refresh-lertiva-zip-"),
    ("evaluate-winlrtzip", "evaluate-lertiva-zip"),
    ("verify-winlrtzip-", "verify-lertiva-zip-"),
    ("sync-vitrin-winlrtzip", "sync-vitrin-lertiva-zip"),
    ("agent-winlrtzip-", "agent-lertiva-zip-"),
    ("pilot-winlrtzip-", "pilot-lertiva-zip-"),
    ("winlrtzip-mock.html", "lertiva-zip-mock.html"),
    ("winlrtrar-mock.html", "lertiva-arc-mock.html"),
    ("winlrtzip-installer-mock.html", "lertiva-zip-installer-mock.html"),
    ("winlrtrar-installer-mock.html", "lertiva-arc-installer-mock.html"),
    ("--test winlrtzip_interop", "--test lertiva_zip_interop"),
    ("winlrtzip_interop", "lertiva_zip_interop"),
    ("Software\\winlrt\\", "Software\\Lertiva\\"),
    ("Software/winlrt/", "Software/Lertiva/"),
    ("prefix=\"winlrt\"", "prefix=\"lertiva\""),
    ("namespace=\"winlrt.", "namespace=\"lertiva."),
]


def protect(content: str) -> tuple[str, list[tuple[str, str]]]:
    tokens: list[tuple[str, str]] = []

    def stash(m: re.Match[str]) -> str:
        k = f"\x00T{len(tokens)}\x00"
        tokens.append((k, m.group(0)))
        return k

    out = content
    for pat in (
        r"github\.com/kutluhanakinci8-ux/winlrt-[^\s\"')>]+",
        r"winlrt-zip-beta[^\s\"')>]*",
        r"winlrt-rar-beta[^\s\"')>]*",
        r"lertiva-arc-beta[^\s\"')>]*",  # future
    ):
        out = re.sub(pat, stash, out)
    return out, tokens


def unstash(content: str, tokens: list[tuple[str, str]]) -> str:
    for k, v in tokens:
        content = content.replace(k, v)
    return content


def main() -> int:
    n = 0
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in SKIP]
        for fn in filenames:
            path = os.path.join(dirpath, fn)
            if "/apps/web/dist/" in path.replace("\\", "/"):
                continue
            ext = os.path.splitext(fn)[1].lower()
            if ext not in EXT and fn not in ("Cargo.lock",):
                continue
            if "rebrand-lertiva-tur2-text" in path:
                continue
            try:
                with open(path, "r", encoding="utf-8", errors="surrogateescape") as f:
                    raw = f.read()
            except (OSError, UnicodeDecodeError):
                continue
            protected, tok = protect(raw)
            out = protected
            for a, b in REPLACEMENTS:
                out = out.replace(a, b)
            out = unstash(out, tok)
            if out != raw:
                with open(path, "w", encoding="utf-8", errors="surrogateescape") as f:
                    f.write(out)
                n += 1
    print(f"OK: tur2 text — {n} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
