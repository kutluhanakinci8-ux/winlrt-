#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

test -f "$ROOT/docs/WINLRTZIP_PHASE0.md"
test -f "$ROOT/docs/WINZIP_FEATURE_MATRIX.md"
test -f "$ROOT/docs/qa/WINLRTZIP_INTEROP_SCENARIOS.md"
grep -q WINLRTZIP_PLAN "$ROOT/apps/web/public/downloads/index.html"
grep -q '"speed_gate": true' "$ROOT/apps/web/public/benchmark/samples/windows-fast-kpi.json"

echo "OK: Madde W0 — winlrtZIP Faz 0 plan kapısı"
