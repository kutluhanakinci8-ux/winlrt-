#!/usr/bin/env bash
# Madde W3: winlrtZIP beta build → release → vitrin link doğrula.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

wait_workflow() {
  sleep 25
  local RUN
  RUN="$(gh run list --workflow=winlrt-zip-windows.yml --repo "$REPO" --branch main --limit 1 --json databaseId --jq '.[0].databaseId')"
  echo "==> winlrtZIP Windows run $RUN"
  gh run watch "$RUN" --repo "$REPO" --exit-status
}

cd "$ROOT"
echo "# madde-w3 build $(date -u +%Y-%m-%dT%H:%MZ)" >>.github/flags/trigger-zip-beta-build
git add .github/flags/trigger-zip-beta-build
git commit -m "ci(madde-w3): trigger-zip-beta-build"
git push origin main
wait_workflow

echo "# madde-w3 release $(date -u +%Y-%m-%dT%H:%MZ)" >>.github/flags/trigger-zip-beta-release
git add .github/flags/trigger-zip-beta-release
git commit -m "ci(madde-w3): trigger-zip-beta-release"
git push origin main
wait_workflow

bash "$ROOT/scripts/sync-vitrin-winlrtzip-beta.sh"
git add apps/web/public/downloads/index.html
git diff --staged --quiet || git commit -m "chore(vitrin): winlrtZIP beta indirme linki (madde W3)"
git push origin main
bash "$ROOT/scripts/verify-madde-w3.sh"
