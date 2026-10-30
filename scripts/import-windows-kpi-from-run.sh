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
echo "OK: vitrin KPI run=$RUN_ID speed_gate=$(jq -r '.kpi.speed_gate' "$OUT/windows-fast-kpi.json")"
