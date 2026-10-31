#!/usr/bin/env bash
# W14 repo dosyalari (Mac'te Rust yokken). Tam: verify-madde-w14.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/.github/workflows/lertiva-zip-macos.yml"
test -f "$ROOT/apps/lertiva-zip/src-tauri/macos/Entitlements.plist"
grep -q 'minimumSystemVersion' "$ROOT/apps/lertiva-zip/src-tauri/tauri.conf.json"
grep -q 'icon.icns' "$ROOT/apps/lertiva-zip/src-tauri/tauri.conf.json"
test -f "$ROOT/docs/qa/MADDE_W14_WINLRTZIP_MACOS.md"

echo "OK: Madde W14 — macOS repo (yerel; tam: cargo + verify-madde-w14.sh)"
