#!/usr/bin/env bash
# Faz G1 — Zip P0×3 + W33 v1 kapı (repo tanımı).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

bash "$ROOT/scripts/verify-madde-w16.sh"
bash "$ROOT/scripts/verify-madde-w32.sh"
bash "$ROOT/scripts/verify-madde-w33.sh"
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-p0-gate.json" >/dev/null
jq -e '.all_pass == true' "$ROOT/apps/web/public/benchmark/samples/lertiva-zip-v1-gate.json" >/dev/null

if [[ -f "$ROOT/scripts/verify-madde-w34.sh" ]]; then
  bash "$ROOT/scripts/verify-madde-w34.sh" || echo "::warning::W34 macOS KPI henüz resmi değil (G3 devam)"
fi

echo "OK: Faz G1 — P0×3 + W33 v1 kapı (W34 opsiyonel uyarı)"
