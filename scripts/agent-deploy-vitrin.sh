#!/usr/bin/env bash
# Cloud Agent: vitrin build + VPS rsync (ortam secret'ları veya export edilmiş VPS_*).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VPS_HOST="${VPS_HOST:-37.230.105.99}"
VPS_PORT="${VPS_PORT:-23422}"
VPS_USER="${VPS_USER:-root}"
VPS_WEB_ROOT="${VPS_WEB_ROOT:-/var/www/winlrt/apps/web/dist}"

KEY_FILE="${RUNNER_TEMP:-/tmp}/agent_vps_deploy_key"
rm -f "${KEY_FILE}"

key_ok() {
  [ -f "$1" ] && chmod 600 "$1" && ssh-keygen -y -f "$1" >/dev/null 2>&1
}

if [ -n "${VPS_SSH_KEY_B64:-}" ]; then
  printf '%s' "${VPS_SSH_KEY_B64}" | tr -d '[:space:]' | base64 -d >"${KEY_FILE}"
elif [ -n "${VPS_SSH_KEY:-}" ]; then
  printf '%s' "${VPS_SSH_KEY}" | sed 's/\r$//g' >"${KEY_FILE}"
  if ! grep -q 'BEGIN OPENSSH PRIVATE KEY' "${KEY_FILE}"; then
    printf '%s' "${VPS_SSH_KEY}" | sed 's/\\n/\n/g' | sed 's/\r$//g' >"${KEY_FILE}"
  fi
fi

AUTH=""
if key_ok "${KEY_FILE}"; then
  AUTH=key
elif [ -n "${VPS_SSH_PASSWORD:-}" ]; then
  AUTH=password
else
  echo "Hata: VPS_SSH_KEY_B64 / VPS_SSH_KEY veya VPS_SSH_PASSWORD gerekli." >&2
  echo "GitHub: scripts/mac-set-github-vps-secret.sh veya mac-vps-key-base64-for-github.sh" >&2
  exit 1
fi

if [ -x "$ROOT/scripts/run-dalga-smoke.sh" ]; then
  echo "==> Dalga smoke"
  bash "$ROOT/scripts/run-dalga-smoke.sh"
fi

echo "==> Vitrin build"
npm ci
npm run build:web

SSH_OPTS=(-o StrictHostKeyChecking=accept-new -p "${VPS_PORT}")
RSYNC_SSH=(ssh "${SSH_OPTS[@]}")
if [ "${AUTH}" = key ]; then
  RSYNC_SSH+=(-i "${KEY_FILE}")
elif [ "${AUTH}" = password ]; then
  command -v sshpass >/dev/null || { echo "sshpass gerekli (password auth)" >&2; exit 1; }
  RSYNC_SSH=(sshpass -p "${VPS_SSH_PASSWORD}" ssh "${SSH_OPTS[@]}")
fi

echo "==> Deploy → ${VPS_USER}@${VPS_HOST}:${VPS_WEB_ROOT}"
"${RSYNC_SSH[@]}" "${VPS_USER}@${VPS_HOST}" "mkdir -p '${VPS_WEB_ROOT}'"
rsync -av --delete -e "${RSYNC_SSH[*]}" apps/web/dist/ "${VPS_USER}@${VPS_HOST}:${VPS_WEB_ROOT}/"
echo "Tamam."
