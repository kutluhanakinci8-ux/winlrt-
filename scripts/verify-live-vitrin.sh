#!/usr/bin/env bash
# Canlı vitrin smoke (deploy sonrası).
set -euo pipefail
BASE="${VITRIN_URL:-https://winlrt.com.tr}"

curl -fsSIL -o /dev/null "${BASE}/"
curl -fsSIL -o /dev/null "${BASE}/downloads/"
curl -fsSIL -o /dev/null "${BASE}/pilot/"
curl -fsSL "${BASE}/pilot/status.json" | grep -q '"active_wave": 2'
curl -fsSL "${BASE}/pilot/status.json" | grep -q '"blocker": "windows_explorer_qa"'
curl -fsSIL -o /dev/null "${BASE}/downloads/winlrtRAR-corporate-gpo.zip"
curl -fsSL "${BASE}/benchmark/samples/windows-balanced-kpi.json" | grep -q '"compression'
curl -fsSL "${BASE}/benchmark/samples/windows-fast-kpi.json" | grep -q speed_gate

echo "OK: canlı vitrin (${BASE})"
