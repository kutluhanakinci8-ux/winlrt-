#!/usr/bin/env bash
# Madde W4: Lertiva Zip Explorer NSIS + CLI startup.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
HOOK="$ROOT/apps/lertiva-zip/src-tauri/windows/installer-hooks.nsh"
test -f "$HOOK"
grep -q NSIS_HOOK_POSTINSTALL "$HOOK"
grep -q Lertiva Zip.exe "$HOOK"
grep -q WinlrtZipCompress "$HOOK"
grep -q parse_startup_mode "$ROOT/apps/lertiva-zip/src-tauri/src/cli.rs"
grep -q installerHooks "$ROOT/apps/lertiva-zip/src-tauri/tauri.conf.json"
cargo check -p lertiva-zip --quiet
echo "OK: Madde W4 — Lertiva Zip Explorer NSIS + CLI"
