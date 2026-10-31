#!/usr/bin/env bash
# Canlı vitrin smoke (deploy sonrası).
set -euo pipefail
BASE="${VITRIN_URL:-https://winlrt.com.tr}"

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
if [[ -f "$ROOT/.env.vps.local" ]]; then
  # shellcheck disable=SC1090
  set -a
  source "$ROOT/.env.vps.local"
  set +a
fi

CURL_AUTH=()
if [[ -n "${VITRIN_BASIC_AUTH_USER:-}" && -n "${VITRIN_BASIC_AUTH_PASSWORD:-}" ]]; then
  CURL_AUTH=(-u "${VITRIN_BASIC_AUTH_USER}:${VITRIN_BASIC_AUTH_PASSWORD}")
fi

curl -fsSIL "${CURL_AUTH[@]}" -o /dev/null "${BASE}/"
curl -fsSIL "${CURL_AUTH[@]}" -o /dev/null "${BASE}/downloads/"
curl -fsSIL "${CURL_AUTH[@]}" -o /dev/null "${BASE}/pilot/"
curl -fsSL "${CURL_AUTH[@]}" "${BASE}/pilot/status.json" | grep -q '"active_wave":'
curl -fsSL "${CURL_AUTH[@]}" "${BASE}/pilot/status.json" | grep -q '"blocker":'
curl -fsSIL "${CURL_AUTH[@]}" -o /dev/null "${BASE}/downloads/lertiva-arc-corporate-gpo.zip"
curl -fsSL "${CURL_AUTH[@]}" "${BASE}/benchmark/samples/windows-balanced-kpi.json" | grep -q '"compression'
curl -fsSL "${CURL_AUTH[@]}" "${BASE}/benchmark/samples/windows-fast-kpi.json" | grep -q speed_gate

echo "OK: canlı vitrin (${BASE})"
