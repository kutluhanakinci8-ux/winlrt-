#!/usr/bin/env bash
# Usage: import-windows-kpi-from-run.sh <github-run-id>
set -euo pipefail
RUN_ID="${1:?run id gerekli}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

gh run download "$RUN_ID" --repo "$REPO" --name bench-report-windows --dir "$TMP"
OUT="$ROOT/apps/web/public/benchmark/samples"
cp "$TMP/vitrin-windows-balanced-kpi.json" "$OUT/windows-balanced-kpi.json"
cp "$TMP/vitrin-windows-fast-kpi.json" "$OUT/windows-fast-kpi.json"
if [[ -f "$TMP/bench-report-windows.json" && -f "$TMP/bench-report-windows-fast.json" ]]; then
  bash "$ROOT/scripts/import-windows-zip-kpi-to-vitrin.sh" \
    "$TMP/bench-report-windows.json" "$TMP/bench-report-windows-fast.json"
elif [[ -f "$TMP/vitrin-winlrtzip-balanced-kpi.json" ]]; then
  cp "$TMP/vitrin-winlrtzip-balanced-kpi.json" "$OUT/winlrtzip-windows-balanced-kpi.json"
  cp "$TMP/vitrin-winlrtzip-fast-kpi.json" "$OUT/winlrtzip-windows-fast-kpi.json"
  [[ -f "$TMP/vitrin-winlrtzip-p0-gate.json" ]] && \
    cp "$TMP/vitrin-winlrtzip-p0-gate.json" "$OUT/winlrtzip-p0-gate.json"
fi
echo "OK: vitrin KPI run=$RUN_ID speed_gate=$(jq -r '.kpi.speed_gate' "$OUT/windows-fast-kpi.json")"
