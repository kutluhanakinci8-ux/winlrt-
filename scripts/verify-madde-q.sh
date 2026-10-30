#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

bash "$ROOT/scripts/verify-madde-c.sh"
grep -q '"compression_gate": true' "$ROOT/apps/web/public/benchmark/samples/windows-balanced-kpi.json"
test -f "$ROOT/apps/web/public/benchmark/samples/windows-fast-kpi.json"
grep -q speed_gate "$ROOT/apps/web/public/benchmark/samples/windows-fast-kpi.json"
grep -q ReportOnly "$ROOT/.github/workflows/bench-windows.yml"

echo "OK: Madde Q — Balanced strict + Fast izleme dokümante"
