#!/usr/bin/env bash
# bench-report-windows artifact → Lertiva Zip vitrin + P0×3 (run id veya en son başarılı).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
RUN_ID="${1:-}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

if [[ -z "$RUN_ID" ]]; then
  RUN_ID="$(gh run list --workflow=bench-windows.yml --repo "$REPO" --branch main --limit 10 \
    --json databaseId,conclusion --jq '[.[] | select(.conclusion=="success")] | .[0].databaseId // empty')"
fi
[[ -n "$RUN_ID" ]] || { echo "Başarılı bench-windows run yok" >&2; exit 1; }

gh run download "$RUN_ID" --repo "$REPO" --name bench-report-windows --dir "$TMP"
OUT="$ROOT/apps/web/public/benchmark/samples"
mkdir -p "$OUT"

if [[ -f "$TMP/vitrin-lertiva-zip-balanced-kpi.json" ]]; then
  cp "$TMP/vitrin-lertiva-zip-balanced-kpi.json" "$OUT/lertiva-zip-windows-balanced-kpi.json"
  cp "$TMP/vitrin-lertiva-zip-fast-kpi.json" "$OUT/lertiva-zip-windows-fast-kpi.json"
fi
if [[ -f "$TMP/vitrin-lertiva-zip-p0-gate.json" ]]; then
  cp "$TMP/vitrin-lertiva-zip-p0-gate.json" "$OUT/lertiva-zip-p0-gate.json"
fi

BAL="$TMP/bench-report-windows.json"
FAST="$TMP/bench-report-windows-fast.json"
if [[ -f "$BAL" && -f "$FAST" ]]; then
  bash "$ROOT/scripts/import-windows-zip-kpi-to-vitrin.sh" "$BAL" "$FAST"
fi

echo "OK: Lertiva Zip Windows KPI import run=$RUN_ID all_pass=$(jq -r '.all_pass' "$OUT/lertiva-zip-p0-gate.json" 2>/dev/null || echo '?')"
