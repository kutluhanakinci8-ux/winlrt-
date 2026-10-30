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
elif [[ -f "$TMP/vitrin-lertiva-zip-balanced-kpi.json" ]]; then
  cp "$TMP/vitrin-lertiva-zip-balanced-kpi.json" "$OUT/lertiva-zip-windows-balanced-kpi.json"
  cp "$TMP/vitrin-lertiva-zip-fast-kpi.json" "$OUT/lertiva-zip-windows-fast-kpi.json"
  [[ -f "$TMP/vitrin-lertiva-zip-p0-gate.json" ]] && \
    cp "$TMP/vitrin-lertiva-zip-p0-gate.json" "$OUT/lertiva-zip-p0-gate.json"
fi
echo "OK: vitrin KPI run=$RUN_ID speed_gate=$(jq -r '.kpi.speed_gate' "$OUT/windows-fast-kpi.json")"
