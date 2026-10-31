#!/usr/bin/env bash
# lertiva-zip-macos bench artifact → vitrin W34 KPI.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
WORKFLOW="${LERTIVA_MACOS_WORKFLOW:-lertiva-zip-macos.yml}"
RUN_ID="${1:-}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

if [[ -z "$RUN_ID" ]]; then
  RUN_ID="$(gh run list --workflow="$WORKFLOW" --repo "$REPO" --branch main --limit 15 \
    --json databaseId,conclusion --jq '[.[] | select(.conclusion=="success")] | .[0].databaseId // empty')"
fi
[[ -n "$RUN_ID" ]] || { echo "Başarılı lertiva-zip-macos run yok" >&2; exit 1; }

gh run download "$RUN_ID" --repo "$REPO" --name lertiva-zip-macos-bench --dir "$TMP"
BAL="$TMP/macos-bench-balanced.json"
FAST="$TMP/macos-bench-fast.json"
[[ -f "$BAL" && -f "$FAST" ]] || { echo "Artifact eksik (bench JSON)" >&2; exit 1; }
bash "$ROOT/scripts/import-macos-zip-kpi-to-vitrin.sh" "$BAL" "$FAST"
echo "OK: macOS KPI import run=$RUN_ID"
