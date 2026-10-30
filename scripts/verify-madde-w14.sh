#!/usr/bin/env bash
# Madde W14: winlrtZIP macOS Tauri + CI kapisi.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/.github/workflows/winlrt-zip-macos.yml"
test -f "$ROOT/apps/winlrt-zip/src-tauri/macos/Entitlements.plist"
grep -q 'minimumSystemVersion' "$ROOT/apps/winlrt-zip/src-tauri/tauri.conf.json"
grep -q 'icon.icns' "$ROOT/apps/winlrt-zip/src-tauri/tauri.conf.json"
test -f "$ROOT/docs/qa/MADDE_W14_WINLRTZIP_MACOS.md"
cargo check -p winlrt-zip --quiet
cargo test -p winlrt-zip platform_info --quiet

echo "OK: Madde W14 — winlrtZIP macOS build kapisi"
