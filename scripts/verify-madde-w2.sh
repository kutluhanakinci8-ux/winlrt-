#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/apps/winlrt-zip/src-tauri/tauri.conf.json"
grep -q '"productName": "winlrtZIP"' "$ROOT/apps/winlrt-zip/src-tauri/tauri.conf.json"
cargo check -p winlrt-zip --quiet
test -f "$ROOT/docs/qa/MADDE_W2_WINLRTZIP_TAURI.md"

echo "OK: Madde W2 — winlrtZIP Tauri MVP"
