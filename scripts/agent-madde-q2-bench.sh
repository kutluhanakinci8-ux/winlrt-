#!/usr/bin/env bash
# Q2: Windows resmi bench → vitrin (speed_gate güncelleme).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$ROOT/scripts/verify-madde-q2.sh"
bash "$ROOT/scripts/agent-madde-n-complete.sh"
grep -q speed_gate "$ROOT/apps/web/public/benchmark/samples/windows-fast-kpi.json"
echo "NOT: speed_gate=$(jq -r '.kpi.speed_gate' "$ROOT/apps/web/public/benchmark/samples/windows-fast-kpi.json")"
