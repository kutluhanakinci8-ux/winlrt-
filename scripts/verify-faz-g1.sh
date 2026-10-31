#!/usr/bin/env bash
# Faz G1 — Zip P0×3 + W33 v1 kapı (repo tanımı).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-madde-w16.sh"
bash "$ROOT/scripts/verify-madde-w32.sh"
bash "$ROOT/scripts/verify-madde-w33.sh"
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-p0-gate.json" >/dev/null
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-v1-gate.json" >/dev/null

if [[ -f "$ROOT/scripts/verify-madde-w34.sh" ]]; then
  bash "$ROOT/scripts/verify-madde-w34.sh" || echo "::warning::W34 macOS KPI henüz resmi değil (G3 devam)"
fi

test -f "$ROOT/docs/qa/G1_BENCH_KPI_IMPORT_RUNBOOK.md"
bash "$ROOT/scripts/refresh-lertiva-zip-macos-distribution.sh" 2>/dev/null || true
bash "$ROOT/scripts/evaluate-lertiva-g1-platform-gate.sh" >/dev/null
jq -e '.repo_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-g1-platform-polish-gate.json" >/dev/null

echo "OK: Faz G1 — P0×3 + W33 v1 + platform polish kapısı"
