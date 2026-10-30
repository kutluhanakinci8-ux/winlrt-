#!/usr/bin/env bash
# Madde C: Windows bench tetik → bekle → artifact → vitrin → commit.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
FLAG="$ROOT/.github/flags/trigger-windows-bench"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

echo "# madde-c $(date -u +%Y-%m-%dT%H:%MZ)" >>"$FLAG"
cd "$ROOT"
git add "$FLAG"
git diff --staged --quiet || git commit -m "ci(madde-c): trigger-windows-bench"
git push origin main

echo "==> Windows bench bekleniyor..."
sleep 15
RUN="$(gh run list --workflow=bench-windows.yml --repo "$REPO" --branch main --limit 1 --json databaseId --jq '.[0].databaseId')"
gh run watch "$RUN" --repo "$REPO" --exit-status

echo "==> Artifact indir"
gh run download "$RUN" --repo "$REPO" --name bench-report-windows --dir "$TMP"

BAL="$TMP/bench-report-windows.json"
FAST="$TMP/bench-report-windows-fast.json"
test -f "$BAL" && test -f "$FAST"

bash "$ROOT/scripts/import-windows-kpi-to-vitrin.sh" "$BAL" "$FAST"
echo "OK: vitrin import"
