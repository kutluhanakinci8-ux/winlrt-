#!/usr/bin/env bash
# W17: Windows resmi bench → Lertiva Zip vitrin (zip_kpi + P0) → commit · push.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"
FLAG="$ROOT/.github/flags/trigger-windows-bench"

cd "$ROOT"
echo "# madde-w17 zip-kpi vitrin $(date -u +%Y-%m-%dT%H:%MZ)" >>"$FLAG"
git add "$FLAG"
git diff --staged --quiet || git commit -m "ci(w17): trigger-windows-bench (Lertiva Zip KPI import)"
git push origin main

echo "==> Windows bench bekleniyor..."
sleep 20
RUN="$(gh run list --workflow=bench-windows.yml --repo "$REPO" --branch main --limit 1 --json databaseId --jq '.[0].databaseId')"
gh run watch "$RUN" --repo "$REPO" --exit-status

bash "$ROOT/scripts/import-windows-zip-kpi-from-run.sh" "$RUN"

git add apps/web/public/benchmark/samples/winlrtzip-windows-*.json \
  apps/web/public/benchmark/samples/winlrtzip-p0-gate.json 2>/dev/null || true
if ! git diff --staged --quiet; then
  git commit -m "chore(vitrin): Lertiva Zip resmi Windows KPI + P0 (W17)"
  git push origin main
fi

bash "$ROOT/scripts/verify-madde-w17.sh"
echo "OK: W17 Windows zip KPI vitrin import"
