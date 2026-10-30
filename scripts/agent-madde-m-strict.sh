#!/usr/bin/env bash
# Imzalı build: TAURI_SIGNING_* secret zorunlu.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

if ! gh secret list --repo "$REPO" 2>/dev/null | grep -q TAURI_SIGNING_PRIVATE_KEY; then
  echo "TAURI_SIGNING_PRIVATE_KEY yok — prep için verify-madde-m.sh" >&2
  exit 1
fi

echo "# madde-m strict $(date -u +%Y-%m-%dT%H:%MZ)" >>"$ROOT/.github/flags/trigger-beta-release"
cd "$ROOT"
git add .github/flags/trigger-beta-release
git commit -m "ci(madde-m): imzalı beta release tetik"
git push origin main
bash "$ROOT/scripts/agent-madde-d-complete.sh" || true
echo "Imzalı artifact sonrası: tag v1.0.1 + gh release (agent-dalga4 veya panel)"
