#!/usr/bin/env bash
# Madde W34: macOS resmi zip_kpi vitrin örnekleri.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

test -f "$ROOT/docs/qa/MADDE_W34_WINLRTZIP_MACOS_KPI.md"
test -f "$ROOT/scripts/import-macos-zip-kpi-to-vitrin.sh"
grep -q macos-zip-kpi "$ROOT/.github/workflows/lertiva-zip-macos.yml"

for f in lertiva-zip-macos-balanced-kpi.json lertiva-zip-macos-fast-kpi.json; do
  test -f "$ROOT/apps/web/public/benchmark/samples/$f"
  jq -e '.source == "macos-official-ci"' "$ROOT/apps/web/public/benchmark/samples/$f" >/dev/null
done

echo "OK: Madde W34 — macOS resmi zip_kpi vitrin"
