#!/usr/bin/env bash
# Madde W14: Lertiva Zip macOS Tauri + CI kapisi.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/.github/workflows/lertiva-zip-macos.yml"
test -f "$ROOT/apps/lertiva-zip/src-tauri/macos/Entitlements.plist"
grep -q 'minimumSystemVersion' "$ROOT/apps/lertiva-zip/src-tauri/tauri.conf.json"
grep -q 'icon.icns' "$ROOT/apps/lertiva-zip/src-tauri/tauri.conf.json"
test -f "$ROOT/docs/qa/MADDE_W14_WINLRTZIP_MACOS.md"
cargo check -p lertiva-zip --quiet
cargo test -p lertiva-zip platform_info --quiet

echo "OK: Madde W14 — Lertiva Zip macOS build kapisi"
