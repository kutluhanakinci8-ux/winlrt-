#!/usr/bin/env bash
# Canlı vitrin nginx basic auth — yalnızca VPS (yerel npm run build:web etkilenmez).
# .env.vps.local: VITRIN_BASIC_AUTH_USER, VITRIN_BASIC_AUTH_PASSWORD
#   Şifre dolu → auth aç/güncelle · VITRIN_BASIC_AUTH_DISABLE=1 → kapat
#   İkisi de yok → uzak nginx ayarına dokunma
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# shellcheck source=scripts/lib/load-vps-deploy-env.sh
source "$ROOT/scripts/lib/load-vps-deploy-env.sh"

if [[ "${VITRIN_BASIC_AUTH_DISABLE:-0}" == "1" ]]; then
  MODE=disable
elif [[ -n "${VITRIN_BASIC_AUTH_PASSWORD:-}" ]]; then
  MODE=enable
else
  echo "==> Vitrin basic auth: atlandı (VITRIN_BASIC_AUTH_PASSWORD yok, DISABLE=0)"
  exit 0
fi

VPS_HOST="${VPS_HOST:-37.230.105.99}"
VPS_PORT="${VPS_PORT:-23422}"
VPS_USER="${VPS_USER:-root}"
AUTH_USER="${VITRIN_BASIC_AUTH_USER:-lertiva}"

KEY_FILE="${RUNNER_TEMP:-/tmp}/agent_vps_deploy_key"
rm -f "${KEY_FILE}"

key_ok() {
  [ -f "$1" ] && chmod 600 "$1" && ssh-keygen -y -f "$1" >/dev/null 2>&1
}

if [ -n "${VPS_SSH_KEY_B64:-}" ]; then
  printf '%s' "${VPS_SSH_KEY_B64}" | tr -d '[:space:]' | base64 -d >"${KEY_FILE}"
elif [ -n "${VPS_SSH_KEY:-}" ]; then
  printf '%s' "${VPS_SSH_KEY}" | sed 's/\r$//g' >"${KEY_FILE}"
fi

run_ssh() {
  local SSH_OPTS=(-o StrictHostKeyChecking=accept-new -p "${VPS_PORT}")
  if key_ok "${KEY_FILE}"; then
    ssh "${SSH_OPTS[@]}" -i "${KEY_FILE}" "${VPS_USER}@${VPS_HOST}" "$@"
  elif [ -n "${VPS_SSH_PASSWORD:-}" ]; then
    command -v sshpass >/dev/null || { echo "sshpass gerekli" >&2; exit 1; }
    sshpass -p "${VPS_SSH_PASSWORD}" ssh "${SSH_OPTS[@]}" "${VPS_USER}@${VPS_HOST}" "$@"
  else
    echo "VPS SSH kimligi yok" >&2
    exit 1
  fi
}

REMOTE_SCRIPT="$ROOT/scripts/vps-remote-vitrin-basic-auth.sh"
echo "==> Vitrin basic auth (${MODE}) → ${VPS_USER}@${VPS_HOST}"
run_ssh "bash -s" "$MODE" "$AUTH_USER" "${VITRIN_BASIC_AUTH_PASSWORD:-}" <"$REMOTE_SCRIPT"
