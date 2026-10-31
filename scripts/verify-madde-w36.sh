#!/usr/bin/env bash
# Madde W36: Lertiva Zip mobil MVP repo kapısı (G6).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/docs/qa/MADDE_W36_WINLRTZIP_MOBILE_MVP.md"
test -f "$ROOT/apps/mobile/README.md"
bash "$ROOT/scripts/verify-madde-w26.sh"
grep -q 'com.lertiva.zip' "$ROOT/apps/lertiva-zip/src-tauri/tauri.conf.json"
bash "$ROOT/scripts/evaluate-lertiva-mobile-mvp-gate.sh"
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-mobile-mvp-gate.json" >/dev/null

echo "OK: Madde W36 — Lertiva Zip mobil MVP (repo)"
