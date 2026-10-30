#!/usr/bin/env bash
# Canlı vitrin smoke (deploy sonrası).
set -euo pipefail
BASE="${VITRIN_URL:-https://winlrt.com.tr}"

curl -fsSIL -o /dev/null "${BASE}/"
curl -fsSIL -o /dev/null "${BASE}/downloads/"
curl -fsSIL -o /dev/null "${BASE}/pilot/"
curl -fsSL "${BASE}/pilot/status.json" | grep -q '"active_wave":'
curl -fsSL "${BASE}/pilot/status.json" | grep -q '"blocker":'
curl -fsSIL -o /dev/null "${BASE}/downloads/lertiva-arc-corporate-gpo.zip"
curl -fsSL "${BASE}/benchmark/samples/windows-balanced-kpi.json" | grep -q '"compression'
curl -fsSL "${BASE}/benchmark/samples/windows-fast-kpi.json" | grep -q speed_gate

echo "OK: canlı vitrin (${BASE})"
