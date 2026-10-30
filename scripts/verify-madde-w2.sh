#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/apps/lertiva-zip/src-tauri/tauri.conf.json"
grep -q '"productName": "Lertiva Zip"' "$ROOT/apps/lertiva-zip/src-tauri/tauri.conf.json"
cargo check -p lertiva-zip --quiet
test -f "$ROOT/docs/qa/MADDE_W2_WINLRTZIP_TAURI.md"

echo "OK: Madde W2 — Lertiva Zip Tauri MVP"
