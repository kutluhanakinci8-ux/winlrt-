#!/usr/bin/env bash
# Madde D: beta build flag → bekle → beta release flag → bekle.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

wait_workflow() {
  sleep 20
  local RUN
  RUN="$(gh run list --workflow=winlrt-rar-windows.yml --repo "$REPO" --branch main --limit 1 --json databaseId --jq '.[0].databaseId')"
  echo "==> winlrtRAR Windows beta run $RUN"
  gh run watch "$RUN" --repo "$REPO" --exit-status
}

cd "$ROOT"
echo "# madde-d build $(date -u +%Y-%m-%dT%H:%MZ)" >>.github/flags/trigger-beta-build
git add .github/flags/trigger-beta-build
git commit -m "ci(madde-d): trigger-beta-build"
git push origin main
wait_workflow

echo "# madde-d release $(date -u +%Y-%m-%dT%H:%MZ)" >>.github/flags/trigger-beta-release
git add .github/flags/trigger-beta-release
git commit -m "ci(madde-d): trigger-beta-release"
git push origin main
wait_workflow

bash "$ROOT/scripts/verify-madde-d.sh"
