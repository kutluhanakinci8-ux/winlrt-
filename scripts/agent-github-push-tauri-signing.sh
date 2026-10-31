#!/usr/bin/env bash
# TAURI_SIGNING_* → GitHub (owner PAT) veya CI provision workflow.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="${GITHUB_REPO:-kutluhanakinci8-ux/winlrt-}"

if [[ -n "${GITHUB_ADMIN_PAT:-}" ]]; then
  export GH_TOKEN="${GITHUB_ADMIN_PAT}"
fi

if gh secret list --repo "$REPO" >/dev/null 2>&1; then
  LERTIVA_DEV_CODESIGN_PASSWORD="${LERTIVA_DEV_CODESIGN_PASSWORD:?}" \
    bash "$ROOT/scripts/push-lertiva-dev-signing-secrets.sh"
  echo "OK: TAURI_SIGNING_* (gh)"
  exit 0
fi

echo "gh secret API yok (403). GitHub Actions kullanın:" >&2
echo "  1) Settings → Secrets → LERTIVA_DEV_CODESIGN_PASSWORD" >&2
echo "  2) bash scripts/agent-github-trigger-provision-signing.sh" >&2
exit 1
