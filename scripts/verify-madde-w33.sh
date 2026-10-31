#!/usr/bin/env bash
# Madde W33: Lertiva Zip v1.0 GA kapı özeti + vitrin örneği.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/docs/qa/MADDE_W33_WINLRTZIP_V1_GATE.md"
test -f "$ROOT/scripts/evaluate-lertiva-zip-v1-gate.sh"
bash "$ROOT/scripts/evaluate-lertiva-zip-v1-gate.sh"
test -f "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-v1-gate.json"
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-v1-gate.json" >/dev/null
jq -e '.gates.p0_kpi.pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-v1-gate.json" >/dev/null
grep -q lertiva-zip-v1-gate "$ROOT/apps/web/public/benchmark/index.html" || grep -q lertiva-zip-v1-gate "$ROOT/apps/web/public/index.html"
bash "$ROOT/scripts/verify-madde-w16.sh"
bash "$ROOT/scripts/verify-madde-w32.sh"
echo "OK: Madde W33 — Lertiva Zip v1.0 kapı"
