#!/usr/bin/env bash
# Madde W17: winlrtZIP resmi Windows KPI vitrin import altyapisi + ornekler.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

test -f "$ROOT/scripts/import-windows-zip-kpi-from-run.sh"
test -f "$ROOT/scripts/agent-madde-w17-windows-zip-kpi-import.sh"
test -f "$ROOT/docs/qa/MADDE_W17_WINLRTZIP_WINDOWS_KPI_VITRIN.md"
grep -q winlrtzip-windows-balanced "$ROOT/apps/web/public/benchmark/index.html"

SAMPLES="$ROOT/apps/web/public/benchmark/samples"
if [[ ! -f "$SAMPLES/winlrtzip-windows-balanced-kpi.json" ]]; then
  RUN="$(gh run list --workflow=bench-windows.yml --repo "$REPO" --branch main --limit 10 \
    --json databaseId,conclusion --jq '[.[] | select(.conclusion=="success")] | .[0].databaseId // empty')"
  if [[ -n "$RUN" ]]; then
    bash "$ROOT/scripts/import-windows-zip-kpi-from-run.sh" "$RUN"
  fi
fi

test -f "$SAMPLES/winlrtzip-windows-balanced-kpi.json"
test -f "$SAMPLES/winlrtzip-windows-fast-kpi.json"
grep -q 'windows-official-ci' "$SAMPLES/winlrtzip-windows-balanced-kpi.json"
jq -e '.kpi.compression_gate != null' "$SAMPLES/winlrtzip-windows-balanced-kpi.json" >/dev/null
test -f "$SAMPLES/winlrtzip-p0-gate.json"

echo "OK: Madde W17 — winlrtZIP Windows KPI vitrin import"
