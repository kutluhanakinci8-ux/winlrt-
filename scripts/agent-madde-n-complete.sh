#!/usr/bin/env bash
# Madde N: Windows bench → vitrin KPI → commit → push → deploy.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
bash "$ROOT/scripts/agent-madde-c-complete.sh"
cd "$ROOT"
git add apps/web/public/benchmark/samples/windows-*.json 2>/dev/null || true
if ! git diff --staged --quiet; then
  git commit -m "chore(vitrin): Windows resmi KPI yenileme (madde N)"
  git push origin main
  VPS_SSH_PASSWORD="${VPS_SSH_PASSWORD:-}" bash "$ROOT/scripts/post-phase-deploy.sh"
fi
bash "$ROOT/scripts/verify-madde-c.sh"
