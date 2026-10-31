#!/usr/bin/env bash
# VPS deploy kimligi: ortam degiskeni veya repo kokundeki .env.vps.local (gitignore).
# Sifre repoya yazilmaz.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LOCAL_ENV="$ROOT/.env.vps.local"
if [[ -f "$LOCAL_ENV" ]]; then
  # shellcheck disable=SC1090
  set -a
  source "$LOCAL_ENV"
  set +a
fi
export VPS_HOST="${VPS_HOST:-37.230.105.99}"
export VPS_PORT="${VPS_PORT:-23422}"
export VPS_USER="${VPS_USER:-root}"
export VPS_WEB_ROOT="${VPS_WEB_ROOT:-/var/www/winlrt/apps/web/dist}"

vps_deploy_ready() {
  if [[ -n "${VPS_SSH_KEY_B64:-}" || -n "${VPS_SSH_KEY:-}" || -n "${VPS_SSH_PASSWORD:-}" ]]; then
    return 0
  fi
  return 1
}
